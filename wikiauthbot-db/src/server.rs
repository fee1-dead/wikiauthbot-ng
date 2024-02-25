use std::fmt::Display;
use std::num::NonZeroU64;

use color_eyre::eyre::ContextCompat;
use fred::prelude::*;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth};

use crate::{ChildDatabaseConnection, DatabaseConnection};

impl ChildDatabaseConnection {
    pub async fn recv_successful_req(&self) -> color_eyre::Result<SuccessfulAuth> {
        let key: String = self.client.blpop("successful_auths", 0.0).await?;
        let (discord_user_id, guild_id, central_user_id, username, brand_new) = self
            .client
            .hmget(
                key,
                &[
                    "discord_user_id",
                    "guild_id",
                    "central_user_id",
                    "username",
                    "brand_new",
                ],
            )
            .await?;
        Ok(SuccessfulAuth {
            discord_user_id: NonZeroU64::new(discord_user_id).context("null discord_user_id")?,
            guild_id: NonZeroU64::new(guild_id).context("null guild_id")?,
            central_user_id,
            username,
            brand_new,
        })
    }
}

impl DatabaseConnection {
    pub async fn get_auth_req(&self, state: &str) -> color_eyre::Result<Option<AuthRequest>> {
        let txn = self.client.multi();
        txn.get(format!("auth_req:{state}:discord_user_id")).await?;
        txn.get(format!("auth_req:{state}:guild_id")).await?;
        let o: Option<(u64, u64)> = txn.exec(true).await?;
        o.map(|(discord_user_id, guild_id)| {
            AuthRequest::from_redis(state, discord_user_id, guild_id)
        })
        .transpose()
    }

    pub async fn record_auth_message(&self, discord_user_id: NonZeroU64, cont_token: &str) -> color_eyre::Result<()> {
        let expiring_key = format!("auth_message:expiry:{cont_token}");
        let client = self.client.pipeline();
        client.set(format!("auth_message:{discord_user_id}"), &expiring_key, Some(Expiration::EX(6*60)), None, false).await?;
        client.set(expiring_key, "", Some(Expiration::EX(5*60)), None, false).await?;
        Ok(())
    }

    pub async fn record_auth_message_successful(&self, discord_user_id: NonZeroU64) -> color_eyre::Result<()> {
        let key = format!("auth_message:{discord_user_id}");
        let expiring_key: String = self.client.get(&key).await?;
        self.client.del(&[key, expiring_key]).await?;
        Ok(())
    }

    pub async fn record_auth_req(
        &self,
        state: impl Display,
        discord_user_id: NonZeroU64,
        guild_id: NonZeroU64,
    ) -> RedisResult<()> {
        let txn = self.client.multi();
        txn.set(
            format!("auth_req:{state}:discord_user_id"),
            discord_user_id.get(),
            Some(Expiration::EX(60 * 10)),
            None,
            false,
        )
        .await?;
        txn.set(
            format!("auth_req:{state}:guild_id"),
            guild_id.get(),
            Some(Expiration::EX(60 * 10)),
            None,
            false,
        )
        .await?;
        txn.exec(true).await
    }

    pub async fn send_successful_req(
        &self,
        SuccessfulAuth {
            discord_user_id,
            guild_id,
            central_user_id,
            username,
            brand_new,
        }: SuccessfulAuth,
    ) -> color_eyre::Result<()> {
        let txn = self.client.multi();

        let key = format!("successful_auth:{}", discord_user_id);
        txn.hset(
            &key,
            [
                ("discord_user_id", discord_user_id.get().try_into()?),
                ("guild_id", guild_id.get().try_into()?),
                ("central_user_id", central_user_id.into()),
                ("username", String::from(username).into()),
                ("brand_new", RedisValue::Boolean(brand_new)),
            ],
        )
        .await?;
        // make the hash expire after a minute.
        txn.expire(&key, 60).await?;
        txn.lpush("successful_auths", key).await?;
        txn.expire("successful_auths", 180).await?;
        txn.exec(true).await?;

        Ok(())
    }
}
