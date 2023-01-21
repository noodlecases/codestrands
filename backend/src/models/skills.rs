use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl Skill {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM skills")
                .fetch_all(pool)
                .await?
        )
    }

    pub async fn create(name: &str, pool: &PgPool) -> Result<Self> {
        Ok(
            sqlx::query_as::<_, Self>(
                "
                    INSERT INTO skills (name)
                    VALUES ($1)
                    ON CONFLICT (name) DO NOTHING
                    RETURNING *
                "
            )
            .bind(name)
            .fetch_one(pool)
            .await?
        )
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM skills WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
