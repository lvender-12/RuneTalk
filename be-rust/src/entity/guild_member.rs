use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::GuildRole;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildMember {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub adventurer_id: Uuid,
    pub nickname: Option<String>,
    pub role: GuildRole,
    pub joined_at: NaiveDateTime,
}
