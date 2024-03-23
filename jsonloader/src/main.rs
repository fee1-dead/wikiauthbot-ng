// historical file that loads the old json data and inserts it into the database.

use std::collections::HashMap;
use std::fs::File;

use serde_json::Value;
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

async fn main_inner() -> color_eyre::Result<()> {
    let auth: AuthJson = serde_json::from_reader(File::open("/home/beef/Downloads/auth3.json")?)?;
    let auth = auth
        ._default
        .into_iter()
        .map(|(_, x)| x)
        .collect::<Vec<_>>();
    let (send_1, mut recv_1) = tokio::sync::mpsc::channel::<AuthUser>(10);
    let (send_2, mut recv_2) = tokio::sync::mpsc::channel::<(u64, u32)>(10);
    let db = DatabaseConnection::prod().await.unwrap();
    let db2 = db.clone();
    db.get_wikimedia_id(468253584421552139).await.unwrap();

    let a = tokio::task::spawn(async move {
        for u in auth.into_iter() {
            match db.get_wikimedia_id(u.id).await {
                Ok(Some(_)) => continue,
                Ok(None) => send_1.send(u).await.unwrap(),
                Err(_) => panic!("error while checking if they exist"),
            };
        }
    });

    let b = tokio::task::spawn(async move {
        let client = wikiauthbot_common::mwclient().await.unwrap();
        {
            let mut val: Value = client
                .get([
                    ("action", "query"),
                    ("meta", "globaluserinfo"),
                    ("guiuser", "0xDeadbeef"),
                ])
                .await
                .unwrap();
            let Value::Number(test_id) = val["query"]["globaluserinfo"]["id"].take() else {
                panic!("something is wrong");
            };
            assert_eq!(Some(64272802), test_id.as_u64());
            println!("uwu");
        }

        let mut failed = vec![];
        while let Some(u) = recv_1.recv().await {
            let mut val: Value = match client
                .get([
                    ("action", "query"),
                    ("meta", "globaluserinfo"),
                    ("guiuser", &u.wnam),
                ])
                .await
            {
                Ok(val) => val,
                Err(e) => {
                    println!("error: {e}");
                    failed.push(u);
                    continue;
                }
            };

            let Some(id) = val["query"]["globaluserinfo"]["id"].take().as_u64() else {
                println!("not a number");
                failed.push(u);
                continue;
            };
            send_2.send((u.id, id as u32)).await.unwrap();
        }
        serde_json::to_writer(File::create("errored2.json").unwrap(), &failed).unwrap();
    });

    let c = tokio::task::spawn(async move {
        while let Some((discord_id, wikimedia_id)) = recv_2.recv().await {
            db2.wmf_auth(discord_id, wikimedia_id).await.unwrap();
            println!("done({discord_id} -> {wikimedia_id})");
        }
    });

    let (a, b, c) = tokio::join!(a, b, c);

    a?;
    b?;
    c?;

    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(main_inner())?;
    Ok(())
}
