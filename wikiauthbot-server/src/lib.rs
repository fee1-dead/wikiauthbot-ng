use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponseBuilder};
use dashmap::DashMap;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(serde::Deserialize)]
struct AuthRequest {
    state: String,
    error: Option<String>,
    error_description: Option<String>,
    message: Option<String>,
    code: String,
}

struct State {
    // key: random state, value: discord user id
    in_progress: DashMap<[u8; 28], u64>,
    // when we are done verifying the auth request, return discord user id, global user id, and current username.
    successful_auths: Sender<(u64, u32, String)>,
}

#[get("/authorize")]
async fn authorize(web::Query(AuthRequest {
    state,
    error,
    error_description,
    message,
    code,
}): web::Query<AuthRequest>) -> impl Responder {
    if let Some(error) = error {
        let message = message.or(error_description).unwrap_or(error);
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(format!("Error: {message}"));
    }



    todo!()
}

#[must_use]
pub async fn start(
    mut new_auth_reqs: Receiver<([u8; 28], u64)>,
    // when we are done verifying the auth request, return discord user id, global user id, and current username.
    successful_auths: Sender<(u64, u32, String)>,
) -> std::io::Result<Server> {
    let state = Arc::new(State {
        in_progress: DashMap::new(),
        successful_auths,
    });
    let state2 = state.clone();
    tokio::spawn(async move {
        while let Some((state, user_id)) = new_auth_reqs.recv().await {
            state2.in_progress.insert(state, user_id);
            let st = state2.clone();
            tokio::spawn(async move {
                // 30 minutes timeout
                tokio::time::sleep(Duration::from_secs(60 * 30)).await;
                st.in_progress.remove(&state)
            });
        }
    });
    let server = HttpServer::new(move || App::new().app_data(state.clone()).service(authorize))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}