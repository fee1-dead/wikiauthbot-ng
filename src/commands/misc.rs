use serenity::all::{GuildId, UserId};

use crate::commands::localize_command;
use crate::commands::whois::whois_menu;
use crate::{Context, Result};

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>, guild: Option<GuildId>) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    if !is_bot_owner {
        // silent fail
        return Ok(());
    }
    let commands =
        poise::builtins::create_application_commands(&ctx.framework().options().commands);

    // poise doesn't currently have a way to localize context menu commands. let's hack around that.
    let commands = commands
        .into_iter()
        .map(|mut c| {
            if serde_json::to_value(&c).unwrap()["name"] == "Get whois" {
                for (lang, val) in localize_command(whois_menu()).name_localizations {
                    c = c.name_localized(lang, val);
                }
            }
            c
        })
        .collect::<Vec<_>>();
    if let Some(guild) = guild {
        guild.set_commands(ctx, commands).await?;
    } else {
        serenity::all::Command::set_global_commands(ctx.http(), commands).await?;
    }
    Ok(())
}

#[poise::command(prefix_command, owners_only, dm_only, hide_in_help)]
pub async fn debug_deauth(ctx: Context<'_>, user_id: UserId, guild_id: Option<GuildId>) -> Result {
    let db = &ctx.data().db;
    ctx.defer_ephemeral().await?;
    if let Some(guild_id) = guild_id {
        let successful = db.in_guild(guild_id).partial_deauth(user_id.get()).await?;
        ctx.reply(if successful { "Done." } else { "Not done." })
            .await?;
    } else {
        let (servers, entries) = db.full_deauth(user_id.get()).await?;
        ctx.reply(format!(
            "found {entries} user authed to {servers} servers, now deleted."
        ))
        .await?;
    }

    Ok(())
}
