use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ally {
    pub id: Uuid,
    pub id_1: Uuid,
    pub id_2: Uuid,
    pub created_at: NaiveDateTime,
}
