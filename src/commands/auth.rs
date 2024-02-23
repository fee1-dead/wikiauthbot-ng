use std::time::Duration;

use color_eyre::eyre::bail;
use poise::CreateReply;
use serde_json::Value;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse, EditMessage,
};
use serenity::builder::{Builder, CreateEmbed, CreateEmbedFooter, EditInteractionResponse};
use tokio::spawn;
use tokio::time::timeout;
use wikiauthbot_common::{AuthRequest, SuccessfulAuth};

use crate::{Context, Result};

// TODO error handling that doesn't crash the bot
#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    let crate::Data {
        client,
        new_auth_reqs_send,
        config,
        ongoing_auth_requests,
        db,
        successful_auths_send,
        ..
    } = ctx.data();
    // use the reqwest client, or else
    ctx.defer_ephemeral().await?;
    let user_id = ctx.author().id;
    let guild_id = ctx.guild_id().unwrap();

    if db
        .is_user_authenticated_in_server(user_id.get(), guild_id.get())
        .await?
    {
        ctx.reply("You are already authenticated to this server. No need to authenticate again.")
            .await?;
        // TODO we might reassign roles as a recheck
        return Ok(());
    }

    if let Some(user) = db.find_user(user_id.get()).await? {
        let mut val: Value = client
            .get([
                ("action", "query"),
                ("meta", "globaluserinfo"),
                ("guiid", &user.wikimedia_id.to_string()),
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
        let mut msg = ctx.send(reply).await?.into_message().await?;
        let cont_token = match ctx {
            Context::Prefix(_) => unreachable!(),
            Context::Application(appctx) => appctx.interaction.token.clone(),
        };
        let ctx = ctx.serenity_context().clone();
        let rxns = msg.await_component_interaction(&ctx);
        let ongoing = ongoing_auth_requests.clone();
        let successful_auths_send = successful_auths_send.clone();
        spawn(async move {
            if let Ok(Some(x)) = timeout(Duration::from_secs(120), rxns.next()).await {
                match &*x.data.custom_id {
                    "yes" => {
                        ongoing.insert(user_id, cont_token);
                        if let Err(e) = successful_auths_send
                            .send(SuccessfulAuth {
                                discord_user_id: user_id.into(),
                                guild_id: guild_id.into(),
                                central_user_id: user.wikimedia_id,
                                username: name.into_boxed_str(),
                                brand_new: false,
                            })
                            .await
                        {
                            tracing::error!(%e, "couldn't send successful auth")
                        }
                        if let Err(e) = x
                            .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                            .await
                        {
                            println!("couldn't respond {e}");
                            return; // TODO error handling needs a lot more improvement
                        }
                    }
                    "no" => {
                        let newmsg = EditMessage::new()
                            .content("Authentication canceled.")
                            .components(vec![]);
                        if let Err(e) = msg.edit(&ctx, newmsg).await {
                            println!("couldn't edit {e}");
                            return;
                        }
                        if let Err(e) = x
                            .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                            .await
                        {
                            println!("couldn't respond {e}");
                            return; // TODO error handling needs a lot more improvement
                        }
                    }
                    id => tracing::error!("invalid custom id: {id}"),
                }
            }
            // TODO remember to clear the message after timeout
        });

        return Ok(()); // TODO move this somewhere else
    }

    let authreq = AuthRequest::new(user_id.into(), guild_id.into());
    let state = authreq.state();
    new_auth_reqs_send.send(authreq).await?;
    // https://www.mediawiki.org/wiki/OAuth/For_Developers
    let client_id = &*config.oauth_consumer_key;
    let url = format!("https://meta.wikimedia.org/w/rest.php/oauth2/authorize?response_type=code&client_id={client_id}&state={state}");
    ctx.send(CreateReply::default().embed(CreateEmbed::new().color(0xCCCCCC).title("WikiAuthBot").description(format!(
        "Please use the following link to authenticate to your Wikimedia account: [Authenticate]({url})"
    )).thumbnail("https://cdn.discordapp.com/emojis/546848856650809344.png").footer(CreateEmbedFooter::new("This link will be valid for 10 minutes.")))).await?;
    let ongoing_map = ongoing_auth_requests.clone();
    let http = ctx.serenity_context().http.clone();
    let cont_token = match ctx {
        Context::Prefix(_) => unreachable!(),
        Context::Application(appctx) => appctx.interaction.token.clone(),
    };
    ongoing_auth_requests.insert(user_id, cont_token.clone());

    tokio::spawn(async move {
        // 5 minutes to expire the message
        tokio::time::sleep(Duration::from_secs(60 * 5)).await;
        if ongoing_map.remove(&user_id).is_some() {
            let edit = EditInteractionResponse::new()
                .content("Authentication request expired.")
                .embeds(vec![]);
            if let Err(e) = edit.execute(http, &cont_token).await {
                // TODO introduce tracing for logging
                eprintln!("error trying to edit original response: {e}");
            }
        }
    });
    Ok(())
}
