use std::time::Duration;

use color_eyre::eyre::bail;
use poise::CreateReply;
use serde_json::Value;
use serenity::all::{
    Builder, ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton,
    CreateInteractionResponse, UserId,
};
use serenity::builder::{CreateEmbed, CreateEmbedFooter, EditInteractionResponse};
use tokio::spawn;
use tokio::time::timeout;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth};
use wikiauthbot_db::{msg, DatabaseConnectionInGuild};

use crate::{Context, Result};

pub async fn handle_interactions(
    ctx: serenity::client::Context,
    discord_user_id: UserId,
    db: DatabaseConnectionInGuild<'_>,
    rxns: ComponentInteractionCollector,
    wikimedia_id: u32,
    username: String,
    cont_token: String,
) -> color_eyre::Result<()> {
    if let Ok(Some(interaction)) = timeout(Duration::from_secs(120), rxns.next()).await {
        match &*interaction.data.custom_id {
            "yes" => {
                interaction
                    .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                    .await?;
                return Ok(());
            }
            "no" => {
                let newmsg = EditInteractionResponse::new()
                    .content(msg!(db, "authreq_canceled")?)
                    .components(vec![]);
                newmsg.execute(&ctx, &cont_token).await?;
                interaction
                    .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                    .await?;
                return Ok(());
            }
            id => tracing::error!("invalid custom id: {id}"),
        }
    }

    let newmsg = EditInteractionResponse::new()
        .content(db.get_message("authreq_expired").await?)
        .components(vec![]);
    newmsg.execute(&ctx, &cont_token).await?;
    Ok(())
}

/// Deauthenticate or remove your data from the bot.
#[poise::command(slash_command, guild_only = true)]
pub async fn deauth(ctx: Context<'_>) -> Result {
    let db = ctx.data().db_guild(&ctx);
    let user_id = ctx.author().id;
    if db.whois(user_id.get()).await?.is_none() {
        ctx.reply(msg!(db, "deauth_not_found")?).await?;
    }

    let num_guilds = db.count_guilds_authed_to(user_id.get()).await?;
    assert_ne!(0, num_guilds);

    Ok(())
}
