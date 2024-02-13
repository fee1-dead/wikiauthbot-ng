use poise::CreateReply;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use wikiauthbot_common::AuthRequest;

use crate::{Context, Result};

// TODO error handling that doesn't crash the bot
#[poise::command(slash_command, guild_only = true)]
pub async fn auth(ctx: Context<'_>) -> Result {
    let crate::Data {
        new_auth_reqs_send, ..
    } = ctx.data();
    // use the reqwest client, or else
    ctx.defer_ephemeral().await?;
    let authreq = AuthRequest::new(ctx.author().id.get());
    let state = authreq.state();
    new_auth_reqs_send.send(authreq).await?;
    // https://www.mediawiki.org/wiki/OAuth/For_Developers
    let url = format!("https://meta.wikimedia.org/w/rest.php/oauth2/authorize?response_type=code&client_id=276b06c8d25935d414da530b26b1a827&state={state}");
    // TODO implement timeout
    ctx.send(CreateReply::default().embed(CreateEmbed::new().color(0xCCCCCC).title("WikiAuthBot").description(format!(
        "Please use the following link to authenticate to your Wikimedia account: [Authenticate]({url})"
    )).thumbnail("https://cdn.discordapp.com/emojis/546848856650809344.png").footer(CreateEmbedFooter::new("This link will be valid for 10 minutes.")))).await?;
    Ok(())
}
