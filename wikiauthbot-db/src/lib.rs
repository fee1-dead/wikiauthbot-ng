use std::borrow::Cow;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use color_eyre::eyre::{ContextCompat, bail};
use dashmap::DashMap;
use fred::prelude::*;
use fred::types::DEFAULT_JITTER_MS;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{MySqlPool, QueryBuilder, Row};
use wikiauthbot_common::i18n::{AtomicLanguageId, LanguageId};
use wikiauthbot_common::{BlockKind, Config};

pub mod server;

#[derive(Clone)]
pub struct RoleRule {
    /// e.g. `zhwiki`, or `*` to take in any wiki/global group
    pub wiki: String,
    /// e.g. `autoconfirmed`
    pub group_name: String,
    /// non-empty and in the form of `"https://zh.wikipedia.org"`` if needs separate query pending https://phabricator.wikimedia.org/T387029
    pub implicit_api_url: String,
    pub role_id: NonZeroU64,
}

#[derive(Clone)]
pub struct DatabaseConnection {
    redis: RedisClient,
    sql: MySqlPool,
    servers: DashMap<NonZeroU64, Arc<ServerSettingsDataCache>>,
    roles: DashMap<NonZeroU64, Vec<RoleRule>>,
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
        sqlx::migrate!("./src/migrations").run(&sql).await?;

