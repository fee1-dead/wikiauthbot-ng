use serenity::all::GuildId;
use wikiauthbot_db::ServerSettingsData;

use crate::{Command, Context, Result};

mod auth;
mod whois;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    if !is_bot_owner {
        // silent fail
        return Ok(());
    }
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, owners_only, dm_only, hide_in_help)]
pub async fn setup_server(
    ctx: Context<'_>,
    guild_id: GuildId,
    welcome_channel_id: u64,
    auth_log_channel_id: u64,
    deauth_log_channel_id: u64,
    authenticated_role_id: u64,
    server_language: String,
    allow_banned_users: bool,
) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    if !is_bot_owner {
        // silent fail
        return Ok(());
    }

    if ctx.data().server_settings.contains_key(&guild_id) {
        ctx.reply("F: server already set up").await?;
        return Ok(());
    }
    
    if !allow_banned_users {
        // TODO
        ctx.reply("F: disallowing banned users is not yet implemented").await?;
        return Ok(());
    }

    if server_language != "en" {
        // TODO
        ctx.reply("F: non-English languages are not yet implemented").await?;
        return Ok(());
    }

    let Ok(members) = guild_id.members(ctx, Some(1), None).await else {
        ctx.reply("F: failed to get members").await?;
        return Ok(());
    };

    if members.len() != 1 {
        ctx.reply("F: expected exactly one member").await?;
        return Ok(());
    }

    let data = ServerSettingsData {
        welcome_channel_id,
        auth_log_channel_id,
        deauth_log_channel_id,
        authenticated_role_id,
        server_language,
        allow_banned_users,
    };

    if !ctx.data().db.set_server_settings(guild_id.get(), data.clone()).await?
    {
        ctx.reply("F: server already set up").await?;
        return Ok(());
    }

    ctx.data().server_settings.insert(guild_id, data);

    ctx.reply("Setup server").await?;

    Ok(())
}

pub fn all_commands() -> Vec<Command> {
    vec![
        register(),
        setup_server(),
        auth::auth(),
        whois::whois(),
        whois::whois_test(),
    ]
}
