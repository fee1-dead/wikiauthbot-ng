use crate::{Command, Context, Result};

mod auth;
mod whois;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result {
    let is_bot_owner = ctx.framework().options().owners.contains(&ctx.author().id);
    if !is_bot_owner {
        // silent fail
        return Ok(());
    }
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub fn all_commands() -> Vec<Command> {
    vec![register(), auth::auth(), whois::whois(), whois::whois_test()]
}