        let servers = Self::load_server_settings(&sql).await?;
        let roles = Self::load_server_role_rules(&sql).await?;
        Ok(Self {
            redis,
            sql,
            servers,
            roles,
        })
    }
    async fn load_server_settings(
        sql: &MySqlPool,
    ) -> color_eyre::Result<DashMap<NonZeroU64, Arc<ServerSettingsDataCache>>> {
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
            allow_partially_blocked_users
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
                authenticated_role_id
            );
            let server_language: String = row.get("server_language");
            let server_language =
                LanguageId::try_from_str(&server_language).context("valid server_language")?;
            fetch!(
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
            map.insert(guild_id, Arc::new(data.into()));
        }

        Ok(map)
    }

    async fn load_server_role_rules(
        sql: &MySqlPool,
    ) -> color_eyre::Result<DashMap<NonZeroU64, Vec<RoleRule>>> {
        let all_rules = sqlx::query(
            "select
            guild_id,
            wiki,
            group_name,
            implicit_api_url,
            role_id
        from guild_roles",
        )
        .fetch_all(sql)
        .await?;

        let map = DashMap::<_, Vec<_>>::new();
        for row in all_rules {
            let guild_id: u64 = row.get("guild_id");
            let guild_id = NonZeroU64::new(guild_id).unwrap();
            let role_id: u64 = row.get("role_id");
            let role_id = NonZeroU64::new(role_id).unwrap();
            macro_rules! fetch {
                ($($name:ident),*$(,)?) => {
                    $(let $name = row.get(stringify!($name));)*
                };
            }
            fetch!(wiki, group_name, implicit_api_url);

            let data = RoleRule {
                wiki,
                group_name,
                implicit_api_url,
                role_id,
            };
            if let Some((_, mut rules)) = map.remove(&guild_id) {
                rules.push(data);
                map.insert(guild_id, rules);
            } else {
                map.insert(guild_id, vec![data]);
            }
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

#[derive(Clone, Copy, Debug)]
pub struct ServerSettingsData {
    pub welcome_channel_id: u64,
    pub auth_log_channel_id: u64,
    pub deauth_log_channel_id: u64,
    pub authenticated_role_id: u64,
    pub server_language: LanguageId,
    pub allow_banned_users: bool,
    pub whois_is_ephemeral: bool,
    pub allow_partially_blocked_users: bool,
}

struct ServerSettingsDataCache {
    pub welcome_channel_id: AtomicU64,
    pub auth_log_channel_id: AtomicU64,
    pub deauth_log_channel_id: AtomicU64,
    pub authenticated_role_id: AtomicU64,
    pub server_language: AtomicLanguageId,
    pub allow_banned_users: AtomicBool,
    pub whois_is_ephemeral: AtomicBool,
    pub allow_partially_blocked_users: AtomicBool,
}

impl ServerSettingsDataCache {
    fn load(&self) -> ServerSettingsData {
        ServerSettingsData {
            welcome_channel_id: self.welcome_channel_id.load(Ordering::SeqCst),
            auth_log_channel_id: self.auth_log_channel_id.load(Ordering::SeqCst),
            deauth_log_channel_id: self.deauth_log_channel_id.load(Ordering::SeqCst),
            authenticated_role_id: self.authenticated_role_id.load(Ordering::SeqCst),
            server_language: self.server_language.load(),
            allow_banned_users: self.allow_banned_users.load(Ordering::SeqCst),
            whois_is_ephemeral: self.whois_is_ephemeral.load(Ordering::SeqCst),
            allow_partially_blocked_users: self
                .allow_partially_blocked_users
                .load(Ordering::SeqCst),
        }
    }
}

impl From<ServerSettingsData> for ServerSettingsDataCache {
    fn from(value: ServerSettingsData) -> Self {
        let ServerSettingsData {
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
            whois_is_ephemeral,
            allow_partially_blocked_users,
        } = value;
        ServerSettingsDataCache {
            welcome_channel_id: welcome_channel_id.into(),
            auth_log_channel_id: auth_log_channel_id.into(),
            deauth_log_channel_id: deauth_log_channel_id.into(),
            authenticated_role_id: authenticated_role_id.into(),
            server_language: AtomicLanguageId::new(server_language),
            allow_banned_users: allow_banned_users.into(),
            whois_is_ephemeral: whois_is_ephemeral.into(),
            allow_partially_blocked_users: allow_partially_blocked_users.into(),
        }
    }
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
            server_settings: self
                .servers
                .get(&guild_id)
                .map(|x| x.value().clone().into()),
            role_rules: self.roles.get(&guild_id).map(|x| x.value().clone()),
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

#[derive(Clone)]
pub struct DatabaseConnectionInGuild<'a> {
    inner: &'a DatabaseConnection,
    guild_id: NonZeroU64,
    server_settings: Option<Arc<ServerSettingsDataCache>>,
    role_rules: Option<Vec<RoleRule>>,
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

    pub fn load_server_settings_weirdly(&self) -> Option<ServerSettingsData> {
        self.server_settings.as_ref().map(|x| x.load())
    }

    pub fn role_rules(&self) -> &Option<Vec<RoleRule>> {
        &self.role_rules
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

    pub fn server_language(&self) -> LanguageId {
        self.server_settings
            .as_ref()
            .unwrap()
            .server_language
            .load()
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
        let data_cache = Arc::new(ServerSettingsDataCache::from(data));
        self.servers.insert(self.guild_id, data_cache.clone());
        self.server_settings = Some(data_cache);
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
            .push_bind(server_language.name())
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
        let data = update(data.load());
        let data_cache = Arc::new(ServerSettingsDataCache::from(data));
        self.server_settings = Some(data_cache.clone());
        self.servers.insert(self.guild_id, data_cache);

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
        .bind(server_language.name())
        .bind(allow_banned_users)
        .bind(whois_is_ephemeral)
        .bind(allow_partially_blocked_users)
        .bind(self.guild_id.get())
        .execute(&self.sql)
        .await?;

        Ok(())
    }

    pub async fn add_role_rule(&mut self, rule: RoleRule) -> color_eyre::Result<()> {
        let mut rules = self.role_rules.take().unwrap_or_default();
        rules.push(rule.clone());
        self.roles.insert(self.guild_id, rules.clone());
        self.role_rules = Some(rules);
        let mut q = QueryBuilder::new("INSERT INTO guild_roles VALUES(");
        let mut separated = q.separated(", ");
        separated
            .push_bind(self.guild_id.get())
            .push_bind(rule.wiki)
            .push_bind(rule.group_name)
            .push_bind(rule.implicit_api_url)
            .push_bind(rule.role_id.get());
        separated.push_unseparated(")");
        q.build().execute(&self.sql).await?;
        Ok(())
    }

    pub async fn remove_role_rule(&mut self, role_id: u64) -> color_eyre::Result<u64> {
        if let Some(rules) = self.role_rules.as_mut() {
            rules.retain_mut(|x| x.role_id.get() != role_id);

            Ok(
                sqlx::query("delete from guild_roles where guild_id = ? and role_id = ?")
                    .bind(self.guild_id.get())
                    .bind(role_id)
                    .execute(&self.sql)
                    .await?
                    .rows_affected(),
            )
        } else {
            Ok(0)
        }
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
            .map(|data| data.welcome_channel_id.load(Ordering::SeqCst))
            .and_then(NonZeroU64::new)
    }

    pub fn auth_log_channel_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data| data.auth_log_channel_id.load(Ordering::SeqCst))
            .and_then(NonZeroU64::new)
    }

    pub fn deauth_log_channel_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data| data.deauth_log_channel_id.load(Ordering::SeqCst))
            .and_then(NonZeroU64::new)
    }

    pub fn authenticated_role_id(&self) -> Option<NonZeroU64> {
        self.server_settings
            .as_ref()
            .map(|data| data.authenticated_role_id.load(Ordering::SeqCst))
            .and_then(NonZeroU64::new)
    }

    pub fn all_managed_roles(&self) -> impl Iterator<Item = NonZeroU64> + '_ {
        self.server_settings
            .as_ref()
            .map(|data| data.authenticated_role_id.load(Ordering::SeqCst))
            .and_then(NonZeroU64::new)
            .into_iter()
            .chain(self.role_rules().iter().flatten().map(|rule| rule.role_id))
    }

    pub fn whois_is_ephemeral(&self) -> bool {
        self.server_settings.as_ref().unwrap().whois_is_ephemeral.load(Ordering::SeqCst)
    }

    pub fn disallows_block_status(&self, status: BlockKind) -> bool {
        match status {
            BlockKind::NotBlocked => false,
            BlockKind::PartiallyBlocked => self.disallows_partially_blocked_users(),
            BlockKind::Blocked => self.disallows_blocked_users(),
        }
    }

    pub fn disallows_blocked_users(&self) -> bool {
        !self.server_settings.as_ref().unwrap().allow_banned_users.load(Ordering::SeqCst)
    }

    pub fn disallows_partially_blocked_users(&self) -> bool {
        !self
            .server_settings
            .as_ref()
            .unwrap()
            .allow_partially_blocked_users.load(Ordering::SeqCst)
    }

    pub fn has_server_settings(&self) -> bool {
        self.server_settings.is_some()
    }
}

impl Deref for DatabaseConnectionInGuild<'_> {
    type Target = DatabaseConnection;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
