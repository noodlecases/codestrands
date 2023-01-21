use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    models::auth::codestrands,
    utils::{chrono::duration_from_secs, Result},
};

const KEYCLOAK_TOKEN_URL: &str =
    "https://id.noodlecases.tech/realms/dev/protocol/openid-connect/token";
const KEYCLOAK_USER_URL: &str =
    "https://id.noodlecases.tech/realms/dev/protocol/openid-connect/userinfo";

#[derive(Deserialize)]
pub struct User {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub preferred_username: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(deserialize_with = "duration_from_secs")]
    pub expires_in: Duration,
    pub refresh_token: String,
}

impl User {
    pub async fn create_account(
        code: &str,
        client_id: &str,
        client_secret: &str,
        oauth_redirect_uri: &str,
        http_client: &reqwest::Client,
        pool: &PgPool,
    ) -> Result<codestrands::User> {
        let token_res = Self::fetch_token(
            code,
            client_id,
            client_secret,
            oauth_redirect_uri,
            http_client,
        )
        .await?;
        let keycloak_user = Self::fetch_user(&token_res.access_token, http_client).await?;

        let user = match codestrands::User::get_by_email(&keycloak_user.email, pool).await {
            Ok(user) => user,
            Err(_) => {
                codestrands::User::insert(
                    &keycloak_user.given_name,
                    &keycloak_user.family_name,
                    &keycloak_user.email,
                    &keycloak_user.preferred_username,
                    None,
                    None,
                    pool,
                )
                .await?
            }
        };

        codestrands::Account::upsert(
            user.id,
            "KEYCLOAK",
            &keycloak_user.sub,
            &token_res.refresh_token,
            &token_res.access_token,
            Utc::now() + token_res.expires_in,
            pool,
        )
        .await?;

        Ok(user)
    }

    pub async fn refresh_account(
        user_id: i32,
        client_id: &str,
        client_secret: &str,
        http_client: &reqwest::Client,
        pool: &PgPool,
    ) -> Result<()> {
        let account = codestrands::Account::get(user_id, "KEYCLOAK", pool).await?;
        let token_res = Self::refresh_token(
            &account.refresh_token,
            client_id,
            client_secret,
            http_client,
        )
        .await?;

        codestrands::Account::update(
            user_id,
            "KEYCLOAK",
            &token_res.refresh_token,
            &token_res.access_token,
            Utc::now() + token_res.expires_in,
            pool,
        )
        .await?;

        Ok(())
    }

    async fn refresh_token(
        refresh_token: &str,
        client_id: &str,
        client_secret: &str,
        http_client: &reqwest::Client,
    ) -> Result<TokenResponse> {
        Ok(http_client
            .post(KEYCLOAK_TOKEN_URL)
            .form(&json!({
                "client_id": client_id,
                "client_secret": client_secret,
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,
            }))
            .send()
            .await?
            .json::<TokenResponse>()
            .await?)
    }

    async fn fetch_token(
        code: &str,
        client_id: &str,
        client_secret: &str,
        oauth_redirect_uri: &str,
        http_client: &reqwest::Client,
    ) -> Result<TokenResponse> {
        Ok(http_client
            .post(KEYCLOAK_TOKEN_URL)
            .form(&json!({
                "client_id": client_id,
                "client_secret": client_secret,
                "grant_type": "authorization_code",
                "code": code,
                "redirect_uri": format!("{}/keycloak", oauth_redirect_uri),
                "scope": "openid id",
            }))
            .send()
            .await?
            .json::<TokenResponse>()
            .await?)
    }

    async fn fetch_user(access_token: &str, http_client: &reqwest::Client) -> Result<User> {
        Ok(http_client
            .get(KEYCLOAK_USER_URL)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<User>()
            .await?)
    }
}
