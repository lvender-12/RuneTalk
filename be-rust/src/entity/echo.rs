use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::MessageType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Echo {
    pub id: Uuid,
    pub rift_id: Uuid,
    pub adventurer_id: Uuid,
    pub reply_to_id: Option<Uuid>,
    pub content: String,
    pub message_type: MessageType,
    pub is_pinned: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}
