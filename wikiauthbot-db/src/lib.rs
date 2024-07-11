use std::borrow::Cow;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use fred::prelude::*;
use fred::types::{Scanner as _, DEFAULT_JITTER_MS};
use futures::TryStreamExt as _;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqlitePool};
use wikiauthbot_common::Config;

pub mod server;

#[derive(Clone)]
pub struct DatabaseConnection {
    client: RedisClient,
    sqlite: SqlitePool,
    lang_cache: DashMap<NonZeroU64, String>,
}

#[derive(Clone, Copy)]
pub struct DatabaseConnectionInGuild<'a> {
    inner: &'a DatabaseConnection,
    guild_id: NonZeroU64,
}

#[macro_export]
macro_rules! msg {
    ($db:expr, $($rest:tt)+) => {
        {
            $db.server_language().await.map_err(|e| e.into()).and_then(|lang| ::wikiauthbot_common::msg!(&lang, $($rest)+))
        }
    };
}

impl<'a> DatabaseConnectionInGuild<'a> {
    pub fn guild_id(&self) -> NonZeroU64 {
        self.guild_id
    }

    pub async fn is_user_authed_in_server(&self, discord_id: u64) -> color_eyre::Result<bool> {
        Ok(
            sqlx::query("select exists(select 1 from auths where user_id = $1 and guild_id = $2)")
                .bind(discord_id as i64)
                .bind(self.guild_id.get() as i64)
                .fetch_one(&self.sqlite)
                .await?
                .try_get(0)?,
        )
    }

    pub async fn get_message(&self, key: &str) -> color_eyre::Result<Cow<'static, str>> {
        let lang = self.server_language().await?;
        wikiauthbot_common::i18n::get_message(&lang, key)
    }

    pub async fn user_link(&self, user_name: &str) -> color_eyre::Result<Cow<'static, str>> {
        let lang = self.server_language().await?;
        let normalized_name = user_name.replace(' ', "+");
        wikiauthbot_common::msg!(&lang, "user_link", normalized_name = normalized_name)
    }

    pub async fn whois(&self, discord_id: u64) -> color_eyre::Result<Option<WhoisResult>> {
        let value = sqlx::query(
            r"
            select users.wikimedia_id from auths
                where auths.user_id = $1
                    and auths.guild_id = $2
                inner join users on users.discord_id = auths.user_id
        ",
        )
        .bind(discord_id as i64)
        .bind(self.guild_id.get() as i64)
        .fetch_optional(&self.sqlite)
        .await?
        .map(|row| WhoisResult {
            wikimedia_id: row.get(0),
        });

        Ok(value)
    }

    pub async fn revwhois(&self, wikimedia_id: u32) -> color_eyre::Result<Vec<u64>> {
        let values = sqlx::query("
            select users.discord_id from users
                where users.wikimedia_id = $1
                inner join auths on users.discord_id = auths.discord_id
                where auths.guild_id = $2
        ")
            .bind(wikimedia_id)
            .bind(self.guild_id.get() as i64)
            .map(|x: SqliteRow| x.get::<i64, _>(0) as u64)
            .fetch_all(&self.sqlite)
            .await?;
        Ok(values)
    }

    // TODO the hashmap should be more ergonomic
    pub async fn server_language(&self) -> color_eyre::Result<String> {
        if let Some(lang) = self.lang_cache.get(&self.guild_id) {
            return Ok(lang.clone());
        }
        let guild_id = self.guild_id;
        let lang: String = try_redis(
            self.client
                .get(format!("guilds:{guild_id}:server_language"))
                .await,
        )?;

        self.lang_cache.insert(guild_id, lang.clone());
        Ok(lang)
    }
}

impl Deref for DatabaseConnectionInGuild<'_> {
    type Target = DatabaseConnection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ChildDatabaseConnection {
    client: RedisClient,
}

async fn make_and_init_redis_client(config: RedisConfig) -> RedisResult<RedisClient> {
    let mut builder = Builder::from_config(config);
    builder.set_policy(ReconnectPolicy::Constant {
        attempts: 0,
        max_attempts: 10,
        delay: 1000,
        jitter: DEFAULT_JITTER_MS,
    });
    let mut conn_config = ConnectionConfig::default();
    conn_config.unresponsive.max_timeout = Some(Duration::from_secs(10));
    builder.set_connection_config(conn_config);
    let client = try_redis(builder.build())?;
    try_redis(client.init().await)?;
    Ok(client)
}

