use serenity::all::{Builder, CreateMessage, GatewayIntents, Mentionable, UserId};
use serenity::client::{ClientBuilder, FullEvent};
use tracing::trace;
use tracing_subscriber::EnvFilter;
use wikiauthbot_common::Config;
use wikiauthbot_db::DatabaseConnection;

mod commands;
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
    color_eyre::install()?;
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
                // TODO link the application command through something like "</auth:1025443470388764714>"
                CreateMessage::new()
                    .content(format!("Welcome {mention}! If you would like to authenticate (validate) your Wikimedia account, please type /auth"))
                    .reactions(['👋'])
                    .execute(ctx, (chan.into(), Some(guild))).await?;
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
    let framework = poise::FrameworkBuilder::default()
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async {
                let db = DatabaseConnection::prod().await?;
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
                prefix: Some("~".into()),
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
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tokio::spawn(bot_start());

    tokio::signal::ctrl_c().await?;

    Ok(())
}
