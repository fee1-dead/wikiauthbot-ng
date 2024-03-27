use serenity::all::{Builder, CreateMessage, GatewayIntents, Mentionable, UserId};
use serenity::client::{ClientBuilder, FullEvent};
use tracing::trace;
use wikiauthbot_common::Config;
use wikiauthbot_db::DatabaseConnection;

use crate::commands::whois::{fetch_whois, user_link};

pub mod commands;
mod events;

pub struct Data {
    // todo: we might want to support multiple CentralAuth instances
    client: mwapi::Client,
    db: DatabaseConnection,
    config: &'static Config,
}

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
            trace!(?guild, "new member");
            if let Some(chan) = u.db.welcome_channel_id(guild.get()).await? {
                let mention = new_member.mention();
                let msg = if let Ok(Some(whois)) =
                    u.db.whois(new_member.user.id.get(), guild.get()).await
                {
                    match (
                        fetch_whois(&u.client, whois.wikimedia_id).await,
                        u.db.server_language(guild.get()).await,
                    ) {
                        (Ok(whois), Ok(lang)) => {
                            let name = whois.name;
                            let user_link = user_link(&name, &lang);
                            CreateMessage::new().content(format!("Welcome {mention}! You've already authenticated as [{name}](<{user_link}>), so you don't need to authenticate again."))
                        }
                        _ => {
                            tracing::error!("failed to fetch whois!");
                            CreateMessage::new().content(format!("Welcome {mention}! You've already authenticated (error while trying to fetch info), so you don't need to authenticate again."))
                        }
                    }
                } else {
                    CreateMessage::new()
                    .content(format!("Welcome {mention}! If you would like to authenticate (validate) your Wikimedia account, please type </auth:1221128504410898571>"))
                };
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
    let db = DatabaseConnection::prod().await?;
    db.keepalive();
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
