use std::fs;
use std::num::NonZeroU64;

use dashmap::DashMap;
use serenity::all::{GatewayIntents, UserId, GuildId};
use serenity::client::{ClientBuilder, FullEvent};
use tracing_subscriber::EnvFilter;
use wikiauthbot_db::{Database, DatabaseConnection, ServerSettingsData};

mod commands;
mod logging;

pub struct Data {
    // todo: we might want to support multiple CentralAuth instances
    client: mwapi::Client,
    db: DatabaseConnection,
    server_settings: DashMap<GuildId, ServerSettingsData>,
}

type Error = color_eyre::Report;
type Command = poise::Command<Data, Error>;
type Context<'a> = poise::Context<'a, Data, Error>;
type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(serde::Deserialize)]
struct PublicCfg {
    owners: Vec<NonZeroU64>,
}

#[derive(serde::Deserialize)]
struct PrivateCfg {
    token: String,
}

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
    ftx: poise::FrameworkContext<'_, Data, Error>,
    u: &Data,
) -> Result {
    Ok(())
}

async fn bot_start() -> Result<()> {
    let PublicCfg { owners } = toml::from_str(&fs::read_to_string("./bot_config.toml")?)?;
    let PrivateCfg { token } = toml::from_str(&fs::read_to_string("./bot_config_secret.toml")?)?;

    let framework = poise::FrameworkBuilder::default()
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async {
                let db = Database::connect().await?;
                let settings = db.get_all_server_settings().await?;
                let data = Data {
                    client: mwapi::Client::builder("https://en.wikipedia.org/w/api.php")
                        .set_user_agent(concat!("wikiauthbot-ng/{}", env!("CARGO_PKG_VERSION")))
                        .build()
                        .await?,
                    db,
                    server_settings: settings.map(|(guild_id, data)| (GuildId::new(guild_id), data)).collect(),
                };
                println!("data setup complete");
                Ok(data)
            })
        })
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            owners: owners.into_iter().map(UserId::from).collect(),
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
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    Ok(client?.start().await?)
}

async fn main_inner() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();


    let (new_auth_reqs_send, new_auth_reqs_recv) = tokio::sync::mpsc::channel(10);
    let (successful_auths_send, successful_auths_recv) = tokio::sync::mpsc::channel(10);

    let h1 = tokio::spawn(bot_start());
    let h2 = tokio::spawn(wikiauthbot_server::start(new_auth_reqs_recv, successful_auths_send));

    
    Ok(())
}
