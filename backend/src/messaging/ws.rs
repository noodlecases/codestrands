use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use log::{error, warn};
use uuid::Uuid;

use crate::messaging::{lobby::Lobby, messages::*};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

pub struct WsConn {
    ws_id: Uuid,
    user_id: i32,
    user_chats: Vec<i32>,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
}

impl WsConn {
    pub fn new(user_id: i32, user_chats: Vec<i32>, lobby_addr: Addr<Lobby>) -> Self {
        Self {
            ws_id: Uuid::new_v4(),
            user_id,
            user_chats,
            hb: Instant::now(),
            lobby_addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if Instant::now().duration_since(actor.hb) > CLIENT_TIMEOUT {
                warn!("disconnecting client without heartbeat");
                actor.lobby_addr.do_send(ActorEvent::Disconnect {
                    user_id: actor.user_id,
                    ws_id: actor.ws_id,
                });
                ctx.stop();
            } else {
                ctx.ping(b"PING");
            }
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let address = ctx.address();
        self.lobby_addr
            .send(ActorEvent::Connect {
                address: address.recipient(),
                user_id: self.user_id,
                user_chats: self.user_chats.clone(),
                ws_id: self.ws_id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                if res.is_err() {
                    ctx.stop()
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.lobby_addr.do_send(ActorEvent::Disconnect {
            user_id: self.user_id,
            ws_id: self.ws_id,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => match msg {
                ws::Message::Ping(msg) => {
                    self.hb = Instant::now();
                    ctx.pong(&msg);
                }
                ws::Message::Pong(_) => {
                    self.hb = Instant::now();
                }
                ws::Message::Binary(bin) => ctx.binary(bin),
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                ws::Message::Continuation(_) => {
                    ctx.stop();
                }
                ws::Message::Nop => (),
                ws::Message::Text(event) => {
                    if let Ok(event) = IncomingWsEvent::from_str(&event) {
                        self.lobby_addr.do_send(WsMessage {
                            ws_id: self.ws_id,
                            user_id: self.user_id,
                            event,
                        });
                    }
                }
            },
            Err(err) => error!("failed to handle ws message: {}", err),
        }
    }
}

impl Handler<OutgoingWsEvent> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: OutgoingWsEvent, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.to_string());
    }
}
