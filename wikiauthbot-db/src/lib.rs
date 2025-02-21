use std::borrow::Cow;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::process::exit;
use std::time::{Duration, Instant};

use color_eyre::eyre::bail;
use dashmap::DashMap;
use fred::prelude::*;
use fred::types::DEFAULT_JITTER_MS;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{MySqlPool, QueryBuilder, Row};
use wikiauthbot_common::Config;

pub mod server;

#[derive(Clone)]
pub struct DatabaseConnection {
    redis: RedisClient,
    sql: MySqlPool,
    servers: DashMap<NonZeroU64, ServerSettingsData>,
}

#[derive(Clone)]
pub struct DatabaseConnectionInGuild<'a> {
    inner: &'a DatabaseConnection,
    guild_id: NonZeroU64,
    server_settings: Option<ServerSettingsData>,
}

#[macro_export]
macro_rules! msg {
    ($db:expr, $($rest:tt)+) => {
        {
            ::wikiauthbot_common::msg!($db.server_language(), $($rest)+)
        }
    };
}

impl<'a> DatabaseConnectionInGuild<'a> {
    pub fn guild_id(&self) -> NonZeroU64 {
        self.guild_id
    }

    pub fn server_settings(&self) -> &Option<ServerSettingsData> {
        &self.server_settings
    }

    pub async fn is_user_authed_in_server(&self, discord_id: u64) -> color_eyre::Result<bool> {
        Ok(
            sqlx::query("select exists(select 1 from auths where user_id = ? and guild_id = ?)")
                .bind(discord_id)
                .bind(self.guild_id.get())
                .fetch_one(&self.sql)
                .await?
                .try_get(0)?,
        )
    }

    pub async fn count_guilds_authed_to(&self, user_id: u64) -> color_eyre::Result<u64> {
        let cnt: i64 =
            sqlx::query("select count(guild_id) as num_guilds from auths where user_id = ?")
                .bind(user_id)
                .fetch_one(&self.sql)
                .await?
                .try_get(0)?;
        Ok(cnt as u64)
    }

    pub async fn get_user_authed_guilds(&self, user_id: u64) -> color_eyre::Result<Vec<u64>> {
        Ok(sqlx::query("select guild_id from auths where user_id = ?")
            .bind(user_id)
            .map(|row| row.get(0))
            .fetch_all(&self.sql)
            .await?)
    }

