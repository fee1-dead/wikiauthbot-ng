use serde_json::Value;
use serenity::all::Mention;
use wikiauthbot_db::msg;

use crate::{Context, Result};

#[poise::command(slash_command, ephemeral, guild_only = true)]
/// List Discord accounts associated to a Wikimedia account
pub async fn revwhois(
    ctx: Context<'_>,
    #[description = "Name of the Wikimedia user"] user: String,
) -> Result {
    ctx.defer_ephemeral().await?;
    let db = ctx.data().db_guild(&ctx);
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
            ctx.reply(db.get_message("revwhois_fail")?).await?;
            return Ok(());
        }
    };

    let Some(id) = val["query"]["globaluserinfo"]["id"].take().as_u64() else {
        ctx.reply(db.get_message("revwhois_fail")?).await?;
        return Ok(());
    };

    let results = db.revwhois(id as u32).await?;

    let user_link = db.user_link(&user)?;
    match &results[..] {
        [] => {
            ctx.reply(msg!(
                db,
                "revwhois_no_auth",
                name = user,
                user_link = user_link
            )?)
            .await?
        }
        &[id] => {
            ctx.reply(msg!(
                db,
                "revwhois_one",
                name = user,
                user_link = user_link,
                mention = Mention::User(id.into()).to_string(),
            )?)
            .await?
        }
        ids => {
            let mentions = ids
                .iter()
                .copied()
                .map(|id| format!("\n* {}", Mention::User(id.into())))
                .collect::<String>();
            ctx.reply(msg!(
                db,
                "revwhois_multiple",
                name = user,
                user_link = user_link,
                mentions = mentions,
            )?)
            .await?
        }
    };
    Ok(())
}
