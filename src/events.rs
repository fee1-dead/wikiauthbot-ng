use std::num::NonZeroU64;
use std::time::Duration;

use serenity::all::{
    Builder, CreateMessage, EditInteractionResponse, GuildId, Mention, RoleId, UserId,
};
use tokio::time::timeout;
use tokio::spawn;
use tracing::error;

use crate::Data;

pub async fn init(ctx: &serenity::all::Context, u: &Data) -> color_eyre::Result<()> {
    let parent_db = u.db.clone();
    let db = parent_db.get_child().await?;
    let http = ctx.http.clone();
    let http2 = ctx.http.clone();

    parent_db.on_keyspace_event(move |event| {
        let key = event.key.as_str_lossy();
        if let Some(token) = key.strip_prefix("auth_message:expiry:") {
            let edit = EditInteractionResponse::new()
                .content("Authentication request expired.")
                .embeds(vec![]);
            let http2 = http2.clone();
            let token = token.to_owned();
            tokio::spawn(async move {
                if let Err(e) = edit.execute(http2, &token).await {
                    tracing::error!("error trying to edit original response: {e}");
                }
            });
        }
        Ok(())
    });
    spawn(async move {
        loop {
            let successful_auth = match timeout(Duration::from_secs(1), db.recv_successful_req()).await {
                Ok(Ok(x)) => x,
                Ok(Err(e)) => {
                    tracing::error!(?e, "couldn't receive successful request");
                    continue;
                }
                Err(_) => {
                    // timeout occured
                    continue;
                }
            };

            let wmf_id = successful_auth.central_user_id;
            let username = successful_auth.username;
            let discord_user_id: UserId = NonZeroU64::into(successful_auth.discord_user_id);
            let guild: GuildId = NonZeroU64::into(successful_auth.guild_id);

            if let Err(e) = parent_db
                .full_auth(discord_user_id.get(), wmf_id, guild.get())
                .await
            {
                tracing::error!(%e, "failed to insert authenticated!");
                continue;
            }

            let Ok(cont_token) = parent_db
                .record_auth_message_successful(discord_user_id.into())
                .await
            else {
                tracing::error!("failed record message as successful");
                // todo we should include e in them
                continue;
            };

            let newmsg = EditInteractionResponse::new()
                .content("Authentication successful.")
                .embeds(vec![])
                .components(vec![]);
            if let Err(e) = newmsg.execute(&http, &cont_token).await {
                tracing::error!(%e, "couldn't edit");
                continue;
            }

            let Ok(authenticated_role_id) = parent_db.authenticated_role_id(guild.get()).await
            else {
                tracing::error!("failed to get information for server: auth role");
                continue;
            };

            let Ok(auth_log_channel_id) = parent_db.auth_log_channel_id(guild.get()).await else {
                tracing::error!("failed to get information for server: auth log channel");
                continue;
            };

            let Ok(lang) = parent_db.server_language(guild.get()).await else {
                tracing::error!("failed to get information for server: server language");
                continue;
            };

            if authenticated_role_id != 0 {
                if let Err(e) = http
                    .add_member_role(
                        guild,
                        discord_user_id,
                        RoleId::from(authenticated_role_id),
                        Some(&format!("authenticated as wikimedia user {wmf_id}")),
                    )
                    .await
                {
                    eprintln!(
                        "failed to add member role to {discord_user_id} in guild {guild}: {e}"
                    );
                }
            }

            if auth_log_channel_id != 0 {
                let mention = Mention::User(discord_user_id);
                let user_link = user_link(&username, &lang);
                if let Err(e) = CreateMessage::new()
                    .content(format!(
                        "{mention} authenticated as [User:{username}](<{user_link}>) (id {wmf_id})"
                    ))
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
