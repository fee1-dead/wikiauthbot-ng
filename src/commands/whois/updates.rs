use std::ops::ControlFlow;

use serenity::all::{Http, UserId};
use wikiauthbot_db::{msg, DatabaseConnectionInGuild};

use super::fetch::EmbeddableWhois;
use crate::Result;

pub(crate) async fn check_blocks(
    http: &Http,
    db: &DatabaseConnectionInGuild<'_>,
    discord_user_id: UserId,
    whois: &EmbeddableWhois,
) -> Result<ControlFlow<()>> {
    if db.disallows_block_status(whois.blocked) {
        let member = http.get_member(db.guild_id().into(), discord_user_id)
            .await?;

        if let Some(auth) = db.authenticated_role_id() {
            if member.roles.contains(&auth.into()) {
                http
                    .remove_member_role(
                        member.guild_id,
                        member.user.id,
                        auth.into(),
                        Some(&msg!(db, "removed_blocked_user_roles")?),
                    )
                    .await?;
            }
        }
        Ok(ControlFlow::Break(()))
    } else {
        Ok(ControlFlow::Continue(()))
    }
}

pub(crate) async fn update_roles(
    http: &Http,
    db: &DatabaseConnectionInGuild<'_>,
    discord_user_id: UserId,
    whois: &EmbeddableWhois,
) -> Result {
    
    Ok(())
}
