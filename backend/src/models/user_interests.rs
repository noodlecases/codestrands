use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInterest {
    pub id: i32,
    pub user_id: i32,
    pub interest_id: i32,
    pub created_at: DateTime<Utc>,
}

impl UserInterest {
    pub async fn create(user_id: i32, interest_id: i32, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO user_interests (
                        user_id,
                        interest_id
                    )
                    VALUES ($1, $2)
                    ON CONFLICT (user_id, interest_id) DO NOTHING
                    RETURNING *
                ",
        )
        .bind(user_id)
        .bind(interest_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM user_interests WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn delete(user_id: i32, interest_id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM user_interests WHERE user_id = $1 AND interest_id = $2")
            .bind(user_id)
            .bind(interest_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
