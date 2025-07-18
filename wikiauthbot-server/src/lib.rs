use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::{Arc, LazyLock};

use actix_web::dev::Server;
use actix_web::error::ParseError;
use actix_web::http::StatusCode;
use actix_web::http::header::{self, ContentType, Header, HeaderName, TryIntoHeaderValue};
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

struct Authorization {
    pub value: String,
}

impl TryIntoHeaderValue for Authorization {
    type Error = header::InvalidHeaderValue;
    fn try_into_value(self) -> Result<header::HeaderValue, Self::Error> {
        header::HeaderValue::from_str(&self.value)
    }
}

impl Header for Authorization {
    fn name() -> HeaderName {
        header::AUTHORIZATION
    }
    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
        msg.headers()
            .get(Authorization::name())
            .ok_or(ParseError::Header)
            .and_then(|value| value.to_str().map_err(|_| ParseError::Header))
            .map(|s| Authorization {
                value: s.to_string(),
            })
    }
}

// see keygen for how this is generated. The values represent discord guild ids.
static API_KEYS: LazyLock<HashMap<[u8; 32], NonZeroU64>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        [
            37, 185, 6, 114, 198, 12, 186, 150, 80, 141, 25, 56, 53, 8, 38, 130, 63, 240, 244, 116,
            25, 29, 241, 52, 182, 27, 251, 180, 99, 99, 5, 229,
        ],
        NonZeroU64::new(221049808784326656).unwrap(),
    );
    map
});

#[get("/whois/{discord_id}")]
async fn whois(auth: web::Header<Authorization>, discord_id: web::Path<u64>, app_state: web::Data<Arc<State>>) -> impl Responder {
    use sha3::{Sha3_256, Digest};
    let mut hasher = Sha3_256::new();
    hasher.update(&auth.0.value);
    let hash: [u8; 32] = hasher.finalize().into();
    let Some(&discord_guild_id) = API_KEYS.get(&hash) else {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).finish();
    };
    let guild = app_state.db.in_guild(discord_guild_id);
    if !guild.has_server_settings() {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    }
    match guild.whois(discord_id.into_inner()).await {
        Ok(Some(x)) => {
            HttpResponseBuilder::new(StatusCode::OK).body(format!("{}", x.wikimedia_id))
        }
        Ok(None) => {
            HttpResponseBuilder::new(StatusCode::OK).body("not found")
        }
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish()
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK)
        .content_type(ContentType::html())
        .body(r#"See <a href="/ps">Privacy Statement</a>"#)
}

#[get("/ps")]
async fn privacy_statement() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("privacy_statement.html"))
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
        return (
            Cow::from(format!("Error: {message}")),
            StatusCode::BAD_REQUEST,
        );
    }

    let Ok(Some(auth_req)) = app_state.db.get_auth_req(&state).await else {
        return ("Auth request was expired or invalid.\nPlease contact beef.w on Discord if the problem persists.".into(), StatusCode::NOT_FOUND);
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
        return ("error while communicating with wikimedia server\nPlease contact beef.w on Discord if the problem persists.".into(), StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(AccessTokenResponse { access_token }) = res.json().await else {
        return ("error while getting access token from server\nPlease contact beef.w on Discord if the problem persists.".into(), StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(res) = app_state
        .client
        .get("https://meta.wikimedia.org/w/rest.php/oauth2/resource/profile")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
    else {
        return ("error while retrieving user profile\nPlease contact beef.w on Discord if the problem persists.".into(), StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(UserProfileResponse { sub, username }) = res.json().await else {
        return ("error while parsing user profile\nPlease contact beef.w on Discord if the problem persists.".into(), StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(sub) = sub.parse::<u32>() else {
        return (
            "error while parsing user id\nPlease contact beef.w on Discord if the problem persists.".into(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    };

    let lang = auth_req.language();
    let success_msg = wikiauthbot_common::i18n::msg!(lang, "server_auth_success").unwrap();

    let success = auth_req.into_successful(sub, username);
    let Ok(()) = app_state.db.send_successful_req(success).await else {
        return ("failed to deliver successful auth request :(\nPlease contact beef.w on Discord if the problem persists.".into(), StatusCode::INTERNAL_SERVER_ERROR);
    };

    (success_msg, StatusCode::OK)
}

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
            .service(privacy_statement)
    })
    .bind(("0.0.0.0", 8000))?
    .run();
    Ok(server)
}