    pub fn get_message(&self, key: &str) -> color_eyre::Result<Cow<'static, str>> {
        let lang = self.server_language();
        wikiauthbot_common::i18n::get_message(lang, key)
    }

    pub fn user_link(&self, user_name: &str) -> color_eyre::Result<Cow<'static, str>> {
        let lang = self.server_language();
        let normalized_name = user_name.replace(' ', "+");
        wikiauthbot_common::msg!(lang, "user_link", normalized_name = normalized_name)
    }

    pub async fn whois(&self, discord_id: u64) -> color_eyre::Result<Option<WhoisResult>> {
        let value = sqlx::query(
            r"
            select users.wikimedia_id from auths
                inner join users on users.discord_id = auths.user_id
                where auths.user_id = ?
                    and auths.guild_id = ?
        ",
        )
        .bind(discord_id)
        .bind(self.guild_id.get())
        .fetch_optional(&self.sql)
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
                where users.wikimedia_id = ? and auths.guild_id = ?
        ",
        )
        .bind(wikimedia_id)
        .bind(self.guild_id.get())
        .map(|x: MySqlRow| x.get(0))
        .fetch_all(&self.sql)
        .await?;
        Ok(values)
    }

    pub fn server_language(&self) -> &str {
        &self.server_settings.as_ref().unwrap().server_language
    }

    pub async fn full_auth(&self, discord_id: u64, wikimedia_id: u32) -> color_eyre::Result<()> {
        let txn = self.sql.begin().await?;
        sqlx::query("INSERT INTO users VALUES(?, ?)")
            .bind(discord_id)
            .bind(wikimedia_id)
            .execute(&self.sql)
            .await?;
        sqlx::query("INSERT INTO auths VALUES(?, ?)")
            .bind(self.guild_id.get())
            .bind(discord_id)
            .execute(&self.sql)
            .await?;
        txn.commit().await?;
        Ok(())
    }

    /// Partially, used when we know what the user is authenticated already.
    pub async fn partial_auth(&self, discord_id: u64) -> color_eyre::Result<()> {
        let txn = self.sql.begin().await?;
        if !self.user_is_authed(discord_id).await? {
            txn.rollback().await?;
            bail!("user isn't authed anymore?");
        }
        sqlx::query("INSERT INTO auths VALUES(?, ?)")
            .bind(self.guild_id.get())
            .bind(discord_id)
            .execute(&self.sql)
            .await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn set_server_settings(
        &mut self,
        data: ServerSettingsData,
    ) -> color_eyre::Result<()> {
        assert!(self.server_settings.is_none());
        self.servers.insert(self.guild_id, data.clone());
        self.server_settings = Some(data.clone());
        let ServerSettingsData {
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
            whois_is_ephemeral,
            allow_partially_blocked_users,
        } = data;
        let mut q = QueryBuilder::new("INSERT INTO guilds VALUES(");
        let mut separated = q.separated(", ");
        separated
            .push_bind(self.guild_id.get())
            .push_bind(welcome_channel_id)
            .push_bind(auth_log_channel_id)
            .push_bind(deauth_log_channel_id)
            .push_bind(authenticated_role_id)
            .push_bind(server_language)
            .push_bind(allow_banned_users)
            .push_bind(whois_is_ephemeral)
            .push_bind(allow_partially_blocked_users);
        separated.push_unseparated(")");
        q.build().execute(&self.sql).await?;
        Ok(())
    }

    pub async fn update_server_settings(
        &mut self,
        update: impl FnOnce(ServerSettingsData) -> ServerSettingsData,
    ) -> color_eyre::Result<()> {
        let data = self.server_settings.take().unwrap();
        let data = update(data);
        self.server_settings = Some(data.clone());
        self.servers.insert(self.guild_id, data.clone());

        let ServerSettingsData {
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
            whois_is_ephemeral,
            allow_partially_blocked_users,
        } = data;

        sqlx::query(
            "update guilds
                set
                    welcome_channel_id = ?,
                    auth_log_channel_id = ?,
                    deauth_log_channel_id = ?,
                    authenticated_role_id = ?, 
                    server_language = ?,
                    allow_banned_users = ?,
                    whois_is_ephemeral = ?,
                    allow_partially_blocked_users = ?
                where guild_id = ?",
        )
        .bind(welcome_channel_id)
        .bind(auth_log_channel_id)
        .bind(deauth_log_channel_id)
        .bind(authenticated_role_id)
        .bind(server_language)
        .bind(allow_banned_users)
        .bind(whois_is_ephemeral)
        .bind(allow_partially_blocked_users)
        .bind(self.guild_id.get())
        .execute(&self.sql)
        .await?;

        Ok(())
    }

    /// Delete the information from a single guild. Does not remove our record
    /// of them in the `users` table.
    pub async fn partial_deauth(&self, user_id: u64) -> color_eyre::Result<bool> {
        Ok(
            sqlx::query("delete from auths where user_id = ? and guild_id = ?")
                .bind(user_id)
                .bind(self.guild_id.get())
                .execute(&self.sql)
                .await?
                .rows_affected()
                != 0,
        )
    }

    pub fn welcome_channel_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data| data.welcome_channel_id)
            .and_then(NonZeroU64::new)
    }

    pub fn auth_log_channel_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data: &ServerSettingsData| data.auth_log_channel_id)
            .and_then(NonZeroU64::new)
    }

    pub fn deauth_log_channel_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data: &ServerSettingsData| data.deauth_log_channel_id)
            .and_then(NonZeroU64::new)
    }

    pub fn authenticated_role_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data: &ServerSettingsData| data.authenticated_role_id)
            .and_then(NonZeroU64::new)
    }

    pub fn whois_is_ephemeral(&self) -> bool {
        self.server_settings.as_ref().unwrap().whois_is_ephemeral
    }

    pub fn disallow_blocked_users(&self) -> bool {
        !self.server_settings.as_ref().unwrap().allow_banned_users
    }

    pub fn has_server_settings(&self) -> bool {
        self.server_settings.is_some()
    }
}

