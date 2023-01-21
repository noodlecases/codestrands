use chrono::{DateTime, Utc};
use paste::paste;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

macro_rules! update_social_link {
    ($($field:ident: $type:ty),+) => {
        paste! {$(
            pub async fn [< update_ $field >](id: i32, user_id: i32, $field: $type, pool: &PgPool) -> Result<Self> {
                Ok(
                    sqlx::query_as::<_, Self>(concat!(
                        "UPDATE social_links SET ",
                        stringify!($field),
                        " = $1 WHERE id = $2 and user_id = $3 RETURNING *"
                    ))
                    .bind($field)
                    .bind(id)
                    .bind(user_id)
                    .fetch_one(pool)
                    .await?,
                )
            }
        )+}
    };
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialLink {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SocialLink {
    update_social_link!(name: &str, url: &str);

    pub async fn create(name: &str, url: &str, user_id: i32, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO social_links (
                        user_id,
                        name,
                        url
                    )
                    VALUES ($1, $2, $3)
                    RETURNING *
                ",
        )
        .bind(name)
        .bind(url)
        .bind(user_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM social_links WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn delete(id: i32, user_id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM social_links WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
