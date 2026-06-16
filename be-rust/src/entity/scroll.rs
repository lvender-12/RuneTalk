use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scroll {
    pub id: Uuid,
    pub initiator_id: Uuid,
    pub recipient_id: Uuid,
    pub created_at: NaiveDateTime,
}
