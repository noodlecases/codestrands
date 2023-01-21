use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSkill {
    pub id: i32,
    pub user_id: i32,
    pub skill_id: i32,
    pub created_at: DateTime<Utc>,
}

impl UserSkill {
    pub async fn create(user_id: i32, skill_id: i32, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO user_skills (
                        user_id,
                        skill_id
                    )
                    VALUES ($1, $2)
                    ON CONFLICT (user_id, skill_id) DO NOTHING
                    RETURNING *
                ",
        )
        .bind(user_id)
        .bind(skill_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM user_skills WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_by_username(username: &str, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as::<_, Self>(
            "
                        SELECT *
                        FROM user_skills
                            JOIN users ON user_skills.user_id = users.id
                        WHERE users.username = $1
                     ",
        )
        .bind(username)
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete(user_id: i32, skill_id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM user_skills WHERE user_id = $1 AND skill_id = $2")
            .bind(user_id)
            .bind(skill_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
