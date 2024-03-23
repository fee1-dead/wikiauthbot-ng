mod config;
pub use config::Config;

mod auth;
pub use auth::{AuthRequest, SuccessfulAuth};

pub async fn mwclient() -> mwapi::Result<mwapi::Client> {
    mwapi::Client::builder("https://meta.wikimedia.org/w/api.php")
        .set_user_agent(concat!("wikiauthbot-ng/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .await
}
