use std::fmt::Display;
use std::num::NonZeroU64;

use color_eyre::eyre::{ContextCompat, bail};

use crate::i18n::LanguageId;

struct AuthInfo {
    /// discord user id who initiated this auth request.
    discord_user_id: NonZeroU64,
    guild_id: NonZeroU64,
    lang: LanguageId,
}

pub struct AuthRequest {
    /// A random generated (anonymised) id for an auth request.
    id: [u8; 28],
    info: AuthInfo,
}

impl AuthRequest {
    pub fn new(discord_user_id: NonZeroU64, guild_id: NonZeroU64, lang: LanguageId) -> AuthRequest {
        AuthRequest {
            id: rand::random(),
            info: AuthInfo {
                discord_user_id,
                guild_id,
                lang,
            },
        }
    }

    pub fn from_redis(
        state: &str,
        discord_user_id: u64,
        guild_id: u64,
        lang: usize,
    ) -> color_eyre::Result<AuthRequest> {
        if state.len() != 28 * 2 {
            bail!("not a valid state string")
        }

        let lang = LanguageId::new(lang).context("valid language ID")?;

        let id = state
            .as_bytes()
            .chunks_exact(2)
            .map(|x| u8::from_str_radix(&String::from_utf8_lossy(x), 16))
            .collect::<Result<Vec<_>, _>>()?;

        let id = id.try_into().unwrap();

        Ok(AuthRequest {
            id,
            info: AuthInfo {
                discord_user_id: NonZeroU64::new(discord_user_id)
                    .context("discord_user_id null")?,
                guild_id: NonZeroU64::new(guild_id).context("guild_id null")?,
                lang,
            },
        })
    }

    pub fn state(&self) -> impl Display {
        struct HexFmt([u8; 28]);

        impl Display for HexFmt {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for n in self.0 {
                    write!(f, "{n:02x}")?
                }
                Ok(())
            }
        }

        HexFmt(self.id)
    }

    pub fn language(&self) -> LanguageId {
        self.info.lang
    }

    pub fn into_successful(self, central_user_id: u32, username: String) -> SuccessfulAuth {
        SuccessfulAuth {
            discord_user_id: self.info.discord_user_id,
            guild_id: self.info.guild_id,
            central_user_id,
            username,
            brand_new: true,
        }
    }
}

pub struct SuccessfulAuth {
    pub discord_user_id: NonZeroU64,
    pub guild_id: NonZeroU64,
    pub central_user_id: u32,
    pub username: String,
    pub brand_new: bool,
}
