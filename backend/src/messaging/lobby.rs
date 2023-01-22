use std::collections::{HashMap, HashSet};

use actix::{Actor, AsyncContext, Context, Handler, Recipient};
use chrono::Utc;
use log::warn;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    messaging::messages::*,
    models::{chat_messages::ChatMessage, chat_users::ChatUser},
};

type Socket = Recipient<OutgoingWsEvent>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    chats: HashMap<i32, HashSet<Uuid>>,
    pool: PgPool,
}

impl Lobby {
    pub fn new(pool: PgPool) -> Self {
        Self {
            sessions: HashMap::new(),
            chats: HashMap::new(),
            pool,
        }
    }

    fn send_event(&self, event: OutgoingWsEvent, ws_id: &Uuid) {
        match self.sessions.get(ws_id) {
            Some(recipient) => recipient.do_send(event),
            None => warn!("attempted to send event to nonexistent user"),
        };
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<ActorEvent> for Lobby {
    type Result = ();

    fn handle(&mut self, event: ActorEvent, _ctx: &mut Self::Context) -> Self::Result {
        match event {
            ActorEvent::Connect {
                address,
                user_id: _,
                user_chats,
                ws_id,
            } => {
                user_chats.iter().for_each(|chat| {
                    self.chats
                        .entry(*chat)
                        .or_insert_with(|| HashSet::new())
                        .insert(ws_id);
                });

                self.sessions.insert(ws_id, address);
            }
            ActorEvent::Disconnect { user_id: _, ws_id } => {
                if self.sessions.remove(&ws_id).is_some() {
                    self.chats
                        .iter_mut()
                        .filter(|item| item.1.contains(&ws_id))
                        .for_each(|item| {
                            item.1.remove(&ws_id);
                        });
                }
            }
        }
    }
}

impl Handler<WsMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        let user_id = msg.user_id;
        let timestamp = Utc::now();

        match msg.event {
            IncomingWsEvent::Message { chat_id, content } => {
                let pool = self.pool.clone();
                let message_content = content.clone();

                ctx.spawn(actix::fut::wrap_future(async move {
                    if ChatUser::get(user_id, chat_id, &pool).await.is_ok() {
                        ChatMessage::create(chat_id, user_id, &content, &pool)
                            .await
                            .unwrap();
                    }
                }));

                self.chats
                    .get(&chat_id)
                    .unwrap()
                    .iter()
                    .for_each(|ws_id| {
                        self.send_event(
                            OutgoingWsEvent::Message {
                                user_id,
                                timestamp,
                                chat_id,
                                content: message_content.clone(),
                            },
                            ws_id,
                        );
                    });
            }
        };
    }
}
