use std::fs;
use std::num::NonZeroU64;

use serenity::all::{GatewayIntents, UserId};
use serenity::client::ClientBuilder;
use tracing_subscriber::EnvFilter;

mod commands;
mod logging;

pub struct Data {
    client: reqwest::Client,
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

async fn main_inner() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let PublicCfg { owners } = toml::from_str(&fs::read_to_string("./bot_config.toml")?)?;
    let PrivateCfg { token } = toml::from_str(&fs::read_to_string("./bot_config_secret.toml")?)?;

    let framework = poise::FrameworkBuilder::default()
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async {
                Ok(Data {
                    client: reqwest::ClientBuilder::new().build()?,
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            owners: owners.into_iter().map(UserId::from).collect(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client?.start().await?;
    Ok(())
}
