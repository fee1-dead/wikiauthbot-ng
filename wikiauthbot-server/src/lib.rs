use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponseBuilder, HttpServer, Responder};
use reqwest::{Client, ClientBuilder};
use tokio::sync::mpsc::{Receiver, Sender};
use wikiauthbot_common::{AuthRequest, AuthRequestsMap, Config, SuccessfulAuth};

#[derive(serde::Deserialize)]
struct AuthRequestQuery {
    state: String,
    error: Option<String>,
    error_description: Option<String>,
    message: Option<String>,
    code: String,
}

// https://www.mediawiki.org/wiki/OAuth/For_Developers#Identifying_the_user_2
#[derive(serde::Deserialize)]
struct AccessTokenResponse {
    access_token: Box<str>,
    // TODO do we need more fields?
    // https://www.oauth.com/oauth2-servers/server-side-apps/example-flow/
}

#[derive(serde::Deserialize)]
struct UserProfileResponse {
    sub: u32,
    username: Box<str>,
}

struct State {
    in_progress: AuthRequestsMap,
    client: Client,
    // when we are done verifying the auth request, return discord user id, global user id, and current username.
    successful_auths: Arc<Sender<SuccessfulAuth>>,
    config: &'static Config,
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
    app_state: web::Data<Arc<State>>,
) -> impl Responder {
    if let Some(error) = error {
        let message = message.or(error_description).unwrap_or(error);
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(format!("Error: {message}"));
    }

    let Some(auth_req) = app_state.in_progress.get_auth_req(&state) else {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND)
            .body("Auth request was expired or invalid");
    };

    let params = &[
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("client_id", &app_state.config.oauth_consumer_key),
        ("client_secret", &app_state.config.oauth_client_secret),
    ];

    let Ok(res) = app_state
        .client
        .post("https://meta.wikimedia.org/w/rest.php/oauth2/access_token")
        .form(params)
        .send()
        .await
    else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while communicating with wikimedia server");
    };

    let Ok(AccessTokenResponse { access_token }) = res.json().await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while getting access token from server");
    };

    let Ok(res) = app_state
        .client
        .get("https://meta.wikimedia.org/w/rest.php/oauth2/resource/profile")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
    else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while retrieving user profile");
    };

    let Ok(UserProfileResponse { sub, username }) = res.json().await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while parsing user profile");
    };

    let success = auth_req.into_successful(sub, username);
    let Ok(()) = app_state.successful_auths.send(success).await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("failed to deliver successful auth request :(");
    };

    HttpResponseBuilder::new(StatusCode::OK)
        .body("Success! Authorization information sent to the bot :)")
}

#[must_use]
pub async fn start(
    mut new_auth_reqs: Receiver<AuthRequest>,
    // when we are done verifying the auth request, return discord user id, global user id, and current username.
    successful_auths: Arc<Sender<SuccessfulAuth>>,
) -> color_eyre::Result<Server> {
    let state = Arc::new(State {
        in_progress: AuthRequestsMap::new(),
        client: ClientBuilder::new().build()?,
        successful_auths,
        // TODO pass this top down to avoid having a global
        config: Config::get()?,
    });
    let state2 = state.clone();
    tokio::spawn(async move {
        while let Some(auth) = new_auth_reqs.recv().await {
            state2.in_progress.add_auth_req(auth);
        }
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(authorize)
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}
