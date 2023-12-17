use crate::{Context, Result};

#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    ctx.defer_ephemeral().await?;
    ctx.reply(format!("user {} at guild {}", ctx.author().id, ctx.guild_id().unwrap())).await?;
    Ok(())
}