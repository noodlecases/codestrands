use std::{fmt::Display, str::FromStr};

use actix::{Message, Recipient};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{messaging::status::Status, utils::error::CodestrandsError};

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub party_id: Uuid,
    pub user_slug: String,
    pub ws_id: Uuid,
    pub event: IncomingWsEvent,
}

#[derive(Clone, Deserialize, Message)]
#[rtype(result = "()")]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncomingWsEvent {
    Message { content: String },
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
    StatusUpdate {
        user_slug: String,
        timestamp: DateTime<Utc>,
        status: Status,
    },
    Message {
        user_slug: String,
        timestamp: DateTime<Utc>,
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
        party_id: Uuid,
        user_slug: String,
        ws_id: Uuid,
    },
    Disconnect {
        party_id: Uuid,
        user_slug: String,
        ws_id: Uuid,
    },
}
