use serenity::all::{Channel, ChannelId, GuildId, UserId};

use crate::Context;

pub(super) async fn is_server_admin(ctx: Context<'_>, g: GuildId, channel: ChannelId, user: UserId) -> bool {
    let Ok(Channel::Guild(gc)) = channel.to_channel(ctx).await else {
        return false;
    };
    let Some(g) = ctx.cache().guild(g) else {
        return false;
    };
    let Some(member) = g.members.get(&user) else { return false };

    g.user_permissions_in(&gc, &member).administrator()
}
