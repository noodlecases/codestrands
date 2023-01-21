use actix_session::Session;
use actix_web::{
    get, post,
    web::{Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use openssl::base64;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::PgPool;
use url::Url;

use crate::{
    config::Config,
    models::auth::{codestrands, keycloak},
    utils::{error::codestrands_error, Result},
};

#[derive(Clone, Deserialize, Serialize)]
struct OAuthState {
    redirect: Option<String>,
}

#[derive(Deserialize)]
struct OAuthCallbackQuery {
    code: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_oauth_state")]
    state: Option<OAuthState>,
    error_description: Option<String>,
}

#[derive(Deserialize)]
struct OAuthLoginQuery {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_oauth_state")]
    state: Option<OAuthState>,
}

fn deserialize_oauth_state<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<OAuthState>, D::Error> {
    Ok(match <Option<String>>::deserialize(d)? {
        Some(v) => {
            let decoded = base64::decode_block(&v).map_err(serde::de::Error::custom)?;
            Some(serde_json::from_slice(&decoded).map_err(serde::de::Error::custom)?)
        }
        None => None,
    })
}

#[post("/auth/callback/keycloak/")]
async fn callback_keycloak(
    session: Session,
    query: Query<OAuthCallbackQuery>,
    config: Data<Config>,
    http_client: Data<reqwest::Client>,
    pool: Data<PgPool>,
) -> Result<Json<OAuthState>> {
    let user = match &query.code {
        Some(code) => {
            keycloak::User::create_account(
                code,
                &config.oauth.client_id,
                &config.oauth.client_secret,
                &config.oauth_redirect_uri,
                &http_client,
                &pool,
            )
            .await?
        }
        None => codestrands_error!(
            400,
            query
                .error_description
                .as_deref()
                .unwrap_or("failed to login with keycloak")
        ),
    };
    session.insert("user_id", user.id)?;
    session.insert("authenticated", true)?;
    Ok(Json(
        query.state.clone().unwrap_or(OAuthState { redirect: None }),
    ))
}

#[get("/auth/login/keycloak/")]
async fn login_keycloak(
    session: Session,
    query: Query<OAuthLoginQuery>,
    config: Data<Config>,
    http_client: Data<reqwest::Client>,
    pool: Data<PgPool>,
) -> Result<Json<OAuthState>> {
    if !session.get::<bool>("authenticated")?.unwrap_or(false) {
        let oauth_url = match &query.state {
            Some(state) => Url::parse_with_params(
                &config.oauth.oauth_url,
                &[(
                    "state",
                    base64::encode_block(serde_json::to_string(state)?.as_bytes()),
                )],
            )?
            .to_string(),
            None => config.oauth.oauth_url.clone(),
        };
        let login_redirect = Ok(Json(OAuthState {
            redirect: Some(oauth_url),
        }));

        let user_id = match session.get("user_id")? {
            Some(user_id) => user_id,
            None => return login_redirect,
        };
        let account = match codestrands::Account::get(user_id, "KEYCLOAK", &pool).await {
            Ok(account) => account,
            Err(_) => return login_redirect,
        };

        match keycloak::User::refresh_account(
            account.user_id,
            &config.oauth.client_id,
            &config.oauth.client_secret,
            &http_client,
            &pool,
        )
        .await
        {
            Ok(_) => {
                session.insert("authenticated", true)?;
                session.renew();
            }
            Err(_) => return login_redirect,
        }
    }

    Ok(Json(
        query.state.clone().unwrap_or(OAuthState { redirect: None }),
    ))
}

#[post("/auth/logout/")]
async fn logout(session: Session) -> HttpResponse {
    session.remove("authenticated");
    HttpResponse::Ok().finish()
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(callback_keycloak)
        .service(login_keycloak)
        .service(logout);
}
