use std::fs;
use std::num::NonZeroU64;
use std::sync::OnceLock;

use color_eyre::Result;

#[derive(serde::Deserialize)]
struct PublicCfg {
    bot_owners: Vec<NonZeroU64>,
    oauth_consumer_key: Box<str>,
}

impl PublicCfg {
    fn read() -> color_eyre::Result<PublicCfg> {
        Ok(toml::from_str(&fs::read_to_string("./config.toml")?)?)
    }
}

#[derive(serde::Deserialize)]
struct PrivateCfg {
    discord_bot_token: String,
    oauth_client_secret: Box<str>,
}

impl PrivateCfg {
    fn read() -> Result<PrivateCfg> {
        Ok(toml::from_str(&fs::read_to_string(
            "./config_secret.toml",
        )?)?)
    }
}

pub struct Config {
    pub bot_owners: Vec<NonZeroU64>,
    pub discord_bot_token: String,
    pub oauth_consumer_key: Box<str>,
    pub oauth_client_secret: Box<str>,
}

impl Config {
    pub fn get() -> Result<&'static Config> {
        static CFG: OnceLock<Config> = OnceLock::new();

        match CFG.get() {
            Some(x) => Ok(x),
            None => {
                // note that if multiple threads try to do this the same time, we'd just be reading the same file
                // multiple times, which is fine, because we can do redundant work without doing data race
                // which is bad >:(
                let PublicCfg {
                    bot_owners,
                    oauth_consumer_key,
                } = PublicCfg::read()?;
                let PrivateCfg {
                    discord_bot_token,
                    oauth_client_secret,
                } = PrivateCfg::read()?;
                let cfg = Config {
                    bot_owners,
                    discord_bot_token,
                    oauth_consumer_key,
                    oauth_client_secret,
                };
                Ok(CFG.get_or_init(move || cfg))
            }
        }
    }
}
