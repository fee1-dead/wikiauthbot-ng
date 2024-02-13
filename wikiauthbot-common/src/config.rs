use std::fs;
use std::num::NonZeroU64;

#[derive(serde::Deserialize)]
pub struct PublicCfg {
    pub owners: Vec<NonZeroU64>,
}

impl PublicCfg {
    pub fn read() -> color_eyre::Result<PublicCfg> {
        Ok(toml::from_str(&fs::read_to_string("./config.toml")?)?)
    }
}

#[derive(serde::Deserialize)]
pub struct PrivateCfg {
    pub discord_bot_token: String,
}

impl PrivateCfg {
    pub fn read() -> color_eyre::Result<PrivateCfg> {
        Ok(toml::from_str(&fs::read_to_string("./config_secret.toml")?)?)
    }
}
