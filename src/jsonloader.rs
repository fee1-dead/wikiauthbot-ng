use std::fs::File;
use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use wikiauthbot_db::Database;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthUser {
    pub id: u64,
    pub wnam: String,
}

async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
    let auth: Vec<AuthUser> = serde_json::from_reader(File::open("auth2.json")?)?;

    let (send_1, mut recv_1) = tokio::sync::mpsc::channel::<AuthUser>(10);
    let (send_2, mut recv_2) = tokio::sync::mpsc::channel::<(u64, u32)>(10);
    let db = Arc::new(Database::connect().await.unwrap());
    let db2 = db.clone();

    let a = tokio::task::spawn(async move {
        for u in auth.into_iter().skip(4399) {
            match db.find_user(u.id).await {
                Ok(Some(_)) => continue,
                Ok(None) => send_1.send(u).await.unwrap(),
                Err(_) => panic!("error while checking if they exist"),
            };
        }
    });

    let b = tokio::task::spawn(async move {
        let replica = sea_orm::Database::connect(
            dotenvy::var("DATABASE2_URL").expect("expected DATABASE2_URL to be set"),
        )
        .await
        .unwrap();
        {
            let test_id = replica
                .query_one(Statement::from_sql_and_values(
                    DatabaseBackend::MySql,
                    "SELECT gu_id from globaluser where gu_name = ?",
                    [sea_orm::Value::String(Some(Box::new(
                        "0xDeadbeef".to_owned(),
                    )))],
                ))
                .await
                .unwrap()
                .unwrap()
                .try_get_by::<i32, _>("gu_id")
                .unwrap();
            assert_eq!(64272802, test_id);
            println!("uwu");
        }

        let mut failed = vec![];
        while let Some(u) = recv_1.recv().await {
            let res = replica
                .query_one(Statement::from_sql_and_values(
                    DatabaseBackend::MySql,
                    "SELECT gu_id from globaluser where gu_name = ?",
                    [sea_orm::Value::String(Some(Box::new(u.wnam.clone())))],
                ))
                .await
                .unwrap();
            let Some(res) = res else {
                println!("fail: {}", u.wnam);
                failed.push(u);
                continue;
            };
            let id = res.try_get_by::<i32, _>("gu_id").unwrap();
            send_2.send((u.id, id as u32)).await.unwrap();
        }
        serde_json::to_writer(File::create("errored2.json").unwrap(), &failed).unwrap();
    });

    let c = tokio::task::spawn(async move {
        while let Some((discord_id, wmf_id)) = recv_2.recv().await {
            db2.add_auth_user(discord_id, wmf_id).await.unwrap();
            println!("done({discord_id} -> {wmf_id})");
        }
    });

    let (a, b, c) = tokio::join!(a, b, c);

    a?;
    b?;
    c?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(main_inner())?;
    Ok(())
}
