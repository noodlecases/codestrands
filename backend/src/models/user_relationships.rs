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
    pub rel_type: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRelationshipType {
    Rejected = 0,
    Matched = 1,
    UserOneInitiated = 2,
    UserTwoInitiated = 3,
}

impl Into<UserRelationshipType> for i32 {
    fn into(self) -> UserRelationshipType {
        match self {
            0 => UserRelationshipType::Rejected,
            1 => UserRelationshipType::Matched,
            2 => UserRelationshipType::UserOneInitiated,
            3 => UserRelationshipType::UserTwoInitiated,
            _ => UserRelationshipType::Rejected, // i guess you get rejected
        }
    }
}

impl UserRelationship {
    pub async fn upsert(
        user1_id: i32,
        user2_id: i32,
        rel_type: UserRelationshipType,
        pool: &PgPool,
    ) -> Result<Self> {
        let (u1, u2) = if user1_id < user2_id {
            (user1_id, user2_id)
        } else {
            (user2_id, user1_id)
        };

        Ok(sqlx::query_as::<_, Self>(
            "
                    INSERT INTO user_relationships (
                        user1_id,
                        user2_id,
                        type
                    )
                    VALUES ($1, $2, $3)
                    ON CONFLICT (user1_id, user2_id)
                    DO UPDATE SET
                        type = $3
                    RETURNING *
                ",
        )
        .bind(u1)
        .bind(u2)
        .bind(rel_type as i32)
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

    pub async fn get_by_pair(user1_id: i32, user2_id: i32, pool: &PgPool) -> Result<Self> {
        let (u1, u2) = if user1_id < user2_id {
            (user1_id, user2_id)
        } else {
            (user2_id, user1_id)
        };

        Ok(sqlx::query_as::<_, Self>(
            "
                SELECT * FROM user_relationships
                WHERE
                    (user1_id = $1 AND user2_id = $1)
                    OR (user1_id = $2 AND user2_id = $1)
            ",
        )
        .bind(u1)
        .bind(u2)
        .fetch_one(pool)
        .await?)
    }

    pub async fn delete(user1_id: i32, user2_id: i32, pool: &PgPool) -> Result<()> {
        let (u1, u2) = if user1_id < user2_id {
            (user1_id, user2_id)
        } else {
            (user2_id, user1_id)
        };

        sqlx::query(
            "
                DELETE FROM user_relationships
                WHERE user1_id = $1 AND user2_id = $1
            ",
        )
        .bind(u1)
        .bind(u2)
        .execute(pool)
        .await?;

        Ok(())
    }
}
