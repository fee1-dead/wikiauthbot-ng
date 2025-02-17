use fred::error::{RedisError, RedisErrorKind};
use tokio::spawn;

use crate::Data;
use crate::commands::handle_successful_auth;

pub async fn init(ctx: &serenity::all::Context, u: &Data) -> color_eyre::Result<()> {
    let parent_db = u.db.clone();
    let db = parent_db.get_child().await?;
    let http = ctx.http.clone();

    spawn(async move {
        loop {
            let succ = match db.recv_successful_req().await {
                Ok(x) => x,
                Err(e) => {
                    if let Some(re) = e.downcast_ref::<RedisError>() {
                        if let RedisErrorKind::Timeout = re.kind() {
                            continue;
                        }
                    }
                    tracing::error!(?e, "couldn't receive successful request");
                    continue;
                }
            };

            handle_successful_auth(succ, &http, &parent_db).await;
        }
    });
    Ok(())
}
