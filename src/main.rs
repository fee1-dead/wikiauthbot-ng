use serenity::all::{Builder, CreateMessage, GatewayIntents, Mentionable, UserId};
use serenity::client::{ClientBuilder, FullEvent};
use tracing::trace;
use wikiauthbot_common::Config;
use wikiauthbot_db::DatabaseConnection;

use crate::commands::whois::fetch_whois;

pub mod commands;
mod events;

mod data_private {
    use wikiauthbot_common::Config;
    use wikiauthbot_db::{DatabaseConnection, DatabaseConnectionInGuild};

    pub struct Data {
        // todo: we might want to support multiple CentralAuth instances
        pub client: mwapi::Client,
        pub db: DatabaseConnection,
        pub config: &'static Config,
    }

    impl Data {
        pub fn db_guild<'a>(&'a self, ctx: &poise::Context<Data, super::Error>) -> DatabaseConnectionInGuild<'a> {
            self.db.in_guild(ctx.guild_id().unwrap())
        }
    }
}


type Data = data_private::Data;

type Error = color_eyre::Report;
type Command = poise::Command<Data, Error>;
type Context<'a> = poise::Context<'a, Data, Error>;
type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    wikiauthbot_common::setup_common()?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(main_inner())
}

async fn event_handler(
    ctx: &serenity::all::Context,
    event: &FullEvent,
    _ftx: poise::FrameworkContext<'_, Data, Error>,
    u: &Data,
) -> Result {
    match event {
        FullEvent::GuildMemberAddition { new_member } => {
            let guild = new_member.guild_id;
            let db = u.db.in_guild(guild);
            trace!(?guild, "new member");
            if let Some(chan) = db.welcome_channel_id(guild.get()).await? {
                let mention = new_member.mention().to_string();
                
                let content = if let Ok(Some(whois)) =
                    db.whois(new_member.user.id.get()).await
                {
                    if let Ok(authenticated_role) = db.authenticated_role_id(guild.get()).await {
                        new_member.add_role(ctx, authenticated_role).await?;
                    }
                    match fetch_whois(&u.client, whois.wikimedia_id).await {
                        Ok(whois) => {
                            let name = whois.name;
                            let user_link = db.user_link(&name).await?;
                            wikiauthbot_db::msg!(db, "welcome_has_auth", mention = mention, name = name, user_link = user_link)?
                        }
                        _ => {
                            tracing::error!("failed to fetch whois!");
                            wikiauthbot_db::msg!(db, "welcome_has_auth_failed", mention = mention)?
                        }
                    }
                    
                } else {
                    wikiauthbot_db::msg!(db, "welcome", mention = mention)?
                };
                let msg = CreateMessage::new().content(content);
                msg.reactions(['👋'])
                    .execute(ctx, (chan.into(), Some(guild)))
                    .await?;
            }
        }
        FullEvent::Ready { .. } => {
            println!("discord bot is ready");
            events::init(ctx, u).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn bot_start() -> Result<()> {
    let config = Config::get()?;
    let db = DatabaseConnection::prod_vps().await?;
    let framework = poise::FrameworkBuilder::default()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                let data = Data {
                    client: wikiauthbot_common::mwclient().await?,
                    config,
                    db,
                };
                println!("data setup complete");
                Ok(data)
            })
        })
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            owners: config
                .bot_owners
                .iter()
                .copied()
                .map(UserId::from)
                .collect(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("wab!".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;
    let mut client = ClientBuilder::new(config.discord_bot_token.clone(), intents)
        .framework(framework)
        .await?;

    Ok(client.start().await?)
}

async fn main_inner() -> Result<()> {
    tokio::spawn(bot_start());

    tokio::signal::ctrl_c().await?;

    Ok(())
}
