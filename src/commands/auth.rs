use std::time::Duration;

use color_eyre::eyre::bail;
use poise::CreateReply;
use serde_json::Value;
use serenity::all::{
    Builder, ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse,
};
use serenity::builder::{CreateEmbed, CreateEmbedFooter, EditInteractionResponse};
use tokio::spawn;
use tokio::time::timeout;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth};

use crate::{Context, Result};

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

    if db
        .is_user_authed_in_server(user_id.get(), guild_id.get())
        .await?
    {
        ctx.reply("You are already authenticated to this server. No need to authenticate again.")
            .await?;
        // TODO we might reassign roles as a recheck
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
        let reply = CreateReply::default().content(format!("You are already identified as [{name}]({url}). Would you like to authenticate this to the server?"))
            .components(vec![
                CreateActionRow::Buttons(vec![
                    CreateButton::new("yes").label("Yes").style(ButtonStyle::Success), CreateButton::new("no").label("No").style(ButtonStyle::Danger)
                ]
            )]);
        let msg = ctx.send(reply).await?.into_message().await?;
        let ctx = ctx.serenity_context().clone();
        let rxns = msg.await_component_interaction(&ctx);
        let db2 = db.clone();
        spawn(async move {
            if let Ok(Some(x)) = timeout(Duration::from_secs(120), rxns.next()).await {
                match &*x.data.custom_id {
                    "yes" => {
                        if let Err(e) = db2
                            .send_successful_req(SuccessfulAuth {
                                discord_user_id: user_id.into(),
                                guild_id: guild_id.into(),
                                central_user_id: wikimedia_id,
                                username: name,
                                brand_new: false,
                            })
                            .await
                        {
                            tracing::error!(%e, "couldn't send successful auth");
                            return;
                        }
                        if let Err(e) = x
                            .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                            .await
                        {
                            tracing::error!(%e, "couldn't respond");
                        }
                        return;
                    }
                    "no" => {
                        let newmsg = EditInteractionResponse::new()
                            .content("Authentication canceled.")
                            .components(vec![]);
                        if let Err(e) = newmsg.execute(&ctx, &cont_token).await {
                            tracing::error!(%e, "couldn't edit");
                            return;
                        }
                        if let Err(e) = x
                            .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                            .await
                        {
                            tracing::error!(%e, "couldn't respond");
                        }
                        return;
                    }
                    id => tracing::error!("invalid custom id: {id}"),
                }
            }

            let newmsg = EditInteractionResponse::new()
                .content("Authentication expired.")
                .components(vec![]);
            if let Err(e) = newmsg.execute(&ctx, &cont_token).await {
                tracing::error!(%e, "couldn't edit");
                return;
            }
        });

        return Ok(()); // TODO move this somewhere else
    }

    let authreq = AuthRequest::new(user_id.into(), guild_id.into());
    let state = authreq.state();
    db.record_auth_req(&state, user_id.into(), guild_id.into())
        .await?;
    // https://www.mediawiki.org/wiki/OAuth/For_Developers
    let client_id = &*config.oauth_consumer_key;
    let url = format!("https://meta.wikimedia.org/w/rest.php/oauth2/authorize?response_type=code&client_id={client_id}&state={state}");

    ctx.send(CreateReply::default().embed(CreateEmbed::new().color(0xCCCCCC).title("WikiAuthBot").description(format!(
        "Please use the following link to authenticate to your Wikimedia account: [Authenticate]({url})"
    )).thumbnail("https://cdn.discordapp.com/emojis/546848856650809344.png").footer(CreateEmbedFooter::new("This link will be valid for 10 minutes.")))).await?;

    Ok(())
}
