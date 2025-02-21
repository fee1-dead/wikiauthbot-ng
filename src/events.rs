use fred::error::{RedisError, RedisErrorKind};
use serenity::all::{Builder, CreateMessage, Mentionable};
use serenity::client::FullEvent;
use tokio::spawn;
use tracing::trace;
use wikiauthbot_common::webhook_println;

use crate::commands::handle_successful_auth;
use crate::commands::whois::fetch_whois;
use crate::{Data, Error, Result};

pub async fn init(ctx: &serenity::all::Context, u: &Data) -> color_eyre::Result<()> {
    let parent_db = u.db.clone();
    let db = parent_db.get_child().await?;
    let http = ctx.http.clone();

    spawn(async move {
        loop {
            let succ = match db.recv_successful_req().await {
                Ok(x) => x,
                Err(e) => {
                    if let Some(re) = e.downcast_ref::<RedisError>() {
                        if let RedisErrorKind::Timeout = re.kind() {
                            continue;
                        }
                    }
                    tracing::error!(?e, "couldn't receive successful request");
                    continue;
                }
            };

            handle_successful_auth(succ, &http, &parent_db).await;
        }
    });
    Ok(())
}

pub async fn event_handler(
    ctx: &serenity::all::Context,
    event: &FullEvent,
    _ftx: poise::FrameworkContext<'_, Data, Error>,
    u: &Data,
) -> Result {
    match event {
        FullEvent::GuildMemberAddition { new_member } => {
            let guild = new_member.guild_id;
            let db = u.db.in_guild(guild);
            trace!(?guild, "new member");
            if let Some(chan) = db.welcome_channel_id() {
                let mention = new_member.mention().to_string();

                let content = if let Ok(Some(whois)) = db.whois(new_member.user.id.get()).await {
                    if let Some(authenticated_role) = db.authenticated_role_id() {
                        new_member.add_role(ctx, authenticated_role).await?;
                    }
                    match fetch_whois(&u.client, whois.wikimedia_id).await {
                        Ok(whois) => {
                            let name = whois.name;
                            let user_link = db.user_link(&name)?;
                            wikiauthbot_db::msg!(
                                db,
                                "welcome_has_auth",
                                mention = mention,
                                name = name,
                                user_link = user_link
                            )?
                        }
                        _ => {
                            tracing::error!("failed to fetch whois!");
                            wikiauthbot_db::msg!(db, "welcome_has_auth_failed", mention = mention)?
                        }
                    }
                } else {
                    wikiauthbot_db::msg!(db, "welcome", mention = mention)?
                };
                let msg = CreateMessage::new().content(content);
                msg.reactions(['👋'])
                    .execute(ctx, (chan.into(), Some(guild)))
                    .await?;
            }
        }
        FullEvent::Ready { .. } => {
            eprintln!("discord bot is ready");
            webhook_println!("Ready");
            init(ctx, u).await?;
        }
        _ => {}
    }
    Ok(())
}

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            tracing::error!("Failed to start bot: {error:?}");
            webhook_println!("Failed to start bot: {error}");
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            tracing::error!("Error in command `{}`: {:?}", ctx.command().name, error);
            webhook_println!("Error in command `{}`: {}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!("Error while handling error: {e:?}");
                webhook_println!("Error while handling error: {e}");
            }
        }
    }
}
