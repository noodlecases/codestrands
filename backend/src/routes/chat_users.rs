
use actix_web::{
    delete, get, patch, put,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::{chat_users::ChatUser, chats::Chat, user_relationships::UserRelationship},
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[put("/chats/{chat_id}/users/")]
async fn join_chat(path: Path<i32>, session: UserSession, pool: Data<PgPool>) -> Result<Json<ChatUser>> {
    let temp = path.into_inner();
    let users = ChatUser::get_by_chat(temp, &pool).await?;
    for user in users {
        let pair = UserRelationship::get_by_pair(session.user_id, user.user_id, &pool).await?;
        if (pair.rel_type != 1) 
        {
            break;
        }
    }
    Ok(Json(ChatUser::create(temp, session.user_id, &pool).await?))
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

pub fn config(config: &mut ServiceConfig) {
    config
        .service(join_chat)
        .service(leave_chat)
        .service(get_chats_for_user)
        .service(get_users_in_chat);
}
