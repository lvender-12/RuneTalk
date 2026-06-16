use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    entity::{Echo, Whisper},
    modules::user::dto::FriendRequest,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SseFriendEvent {
    FriendRequestReceived { data: FriendRequest },
    FriendRequestAccepted {
        user_id: Uuid,
        username: String,
        display_name: Option<String>,
    },
    FriendRequestRejected { user_id: Uuid },
    FriendRemoved { user_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SseMessageEvent {
    WhisperReceived { data: Whisper },
    EchoReceived { data: Echo },
}
