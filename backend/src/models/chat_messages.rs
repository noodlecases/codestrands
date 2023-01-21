use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub async fn get_by_chat(chat_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM chat_messages WHERE chat_id = $1")
                .bind(chat_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get(id: i32, pool: &PgPool) -> Result<Self> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM chat_messages WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn create(chat_id: i32, user_id: i32, content: &str, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO chat_messages (
                        chat_id,
                        user_id,
                        content
                    )
                    VALUES ($1, $2, $3)
                    ON CONFLICT (chat_id, user_id) DO NOTHING
                    RETURNING *
                ",
        )
        .bind(chat_id)
        .bind(user_id)
        .bind(content)
        .fetch_one(pool)
        .await?)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM chat_messages WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
