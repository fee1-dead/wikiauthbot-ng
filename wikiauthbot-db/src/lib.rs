use std::borrow::Cow;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::process::exit;
use std::time::{Duration, Instant};

use color_eyre::eyre::bail;
use dashmap::DashMap;
use fred::prelude::*;
use fred::types::DEFAULT_JITTER_MS;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow};
use sqlx::{QueryBuilder, Row, SqlitePool};
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
                inner join users on users.discord_id = auths.user_id
                where auths.user_id = $1
                    and auths.guild_id = $2
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
        let values = sqlx::query(
            "
            select users.discord_id from users
                inner join auths on users.discord_id = auths.user_id
                where users.wikimedia_id = $1 and auths.guild_id = $2
        ",
        )
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
        let lang: String = sqlx::query("select server_language from guilds where guild_id = $1")
            .bind(self.guild_id.get() as i64)
            .map(|x: SqliteRow| x.get(0))
            .fetch_one(&self.sqlite)
            .await?;

        self.lang_cache.insert(self.guild_id, lang.clone());
        Ok(lang)
    }

    pub async fn full_auth(&self, discord_id: u64, wikimedia_id: u32) -> color_eyre::Result<()> {
        let txn = self.sqlite.begin().await?;
        sqlx::query("INSERT INTO users VALUES($1, $2)")
            .bind(discord_id as i64)
            .bind(wikimedia_id)
            .execute(&self.sqlite)
            .await?;
        sqlx::query("INSERT INTO auths VALUES($1, $2)")
            .bind(self.guild_id.get() as i64)
            .bind(discord_id as i64)
            .execute(&self.sqlite)
            .await?;
        txn.commit().await?;
        Ok(())
    }

    /// Partially, used when we know what the user is authenticated already.
    pub async fn partial_auth(&self, discord_id: u64) -> color_eyre::Result<()> {
        let txn = self.sqlite.begin().await?;
        if !self.user_is_authed(discord_id).await? {
            txn.rollback().await?;
            bail!("user isn't authed anymore?");
        }
        sqlx::query("INSERT INTO auths VALUES($1, $2)")
            .bind(self.guild_id.get() as i64)
            .bind(discord_id as i64)
            .execute(&self.sqlite)
            .await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn set_server_settings(
        &self,
        ServerSettingsData {
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
        }: ServerSettingsData,
    ) -> color_eyre::Result<()> {
        let mut q = QueryBuilder::new("INSERT INTO guilds VALUES(");
        let mut separated = q.separated(", ");
        separated
            .push_bind(self.guild_id.get() as i64)
            .push_bind(welcome_channel_id as i64)
            .push_bind(auth_log_channel_id as i64)
            .push_bind(deauth_log_channel_id as i64)
            .push_bind(authenticated_role_id as i64)
            .push_bind(server_language)
            .push_bind(allow_banned_users);
        separated.push_unseparated(")");
        q.build().execute(&self.sqlite).await?;
        Ok(())
    }

    /// Delete the information from a single guild. Does not remove our record
    /// of them in the `users` table.
    pub async fn partial_deauth(&self, user_id: u64) -> color_eyre::Result<bool> {
        Ok(
            sqlx::query("delete from auths where user_id = $1 and guild_id = $2")
                .bind(user_id as i64)
                .bind(self.guild_id.get() as i64)
                .execute(&self.sqlite)
                .await?
                .rows_affected()
                != 0,
        )
    }

    pub async fn welcome_channel_id(&self) -> color_eyre::Result<Option<NonZeroU64>> {
        let n: Option<i64> =
            sqlx::query("select welcome_channel_id from guilds where guild_id = $1")
                .bind(self.guild_id.get() as i64)
                .fetch_optional(&self.sqlite)
                .await?
                .map(|r| r.get(0));
        Ok(n.map(|x| x as u64).and_then(NonZeroU64::new))
    }

    pub async fn auth_log_channel_id(&self) -> color_eyre::Result<u64> {
        let n: i64 = sqlx::query("select auth_log_channel_id from guilds where guild_id = $1")
            .bind(self.guild_id.get() as i64)
            .fetch_one(&self.sqlite)
            .await?
            .get(0);
        Ok(n as u64)
    }

    pub async fn authenticated_role_id(&self) -> color_eyre::Result<u64> {
        let n: i64 = sqlx::query("select authenticated_role_id from guilds where guild_id = $1")
            .bind(self.guild_id.get() as i64)
            .fetch_one(&self.sqlite)
            .await?
            .get(0);
        Ok(n as u64)
    }

    pub async fn has_server_settings(&self) -> color_eyre::Result<bool> {
        Ok(
            sqlx::query("select exists(select 1 from guilds where guild_id = $1)")
                .bind(self.guild_id.get() as i64)
                .fetch_one(&self.sqlite)
                .await?
                .try_get(0)?,
        )
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
    pub async fn connect_sqlite() -> color_eyre::Result<SqlitePool> {
        let options = SqliteConnectOptions::new()
            .filename("wikiauthbot-prod.db")
            .journal_mode(SqliteJournalMode::Wal);
        Ok(SqlitePoolOptions::new().max_connections(100).test_before_acquire(false).connect_with(options).await?)
    }
    pub async fn prod() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@redis");
        let client = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;
        let sqlite = Self::connect_sqlite().await?;
        Ok(Self {
            client,
            sqlite,
            lang_cache: DashMap::new(),
        })
    }

    /// Use a tunnel to the redis server, but use a local file for sqlite.
    /// You most certainly do not want to use this.
    pub async fn prod_tunnelled() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@127.0.0.1:16379");
        let client = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;
        let sqlite = Self::connect_sqlite().await?;
        Ok(Self {
            client,
            sqlite,
            lang_cache: DashMap::new(),
        })
    }

    pub fn into_parts(self) -> (RedisClient, SqlitePool) {
        (self.client, self.sqlite)
    }

    pub async fn dev() -> color_eyre::Result<Self> {
        let client = make_and_init_redis_client(RedisConfig::default()).await?;
        let sqlite = Self::connect_sqlite().await?;
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
                eprintln!("crashing due to error: {redis}");
                exit(1)
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

    pub async fn full_deauth(&self, discord_id: u64) -> color_eyre::Result<(u64, u64)> {
        let txn = self.sqlite.begin().await?;
        let a = sqlx::query("delete from auths where user_id = $1")
            .bind(discord_id as i64)
            .execute(&self.sqlite)
            .await?
            .rows_affected();
        let b = sqlx::query("delete from users where discord_id = $1")
            .bind(discord_id as i64)
            .execute(&self.sqlite)
            .await?
            .rows_affected();
        txn.commit().await?;
        Ok((a, b))
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

    pub async fn wmf_auth(&self, discord_id: u64, wikimedia_id: u32) -> color_eyre::Result<()> {
        sqlx::query("INSERT INTO users VALUES($1, $2)")
            .bind(discord_id as i64)
            .bind(wikimedia_id)
            .execute(&self.sqlite)
            .await?;
        Ok(())
    }
}
