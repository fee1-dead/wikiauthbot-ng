ALTER TABLE guilds ADD COLUMN IF NOT EXISTS allow_partially_blocked_users BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE guilds ALTER COLUMN allow_partially_blocked_users DROP DEFAULT;

CREATE TABLE IF NOT EXISTS guild_roles
(
    guild_id BIGINT UNSIGNED NOT NULL,
    wiki VARCHAR(255) NOT NULL,
    group_name VARCHAR(255) NOT NULL,
    implicit_api_url VARCHAR(255) NOT NULL,
    role_id BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS guild_roles_guild_index ON guild_roles(guild_id);
CREATE INDEX IF NOT EXISTS guild_roles_role_index ON guild_roles(role_id);
