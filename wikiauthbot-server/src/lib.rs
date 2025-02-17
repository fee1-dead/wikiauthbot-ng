use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{App, HttpResponseBuilder, HttpServer, Responder, get, web};
use reqwest::{Client, ClientBuilder};
use wikiauthbot_common::Config;
use wikiauthbot_db::DatabaseConnection;

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
    // https://www.oauth.com/oauth2-servers/server-side-apps/example-flow/
}

#[derive(serde::Deserialize)]
struct UserProfileResponse {
    sub: String,
    username: String,
}

struct State {
    db: DatabaseConnection,
    client: Client,
    config: &'static Config,
}

#[get("/")]
async fn index(_app_state: web::Data<Arc<State>>) -> String {
    "nothing to see here".into()
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

    let Ok(Some(auth_req)) = app_state.db.get_auth_req(&state).await else {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND)
            .body("Auth request was expired or invalid.\nPlease contact dbeef on Discord if the problem persists.");
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
            .body("error while communicating with wikimedia server\nPlease contact dbeef on Discord if the problem persists.");
    };

    let Ok(AccessTokenResponse { access_token }) = res.json().await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while getting access token from server\nPlease contact dbeef on Discord if the problem persists.");
    };

    let Ok(res) = app_state
        .client
        .get("https://meta.wikimedia.org/w/rest.php/oauth2/resource/profile")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
    else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while retrieving user profile\nPlease contact dbeef on Discord if the problem persists.");
    };

    let Ok(UserProfileResponse { sub, username }) = res.json().await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("error while parsing user profile\nPlease contact dbeef on Discord if the problem persists.");
    };

    let Ok(sub) = sub.parse::<u32>() else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).body(
            "error while parsing user id\nPlease contact dbeef on Discord if the problem persists.",
        );
    };

    let lang = auth_req.language();
    let success_msg = wikiauthbot_common::i18n::msg!(lang, "server_auth_success")
        .unwrap_or_else(|_| "Success! Authorization information sent to the bot :)".into());

    let success = auth_req.into_successful(sub, username);
    let Ok(()) = app_state.db.send_successful_req(success).await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("failed to deliver successful auth request :(\nPlease contact dbeef on Discord if the problem persists.");
    };

    HttpResponseBuilder::new(StatusCode::OK).body(success_msg)
}

#[must_use]
pub async fn start(db: DatabaseConnection) -> color_eyre::Result<Server> {
    let state = Arc::new(State {
        db,
        client: ClientBuilder::new().build()?,
        config: Config::get()?,
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(authorize)
            .service(index)
    })
    .bind(("0.0.0.0", 8000))?
    .run();
    Ok(server)
}
