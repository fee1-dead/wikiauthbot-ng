// tool used for translating data between places
/*
use std::collections::HashMap;
use std::time::Instant;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
// use fred::types::Scanner;
// use futures::StreamExt;
use sqlx::{MySqlPool, QueryBuilder, Row, SqlitePool};
use wikiauthbot_db::DatabaseConnection;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthJson {
    _default: HashMap<String, AuthUser>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthUser {
    pub id: u64,
    pub wnam: String,
}

pub async fn sqlite_to_mariadb(sqlite: SqlitePool, sql: MySqlPool) -> color_eyre::Result<()> {
    let txn = sql.begin().await?;
    /*println!("-- users table start");
    let instant = Instant::now();
    let rows = sqlx::query("select discord_id, wikimedia_id from users").fetch_all(&sqlite).await?;
    println!("-- users table fetched - {:?} - {}", instant.elapsed(), rows.len());
    // sqlx::query("delete from users").execute(&sql).await?;
    let instant = Instant::now();
    for chunk in rows.chunks(10000) {
        QueryBuilder::new("insert into users ").push_values(chunk, |mut s, row| {
            let discord_id: i64 = row.get(0);
            let discord_id = discord_id as u64;
            let wikimedia_id: u32 = row.get(1);
            s.push_bind(discord_id).push_bind(wikimedia_id);
        }).build().execute(&sql).await?;
        println!("--- uwu");
    }
    println!("-- users table done - {:?}", instant.elapsed());*/

    println!("-- guilds table start");
    let instant = Instant::now();
    let rows = sqlx::query("select
        guild_id, welcome_channel_id, auth_log_channel_id,
        deauth_log_channel_id, authenticated_role_id,
        server_language, allow_banned_users, whois_is_ephemeral
    from guilds").fetch_all(&sqlite).await?;
    println!("-- guilds table fetched - {:?}", instant.elapsed());
    let instant = Instant::now();

    for row in rows {
        macro_rules! fetch_u64 {
            ($name:ident) => {
                let $name: i64 = row.get(stringify!($name));
                let $name = $name as u64;
            };
        }
        macro_rules! fetch {
            ($name:ident: $ty:ty) => {
                let $name: $ty = row.get(stringify!($name));
            };
        }
        fetch_u64!(guild_id);
        fetch_u64!(welcome_channel_id);
        fetch_u64!(auth_log_channel_id);
        fetch_u64!(deauth_log_channel_id);
        fetch_u64!(authenticated_role_id);
        fetch!(server_language: String);
        fetch!(allow_banned_users: bool);
        fetch!(whois_is_ephemeral: bool);

        sqlx::query("insert into guilds values(?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(guild_id).bind(welcome_channel_id)
            .bind(auth_log_channel_id).bind(deauth_log_channel_id)
            .bind(authenticated_role_id).bind(server_language)
            .bind(allow_banned_users).bind(whois_is_ephemeral)
            .execute(&sql).await?;
    }
    println!("-- guilds table done - {:?}", instant.elapsed());

    println!("-- auths table start");
    let instant = Instant::now();
    let rows = sqlx::query("select guild_id, user_id from auths").fetch_all(&sqlite).await?;
    println!("-- auths table fetched - {:?} - {}", instant.elapsed(), rows.len());
    let instant = Instant::now();
    for chunk in rows.chunks(10000) {
        QueryBuilder::new("insert into auths ").push_values(chunk, |mut s, row| {
            let guild_id: i64 = row.get(0);
            let guild_id = guild_id as u64;
            let user_id: i64 = row.get(1);
            let user_id = user_id as u64;
            s.push_bind(guild_id).push_bind(user_id);
        }).build().execute(&sql).await?;
        println!("--- uwu");
    }
    println!("-- auths table done - {:?}", instant.elapsed());

    txn.commit().await?;

    println!("Phew! All done.");

    Ok(())
}*/

async fn main_inner() -> color_eyre::Result<()> {
    /*println!("Connecting to sqlite and writing to mariadb..");
    let sql = DatabaseConnection::connect_mysql().await?;
    sqlx::migrate!("../wikiauthbot-db/src/migrations").run(&sql).await?;
    let options = SqliteConnectOptions::new().filename("wikiauthbot-prod.db");
    let sqlite = SqlitePoolOptions::new()
    .max_connections(100)
    .test_before_acquire(false)
    .connect_with(options)
    .await?;
    sqlite_to_mariadb(sqlite, sql).await?;*/
    // redis.build_revauth().await?;
    // load_from_json().await?;
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(main_inner())?;
    Ok(())
}
