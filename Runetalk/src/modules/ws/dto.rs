use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entity::{Echo, MessageType, PresenceStatus, Whisper};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsClientMessage {
    SubscribeRift {
        rift_id: Uuid,
    },
    UnsubscribeRift {
        rift_id: Uuid,
    },
    SubscribeScroll {
        scroll_id: Uuid,
    },
    UnsubscribeScroll {
        scroll_id: Uuid,
    },
    SendEcho {
        #[serde(flatten)]
        payload: SendEchoDto,
    },
    SendWhisper {
        #[serde(flatten)]
        payload: SendWhisperDto,
    },
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendEchoDto {
    pub rift_id: Uuid,
    #[validate(length(min = 1, max = 4000, message = "Pesan tidak boleh kosong"))]
    pub content: String,
    pub reply_to_id: Option<Uuid>,
    #[serde(default)]
    pub message_type: MessageType,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendWhisperDto {
    pub scroll_id: Uuid,
    #[validate(length(min = 1, max = 4000, message = "Pesan tidak boleh kosong"))]
    pub content: String,
    pub reply_to_id: Option<Uuid>,
    #[serde(default)]
    pub message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsServerMessage {
    Echo { data: Echo },
    Whisper { data: Whisper },
    Error { message: String },
    SubscribedRift { rift_id: Uuid },
    UnsubscribedRift { rift_id: Uuid },
    SubscribedScroll { scroll_id: Uuid },
    UnsubscribedScroll { scroll_id: Uuid },
    PresenceUpdate {
        user_id: Uuid,
        status: PresenceStatus,
    },
    PresenceSnapshot {
        online_users: Vec<Uuid>,
    },
}

impl WsServerMessage {
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
        }
    }
}
