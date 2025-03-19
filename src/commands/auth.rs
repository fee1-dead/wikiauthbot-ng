use std::ops::ControlFlow;
use std::sync::Arc;
use std::time::Duration;

use color_eyre::eyre::bail;
use poise::CreateReply;
use serde_json::Value;
use serenity::all::{
    Builder, ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateMessage, GuildId, Http, Mention, RoleId, UserId,
};
use serenity::builder::{CreateEmbed, CreateEmbedFooter, EditInteractionResponse};
use tokio::spawn;
use tokio::time::timeout;
use tracing::error;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth, webhook_println};
use wikiauthbot_db::{DatabaseConnection, DatabaseConnectionInGuild, msg};

use super::whois::{check_blocks, fetch_whois, update_roles};
use crate::{Context, Result};

pub async fn handle_interactions(
    ctx: serenity::client::Context,
    discord_user_id: UserId,
    db: DatabaseConnectionInGuild<'_>,
    rxns: ComponentInteractionCollector,
    wikimedia_id: u32,
    username: String,
    cont_token: String,
) -> color_eyre::Result<()> {
    if let Ok(Some(interaction)) = timeout(Duration::from_secs(120), rxns.next()).await {
        match &*interaction.data.custom_id {
            "yes" => {
                db.send_successful_req(SuccessfulAuth {
                    discord_user_id: discord_user_id.into(),
                    guild_id: db.guild_id(),
                    central_user_id: wikimedia_id,
                    username,
                    brand_new: false,
                })
                .await?;
                interaction
                    .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                    .await?;
                return Ok(());
            }
            "no" => {
                let newmsg = EditInteractionResponse::new()
                    .content(msg!(db, "authreq_canceled")?)
                    .components(vec![]);
                newmsg.execute(&ctx, &cont_token).await?;
                interaction
                    .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                    .await?;
                return Ok(());
            }
            id => error!("invalid custom id: {id}"),
        }
    }

    let newmsg = EditInteractionResponse::new()
        .content(db.get_message("authreq_expired")?)
        .components(vec![]);
    newmsg.execute(&ctx, &cont_token).await?;
    Ok(())
}

pub async fn auth_data_already_exists(
    ctx: Context<'_>,
    wikimedia_id: u32,
    cont_token: String,
) -> Result {
    let crate::Data { client, db, .. } = ctx.data();
    let user_id = ctx.author().id;
    let guild_id = ctx.guild_id().unwrap();
    let db2 = db.clone();
    let db = db.in_guild(guild_id);
    let mut val: Value = client
        .get([
            ("action", "query"),
            ("meta", "globaluserinfo"),
            ("guiid", &wikimedia_id.to_string()),
        ])
        .await?;
    let Value::String(name) = val["query"]["globaluserinfo"]["name"].take() else {
        bail!("name isn't string?")
    };
    let url = url::Url::parse_with_params(
        "https://meta.wikimedia.org/wiki/Special:CentralAuth",
        [("target", &name)],
    )
    .unwrap();
    let msg = msg!(db, "auth_to_server", name = &name, url = url.to_string())?;
    let yes = msg!(db, "yes")?;
    let no = msg!(db, "no")?;
    let reply = CreateReply::default()
        .content(msg)
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("yes")
                .label(yes)
                .style(ButtonStyle::Success),
            CreateButton::new("no").label(no).style(ButtonStyle::Danger),
        ])]);
    let msg = ctx.send(reply).await?.into_message().await?;
    let ctx = ctx.serenity_context().clone();
    let rxns = msg.await_component_interaction(&ctx);
    spawn(async move {
        let db2 = db2.in_guild(guild_id);
        if let Err(e) = handle_interactions(
            ctx,
            user_id,
            db2.in_guild(guild_id),
            rxns,
            wikimedia_id,
            name,
            cont_token,
        )
        .await
        {
            error!(?e, "Error occured while handling interactions.");
        }
    });
    Ok(())
}

/// Authenticate to your Wikimedia account
#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    let crate::Data { config, db, .. } = ctx.data();
    ctx.defer_ephemeral().await?;

    let user_id = ctx.author().id;
    let guild_id = ctx.guild_id().unwrap();
    let db = db.in_guild(guild_id);

    if !db.has_server_settings() {
        ctx.reply("This server has not been properly setup. Please contact the server owner.")
            .await?;
        return Ok(());
    }

    if db.is_user_authed_in_server(user_id.get()).await? {
        ctx.reply(db.get_message("auth_exists_in_server")?).await?;
        if let Some(authenticated_role) = db.authenticated_role_id() {
            ctx.author_member()
                .await
                .unwrap()
                .add_role(ctx, authenticated_role)
                .await?;
        }
        return Ok(());
    }

    let cont_token = match ctx {
        Context::Prefix(_) => unreachable!(),
        Context::Application(appctx) => appctx.interaction.token.clone(),
    };
    db.record_auth_message(user_id.into(), &cont_token).await?;

    if let Some(wikimedia_id) = db.get_wikimedia_id(user_id.get()).await? {
        return auth_data_already_exists(ctx, wikimedia_id, cont_token).await;
    }

    let authreq = AuthRequest::new(
        user_id.into(),
        guild_id.into(),
        db.server_language().to_owned(),
    );
    let state = authreq.state();
    db.record_auth_req(
        &state,
        user_id.into(),
        guild_id.into(),
        db.server_language(),
    )
    .await?;

    // https://www.mediawiki.org/wiki/OAuth/For_Developers
    let client_id = &*config.oauth_consumer_key;
    let url = format!(
        "https://meta.wikimedia.org/w/rest.php/oauth2/authorize?response_type=code&client_id={client_id}&state={state}"
    );

    let auth = msg!(db, "auth", url = url)?;
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .color(0xCCCCCC)
                .title(db.get_message("bot")?)
                .description(auth)
                .thumbnail("https://cdn.discordapp.com/emojis/546848856650809344.png")
                .footer(CreateEmbedFooter::new(
                    "This link will be valid for 10 minutes.",
                )),
        ),
    )
    .await?;

    Ok(())
}

