use serde_json::Value;
use serenity::all::Mention;

use crate::{Context, Result};

#[poise::command(slash_command, ephemeral, guild_only = true)]
/// Check account details for a WMF account
pub async fn revwhois(
    ctx: Context<'_>,
    #[description = "Name of the Wikimedia user"] user: String,
) -> Result {
    ctx.defer_ephemeral().await?;
    let Some(guild_id) = ctx.guild_id() else {
        ctx.reply("not in a guild").await?;
        return Ok(());
    };
    let mut val: Value = match ctx
        .data()
        .client
        .get([
            ("action", "query"),
            ("meta", "globaluserinfo"),
            ("guiuser", &user),
        ])
        .await
    {
        Ok(val) => val,
        Err(_) => {
            ctx.reply("Could not fetch info for given user. Please make sure you have supplied the correct username").await?;
            return Ok(());
        }
    };

    let Some(id) = val["query"]["globaluserinfo"]["id"].take().as_u64() else {
        ctx.reply("Could not fetch info for given user. Please make sure you have supplied the correct username").await?;
        return Ok(());
    };
    let results = ctx.data().db.revwhois(id as u32, guild_id.get()).await?;

    match &results[..] {
        [] => ctx.reply("The given user has not authenticated to this server.").await?,
        &[id] => ctx.reply(format!("{user} is authenticated to {}", Mention::User(id.into()))).await?,
        [ids @ ..] => {
            let s = ids.iter().copied().map(|id| format!("\n* {}", Mention::User(id.into()))).collect::<String>();
            ctx.reply(format!("{user} is authenticated to the following accounts:{s}")).await?
        }
    };
    Ok(())
}
