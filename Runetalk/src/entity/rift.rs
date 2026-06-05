use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::RiftType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Rift {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub name: String,
    pub topic: Option<String>,
    #[sqlx(rename = "type")]
    pub rift_type: RiftType,
    pub position: i32,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
}