impl DatabaseConnection {
    pub async fn prod() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@redis.discordbots.eqiad1.wikimedia.cloud:6379");
        let client = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;
        let sqlite = SqlitePool::connect("sqlite:wikiauthbot-prod.db").await?;
        Ok(Self {
            client,
            sqlite,
            lang_cache: DashMap::new(),
        })
    }

    /* pub async fn prod_tunnelled() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@127.0.0.1:16379");
        let client = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;
        Ok(Self { client, lang_cache: DashMap::new() })
    } */

    pub async fn prod_vps() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@127.0.0.1:6379");
        let client = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;
        let sqlite = SqlitePool::connect("sqlite:wikiauthbot-prod.db").await?;
        Ok(Self {
            client,
            sqlite,
            lang_cache: DashMap::new(),
        })
    }

    pub async fn dev() -> color_eyre::Result<Self> {
        let client = make_and_init_redis_client(RedisConfig::default()).await?;
        let sqlite = SqlitePool::connect("sqlite:wikiauthbot-prod.db").await?;
        Ok(Self {
            client,
            sqlite,
            lang_cache: DashMap::new(),
        })
    }

    pub async fn get_child(&self) -> RedisResult<ChildDatabaseConnection> {
        let client = self.client.clone_new();
        try_redis(client.init().await)?;
        Ok(ChildDatabaseConnection { client })
    }

    pub async fn ping(&self) -> color_eyre::Result<Duration> {
        let instant = Instant::now();
        let _ = sqlx::query("select 1 from auth where user_id = 468253584421552139")
            .fetch_one(&self.sqlite)
            .await?;
        Ok(instant.elapsed())
    }
}

pub struct WhoisResult {
    pub wikimedia_id: u32,
}

#[derive(Clone)]
pub struct ServerSettingsData {
    pub welcome_channel_id: u64,
    pub auth_log_channel_id: u64,
    pub deauth_log_channel_id: u64,
    pub authenticated_role_id: u64,
    pub server_language: String,
    pub allow_banned_users: bool,
}

fn try_redis<T>(x: RedisResult<T>) -> RedisResult<T> {
    match x {
        Ok(x) => Ok(x),
        Err(redis) => match redis.kind() {
            RedisErrorKind::IO
            | RedisErrorKind::Timeout
            | RedisErrorKind::Canceled
            | RedisErrorKind::Unknown => {
                panic!("crashing due to error: {redis}");
            }
            _ => Err(redis),
        },
    }
}

impl DatabaseConnection {
    pub async fn user_is_authed(&self, discord_id: u64) -> color_eyre::Result<bool> {
        let row = sqlx::query("select exists(select 1 from users where discord_id = $1)")
            .bind(discord_id as i64)
            .fetch_one(&self.sqlite)
            .await?;
        Ok(row.try_get(0)?)
    }

    pub async fn get_wikimedia_id(&self, discord_id: u64) -> color_eyre::Result<Option<u32>> {
        let row = sqlx::query("select wikimedia_id from users where discord_id = $1")
            .bind(discord_id as i64)
            .fetch_optional(&self.sqlite)
            .await?;
        Ok(row.map(|r| r.get(0)))
    }

