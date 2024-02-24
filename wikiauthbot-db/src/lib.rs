use fred::prelude::*;

#[derive(Clone)]
pub struct DatabaseConnection {
    inner: RedisClient,
}

impl DatabaseConnection {
    pub async fn prod() -> RedisResult<Self> {
        todo!()
    }

    pub async fn dev() -> RedisResult<Self> {
        let client = RedisClient::default();
        client.init().await?;
        Ok(Self {
            inner: client,
        })
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
    pub async fn user_is_authed(&self, discord_id: u64) -> RedisResult<bool> {
        self.inner.exists(format!("auth:{discord_id}")).await
    }

    pub async fn get_wikimedia_id(&self, discord_id: u64) -> RedisResult<Option<u32>> {
        self.inner.get(format!("auth:{discord_id}")).await
    }

    pub async fn is_user_authed_in_server(
        &self,
        discord_id: u64,
        guild_id: u64,
    ) -> RedisResult<bool> {
        self.inner.sismember(format!("guilds:{guild_id}:authed"), discord_id).await
    }

    pub async fn full_auth(&self, discord_id: u64, wikimedia_id: u32, guild_id: u64) -> RedisResult<()> {
        let txn = self.inner.multi();
        txn.set(format!("auth:{discord_id}"), wikimedia_id, None, Some(SetOptions::NX), false).await?;
        txn.sadd(format!("guilds:{guild_id}:authed"), discord_id).await?;
        txn.exec(true).await
    }

    /// Partially, used when we know what the user is authenticated already.
    pub async fn partial_auth(
        &self,
        discord_id: u64,
        guild_id: u64,
    ) -> RedisResult<()> {
        self.inner.sadd(format!("guilds:{guild_id}:authed"), discord_id).await
    }

    pub async fn whois(
        &self,
        discord_id: u64,
        guild_id: u64,
    ) -> RedisResult<Option<WhoisResult>> {
        if !self.is_user_authed_in_server(discord_id, guild_id).await? {
            Ok(None)
        } else {
            self.get_wikimedia_id(discord_id).await.map(|user| user.map(|wikimedia_id| WhoisResult { wikimedia_id }))
        }
    }

    pub async fn auth_log_channel_id(&self, guild_id: u64) -> RedisResult<u64> {
        self.inner.get(format!("guilds:{guild_id}:auth_log_channel_id")).await
    }

    pub async fn authenticated_role_id(&self, guild_id: u64) -> RedisResult<u64> {
        self.inner.get(format!("guilds:{guild_id}:authenticated_role_id")).await
    }

    /* pub async fn get_all_server_settings(
        &self,
    ) -> Result<impl Iterator<Item = (u64, ServerSettingsData)>, DbErr> {
        let models = ServerSettings::find().all(&self.inner).await?;
        Ok(models.into_iter().map(
            |server_settings::Model {
                 server_id,
                 welcome_channel_id,
                 auth_log_channel_id,
                 deauth_log_channel_id,
                 authenticated_role_id,
                 server_language,
                 allow_banned_users,
             }| {
                (
                    server_id,
                    ServerSettingsData {
                        welcome_channel_id,
                        auth_log_channel_id,
                        deauth_log_channel_id,
                        authenticated_role_id,
                        server_language,
                        allow_banned_users,
                    },
                )
            },
        ))
    }

    pub async fn get_server_settings(
        &self,
        discord_server_id: u64,
    ) -> Result<Option<ServerSettingsData>, DbErr> {
        let model = ServerSettings::find_by_id(discord_server_id)
            .one(&self.inner)
            .await?;
        Ok(model.map(
            |server_settings::Model {
                 server_id: _,
                 welcome_channel_id,
                 auth_log_channel_id,
                 deauth_log_channel_id,
                 authenticated_role_id,
                 server_language,
                 allow_banned_users,
             }| ServerSettingsData {
                welcome_channel_id,
                auth_log_channel_id,
                deauth_log_channel_id,
                authenticated_role_id,
                server_language,
                allow_banned_users,
            },
        ))
    } */

    pub async fn has_server_settings(&self, guild_id: u64) -> RedisResult<bool> {
        self.inner.exists(format!("guilds:{guild_id}:server_language")).await
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
        let multi = self.inner.multi();
        let key = |subkey| format!("guilds:{guild_id}:{subkey}");
        multi.set(key("welcome_channel_id"), welcome_channel_id, None, None, false).await?;
        multi.set(key("auth_log_channel_id"), auth_log_channel_id, None, None, false).await?;
        multi.set(key("deauth_log_channel_id"), deauth_log_channel_id, None, None, false).await?;
        multi.set(key("authenticated_role_id"), authenticated_role_id, None, None, false).await?;
        multi.set(key("server_language"), server_language, None, None, false).await?;
        multi.set(key("allow_banned_users"), allow_banned_users, None, None, false).await?;
        multi.exec(true).await
    }
}
