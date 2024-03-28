use std::fmt::Display;
use std::num::NonZeroU64;

use color_eyre::eyre::ContextCompat;
use fred::prelude::*;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth};

use crate::{try_redis, ChildDatabaseConnection, DatabaseConnection};

impl ChildDatabaseConnection {
    pub async fn recv_successful_req(&self) -> color_eyre::Result<SuccessfulAuth> {
        // can't use try_redis on this because it would always time out
        let (_, key): (String, String) =
            self.client.blpop("successful_auths", 600.0).await?;
        let (discord_user_id, guild_id, central_user_id, username, brand_new) = try_redis(
            self.client
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
                .await,
        )?;
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
        try_redis(txn.get(format!("auth_req:{state}:discord_user_id")).await)?;
        try_redis(txn.get(format!("auth_req:{state}:guild_id")).await)?;
        let o: Option<(u64, u64)> = try_redis(txn.exec(true).await)?;
        o.map(|(discord_user_id, guild_id)| {
            AuthRequest::from_redis(state, discord_user_id, guild_id)
        })
        .transpose()
    }

    pub async fn record_auth_message(
        &self,
        discord_user_id: NonZeroU64,
        cont_token: &str,
    ) -> color_eyre::Result<()> {
        let expiring_key = format!("auth_message:expiry:{cont_token}");
        let client = self.client.pipeline();
        try_redis(
            client
                .set(
                    format!("auth_message:{discord_user_id}"),
                    &expiring_key,
                    Some(Expiration::EX(6 * 60)),
                    None,
                    false,
                )
                .await,
        )?;
        try_redis(
            client
                .set(expiring_key, "", Some(Expiration::EX(5 * 60)), None, false)
                .await,
        )?;
        try_redis(client.all().await)?;
        Ok(())
    }

    pub async fn record_auth_message_successful(
        &self,
        discord_user_id: NonZeroU64,
    ) -> RedisResult<String> {
        let key = format!("auth_message:{discord_user_id}");
        let mut expiring_key: String = try_redis(self.client.get(&key).await)?;
        try_redis(self.client.del(&[&key, &expiring_key]).await)?;
        let cont_token = expiring_key.split_off("auth_message:expiry:".len());
        Ok(cont_token)
    }

    pub async fn record_auth_req(
        &self,
        state: impl Display,
        discord_user_id: NonZeroU64,
        guild_id: NonZeroU64,
    ) -> RedisResult<()> {
        let txn = self.client.multi();
        try_redis(
            txn.set(
                format!("auth_req:{state}:discord_user_id"),
                discord_user_id.get(),
                Some(Expiration::EX(60 * 10)),
                None,
                false,
            )
            .await,
        )?;
        try_redis(
            txn.set(
                format!("auth_req:{state}:guild_id"),
                guild_id.get(),
                Some(Expiration::EX(60 * 10)),
                None,
                false,
            )
            .await,
        )?;
        try_redis(txn.exec(true).await)
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
    ) -> RedisResult<()> {
        let txn = self.client.multi();

        let key = format!("successful_auth:{}", discord_user_id);
        try_redis(
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
            .await,
        )?;
        // make the hash expire after ten minutes.
        try_redis(txn.expire(&key, 600).await)?;
        try_redis(txn.lpush("successful_auths", key).await)?;
        try_redis(txn.expire("successful_auths", 180).await)?;
        try_redis(txn.exec(true).await)?;

        Ok(())
    }
}
