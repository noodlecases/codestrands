use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatUser {
    pub id: i32,
    pub user_id: i32,
    pub chat_id: i32,
    pub created_at: DateTime<Utc>,
}

impl ChatUser {
    pub async fn create(chat_id: i32, user_id: i32, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO chat_users (
                        chat_id,
                        user_id
                    )
                    VALUES ($1, $2)
                    ON CONFLICT (chat_id, user_id) DO NOTHING
                    RETURNING *
                ",
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_user(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM chat_users WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_by_chat(chat_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM chat_users WHERE chat_id = $1")
                .bind(chat_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn delete(chat_id: i32, user_id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM chat_users WHERE chat_id = $1 AND user_id = $2")
            .bind(chat_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
