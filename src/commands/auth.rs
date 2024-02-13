use std::time::Duration;

use poise::CreateReply;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, EditMessage};
use wikiauthbot_common::AuthRequest;

use crate::{Context, Result};

// TODO error handling that doesn't crash the bot
#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    let crate::Data {
        new_auth_reqs_send, config, ongoing_auth_requests, ..
    } = ctx.data();
    // use the reqwest client, or else
    ctx.defer_ephemeral().await?;
    let user_id = ctx.author().id;
    let authreq = AuthRequest::new(user_id.get());
    let state = authreq.state();
    new_auth_reqs_send.send(authreq).await?;
    // https://www.mediawiki.org/wiki/OAuth/For_Developers
    let client_id = &*config.oauth_consumer_key;
    let url = format!("https://meta.wikimedia.org/w/rest.php/oauth2/authorize?response_type=code&client_id={client_id}&state={state}");
    // TODO implement timeout
    let msg = ctx.send(CreateReply::default().embed(CreateEmbed::new().color(0xCCCCCC).title("WikiAuthBot").description(format!(
        "Please use the following link to authenticate to your Wikimedia account: [Authenticate]({url})"
    )).thumbnail("https://cdn.discordapp.com/emojis/546848856650809344.png").footer(CreateEmbedFooter::new("This link will be valid for 10 minutes.")))).await?;
    let msg_id = msg.message().await?.id;
    ongoing_auth_requests.insert(user_id, msg_id);
    let channel = ctx.channel_id();
    let ongoing_map = ongoing_auth_requests.clone();
    let http = ctx.serenity_context().http.clone();
    tokio::spawn(async move {
        // TODO change this to longer
        tokio::time::sleep(Duration::from_secs(3)).await;
        if ongoing_map.remove(&user_id).is_some() {
            let _ = channel.edit_message(http, msg_id, EditMessage::new().content("Authentication request expired.")).await;
        }
    });
    Ok(())
}
