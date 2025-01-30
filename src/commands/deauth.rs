use std::time::Duration;

use poise::CreateReply;
use serenity::all::{
    Builder, ButtonStyle, ChannelId, ComponentInteractionCollector, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateMessage, GuildId, Mentionable, RoleId, UserId,
};
use serenity::builder::EditInteractionResponse;
use tokio::spawn;
use tokio::time::timeout;
use wikiauthbot_db::{DatabaseConnectionInGuild, msg};

use crate::{Context, Result};

pub async fn handle_interactions(
    ctx: serenity::client::Context,
    discord_user_id: UserId,
    db: DatabaseConnectionInGuild<'_>,
    rxns: ComponentInteractionCollector,
    cont_token: String,
) -> color_eyre::Result<()> {
    if let Ok(Some(interaction)) = timeout(Duration::from_secs(120), rxns.next()).await {
        interaction
            .create_response(&ctx, CreateInteractionResponse::Acknowledge)
            .await?;
        let command = &*interaction.data.custom_id;
        match command {
            "full_multi" => {
                // do stuff on discord side first, before removing from db.
                let guilds = db.get_user_authed_guilds(discord_user_id.get()).await?;
                for guild in guilds {
                    let guild = GuildId::new(guild);
                    let db = db.in_guild(guild);
                    if let Some(role) = db.authenticated_role_id() {
                        let msg = msg!(db, "deauth_audit_log")?;
                        ctx.http
                            .remove_member_role(
                                guild,
                                discord_user_id,
                                RoleId::from(role),
                                Some(&msg),
                            )
                            .await?;
                    }
                    if let Some(chan) = db.deauth_log_channel_id() {
                        let msg = msg!(
                            db,
                            "deauth_log",
                            mention = discord_user_id.mention().to_string()
                        )?;
                        ChannelId::from(chan)
                            .send_message(&ctx, CreateMessage::new().content(msg))
                            .await?;
                    }
                }
                let (_, num_servers_authed) = db.full_deauth(discord_user_id.get()).await?;
                let newmsg = EditInteractionResponse::new()
                    .content(msg!(
                        db,
                        "deauth_more_multi_done",
                        num_servers_authed = num_servers_authed
                    )?)
                    .components(vec![]);
                newmsg.execute(&ctx, &cont_token).await?;
                return Ok(());
            }
            "yes_single" | "partial" => {
                // do stuff on discord side first, before removing from db.
                if let Some(role) = db.authenticated_role_id() {
                    let guild = GuildId::from(db.guild_id());
                    let msg = msg!(db, "deauth_audit_log")?;
                    ctx.http
                        .remove_member_role(guild, discord_user_id, RoleId::from(role), Some(&msg))
                        .await?;
                }
                if let Some(chan) = db.deauth_log_channel_id() {
                    let msg = msg!(
                        db,
                        "deauth_log",
                        mention = discord_user_id.mention().to_string()
                    )?;
                    ChannelId::from(chan)
                        .send_message(&ctx, CreateMessage::new().content(msg))
                        .await?;
                }
                let message = if command == "partial" {
                    db.partial_deauth(discord_user_id.get()).await?;
                    msg!(db, "deauth_more_single_done")?
                } else {
                    db.full_deauth(discord_user_id.get()).await?;
                    msg!(db, "deauth_done")?
                };
                let newmsg = EditInteractionResponse::new()
                    .content(message)
                    .components(vec![]);
                newmsg.execute(&ctx, &cont_token).await?;
                return Ok(());
            }
            "no" => {
                let newmsg = EditInteractionResponse::new()
                    .content(msg!(db, "deauth_canceled")?)
                    .components(vec![]);
                newmsg.execute(&ctx, &cont_token).await?;
                return Ok(());
            }
            id => tracing::error!("invalid custom id: {id}"),
        }
    }

    let newmsg = EditInteractionResponse::new()
        .content(db.get_message("deauth_expired").await?)
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

    let guild_id = db.guild_id();
    let cont_token = match ctx {
        Context::Prefix(_) => unreachable!(),
        Context::Application(appctx) => appctx.interaction.token.clone(),
    };

    // one guild left to deauth, data will be fully deleted.
    if num_guilds == 1 {
        // "Are you sure you want to remove your authentication from this server?"
        let msg = msg!(db, "deauth")?;
        let yes = msg!(db, "yes")?;
        let no = msg!(db, "cancel")?;
        let reply = CreateReply::default()
            .content(msg)
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("yes_single")
                    .label(yes)
                    .style(ButtonStyle::Danger),
                CreateButton::new("no")
                    .label(no)
                    .style(ButtonStyle::Secondary),
            ])]);
        let msg = ctx.send(reply).await?.into_message().await?;
        let db = ctx.data().db.clone();
        let ctx = ctx.serenity_context().clone();
        let rxns = msg.await_component_interaction(&ctx);
        spawn(async move {
            let db = db.in_guild(guild_id);
            if let Err(e) = handle_interactions(ctx, user_id, db, rxns, cont_token).await {
                tracing::error!(?e, "Error occured while handling interactions.");
            }
        });
    } else {
        // prompt how they want their data deleted
        // "Are you sure you want to remove your authentication from this server?"
        let msg = msg!(db, "deauth_more")?;
        let full = msg!(db, "deauth_more_multi")?;
        let partial = msg!(db, "deauth_more_single")?;
        let no = msg!(db, "cancel")?;
        let reply = CreateReply::default()
            .content(msg)
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("full_multi")
                    .label(full)
                    .style(ButtonStyle::Danger),
                CreateButton::new("partial")
                    .label(partial)
                    .style(ButtonStyle::Danger),
                CreateButton::new("no")
                    .label(no)
                    .style(ButtonStyle::Secondary),
            ])]);
        let msg = ctx.send(reply).await?.into_message().await?;
        let db = ctx.data().db.clone();
        let ctx = ctx.serenity_context().clone();
        let rxns = msg.await_component_interaction(&ctx);
        spawn(async move {
            let db = db.in_guild(guild_id);
            if let Err(e) = handle_interactions(ctx, user_id, db, rxns, cont_token).await {
                tracing::error!(?e, "Error occured while handling interactions.");
            }
        });
    }

    Ok(())
}
