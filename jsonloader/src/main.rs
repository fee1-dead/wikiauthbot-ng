// historical file that loads the old json data and inserts it into the database.

use std::collections::HashMap;
use std::path::Path;

use fred::interfaces::{KeysInterface, SetsInterface};
// use fred::types::Scanner;
// use futures::StreamExt;
use sqlx::QueryBuilder;
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

pub async fn load_from_json() -> color_eyre::Result<()> {
    if !Path::new("wikiauthbot-prod.db").exists() {
        DatabaseConnection::create_sqlite().await?;
    }
    let db = DatabaseConnection::prod_tunnelled().await.unwrap();
    let (redis, sqlite) = db.into_parts();
    // let mut scan_keys = redis.scan("guilds:*:server_language", None, None);
    // let mut discord_ids = Vec::new();
    // let mut guild_ids = Vec::new();
    let guild_ids: Vec<u64> = vec![819432579013935125];
    /* while let Some(x) = scan_keys.next().await {
        let mut r = x?;

        let Some(res) = r.take_results() else { break };

        for key in res {
            let key = key.into_string().unwrap();
            if let Some(discord_id) = key.strip_prefix("auth:") {
                let discord_id = discord_id.parse::<u64>()?;
                discord_ids.push(discord_id);
            } else if let Some(guild_id) = key.strip_prefix("guilds:").and_then(|x| x.strip_suffix(":server_language")) {
                let guild_id = guild_id.parse::<u64>()?;
                guild_ids.push(guild_id);
            }
        }

        if !r.has_more() {
            break;
        }
        r.next()?;
    } */
    println!("obtained discord ids and guild ids. Moving on to stage 2..");
    /*
        let discord_id_auths: Vec<_> = discord_ids.iter().copied().map(|id| format!("auth:{id}")).collect();
        let authed: Vec<u32> = redis.mget(discord_id_auths).await?;
        println!("-- obtained wikimedia ids for all users from redis");
        let auth: Vec<_> = discord_ids.into_iter().zip(authed).collect();

        for chunk in auth.chunks(10000) {
            let mut q = QueryBuilder::new("INSERT INTO users ");
            q.push_values(chunk.iter().copied(), |mut q, (discord_id, wikimedia_id)| {
                q.push_bind(discord_id as i64).push_bind(wikimedia_id);
            });

            q.build().persistent(false).execute(&sqlite).await?;
        }
        println!("-- inserted everything to the users table");
    */
    let fields = [
        "welcome_channel_id",
        "auth_log_channel_id",
        "deauth_log_channel_id",
        "authenticated_role_id",
        "server_language",
        "allow_banned_users",
    ];
    dbg!(&guild_ids);
    for guild in guild_ids {
        println!("-- processing {guild}");
        let (
            welcome_channel_id,
            auth_log_channel_id,
            deauth_log_channel_id,
            authenticated_role_id,
            server_language,
            allow_banned_users,
        ): (u64, u64, u64, u64, String, bool) = redis
            .mget(fields.map(|s| format!("guilds:{guild}:{s}")).to_vec())
            .await?;
        sqlx::query("INSERT INTO guilds VALUES($1, $2, $3, $4, $5, $6, $7)")
            .bind(guild as i64)
            .bind(welcome_channel_id as i64)
            .bind(auth_log_channel_id as i64)
            .bind(deauth_log_channel_id as i64)
            .bind(authenticated_role_id as i64)
            .bind(server_language)
            .bind(allow_banned_users)
            .execute(&sqlite)
            .await?;
        println!("-- inserted info about {guild} into the guilds table");
        let authed_ids: Vec<u64> = redis.smembers(format!("guilds:{guild}:authed")).await?;
        for chunk in authed_ids.chunks(10000) {
            let mut q = QueryBuilder::new("INSERT INTO auths ");
            q.push_values(chunk.iter().copied(), |mut q, authed_id| {
                q.push_bind(guild as i64).push_bind(authed_id as i64);
            });

            q.build().persistent(false).execute(&sqlite).await?;
        }
        println!("-- inserted authed people from {guild} into the auths table");
    }

    println!("Phew! All done.");

    Ok(())
}

async fn main_inner() -> color_eyre::Result<()> {
    println!("Connecting to redis and writing to sqlite..");
    // let redis = DatabaseConnection::prod_tunnelled().await?;
    // redis.build_revauth().await?;
    load_from_json().await?;
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
