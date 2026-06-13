use crate::entity::{Guild, Rift};
use async_graphql::SimpleObject;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct GuildNode {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_public: bool,
    pub members: Vec<GuildMemberNode>,
    pub rifts: Vec<RiftNode>,
}

#[derive(SimpleObject)]
pub struct GuildMemberNode {
    pub id: String,
    pub guild_id: String,
    pub adventurer_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub nickname: Option<String>,
    pub role: String,
}

#[derive(SimpleObject)]
pub struct RiftNode {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    pub topic: Option<String>,
    pub rift_type: String,
    pub position: i32,
    pub is_private: bool,
}

#[derive(FromRow)]
pub struct GuildMemberRow {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub adventurer_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub nickname: Option<String>,
    pub role: String,
}

impl From<Guild> for GuildNode {
    fn from(e: Guild) -> Self {
        Self {
            id: e.id.to_string(),
            name: e.name,
            description: e.description,
            icon_url: e.icon_url,
            banner_url: e.banner_url,
            is_public: e.is_public,
            members: vec![],
            rifts: vec![],
        }
    }
}

impl From<GuildMemberRow> for GuildMemberNode {
    fn from(e: GuildMemberRow) -> Self {
        Self {
            id: e.id.to_string(),
            guild_id: e.guild_id.to_string(),
            adventurer_id: e.adventurer_id.to_string(),
            username: e.username,
            display_name: e.display_name,
            avatar_url: e.avatar_url,
            nickname: e.nickname,
            role: e.role,
        }
    }
}

impl From<Rift> for RiftNode {
    fn from(e: Rift) -> Self {
        Self {
            id: e.id.to_string(),
            guild_id: e.guild_id.to_string(),
            name: e.name,
            topic: e.topic,
            rift_type: format!("{:?}", e.rift_type),
            position: e.position,
            is_private: e.is_private,
        }
    }
}
