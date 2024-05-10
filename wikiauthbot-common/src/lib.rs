mod config;
pub mod i18n;

pub use config::Config;

mod auth;
pub use auth::{AuthRequest, SuccessfulAuth};

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

pub async fn mwclient() -> mwapi::Result<mwapi::Client> {
    mwapi::Client::builder("https://meta.wikimedia.org/w/api.php")
        .set_user_agent(concat!("wikiauthbot-ng/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .await
}


pub fn setup_common() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(LevelFilter::WARN.into())
                .add_directive(
                    "wikiauthbot_ng"
                        .parse()
                        .unwrap(),
                )
                .add_directive(
                    "wikiauthbot_server"
                        .parse()
                        .unwrap(),
                )
                .add_directive(
                    "wikiauthbot_db"
                        .parse()
                        .unwrap(),
                )
                .add_directive(
                    "wikiauthbot_common"
                        .parse()
                        .unwrap(),
                ),
        )
        .init();
    Ok(())
}