use crate::{Context, Result};

use color_eyre::eyre::ContextCompat;
use serenity::all::UserId;
use wikiauthbot_db::WhoisResult;

#[poise::command(
    slash_command,
    ephemeral,
    guild_only = true,
)]
pub async fn whois(ctx: Context<'_>, user: Option<UserId>) -> Result {
    let crate::Data { client, db } = ctx.data();
    ctx.defer_ephemeral().await?;

    let user = user.unwrap_or_else(|| ctx.author().id).get();

    let whois = db.whois(user, ctx.guild_id().context("must be in guild")?.get()).await?;

    let Some(WhoisResult { wikimedia_id }) = whois else {
        ctx.reply("no user found. either the user is not in this server or is unauthenticated").await?;
        return Ok(());
    };


    
    // use the reqwest client, or else
    ctx.reply(format!(
        "user {} at guild {}",
        ctx.author().id,
        ctx.guild_id().unwrap()
    ))
    .await?;
    Ok(())
}
