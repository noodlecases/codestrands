use actix_web::{
    get,
    web::{Data, Json, Path, ServiceConfig},
};
use sqlx::PgPool;

use crate::{
    models::chat_messages::ChatMessage,
    utils::{auth::UserSession, Result},
};

#[get("/chats/{chat_id}/messages/")]
async fn get_all_chat_messages(
    _: UserSession,
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<Json<Vec<ChatMessage>>> {
    Ok(Json(
        ChatMessage::get_by_chat(path.into_inner(), &pool).await?,
    ))
}

#[get("/chats/{chat_id}/messages/{message_id}/")]
async fn get_message(
    _: UserSession,
    path: Path<(i32, i32)>,
    pool: Data<PgPool>,
) -> Result<Json<ChatMessage>> {
    Ok(Json(ChatMessage::get(path.1, &pool).await?))
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_all_chat_messages).service(get_message);
}
