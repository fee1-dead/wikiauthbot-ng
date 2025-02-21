use std::fmt::Display;
use std::sync::LazyLock;

use reqwest::Client;

use crate::Config;

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

#[derive(serde::Serialize)]
struct Message {
    content: String,
}

#[macro_export]
macro_rules! webhook_println {
    ($($tt:tt)*) => {
        $crate::webhook::send_message(format!($($tt)*))
    };
}

pub use webhook_println;

pub fn send_message(x: impl Display) {
    let Ok(Config { discord_webhook_url: Some(url), .. }) = crate::config::Config::get() else { return };

    let content = x.to_string();
    tokio::spawn(async move {
        if let Err(e) = CLIENT.post(url).json(&Message { content }).send().await {
            tracing::warn!(?e, "webhook post failed")
        }
    });
}