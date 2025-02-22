use serenity::all::{GatewayIntents, UserId};
use serenity::client::ClientBuilder;
use wikiauthbot_common::{webhook_println, Config};
use wikiauthbot_db::DatabaseConnection;

pub mod commands;
mod events;
mod integrity;

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
        pub fn db_guild<'a>(
            &'a self,
            ctx: &poise::Context<Data, super::Error>,
        ) -> DatabaseConnectionInGuild<'a> {
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

async fn bot_start() -> Result<()> {
    let config = Config::get()?;
    let db = DatabaseConnection::prod().await?;
    let framework = poise::FrameworkBuilder::default()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                let data = Data {
                    client: wikiauthbot_common::mwclient().await?,
                    config,
                    db,
                };
                eprintln!("data setup complete");
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
            on_error: |error| Box::pin(events::on_error(error)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::event_handler(ctx, event, framework, data))
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
    webhook_println!("I'm very early in the starting process :3");
    tokio::spawn(bot_start());

    tokio::signal::ctrl_c().await?;

    Ok(())
}
