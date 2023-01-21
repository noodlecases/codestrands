use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::utils::Result;

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRelationship {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    #[sqlx(rename = "type")]
    pub rel_type: UserRelationshipType,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum UserRelationshipType {
    Rejected = 0,
    Matched = 1,
    UserOneInitiated = 2,
    UserTwoInitiated = 3,
}

impl UserRelationship {
    pub async fn create(
        user1_id: i32,
        user2_id: i32,
        rel_type: UserRelationshipType,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO user_relationships (
                        user1_id,
                        user2_id,
                        type
                    )
                    VALUES ($1, $2, $3)
                    RETURNING *
                ",
        )
        .bind(user1_id)
        .bind(user2_id)
        .bind(rel_type)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as::<_, Self>(
            "SELECT * FROM user_relationships WHERE user1_id = $1 OR user2_id = $1",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete(user1_id: i32, user2_id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query(
            "
                DELETE FROM user_relationships
                WHERE
                    (user1_id = $1 AND user2_id = $1) OR
                    (user1_id = $2 AND user2_id = $1)
            ",
        )
        .bind(user1_id)
        .bind(user2_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
