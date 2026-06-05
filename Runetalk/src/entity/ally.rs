use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ally {
    pub id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub created_at: NaiveDateTime,
}
