use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::{chat_messages::ChatMessage},
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
struct MessageInfo {
    content: String,
    user_id: i32
}

#[get("/chats/{chat_id}/messages/")]
async fn get_all_chat_messages(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<ChatMessage>>>{
    Ok(Json(ChatMessage::get_by_chat(path.into_inner(), &pool).await?))
}

#[get("/chats/{chat_id}/messages/{message_id}")]
async fn get_message(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<ChatMessage>>{
    Ok(Json(ChatMessage::get(path.into_inner(), &pool).await?))
}

#[post("/chats/{chat_id}/messages/")]
async fn send_message(message: Json<MessageInfo>, path: Path<i32>, pool: Data<PgPool>) -> Result<Json<ChatMessage>>{
    Ok(Json(ChatMessage::create(path.into_inner(), message.user_id, message.content, &pool).await?))
}

#[delete("/chats/{chat_id}/messages/")]
async fn delete_message(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<()>>{
    Ok(Json(ChatMessage::delete(path.into_inner(), &pool).await?))
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_all_chat_messages)
        .service(get_message)
        .service(send_message)
        .service(delete_message);
}

