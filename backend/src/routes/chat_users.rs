use actix_web::{
    delete, get, patch, put,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::{chat_users::ChatUser, chats::Chat},
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[put("/chats/{chat_id}/users/")]
async fn join_chat(path: Path<i32>, session: UserSession, pool: Data<PgPool>) -> Result<Json<ChatUser>> {
    Ok(Json(ChatUser::create(path.into_inner(), session.user_id, &pool).await?))
}

#[delete("/chats/{chat_id}/users/")]
async fn leave_chat(path: Path<i32>, session: UserSession, pool: Data<PgPool>) -> Result<Json<()>> {
    Ok(Json(ChatUser::delete(path.into_inner(), session.user_id, &pool).await?))
}

#[get("/chats/{chat_id}/users/")]
async fn get_chats_for_user(session: UserSession, pool: Data<PgPool>) -> Result<Json<Vec<ChatUser>>> {
    Ok(Json(ChatUser::get_by_user(session.user_id, &pool).await?))
}

#[get("/chats/{chat_id}/users/")]
async fn get_users_in_chat(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<ChatUser>>>{
    Ok(Json(ChatUser::get_by_chat(path.into_inner(), &pool).await?))
}