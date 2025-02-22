use std::collections::HashSet;
use std::ops::ControlFlow;

use color_eyre::eyre::Context;
use serenity::all::{Http, RoleId, UserId};
use wikiauthbot_common::mwclient_with_url;
use wikiauthbot_db::{DatabaseConnectionInGuild, msg};

use super::fetch::EmbeddableWhois;
use crate::Result;

pub(crate) async fn check_blocks(
    http: &Http,
    db: &DatabaseConnectionInGuild<'_>,
    discord_user_id: UserId,
    whois: &EmbeddableWhois,
) -> Result<ControlFlow<()>> {
    if db.disallows_block_status(whois.blocked) {
        let member = http
            .get_member(db.guild_id().into(), discord_user_id)
            .await?;

        if let Some(auth) = db.authenticated_role_id() {
            if member.roles.contains(&auth.into()) {
                http.remove_member_role(
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
    let guild_id = db.guild_id().into();
    let roles_before = {
        let member = http.get_member(guild_id, discord_user_id).await?;
        let member_roles = member.roles.into_iter().collect::<HashSet<_>>();
        let managed_roles = db
            .authenticated_role_id()
            .into_iter()
            .chain(db.role_rules().iter().flatten().map(|rule| rule.role_id))
            .map(RoleId::from)
            .collect::<HashSet<_>>();
        member_roles
            .intersection(&managed_roles)
            .copied()
            .collect::<HashSet<_>>()
    };

    let mut roles_after = HashSet::new();
    roles_after.extend(db.authenticated_role_id().map(RoleId::from));

    if let Some(rules) = db.role_rules() {
        for rule in rules {
            if !rule.implicit_api_url.is_empty() {
                let url = format!("{}/w/api.php", rule.implicit_api_url);
                let v = mwclient_with_url(&url)
                    .await?
                    .get_value(&[
                        ("action", "query"),
                        ("list", "users"),
                        ("ususers", &whois.name),
                        ("usprop", "groups"),
                    ])
                    .await
                    .wrap_err("querying API")?["query"]["users"][0]["groups"]
                    .take();

                let groups: Vec<String> = serde_json::from_value(v)?;

                if groups.contains(&rule.group_name) {
                    roles_after.insert(rule.role_id.into());
                }
            } else if (rule.wiki == "*" || rule.wiki == "global")
                && whois.groups.contains(&rule.group_name)
            {
                roles_after.insert(rule.role_id.into());
            } else {
                for wiki in &whois.wikis {
                    if wiki.wiki == rule.wiki && wiki.groups.contains(&rule.group_name) {
                        roles_after.insert(rule.role_id.into());
                    }
                }
            }
        }
    }

    let roles_to_add = roles_after.difference(&roles_before);

    for &role in roles_to_add {
        http.add_member_role(
            guild_id,
            discord_user_id,
            role,
            Some(&msg!(db, "adding_managed_role")?),
        )
        .await?;
    }

    for &role in roles_before.difference(&roles_after) {
        http.remove_member_role(
            guild_id,
            discord_user_id,
            role,
            Some(&msg!(db, "removing_managed_role")?),
        )
        .await?;
    }

    Ok(())
}
