use chrono::{DateTime, Utc};
use paste::paste;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub created_at: DateTime<Utc>,
}

macro_rules! update_project {
    ($($field:ident: $type:ty),+) => {
        paste! {$(
            pub async fn [< update_ $field >](user_id: i32, id: i32, $field: $type, pool: &PgPool) -> Result<Self> {
                Ok(
                    sqlx::query_as::<_, Self>(concat!(
                        "UPDATE projects SET ",
                        stringify!($field),
                        " = $1 WHERE id = $2 and user_id = $3 RETURNING *"
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

impl Project {
    update_project!(name: &str, description: &str, url: &str, image: &str);

    pub async fn create(
        user_id: i32,
        name: &str,
        description: &str,
        url: &str,
        image: &str,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO projects (
                        user_id,
                        name,
                        description,
                        url,
                        image,
                    )
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING *
                ",
        )
        .bind(user_id)
        .bind(name)
        .bind(description)
        .bind(url)
        .bind(image)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(id: i32, pool: &PgPool) -> Result<Self> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM projects WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_user(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM projects WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as::<_, Self>("SELECT * FROM projects")
            .fetch_all(pool)
            .await?)
    }

    pub async fn delete(user_id: i32, id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM user_relationships WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
