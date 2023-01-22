use std::{fmt::Display, str::FromStr};

use actix::{Message, Recipient};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::error::CodestrandsError;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub user_id: i32,
    pub ws_id: Uuid,
    pub event: IncomingWsEvent,
}

#[derive(Clone, Deserialize, Message)]
#[rtype(result = "()")]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncomingWsEvent {
    Message { chat_id: i32, content: String },
}

impl FromStr for IncomingWsEvent {
    type Err = CodestrandsError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(event)?)
    }
}

#[derive(Clone, Message, Serialize)]
#[rtype(result = "()")]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutgoingWsEvent {
    Message {
        user_id: i32,
        timestamp: DateTime<Utc>,
        chat_id: i32,
        content: String,
    },
}

impl Display for OutgoingWsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum ActorEvent {
    Connect {
        address: Recipient<OutgoingWsEvent>,
        user_id: i32,
        user_chats: Vec<i32>,
        ws_id: Uuid,
    },
    Disconnect {
        user_id: i32,
        ws_id: Uuid,
    },
}
