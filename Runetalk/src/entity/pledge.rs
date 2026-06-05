use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::PledgeStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pledge {
    pub id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub status: PledgeStatus,
    pub created_at: NaiveDateTime,
}
