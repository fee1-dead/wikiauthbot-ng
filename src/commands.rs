use std::sync::atomic::{AtomicUsize, Ordering};

use serenity::all::{GuildId, RoleId, UserId};
use serenity::futures::TryStreamExt;
use wikiauthbot_db::ServerSettingsData;

use crate::{Command, Context, Result};

mod auth;
mod revwhois;
pub mod whois;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>, guild: Option<GuildId>) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    if !is_bot_owner {
        // silent fail
        return Ok(());
    }
    if let Some(guild) = guild {
        guild
            .set_commands(
                ctx,
                poise::samples::create_application_commands(&ctx.framework().options().commands),
            )
            .await?;
    } else {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    }
    Ok(())
}

#[poise::command(prefix_command, slash_command, hide_in_help)]
pub async fn cleanup_roles(ctx: Context<'_>) -> Result {
    ctx.defer_ephemeral().await?;
    let Some(mem) = ctx.author_member().await else {
        ctx.reply("not a member").await?;
        return Ok(());
    };

    let Some(guild_id) = ctx.guild_id() else {
        ctx.reply("command must be used in a guild.").await?;
        return Ok(());
    };

    let perms = mem.permissions(ctx)?;
    if !perms.administrator() {
        ctx.reply("You must have the Administrator permission to use this command.")
            .await?;
        return Ok(());
    }

    let db = &ctx.data().db;
    if !db.has_server_settings(guild_id.get()).await? {
        ctx.reply("This server was not set up. Please contact deadbf to set it up first.")
            .await?;
    }
    let auth_role = db.authenticated_role_id(guild_id.get()).await?;
    let role_id = RoleId::from(auth_role);
    let count = AtomicUsize::new(0);

    guild_id
        .members_iter(ctx.http())
        .map_err(color_eyre::Report::from)
        .try_for_each_concurrent(None, |member| {
            let db = &db;
            let count = &count;
            let http = ctx.http();
            async move {
                if member.roles.contains(&role_id) {
                    let discord_id = member.user.id.get();
                    if db.get_wikimedia_id(discord_id).await?.is_none() {
                        member.remove_role(http, role_id).await?;
                        count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                Ok(())
            }
        })
        .await?;

    ctx.reply(format!(
        "Removed authenticated role from {} members that are not known to the bot.",
        count.load(Ordering::Relaxed)
    ))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, dm_only, hide_in_help)]
pub async fn premigrate_server_check(ctx: Context<'_>, guild_id: GuildId, role_id: RoleId) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    let is_server_admin = guild_id.member(ctx, ctx.author().id).await?.permissions(ctx)?.administrator();

    if !is_bot_owner && !is_server_admin {
        // silent fail
        return Ok(());
    }

    let db = &ctx.data().db;

    let pauthed = AtomicUsize::new(0);
    let unauthed = AtomicUsize::new(0);

    guild_id
        .members_iter(ctx.http())
        .map_err(color_eyre::Report::from)
        .try_for_each_concurrent(None, |member| {
            let db = db.clone();
            let (pauthed, unauthed) = (&pauthed, &unauthed);
            async move {
                if member.roles.contains(&role_id) {
                    let discord_id = member.user.id.get();
                    if db.get_wikimedia_id(discord_id).await?.is_some() {
                        db.partial_auth(discord_id, guild_id.get()).await?;
                        pauthed.fetch_add(1, Ordering::Relaxed);
                    } else {
                        unauthed.fetch_add(1, Ordering::Relaxed);
                    }
                }
                Ok(())
            }
        })
        .await?;

    let (pauthed, unauthed) = (
        pauthed.load(Ordering::Relaxed),
        unauthed.load(Ordering::Relaxed),
    );

    ctx.reply(format!("\
    there are {pauthed} linked accounts, and {unauthed} accounts with the given role that have no data and would need to reauth.\n\
    If you would like to obtain an exact list of people who are not recognized by the bot, contact deadbf.")).await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only, dm_only, hide_in_help)]
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
    let is_server_admin = guild_id.member(ctx, ctx.author().id).await?.permissions(ctx)?.administrator();

    if !is_bot_owner && !is_server_admin {
        // silent fail
        return Ok(());
    }

    if !allow_banned_users {
        // TODO
        ctx.reply("F: disallowing banned users is not yet implemented")
            .await?;
        return Ok(());
    }

    if server_language != "en" {
        // TODO
        ctx.reply("F: non-English languages are not yet implemented")
            .await?;
        return Ok(());
    }

    let Ok(members) = guild_id.members(ctx, Some(1), None).await else {
        ctx.reply("F: failed to get members").await?;
        return Ok(());
    };

    if members.len() != 1 {
        ctx.reply("F: members check failed").await?;
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

    let db = &ctx.data().db;

    if db.has_server_settings(guild_id.get()).await? {
        ctx.reply("F: server already set up").await?;
        return Ok(());
    }

    db.set_server_settings(guild_id.get(), data).await?;

    ctx.reply("Setup server").await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only, dm_only, hide_in_help)]
pub async fn debug_deauth(ctx: Context<'_>, user_id: UserId, guild_id: GuildId) -> Result {
    let db = &ctx.data().db;
    ctx.defer_ephemeral().await?;
    db.debug_deauth(user_id.get(), guild_id.get()).await?;
    ctx.reply("Done.").await?;
    Ok(())
}

pub fn all_commands() -> Vec<Command> {
    vec![
        register(),
        setup_server(),
        auth::auth(),
        whois::whois(),
        premigrate_server_check(),
        revwhois::revwhois(),
        cleanup_roles(),
        debug_deauth(),
        // whois::whois_test(),
    ]
}
