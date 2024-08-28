use serenity::all::{Context, GuildId, RoleId};
use serenity::futures::TryStreamExt;
use wikiauthbot_db::DatabaseConnectionInGuild;

pub async fn role_to_db(
    ctx: &Context,
    db: DatabaseConnectionInGuild<'_>,
    guild_id: GuildId,
    role_id: RoleId,
) -> color_eyre::Result<()> {
    assert!(db.has_server_settings());

    guild_id
        .members_iter(ctx)
        .map_err(color_eyre::Report::from)
        .try_for_each(|member| {
            let db = db.clone();
            async move {
                if member.roles.contains(&role_id) {
                    let discord_id = member.user.id.get();
                    if db.get_wikimedia_id(discord_id).await?.is_some() {
                        db.in_guild(guild_id).partial_auth(discord_id).await?;
                    }
                }
                Ok(())
            }
        })
        .await?;
    Ok(())
}
