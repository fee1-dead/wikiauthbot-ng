use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Duration;

use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponseBuilder, HttpServer, Responder};
use dashmap::DashMap;
use tokio::sync::mpsc::{Receiver, Sender};
use reqwest::{Client, ClientBuilder};
use wikiauthbot_common::{AuthRequest, AuthRequestsMap};

#[derive(serde::Deserialize)]
struct AuthRequestQuery {
    state: String,
    error: Option<String>,
    error_description: Option<String>,
    message: Option<String>,
    code: String,
}

struct State {
    in_progress: AuthRequestsMap,
    client: Client,
    // when we are done verifying the auth request, return discord user id, global user id, and current username.
    successful_auths: Sender<(u64, u32, String)>,
}

#[get("/authorize")]
async fn authorize(
    web::Query(AuthRequestQuery {
        state,
        error,
        error_description,
        message,
        code,
    }): web::Query<AuthRequestQuery>,
    app_state: web::Data<State>,
) -> impl Responder {
    if let Some(error) = error {
        let message = message.or(error_description).unwrap_or(error);
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(format!("Error: {message}"));
    }

    let Some(auth_req) = app_state.in_progress.get_auth_req(&state) else {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND)
            .body(format!("Auth request was expired or invalid"));
    };

    let mut params = &[
        ("grant_type", "authorization_code"),
        ("code", &code),

    ];

    app_state.client.post("https://meta.wikimedia.org/w/rest.php/oauth2/access_token").form(params).send().await;

    todo!()
}

#[must_use]
pub async fn start(
    mut new_auth_reqs: Receiver<AuthRequest>,
    // when we are done verifying the auth request, return discord user id, global user id, and current username.
    successful_auths: Sender<(u64, u32, String)>,
) -> color_eyre::Result<Server> {
    let state = Arc::new(State {
        in_progress: AuthRequestsMap::new(),
        client: ClientBuilder::new().build()?,
        successful_auths,
    });
    let state2 = state.clone();
    tokio::spawn(async move {
        while let Some(auth) = new_auth_reqs.recv().await {
            state2.in_progress.add_auth_req(auth);
        }
    });
    let server = HttpServer::new(move || App::new().app_data(state.clone()).service(authorize))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
