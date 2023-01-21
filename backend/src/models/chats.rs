use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct Chat {
    pub id: i32,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Chat {
    pub async fn create(title: &str, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO chats (title)
                    VALUES ($1)
                    RETURNING *
                ",
        )
        .bind(title)
        .fetch_one(pool)
        .await?)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM chats WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get(id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM chats WHERE id = $1")
                .bind(id)
                .fetch_all(pool)
                .await?,
        )
    }
}
