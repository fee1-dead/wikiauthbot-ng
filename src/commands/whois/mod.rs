use std::time::Instant;

use poise::CreateReply;
use serenity::all::{GuildId, User, UserId};
use wikiauthbot_db::WhoisResult;

use crate::{Context, Result};

mod fetch;
pub use fetch::fetch_whois;

pub async fn whois_impl(ctx: Context<'_>, user_id: UserId) -> Result {
    let crate::Data { client, .. } = ctx.data();
    let db = ctx.data().db_guild(&ctx);

    if !db.has_server_settings() {
        ctx.reply("this server has not been setup. Please contact beef.w for setup assistance.")
            .await?;
    }

    if db.whois_is_ephemeral() {
        ctx.defer_ephemeral().await?;
    } else {
        ctx.defer().await?;
    }

    let user = user_id.get();
    let whois = db.whois(user).await?;

    let Some(WhoisResult { wikimedia_id }) = whois else {
        ctx.reply(db.get_message("whois_no_user_found")?)
            .await?;
        return Ok(());
    };

    let whois = fetch_whois(client, wikimedia_id).await?;
    let embeddable = whois.into_embeddable(user_id).await?;

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .embed(embeddable.create_embed(db)?),
    )
    .await?;

    Ok(())
}

#[poise::command(context_menu_command = "Get whois", ephemeral, guild_only = true)]
pub async fn whois_menu(ctx: Context<'_>, user: User) -> Result {
    whois_impl(ctx, user.id).await
}

#[poise::command(slash_command, ephemeral, guild_only = true)]
/// Check account details for an authenticated member
pub async fn whois(
    ctx: Context<'_>,
    // TODO i18n description of commands
    #[description = "User to check, leave blank for yourself"] user: Option<UserId>,
) -> Result {
    whois_impl(ctx, user.unwrap_or_else(|| ctx.author().id)).await
}

#[poise::command(prefix_command)]
pub(crate) async fn whois_bench(ctx: Context<'_>, guild: GuildId, user: Option<UserId>) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    if !is_bot_owner {
        // silent fail
        return Ok(());
    }

    let start = Instant::now();
    let res = ctx
        .data()
        .db
        .in_guild(guild)
        .whois(user.unwrap_or(ctx.author().id).get())
        .await;
    let elapsed = start.elapsed();

    ctx.reply(format!("elapsed {elapsed:?} for result {res:?}"))
        .await?;

    Ok(())
}
