use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig, Payload},
    Error,
    HttpRequest,
    HttpResponse,
};
use actix_web_actors::ws;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::{chats::Chat, chat_users::ChatUser},
    utils::{auth::UserSession, Result}, messaging::{WsConn, Lobby},
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

#[get("/chats/ws/")]
async fn chat_ws(
    req: HttpRequest,
    stream: Payload,
    session: UserSession,
    pool: Data<PgPool>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let actor = WsConn::new(session.user_id, srv.get_ref().clone());

    ws::start(actor, &req, stream)
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_chats)
        .service(create_chat);
}
