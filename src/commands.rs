use std::sync::atomic::{AtomicUsize, Ordering};

use poise::CreateReply;
use serenity::all::{ChannelId, GuildId, RoleId, UserId};
use serenity::futures::TryStreamExt;
use wikiauthbot_db::ServerSettingsData;

use crate::{Command, Context, Result, integrity};

mod auth;
mod deauth;
mod revwhois;
pub mod whois;

pub use auth::handle_successful_auth;

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

    let db = ctx.data().db.in_guild(guild_id);
    if !db.has_server_settings() {
        ctx.reply("This server was not set up. Please contact dbeef to set it up first.")
            .await?;
        return Ok(());
    }
    let auth_role = db.authenticated_role_id().unwrap();
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
pub async fn unauthed_list(ctx: Context<'_>, guild_id: GuildId) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    let is_server_admin = guild_id
        .member(ctx, ctx.author().id)
        .await?
        .permissions(ctx)?
        .administrator();

    if !is_bot_owner && !is_server_admin {
        ctx.reply("Must be a bot owner or server admin to use this command.")
            .await?;
        return Ok(());
    }
    let db = ctx.data().db.in_guild(guild_id);
    let Some(role) = db.authenticated_role_id() else {
        ctx.reply("Server is not setup with an authenticated role")
            .await?;
        return Ok(());
    };

    let members = guild_id
        .members_iter(ctx.http())
        .map_err(color_eyre::Report::from)
        .try_filter_map(|member| {
            let db = db.clone();
            async move {
                Ok(if member.roles.contains(&RoleId::new(role.get())) {
                    let discord_id = member.user.id.get();
                    if db.get_wikimedia_id(discord_id).await?.is_some() {
                        db.partial_auth(discord_id).await?;
                        None
                    } else {
                        Some(discord_id)
                    }
                } else {
                    None
                })
            }
        })
        .try_collect::<Vec<_>>()
        .await?;

    let s = members
        .into_iter()
        .map(|id| format!("* <@{id}>\n"))
        .collect::<String>();

    let s = if s.is_empty() {
        "No unauthed members found.".to_owned()
    } else {
        s
    };
    ctx.reply(s).await?;
    Ok(())
}

#[poise::command(prefix_command, dm_only, hide_in_help)]
pub async fn premigrate_server_check(
    ctx: Context<'_>,
    guild_id: GuildId,
    role_id: RoleId,
) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    let is_server_admin = guild_id
        .member(ctx, ctx.author().id)
        .await?
        .permissions(ctx)?
        .administrator();

    if !is_bot_owner && !is_server_admin {
        ctx.reply("Must be a bot owner or server admin to use this command.")
            .await?;
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
    If you would like to obtain an exact list of people who are not recognized by the bot, contact dbeef.")).await?;

    Ok(())
}

pub async fn server_settings_sanity_check(
    ctx: Context<'_>,
    guild_id: GuildId,
    ServerSettingsData {
        welcome_channel_id,
        auth_log_channel_id,
        deauth_log_channel_id,
        authenticated_role_id,
        server_language,
        allow_banned_users: _,
        whois_is_ephemeral: _,
    }: &ServerSettingsData,
) -> Result<bool> {
    if !wikiauthbot_common::i18n::lang_is_supported(&server_language) {
        ctx.reply("F: The language you have specified is not supported.")
            .await?;
        return Ok(false);
    }

    let Ok(members) = guild_id.members(ctx, Some(1), None).await else {
        ctx.reply("F: failed to get members").await?;
        return Ok(false);
    };

    if members.len() != 1 {
        ctx.reply("F: members check failed").await?;
        return Ok(false);
    }

    let guild = guild_id.to_partial_guild(ctx).await?;
    let channels = guild.channels(ctx).await?;
    let id = ctx.serenity_context().cache.current_user().id;
    let member = guild_id.member(ctx, id).await?;
    if !member.permissions(ctx)?.manage_roles() {
        ctx.reply("Please give the bot permissions to manage roles")
            .await?;
        return Ok(false);
    }
    let bot_pos = member.highest_role_info(ctx).unwrap().1;
    let role_id = RoleId::new(*authenticated_role_id);
    let Some(role) = guild.roles.get(&role_id) else {
        ctx.reply("The bot is unable to get information about the role ID specified. Please make sure the role ID is correct and try again.").await?;
        return Ok(false);
    };
    let role_pos = role.position;
    if bot_pos <= role_pos {
        ctx.reply(
            "It looks like the position of the bot role is lower than the authenticated role.\
        Please reorder the roles so the bot can add the authenticated role properly.",
        )
        .await?;
        return Ok(false);
    }

    for (chan, desc) in [
        (welcome_channel_id, "welcome channel"),
        (auth_log_channel_id, "authentication log channel"),
        (deauth_log_channel_id, "deauthentication log channel"),
    ] {
        if *chan == 0 {
            continue;
        }

        let Some(chan) = channels.get(&ChannelId::new(*chan)) else {
            ctx.reply(format!(
                "The bot is unable to get information about the channel ID specified for the {desc}. \
                Please make sure the ID is correct, the bot has access to the given channel, and try again."
            )).await?;
            return Ok(false);
        };
        let perms = chan.permissions_for_user(ctx, id)?;
        if !perms.send_messages() {
            ctx.reply(format!("Oops! Looks like I cannot send message in the {desc}. Please make sure the bot has the right permissions and try again.")).await?;
            return Ok(false);
        }
    }
    Ok(true)
}