impl Deref for DatabaseConnectionInGuild<'_> {
    type Target = DatabaseConnection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ChildDatabaseConnection {
    redis: RedisClient,
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
    async fn new(redis: RedisClient, sql: MySqlPool) -> color_eyre::Result<Self> {
        let servers = Self::load_server_settings(&sql).await?;
        sqlx::migrate!("./src/migrations").run(&sql).await?;
        Ok(Self {
            redis,
            sql,
            servers,
        })
    }
    async fn load_server_settings(
        sql: &MySqlPool,
    ) -> color_eyre::Result<DashMap<NonZeroU64, ServerSettingsData>> {
        let all_settings = sqlx::query(
            "select
            guild_id,
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
            whois_is_ephemeral,
            allow_partially_blocked_users,
        from guilds",
        )
        .fetch_all(sql)
        .await?;

        let map = DashMap::new();
        for row in all_settings {
            let id: u64 = row.get("guild_id");
            let guild_id = NonZeroU64::new(id).unwrap();
            macro_rules! fetch {
                ($($name:ident),*$(,)?) => {
                    $(let $name = row.get(stringify!($name));)*
                };
            }
            fetch!(
                welcome_channel_id,
                auth_log_channel_id,
                deauth_log_channel_id,
                authenticated_role_id,
                server_language,
                allow_banned_users,
                whois_is_ephemeral,
                allow_partially_blocked_users,
            );

            let data = ServerSettingsData {
                welcome_channel_id,
                auth_log_channel_id,
                deauth_log_channel_id,
                authenticated_role_id,
                server_language,
                allow_banned_users,
                whois_is_ephemeral,
                allow_partially_blocked_users,
            };
            map.insert(guild_id, data);
        }

        Ok(map)
    }

    pub async fn connect_mysql() -> color_eyre::Result<MySqlPool> {
        Ok(MySqlPoolOptions::new()
            .connect(&Config::get()?.sql_url)
            .await?)
    }
    pub async fn prod() -> color_eyre::Result<Self> {
        let cfg = Config::get()?;
        let password = &cfg.redis_password;
        let url = format!("redis://:{password}@redis");
        let redis = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;
        Self::new(redis, Self::connect_mysql().await?).await
    }

    /// Use a tunnel to the redis server.
    pub async fn prod_tunnelled() -> color_eyre::Result<Self> {
        let cfg = Config::get()?;
        let password = &cfg.redis_password;
        let url = format!("redis://:{password}@127.0.0.1:16379");
        let redis = make_and_init_redis_client(try_redis(RedisConfig::from_url(&url))?).await?;

        Self::new(redis, Self::connect_mysql().await?).await
    }

    pub fn into_parts(self) -> (RedisClient, MySqlPool) {
        (self.redis, self.sql)
    }

    pub async fn get_child(&self) -> RedisResult<ChildDatabaseConnection> {
        let redis = self.redis.clone_new();
        try_redis(redis.init().await)?;
        Ok(ChildDatabaseConnection { redis })
    }

    pub async fn ping(&self) -> color_eyre::Result<Duration> {
        let instant = Instant::now();
        let _ = sqlx::query("select 1 from auth where user_id = 468253584421552139")
            .fetch_one(&self.sql)
            .await?;
        Ok(instant.elapsed())
    }
}

#[derive(Debug)]
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
    pub whois_is_ephemeral: bool,
    pub allow_partially_blocked_users: bool,
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
        let row = sqlx::query("select exists(select 1 from users where discord_id = ?)")
            .bind(discord_id)
            .fetch_one(&self.sql)
            .await?;
        Ok(row.try_get(0)?)
    }

    pub async fn full_deauth(&self, discord_id: u64) -> color_eyre::Result<(u64, u64)> {
        let txn = self.sql.begin().await?;
        let a = sqlx::query("delete from auths where user_id = ?")
            .bind(discord_id)
            .execute(&self.sql)
            .await?
            .rows_affected();
        let b = sqlx::query("delete from users where discord_id = ?")
            .bind(discord_id)
            .execute(&self.sql)
            .await?
            .rows_affected();
        txn.commit().await?;
        Ok((a, b))
    }

    pub async fn get_wikimedia_id(&self, discord_id: u64) -> color_eyre::Result<Option<u32>> {
        let row = sqlx::query("select wikimedia_id from users where discord_id = ?")
            .bind(discord_id)
            .fetch_optional(&self.sql)
            .await?;
        Ok(row.map(|r| r.get(0)))
    }

    pub fn in_guild(&self, guild_id: impl Into<NonZeroU64>) -> DatabaseConnectionInGuild<'_> {
        let guild_id = guild_id.into();
        DatabaseConnectionInGuild {
            inner: self,
            guild_id,
            server_settings: self.servers.get(&guild_id).map(|x| x.value().clone()),
        }
    }

    pub async fn wmf_auth(&self, discord_id: u64, wikimedia_id: u32) -> color_eyre::Result<()> {
        sqlx::query("INSERT INTO users VALUES(?, ?)")
            .bind(discord_id)
            .bind(wikimedia_id)
            .execute(&self.sql)
            .await?;
        Ok(())
    }
}
