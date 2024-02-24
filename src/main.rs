use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use serenity::all::{Builder, CreateMessage, GatewayIntents, GuildId, Mention, RoleId, UserId};
use serenity::client::{ClientBuilder, FullEvent};
use tracing::error;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing_subscriber::EnvFilter;
use wikiauthbot_common::{AuthRequest, Config, SuccessfulAuth};
use wikiauthbot_db::{DatabaseConnection, ServerSettingsData};

mod commands;
mod logging;

pub struct Data {
    // todo: we might want to support multiple CentralAuth instances
    client: mwapi::Client,
    db: DatabaseConnection,
    ongoing_auth_requests: Arc<DashMap<UserId, String>>,
    new_auth_reqs_send: Sender<AuthRequest>,
    config: &'static Config,
    successful_auths_send: Arc<Sender<SuccessfulAuth>>,
    // this stream gets stolen as soon as serenity starts.
    successful_auths_recv: Mutex<Option<Receiver<SuccessfulAuth>>>,
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
        FullEvent::Ready { .. } => {
            println!("discord bot is ready");
            let receiver = u.successful_auths_recv.lock().unwrap().take();
            // TODO change this to use redis's list
            if let Some(mut successful_auths_recv) = receiver {
                let ongoing_auth_requests = u.ongoing_auth_requests.clone();
                let db = u.db.clone();
                let http = ctx.http.clone();
                spawn(async move {
                    while let Some(successful_auth) = successful_auths_recv.recv().await {
                        let wmf_id = successful_auth.central_user_id;
                        let username = successful_auth.username;
                        let discord_user_id: UserId = NonZeroU64::into(successful_auth.discord_user_id);
                        let guild: GuildId = NonZeroU64::into(successful_auth.guild_id);

                        if let Err(e) = db.full_auth(discord_user_id.get(), wmf_id, guild.get()).await {
                            tracing::error!(%e, "failed to insert authenticated!");
                            continue;
                        }

                        ongoing_auth_requests.remove(&discord_user_id);

                        let Ok(authenticated_role_id) = db.authenticated_role_id(guild.get()).await else {
                            tracing::error!("failed to get information for server: auth role channel");
                            continue;
                        };

                        let Ok(auth_log_channel_id) = db.auth_log_channel_id(guild.get()).await else {
                            tracing::error!("failed to get information for server: auth log channel");
                            continue;
                        };

                        if authenticated_role_id != 0 {
                            if let Err(e) = http
                                .add_member_role(
                                    guild,
                                    discord_user_id,
                                    RoleId::from(authenticated_role_id),
                                    Some(&format!("authenticated as wikimedia user {wmf_id}")),
                                )
                                .await
                            {
                                eprintln!("failed to add member role to {discord_user_id} in guild {guild}: {e}");
                            }
                        }

                        if auth_log_channel_id != 0 {
                            let mention = Mention::User(discord_user_id);
                            if let Err(e) = CreateMessage::new()
                                .content(format!("{mention} authenticated as [[User:{username}]]"))
                                .execute(&http, (auth_log_channel_id.into(), Some(guild)))
                                .await
                            {
                                error!(
                                    "failed to send message to channel {auth_log_channel_id} in guild {guild}: {e}"
                                );
                            }
                        }
                    }
                });
            }
        }
        _ => {}
    }
    Ok(())
}

async fn bot_start(
    new_auth_reqs_send: Sender<AuthRequest>,
    successful_auths_send: Arc<Sender<SuccessfulAuth>>,
    successful_auths_recv: Receiver<SuccessfulAuth>,
) -> Result<()> {
    let config = Config::get()?;
    let framework = poise::FrameworkBuilder::default()
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async {
                let db = DatabaseConnection::dev().await?;
                let data = Data {
                    client: mwapi::Client::builder("https://meta.wikimedia.org/w/api.php")
                        .set_user_agent(concat!("wikiauthbot-ng/{}", env!("CARGO_PKG_VERSION")))
                        .build()
                        .await?,
                    config,
                    db,
                    new_auth_reqs_send,
                    ongoing_auth_requests: Arc::default(),
                    successful_auths_send,
                    successful_auths_recv: Mutex::new(Some(successful_auths_recv)),
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

    let (new_auth_reqs_send, new_auth_reqs_recv) = tokio::sync::mpsc::channel(10);
    let (successful_auths_send, successful_auths_recv) = tokio::sync::mpsc::channel(10);

    let successful_auths_send = Arc::new(successful_auths_send);
    tokio::spawn(bot_start(
        new_auth_reqs_send,
        successful_auths_send.clone(),
        successful_auths_recv,
    ));
    tokio::spawn(async {
        wikiauthbot_server::start(new_auth_reqs_recv, successful_auths_send)
            .await?
            .await?;
        Result::<_, Error>::Ok(())
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
