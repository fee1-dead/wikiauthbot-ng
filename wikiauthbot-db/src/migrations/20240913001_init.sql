CREATE TABLE IF NOT EXISTS users
(
    discord_id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
    wikimedia_id INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS wikimedia_id_index ON users(wikimedia_id);

CREATE TABLE IF NOT EXISTS guilds
(
    guild_id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
    welcome_channel_id BIGINT UNSIGNED NOT NULL,
    auth_log_channel_id BIGINT UNSIGNED NOT NULL,
    deauth_log_channel_id BIGINT UNSIGNED NOT NULL,
    authenticated_role_id BIGINT UNSIGNED NOT NULL,
    server_language VARCHAR(255) NOT NULL,
    allow_banned_users BOOLEAN NOT NULL,
    whois_is_ephemeral BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS auths
(
    guild_id BIGINT UNSIGNED,
    user_id BIGINT UNSIGNED,
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (discord_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS auths_index ON auths(user_id, guild_id);
