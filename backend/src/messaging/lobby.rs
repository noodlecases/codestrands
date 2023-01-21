use std::{collections::HashMap, time::Duration};

use actix::{Actor, AsyncContext, Context, Handler, Recipient};
use chrono::Utc;
use log::warn;
use sqlx::PgPool;
use uuid::Uuid;

use crate::messaging::{messages::*, party::Party, status::Status};

type Socket = Recipient<OutgoingWsEvent>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    parties: HashMap<Uuid, Party>,
    pool: PgPool,
}

impl Lobby {
    pub fn new(pool: PgPool) -> Self {
        Self {
            sessions: HashMap::new(),
            parties: HashMap::new(),
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

    fn handle(&mut self, event: ActorEvent, ctx: &mut Self::Context) -> Self::Result {
        match event {
            ActorEvent::Connect {
                address,
                party_id,
                user_slug,
                ws_id,
            } => {
                self.parties
                    .entry(party_id)
                    .or_insert_with(|| Party::new(party_id))
                    .members
                    .insert(ws_id);

                self.sessions.insert(ws_id, address);

                // TODO: Implement HTTP route to fetch user status
                self.parties
                    .get(&party_id)
                    .unwrap()
                    .members
                    .iter()
                    .for_each(|id| {
                        self.send_event(
                            OutgoingWsEvent::StatusUpdate {
                                user_slug: user_slug.clone(),
                                timestamp: Utc::now(),
                                status: Status::Online,
                            },
                            id,
                        )
                    });

                let party = self.parties.get(&party_id).unwrap();

                // TODO: Remove pending removal
                if let Some(pending_removal) = party.pending_removal {
                    ctx.cancel_future(pending_removal);
                    self.parties.get_mut(&party_id).unwrap().pending_removal = None;
                }
            }
            ActorEvent::Disconnect {
                party_id,
                user_slug,
                ws_id,
            } => {
                if self.sessions.remove(&ws_id).is_some() {
                    // TODO: Implement HTTP route to fetch user status
                    self.parties
                        .get(&party_id)
                        .unwrap()
                        .members
                        .iter()
                        .filter(|id| **id != ws_id)
                        .for_each(|id| {
                            self.send_event(
                                OutgoingWsEvent::StatusUpdate {
                                    user_slug: user_slug.clone(),
                                    timestamp: Utc::now(),
                                    status: Status::Offline,
                                },
                                id,
                            )
                        });

                    if let Some(party) = self.parties.get_mut(&party_id) {
                        party.members.remove(&ws_id);

                        if party.members.is_empty() {
                            if let Some(pending_removal) = party.pending_removal {
                                ctx.cancel_future(pending_removal);
                            }

                            let pending_removal =
                                ctx.run_later(Duration::from_secs(60), move |actor, _ctx| {
                                    if let Some(party) = actor.parties.get_mut(&party_id) {
                                        if party.members.is_empty() {
                                            actor.parties.remove(&party_id);
                                        } else {
                                            party.pending_removal = None;
                                        }
                                    }
                                });

                            party.pending_removal = Some(pending_removal);
                        }
                    }
                }
            }
        }
    }
}

impl Handler<WsMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        let user_slug = msg.user_slug;
        let timestamp = Utc::now();

        let outgoing_event = match msg.event {
            IncomingWsEvent::Message { content } => Some(OutgoingWsEvent::Message {
                user_slug,
                timestamp,
                content,
            }),
        };

        if let Some(event) = outgoing_event {
            self.parties
                .get(&msg.party_id)
                .unwrap()
                .members
                .iter()
                .for_each(|ws_id| self.send_event(event.clone(), ws_id));
        }
    }
}
