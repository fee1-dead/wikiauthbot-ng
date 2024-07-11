use std::time::Duration;

use color_eyre::eyre::bail;
use poise::CreateReply;
use serde_json::Value;
use serenity::all::{
    Builder, ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateInteractionResponse, UserId
};
use serenity::builder::{CreateEmbed, CreateEmbedFooter, EditInteractionResponse};
use tokio::spawn;
use tokio::time::timeout;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth};
use wikiauthbot_db::{msg, DatabaseConnectionInGuild};

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
                db
                    .send_successful_req(SuccessfulAuth {
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
                return Ok(())
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
            id => tracing::error!("invalid custom id: {id}"),
        }
    }

    let newmsg = EditInteractionResponse::new()
        .content(db.get_message("authreq_expired").await?)
        .components(vec![]);
    newmsg.execute(&ctx, &cont_token).await?;
    Ok(())
}

/// Authenticate to your Wikimedia account
#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    let crate::Data {
        client, config, db, ..
    } = ctx.data();
    // use the reqwest client, or else
    ctx.defer_ephemeral().await?;
    let user_id = ctx.author().id;
    let guild_id = ctx.guild_id().unwrap();
    let db2 = db.clone();
    let db = db.in_guild(guild_id);

    if db
        .is_user_authed_in_server(user_id.get())
        .await?
    {
        ctx.reply(db.get_message("auth_exists_in_server").await?)
            .await?;
        if let Ok(authenticated_role) = db.authenticated_role_id().await {
            ctx.author_member().await.unwrap().add_role(ctx, authenticated_role).await?;
        }
        return Ok(());
    }

    let cont_token = match ctx {
        Context::Prefix(_) => unreachable!(),
        Context::Application(appctx) => appctx.interaction.token.clone(),
    };
    db.record_auth_message(user_id.into(), &cont_token).await?;

    if let Some(wikimedia_id) = db.get_wikimedia_id(user_id.get()).await? {
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
        let reply = CreateReply::default().content(msg)
            .components(vec![
                CreateActionRow::Buttons(vec![
                    CreateButton::new("yes").label(yes).style(ButtonStyle::Success), CreateButton::new("no").label(no).style(ButtonStyle::Danger)
                ]
            )]);
        let msg = ctx.send(reply).await?.into_message().await?;
        let ctx = ctx.serenity_context().clone();
        let rxns = msg.await_component_interaction(&ctx);
        spawn(async move {
            let db2 = db2.in_guild(guild_id);
            if let Err(e) = handle_interactions(ctx, user_id, db2.in_guild(guild_id), rxns, wikimedia_id, name, cont_token).await {
                tracing::error!(?e, "Error occured while handling interactions.");
            }
        });

        return Ok(()); // TODO move this somewhere else
    }

    // TODO implement expiry message (for people that actually have not authed) here.
    let authreq = AuthRequest::new(user_id.into(), guild_id.into());
    let state = authreq.state();
    db.record_auth_req(&state, user_id.into(), guild_id.into())
        .await?;
    // https://www.mediawiki.org/wiki/OAuth/For_Developers
    let client_id = &*config.oauth_consumer_key;
    let url = format!("https://meta.wikimedia.org/w/rest.php/oauth2/authorize?response_type=code&client_id={client_id}&state={state}");

    let auth = msg!(db, "auth", url = url)?;
    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .color(0xCCCCCC)
                    .title(db.get_message("bot").await?)
                    .description(auth)
                    .thumbnail("https://cdn.discordapp.com/emojis/546848856650809344.png")
                    .footer(CreateEmbedFooter::new("This link will be valid for 10 minutes."))
            )
    ).await?;

    Ok(())
}
