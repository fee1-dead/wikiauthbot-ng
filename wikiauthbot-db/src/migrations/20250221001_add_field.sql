ALTER TABLE guilds ADD allow_partially_blocked_users BOOLEAN NOT NULL CONSTRAINT tbl_temp_default DEFAULT 1;
ALTER TABLE guilds drop constraint tbl_temp_default;
