use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Payload, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    messaging::{Lobby, WsConn},
    models::{chat_users::ChatUser, chats::Chat},
    utils::{auth::UserSession, Result},
};

#[derive(Deserialize)]
struct ChatTitle {
    name: String,
}

#[post("/chats/")]
async fn create_chat(
    title: Json<ChatTitle>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<Chat>> {
    let chat = Chat::create(&title.name, &pool).await?;
    ChatUser::create(chat.id, session.user_id, &pool).await?;
    Ok(Json(chat))
}

#[get("/chats/")]
async fn get_chats(session: UserSession, pool: Data<PgPool>) -> Result<Json<Vec<Chat>>> {
    Ok(Json(Chat::get(session.user_id, &pool).await?))
}

#[get("/chats/ws/")]
async fn chat_ws(
    req: HttpRequest,
    stream: Payload,
    session: UserSession,
    pool: Data<PgPool>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let chats = ChatUser::get_by_user(session.user_id, &pool)
        .await?
        .iter()
        .map(|chat_user| chat_user.chat_id)
        .collect::<Vec<i32>>();

    let actor = WsConn::new(session.user_id, chats, srv.get_ref().clone());

    ws::start(actor, &req, stream)
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_chats).service(create_chat).service(chat_ws);
}
