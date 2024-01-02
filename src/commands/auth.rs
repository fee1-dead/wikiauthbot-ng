use crate::{Context, Result};

#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    let crate::Data { client, .. } = ctx.data();
    // use the reqwest client, or else
    ctx.defer_ephemeral().await?;
    ctx.reply(format!(
        "user {} at guild {}",
        ctx.author().id,
        ctx.guild_id().unwrap()
    ))
    .await?;
    Ok(())
}
