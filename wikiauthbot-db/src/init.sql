CREATE TABLE IF NOT EXISTS users
(
    discord_id INTEGER PRIMARY KEY NOT NULL,
    wikimedia_id INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS wikimedia_id_index ON users(wikimedia_id);

CREATE TABLE IF NOT EXISTS guilds
(
    guild_id INTEGER PRIMARY KEY NOT NULL,
    welcome_channel_id INTEGER NOT NULL,
    auth_log_channel_id INTEGER NOT NULL,
    deauth_log_channel_id INTEGER NOT NULL,
    authenticated_role_id INTEGER NOT NULL,
    server_language TEXT NOT NULL,
    allow_banned_users BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS auths
(
    guild_id INTEGER,
    user_id INTEGER,
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (discord_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS auths_index ON auths(user_id, guild_id);
