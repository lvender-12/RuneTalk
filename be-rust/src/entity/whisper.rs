use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::MessageType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Whisper {
    pub id: Uuid,
    pub scroll_id: Uuid,
    pub sender_id: Uuid,
    pub reply_to_id: Option<Uuid>,
    pub content: String,
    pub message_type: MessageType,
    pub is_read: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}
