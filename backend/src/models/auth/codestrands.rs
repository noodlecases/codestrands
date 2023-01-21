use chrono::{DateTime, Utc};
use paste::paste;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

macro_rules! get_user_by {
    ($($field:ident: $type:ty),+) => {
        paste! {$(
            pub async fn [< get_by_ $field >]($field: $type, pool: &PgPool) -> Result<Self> {
                Ok(
                    sqlx::query_as::<_, Self>(concat!(
                        "SELECT * FROM users WHERE ",
                        stringify!($field),
                        " = $1"
                    ))
                    .bind($field)
                    .fetch_one(pool)
                    .await?,
                )
            }
        )+}
    };
}

macro_rules! update_user {
    ($($field:ident: $type:ty),+) => {
        paste! {$(
            pub async fn [< update_ $field >](id: i32, $field: $type, pool: &PgPool) -> Result<Self> {
                Ok(
                    sqlx::query_as::<_, Self>(concat!(
                        "UPDATE users SET ",
                        stringify!($field),
                        " = $1 WHERE id = $2 RETURNING *"
                    ))
                    .bind($field)
                    .bind(id)
                    .fetch_one(pool)
                    .await?,
                )
            }
        )+}
    };
}

impl User {
    get_user_by!(id: i32, email: &str, username: &str);
    update_user!(first_name: &str, last_name: &str, bio: Option<String>, username: &str);

    pub async fn insert(
        first_name: &str,
        last_name: &str,
        email: &str,
        username: &str,
        bio: Option<String>,
        image: Option<String>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                INSERT INTO users (
                    first_name,
                    last_name,
                    email,
                    username,
                    bio,
                    image
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
            ",
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(username)
        .bind(bio)
        .bind(image)
        .fetch_one(pool)
        .await?)
    }

    pub fn into_public(self) -> PublicUser {
        PublicUser {
            first_name: self.first_name,
            last_name: self.last_name,
            username: self.username,
            bio: self.bio,
            image: self.image,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(FromRow)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: String,
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
}

impl Account {
    pub async fn get(user_id: i32, provider: &str, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                SELECT * FROM accounts
                WHERE user_id = $1 AND provider = $2
            ",
        )
        .bind(user_id)
        .bind(provider)
        .fetch_one(pool)
        .await?)
    }

    pub async fn upsert(
        user_id: i32,
        provider: &str,
        provider_account_id: &str,
        refresh_token: &str,
        access_token: &str,
        expires_at: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                INSERT INTO accounts (
                    user_id,
                    provider,
                    provider_account_id,
                    refresh_token,
                    access_token,
                    expires_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (user_id, provider) DO UPDATE SET
                    refresh_token = $4,
                    access_token = $5,
                    expires_at = $6
                RETURNING *
            ",
        )
        .bind(user_id)
        .bind(provider)
        .bind(provider_account_id)
        .bind(refresh_token)
        .bind(access_token)
        .bind(expires_at)
        .fetch_one(pool)
        .await?)
    }

    pub async fn update(
        user_id: i32,
        provider: &str,
        refresh_token: &str,
        access_token: &str,
        expires_at: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                UPDATE accounts
                SET
                    refresh_token = $3,
                    access_token = $4,
                    expires_at = $5
                WHERE
                    user_id = $1 AND provider = $2
                RETURNING *
            ",
        )
        .bind(user_id)
        .bind(provider)
        .bind(refresh_token)
        .bind(access_token)
        .bind(expires_at)
        .fetch_one(pool)
        .await?)
    }
}