pub async fn handle_successful_auth_inner(
    SuccessfulAuth {
        central_user_id,
        discord_user_id,
        guild_id,
        username,
        brand_new,
    }: SuccessfulAuth,
    http: &Arc<Http>,
    parent_db: &DatabaseConnection,
    client: &mwapi::Client,
) -> Result<ControlFlow<()>> {
    let wmf_id = central_user_id;
    let discord_user_id = UserId::from(discord_user_id);
    let guild = GuildId::from(guild_id);
    let parent_db = parent_db.in_guild(guild);

    let whois = fetch_whois(client, wmf_id).await?;
    let whois = whois.into_embeddable(discord_user_id).await?;

    if check_blocks(http, &parent_db, discord_user_id, &whois)
        .await?
        .is_break()
    {
        return Ok(ControlFlow::Break(()));
    }

    if brand_new {
        parent_db.full_auth(discord_user_id.get(), wmf_id).await?
    } else {
        parent_db.partial_auth(discord_user_id.get()).await?
    };

    if let Some(auth_log_channel_id) = parent_db.auth_log_channel_id() {
        let mention = Mention::User(discord_user_id).to_string();
        let user_link = parent_db.user_link(&username)?;
        let authlog = msg!(
            parent_db,
            "authlog",
            mention = &mention,
            username = &username,
            user_link = &*user_link,
            wmf_id = wmf_id
        );
        let authlog = authlog.unwrap();
        CreateMessage::new()
            .content(authlog)
            .execute(http, (auth_log_channel_id.into(), Some(guild)))
            .await?;
    }

    let auditlog = msg!(parent_db, "auditlog_successful_auth", wmf_id = wmf_id)?;

    if let Some(authenticated_role_id) = parent_db.authenticated_role_id() {
        if let Err(e) = http
            .add_member_role(
                guild,
                discord_user_id,
                RoleId::from(authenticated_role_id),
                Some(&auditlog),
            )
            .await
        {
            tracing::error!("failed to add member role to {discord_user_id} in guild {guild}: {e}");
            webhook_println!(
                "failed to add member role to {discord_user_id} in guild {guild}: {e}"
            );
        }
    }

    update_roles(http, &parent_db, discord_user_id, &whois).await?;

    Ok(ControlFlow::Continue(()))
}

pub async fn handle_successful_auth(
    successful_auth: SuccessfulAuth,
    http: &Arc<Http>,
    parent_db: &DatabaseConnection,
    client: &mwapi::Client,
) {
    let cont_token = match parent_db
        .get_auth_message_cont_token(successful_auth.discord_user_id)
        .await
    {
        Ok(cont_token) => cont_token,
        Err(e) => {
            webhook_println!("failed to insert authenticated! {e}");
            tracing::error!(?e, "failed to record message as successful");
            return;
        }
    };
    let parent_db = parent_db.in_guild(successful_auth.guild_id);
    match handle_successful_auth_inner(successful_auth, http, &parent_db, client).await {
        Ok(cf) => {
            let msg = if cf.is_continue() {
                "authreq_successful"
            } else {
                "auth_failed_blocked"
            };
            let msg = parent_db.get_message(msg).unwrap();

            let newmsg = EditInteractionResponse::new()
                .content(msg)
                .embeds(vec![])
                .components(vec![]);
            if let Err(e) = newmsg.execute(http, &cont_token).await {
                webhook_println!("couldn't edit! {e}");
                tracing::error!(?e, "couldn't edit");
            }
        }
        Err(e) => {
            webhook_println!("couldn't auth! {e}");
            tracing::error!(?e, "couldn't auth!");
            let msg = parent_db
                .get_message("authreq_successful")
                .unwrap_or("Authentication successful".into());

            let newmsg = EditInteractionResponse::new()
                .content(msg)
                .embeds(vec![])
                .components(vec![]);
            if let Err(e) = newmsg.execute(http, &cont_token).await {
                webhook_println!("couldn't edit for error! {e}");
                tracing::error!(?e, "couldn't edit for error!");
            }
        }
    }
}