#[poise::command(prefix_command, dm_only, hide_in_help)]
pub async fn setup_server(
    ctx: Context<'_>,
    guild_id: GuildId,
    welcome_channel_id: u64,
    auth_log_channel_id: u64,
    deauth_log_channel_id: u64,
    authenticated_role_id: u64,
    server_language: String,
    allow_banned_users: bool,
    whois_is_ephemeral: bool,
) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    let is_server_admin = guild_id
        .member(ctx, ctx.author().id)
        .await?
        .permissions(ctx)?
        .administrator();

    if !is_bot_owner && !is_server_admin {
        ctx.reply("Must be a bot owner or server admin to use this command.")
            .await?;
        return Ok(());
    }

    let data = ServerSettingsData {
        welcome_channel_id,
        auth_log_channel_id,
        deauth_log_channel_id,
        authenticated_role_id,
        server_language,
        allow_banned_users,
        whois_is_ephemeral,
    };

    if !server_settings_sanity_check(ctx, guild_id, &data).await? {
        // sanity check found something bad, just return here since the error is already given.
        return Ok(());
    }

    let mut db = ctx.data().db.in_guild(guild_id);

    if db.has_server_settings() {
        ctx.reply("F: server already set up").await?;
        return Ok(());
    }

    db.set_server_settings(data).await?;

    let handle = ctx
        .reply(
            "Server has been setup; please wait for database to be updated.\
    If you still see this message after a minute please let dbeef know.",
        )
        .await?;

    integrity::role_to_db(
        ctx.serenity_context(),
        db,
        guild_id,
        RoleId::from(authenticated_role_id),
    )
    .await?;

    handle
        .edit(ctx, CreateReply::default().content("All done!"))
        .await?;

    Ok(())
}

#[poise::command(prefix_command, dm_only, hide_in_help)]
pub async fn set_server_language(
    ctx: Context<'_>,
    guild_id: GuildId,
    server_language: String,
) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);

    if !is_bot_owner {
        ctx.reply("Must be a bot owner to use this command.")
            .await?;
        return Ok(());
    }
    let mut db = ctx.data().db.in_guild(guild_id);
    let mut data = db.server_settings().clone().unwrap();

    data.server_language = server_language;

    if !server_settings_sanity_check(ctx, guild_id, &data).await? {
        // sanity check found something bad, just return here since the error is already given.
        return Ok(());
    }

    db.update_server_settings(|_| data).await?;

    ctx.reply("Done! uwu").await?;

    Ok(())
}

#[poise::command(prefix_command, dm_only, hide_in_help)]
pub async fn set_server_whois_is_ephemeral(
    ctx: Context<'_>,
    guild_id: GuildId,
    whois_is_ephemeral: bool,
) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);

    if !is_bot_owner {
        ctx.reply("Must be a bot owner to use this command.")
            .await?;
        return Ok(());
    }
    let mut db = ctx.data().db.in_guild(guild_id);
    let mut data = db.server_settings().clone().unwrap();

    data.whois_is_ephemeral = whois_is_ephemeral;

    if !server_settings_sanity_check(ctx, guild_id, &data).await? {
        // sanity check found something bad, just return here since the error is already given.
        return Ok(());
    }

    db.update_server_settings(|_| data).await?;

    ctx.reply("Done! uwu").await?;

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

/*
#[poise::command(prefix_command, owners_only, dm_only, hide_in_help)]
pub async fn stats(ctx: Context<'_>) -> Result {
    ctx.defer_ephemeral().await?;
    let mut target = None;
    let mut nguilds = 0;
    let mut nusers = 0;
    loop {
        let guilds = ctx.serenity_context().http.get_guilds(target, None).await.wrap_err("Unable to fetch guilds")?;
        if guilds.is_empty() {
            break;
        }
        nguilds += guilds.len();
        target = Some(GuildPagination::After(guilds.last().unwrap().id));
        for guild in guilds {

        }

    }
    Ok(())
}
*/

pub fn all_commands() -> Vec<Command> {
    vec![
        register(),
        setup_server(),
        auth::auth(),
        deauth::deauth(),
        whois::whois(),
        whois::whois_menu(),
        whois::whois_bench(),
        premigrate_server_check(),
        revwhois::revwhois(),
        cleanup_roles(),
        debug_deauth(),
        unauthed_list(),
        set_server_language(),
        set_server_whois_is_ephemeral(),
        // whois::whois_test(),
    ]
}
