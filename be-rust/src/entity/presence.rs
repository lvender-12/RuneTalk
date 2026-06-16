use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::PresenceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Presence {
    pub adventurer_id: Uuid,
    pub status: PresenceStatus,
    pub custom_status: Option<String>,
    pub last_seen: NaiveDateTime,
}
