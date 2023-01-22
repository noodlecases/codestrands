use actix_web::{
    delete, get, put,
    web::{Data, Json, Path, ServiceConfig, Query},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::{chat_users::ChatUser, user_relationships::{UserRelationship, UserRelationshipType}},
    utils::{auth::UserSession, Result, error::codestrands_error},
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ChatInviteQuery {
    user_id: i32,
}

#[put("/users/@me/chats/{chat_id}/invite/")]
async fn chat_invite(
    path: Path<i32>,
    query: Query<ChatInviteQuery>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<ChatUser>> {
    let chat_id = path.into_inner();

    if ChatUser::get(session.user_id, chat_id, &pool).await.is_err() {
        codestrands_error!(403, "Forbidden");
    }

    if let Ok(relationship) = UserRelationship::get_by_pair(session.user_id, query.user_id, &pool).await {
        if matches!(relationship.rel_type.into(), UserRelationshipType::Matched) {
            return Ok(Json(
                ChatUser::create(chat_id, query.user_id, &pool).await?,
            ));
        }
    }

    codestrands_error!(403, "Forbidden");
}

#[delete("/users/@me/chats/{chat_id}/")]
async fn leave_chat(path: Path<i32>, session: UserSession, pool: Data<PgPool>) -> Result<Json<()>> {
    Ok(Json(
        ChatUser::delete(path.into_inner(), session.user_id, &pool).await?,
    ))
}

#[get("/users/@me/chats/")]
async fn get_chats_for_user(
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<Vec<ChatUser>>> {
    Ok(Json(ChatUser::get_by_user(session.user_id, &pool).await?))
}

#[get("/chats/{chat_id}/users/")]
async fn get_users_in_chat(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<ChatUser>>> {
    Ok(Json(ChatUser::get_by_chat(path.into_inner(), &pool).await?))
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(chat_invite)
        .service(leave_chat)
        .service(get_chats_for_user)
        .service(get_users_in_chat);
}
