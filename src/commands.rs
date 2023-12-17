use crate::{Context, Command, Result};

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

// TODO
pub fn all_commands() -> Vec<Command> {
    vec![]
}
