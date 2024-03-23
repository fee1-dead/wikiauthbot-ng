use std::num::NonZeroU64;

use fred::prelude::*;
use fred::types::{KeyspaceEvent, Scanner as _};
use futures::TryStreamExt as _;
use wikiauthbot_common::Config;

pub mod server;

#[derive(Clone)]
pub struct DatabaseConnection {
    client: RedisClient,
}

pub struct ChildDatabaseConnection {
    client: RedisClient,
}

impl DatabaseConnection {
    pub async fn prod() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@redis.discordbots.eqiad1.wikimedia.cloud:6379");
        let client = Builder::from_config(RedisConfig::from_url(&url)?).build()?;
        client.init().await?;
        Ok(Self { client })
    }

    pub async fn prod_tunnelled() -> color_eyre::Result<Self> {
        let password = &Config::get()?.redis_password;
        let url = format!("redis://:{password}@127.0.0.1:16379");
        let client = Builder::from_config(RedisConfig::from_url(&url)?).build()?;
        client.init().await?;
        Ok(Self { client })
    }

    pub async fn dev() -> RedisResult<Self> {
        let client = RedisClient::default();
        client.init().await?;
        Ok(Self { client })
    }

    pub async fn get_child(&self) -> RedisResult<ChildDatabaseConnection> {
        let client = self.client.clone_new();
        client.init().await?;
        Ok(ChildDatabaseConnection { client })
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

impl DatabaseConnection {
    // TODO we should abstract this
    pub fn on_keyspace_event<F>(&self, func: F)
    where
        F: Fn(KeyspaceEvent) -> RedisResult<()> + Send + 'static,
    {
        self.client.on_keyspace_event(func);
    }

    pub async fn user_is_authed(&self, discord_id: u64) -> RedisResult<bool> {
        self.client.exists(format!("auth:{discord_id}")).await
    }

    pub async fn get_wikimedia_id(&self, discord_id: u64) -> RedisResult<Option<u32>> {
        self.client.get(format!("auth:{discord_id}")).await
    }

    pub async fn get_discord_ids(&self, wikimedia_id: u32) -> RedisResult<Vec<u64>> {
        self.client
            .lrange(format!("revauth:{wikimedia_id}"), 0, -1)
            .await
    }

    pub async fn is_user_authed_in_server(
        &self,
        discord_id: u64,
        guild_id: u64,
    ) -> RedisResult<bool> {
        self.client
            .sismember(format!("guilds:{guild_id}:authed"), discord_id)
            .await
    }

    pub async fn full_auth(
        &self,
        discord_id: u64,
        wikimedia_id: u32,
        guild_id: u64,
    ) -> RedisResult<()> {
        let txn = self.client.multi();
        txn.set(
            format!("auth:{discord_id}"),
            wikimedia_id,
            None,
            Some(SetOptions::NX),
            false,
        )
        .await?;
        txn.lpush(format!("revauth:{wikimedia_id}"), discord_id)
            .await?;
        txn.sadd(format!("guilds:{guild_id}:authed"), discord_id)
            .await?;
        txn.exec(true).await
    }

    pub async fn wmf_auth(&self, discord_id: u64, wikimedia_id: u32) -> RedisResult<()> {
        let txn = self.client.multi();
        txn.lpush(format!("revauth:{wikimedia_id}"), discord_id)
            .await?;
        txn.set(
            format!("auth:{discord_id}"),
            wikimedia_id,
            None,
            Some(SetOptions::NX),
            false,
        )
        .await?;
        txn.exec(true).await
    }

    /// Partially, used when we know what the user is authenticated already.
    pub async fn partial_auth(&self, discord_id: u64, guild_id: u64) -> RedisResult<()> {
        self.client
            .sadd(format!("guilds:{guild_id}:authed"), discord_id)
            .await
    }

    pub async fn whois(&self, discord_id: u64, guild_id: u64) -> RedisResult<Option<WhoisResult>> {
        if !self.is_user_authed_in_server(discord_id, guild_id).await? {
            Ok(None)
        } else {
            self.get_wikimedia_id(discord_id)
                .await
                .map(|user| user.map(|wikimedia_id| WhoisResult { wikimedia_id }))
        }
    }

    pub async fn revwhois(&self, wikimedia_id: u32, guild_id: u64) -> RedisResult<Vec<u64>> {
        let discord_ids = self.get_discord_ids(wikimedia_id).await?;
        let mut filtered = Vec::new();
        for identity in discord_ids {
            if self.is_user_authed_in_server(identity, guild_id).await? {
                filtered.push(identity);
            }
        }
        Ok(filtered)
    }

    pub async fn welcome_channel_id(&self, guild_id: u64) -> RedisResult<Option<NonZeroU64>> {
        let n: Option<u64> = self
            .client
            .get(format!("guilds:{guild_id}:welcome_channel_id"))
            .await?;
        println!("{n:?}");
        Ok(n.and_then(NonZeroU64::new))
    }

    pub async fn auth_log_channel_id(&self, guild_id: u64) -> RedisResult<u64> {
        self.client
            .get(format!("guilds:{guild_id}:auth_log_channel_id"))
            .await
    }

    pub async fn authenticated_role_id(&self, guild_id: u64) -> RedisResult<u64> {
        self.client
            .get(format!("guilds:{guild_id}:authenticated_role_id"))
            .await
    }

    pub async fn has_server_settings(&self, guild_id: u64) -> RedisResult<bool> {
        self.client
            .exists(format!("guilds:{guild_id}:server_language"))
            .await
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
        pipeline
            .set(
                key("welcome_channel_id"),
                welcome_channel_id,
                None,
                None,
                false,
            )
            .await?;
        pipeline
            .set(
                key("auth_log_channel_id"),
                auth_log_channel_id,
                None,
                None,
                false,
            )
            .await?;
        pipeline
            .set(
                key("deauth_log_channel_id"),
                deauth_log_channel_id,
                None,
                None,
                false,
            )
            .await?;
        pipeline
            .set(
                key("authenticated_role_id"),
                authenticated_role_id,
                None,
                None,
                false,
            )
            .await?;
        pipeline
            .set(key("server_language"), server_language, None, None, false)
            .await?;
        pipeline
            .set(
                key("allow_banned_users"),
                allow_banned_users,
                None,
                None,
                false,
            )
            .await?;
        pipeline.all().await?;
        Ok(())
    }

    pub async fn build_revauth(&self) -> RedisResult<()> {
        self.client
            .scan("auth:*", Some(100), None)
            .try_for_each_concurrent(None, |mut result| async move {
                let client = result.create_client();
                let results = result.take_results().unwrap();
                let wikimedia_ids: Vec<u32> = client.mget(results.clone()).await?;
                let pipeline = client.pipeline();
                for (key, wikimedia_id) in results.into_iter().zip(wikimedia_ids) {
                    let discord_id = key
                        .as_str_lossy()
                        .strip_prefix("auth:")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    pipeline
                        .lpush(format!("revauth:{wikimedia_id}"), discord_id)
                        .await?;
                }
                pipeline.all().await?;
                result.next()?;
                Ok(())
            })
            .await
    }
}
