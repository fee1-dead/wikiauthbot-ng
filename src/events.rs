use fred::error::{RedisError, RedisErrorKind};
use serenity::all::{
    Builder, CreateMessage, EditInteractionResponse, GuildId, Mention, RoleId, UserId,
};
use tokio::spawn;
use tracing::error;
use wikiauthbot_common::SuccessfulAuth;
use wikiauthbot_db::msg;

use crate::Data;

pub async fn init(ctx: &serenity::all::Context, u: &Data) -> color_eyre::Result<()> {
    let parent_db = u.db.clone();
    let db = parent_db.get_child().await?;
    let http = ctx.http.clone();

    spawn(async move {
        loop {
            let SuccessfulAuth {
                central_user_id,
                discord_user_id,
                guild_id,
                username,
                brand_new,
            } = match db.recv_successful_req().await {
                Ok(x) => x,
                Err(e) => {
                    if let Some(re) = e.downcast_ref::<RedisError>() {
                        if let RedisErrorKind::Timeout = re.kind() {
                            continue;
                        }
                    }
                    tracing::error!(?e, "couldn't receive successful request");
                    continue;
                }
            };

            let wmf_id = central_user_id;
            let discord_user_id = UserId::from(discord_user_id);
            let guild = GuildId::from(guild_id);
            let parent_db = parent_db.in_guild(guild);

            let res = if brand_new {
                parent_db.full_auth(discord_user_id.get(), wmf_id).await
            } else {
                parent_db.partial_auth(discord_user_id.get()).await
            };

            if let Err(e) = res {
                tracing::error!(%e, "failed to insert authenticated!");
                continue;
            }

            let cont_token = match parent_db
                .record_auth_message_successful(discord_user_id.into())
                .await
            {
                Ok(cont_token) => cont_token,
                Err(e) => {
                    tracing::error!(%e, "failed to record message as successful");
                    continue;
                }
            };

            let msg = parent_db
                .get_message("authreq_successful")
                .await
                .unwrap_or("Authentication successful".into());

            let newmsg = EditInteractionResponse::new()
                .content(msg)
                .embeds(vec![])
                .components(vec![]);
            if let Err(e) = newmsg.execute(&http, &cont_token).await {
                tracing::error!(%e, "couldn't edit");
                continue;
            }

            let auditlog = msg!(parent_db, "auditlog_successful_auth", wmf_id = wmf_id)
                .unwrap();

            if let Some(authenticated_role_id) = parent_db.authenticated_role_id() {
                if let Err(e) = http
                    .add_member_role(
                        guild,
                        discord_user_id,
                        RoleId::from(authenticated_role_id),
                        Some(&auditlog),
                    )
                    .await
                {
                    eprintln!(
                        "failed to add member role to {discord_user_id} in guild {guild}: {e}"
                    );
                }
            }

            if let Some(auth_log_channel_id) = parent_db.auth_log_channel_id() {
                let mention = Mention::User(discord_user_id).to_string();
                let Ok(user_link) = parent_db.user_link(&username).await else {
                    tracing::error!("couldn't get user link");
                    continue;
                };
                let authlog = msg!(
                    parent_db,
                    "authlog",
                    mention = &mention,
                    username = &username,
                    user_link = &*user_link,
                    wmf_id = wmf_id
                );
                let authlog = authlog.unwrap();
                if let Err(e) = CreateMessage::new()
                    .content(authlog)
                    .execute(&http, (auth_log_channel_id.into(), Some(guild)))
                    .await
                {
                    error!(
                        "failed to send message to channel {auth_log_channel_id} in guild {guild}: {e}"
                    );
                }
            }
        }
    });
    Ok(())
}