    pub fn in_guild(&self, guild_id: impl Into<NonZeroU64>) -> DatabaseConnectionInGuild<'_> {
        DatabaseConnectionInGuild {
            inner: self,
            guild_id: guild_id.into(),
        }
    }

    // this is not clean since revauth2 is not deleted.
    pub async fn debug_deauth(&self, user_id: u64, guild_id: u64) -> RedisResult<()> {
        let txn = self.client.pipeline();
        txn.srem(format!("guilds:{guild_id}:authed"), user_id)
            .await?;
        txn.del(format!("auth:{user_id}")).await?;
        txn.all().await?;
        Ok(())
    }

    pub async fn full_auth(
        &self,
        discord_id: u64,
        wikimedia_id: u32,
        guild_id: u64,
    ) -> RedisResult<()> {
        let txn = self.client.multi();
        try_redis(
            txn.set(
                format!("auth:{discord_id}"),
                wikimedia_id,
                None,
                Some(SetOptions::NX),
                false,
            )
            .await,
        )?;
        try_redis(
            txn.sadd(format!("revauth2:{wikimedia_id}"), discord_id)
                .await,
        )?;
        try_redis(
            txn.sadd(format!("guilds:{guild_id}:authed"), discord_id)
                .await,
        )?;
        try_redis(txn.exec(true).await)
    }

    pub async fn wmf_auth(&self, discord_id: u64, wikimedia_id: u32) -> RedisResult<()> {
        let txn = self.client.multi();
        try_redis(
            txn.sadd(format!("revauth2:{wikimedia_id}"), discord_id)
                .await,
        )?;
        try_redis(
            txn.set(
                format!("auth:{discord_id}"),
                wikimedia_id,
                None,
                Some(SetOptions::NX),
                false,
            )
            .await,
        )?;
        try_redis(txn.exec(true).await)
    }

    /// Partially, used when we know what the user is authenticated already.
    pub async fn partial_auth(&self, discord_id: u64, guild_id: u64) -> RedisResult<()> {
        // TODO do a sanity check
        try_redis(
            self.client
                .sadd(format!("guilds:{guild_id}:authed"), discord_id)
                .await,
        )
    }

    pub async fn welcome_channel_id(&self, guild_id: u64) -> RedisResult<Option<NonZeroU64>> {
        let n: Option<u64> = try_redis(
            self.client
                .get(format!("guilds:{guild_id}:welcome_channel_id"))
                .await,
        )?;
        Ok(n.and_then(NonZeroU64::new))
    }

    pub async fn auth_log_channel_id(&self, guild_id: u64) -> RedisResult<u64> {
        try_redis(
            self.client
                .get(format!("guilds:{guild_id}:auth_log_channel_id"))
                .await,
        )
    }

    pub async fn authenticated_role_id(&self, guild_id: u64) -> RedisResult<u64> {
        try_redis(
            self.client
                .get(format!("guilds:{guild_id}:authenticated_role_id"))
                .await,
        )
    }

    pub async fn has_server_settings(&self, guild_id: u64) -> RedisResult<bool> {
        try_redis(
            self.client
                .exists(format!("guilds:{guild_id}:server_language"))
                .await,
        )
    }

    pub async fn set_server_settings(
        &self,
        guild_id: u64,
        ServerSettingsData {
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
        }: ServerSettingsData,
    ) -> RedisResult<()> {
        let pipeline = self.client.pipeline();
        let key = |subkey| format!("guilds:{guild_id}:{subkey}");
        try_redis(
            pipeline
                .set(
                    key("welcome_channel_id"),
                    welcome_channel_id,
                    None,
                    None,
                    false,
                )
                .await,
        )?;
        try_redis(
            pipeline
                .set(
                    key("auth_log_channel_id"),
                    auth_log_channel_id,
                    None,
                    None,
                    false,
                )
                .await,
        )?;
        try_redis(
            pipeline
                .set(
                    key("deauth_log_channel_id"),
                    deauth_log_channel_id,
                    None,
                    None,
                    false,
                )
                .await,
        )?;
        try_redis(
            pipeline
                .set(
                    key("authenticated_role_id"),
                    authenticated_role_id,
                    None,
                    None,
                    false,
                )
                .await,
        )?;
        try_redis(
            pipeline
                .set(key("server_language"), server_language, None, None, false)
                .await,
        )?;
        try_redis(
            pipeline
                .set(
                    key("allow_banned_users"),
                    allow_banned_users,
                    None,
                    None,
                    false,
                )
                .await,
        )?;
        try_redis(pipeline.all().await)?;
        Ok(())
    }

    pub async fn build_revauth(&self) -> RedisResult<()> {
        try_redis(
            self.client
                .scan("auth:*", Some(100), None)
                .try_for_each_concurrent(None, |mut result| async move {
                    let client = result.create_client();
                    let results = result.take_results().unwrap();
                    let wikimedia_ids: Vec<u32> = try_redis(client.mget(results.clone()).await)?;
                    let pipeline = client.pipeline();
                    for (key, wikimedia_id) in results.into_iter().zip(wikimedia_ids) {
                        let discord_id = key
                            .as_str_lossy()
                            .strip_prefix("auth:")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap();
                        try_redis(
                            pipeline
                                .sadd(format!("revauth2:{wikimedia_id}"), discord_id)
                                .await,
                        )?;
                    }
                    try_redis(pipeline.all().await)?;
                    try_redis(result.next())?;
                    Ok(())
                })
                .await,
        )
    }
}
