PRAGMA foreign_keys = false;
begin;

CREATE TABLE IF NOT EXISTS guilds_new
(
    guild_id INTEGER PRIMARY KEY NOT NULL,
    welcome_channel_id INTEGER NOT NULL,
    auth_log_channel_id INTEGER NOT NULL,
    deauth_log_channel_id INTEGER NOT NULL,
    authenticated_role_id INTEGER NOT NULL,
    server_language TEXT NOT NULL,
    allow_banned_users BOOLEAN NOT NULL,
    whois_is_ephemeral BOOLEAN NOT NULL,
);

insert into guilds_new (
    guild_id,
    welcome_channel_id,
    auth_log_channel_id,
    deauth_log_channel_id,
    authenticated_role_id,
    server_language,
    allow_banned_users,
    true
) select
    guild_id,
    welcome_channel_id,
    auth_log_channel_id,
    deauth_log_channel_id,
    authenticated_role_id,
    server_language,
    allow_banned_users
from guilds;

drop table guilds;

alter table guilds_new rename to guilds;

PRAGMA foreign_keys_check;

commit;
PRAGMA foreign_keys = true;
