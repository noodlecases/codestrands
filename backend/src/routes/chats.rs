use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::{chats::Chat, chat_users::ChatUser},
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
struct ChatTitle {
    name: String,
}

#[post("/chats/")]
async fn create_chat(title: Json<ChatTitle>, session: UserSession, pool: Data<PgPool>) -> Result<Json<Chat>> {
    let chat = Chat::create(&title.name, &pool).await?;
    ChatUser::create(chat.id, session.user_id, &pool).await?;
    Ok(Json(chat))
}

#[get("/chats/")]
async fn get_chats(session: UserSession, pool: Data<PgPool>) -> Result<Json<Vec<Chat>>> {
    Ok(Json(Chat::get(session.user_id, &pool).await?))
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_chats)
        .service(create_chat);
}
