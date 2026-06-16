use std::collections::HashMap;

use async_graphql::{Context, Error, Object, Result};
use uuid::Uuid;

use crate::{
    app::AppState,
    entity::{Guild, Rift},
    modules::graphql::dto::{GuildMemberNode, GuildMemberRow, GuildNode, RiftNode},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "pong!"
    }

    async fn my_guilds(&self, ctx: &Context<'_>) -> Result<Vec<GuildNode>> {
        let state = ctx.data_unchecked::<AppState>();
        let adventurer_id = *ctx.data_unchecked::<Uuid>();

        let guilds = sqlx::query_as::<_, Guild>(
            "SELECT g.*
             FROM guilds g
             JOIN guild_members gm ON gm.guild_id = g.id
             WHERE gm.adventurer_id = $1
             ORDER BY g.created_at DESC",
        )
        .bind(adventurer_id)
        .fetch_all(state.db.as_ref())
        .await
        .map_err(|err| Error::new(err.to_string()))?;

        if guilds.is_empty() {
            return Ok(vec![]);
        }

        let guild_ids = guilds.iter().map(|guild| guild.id).collect::<Vec<_>>();

        let member_rows = sqlx::query_as::<_, GuildMemberRow>(
            "SELECT
                gm.id,
                gm.guild_id,
                gm.adventurer_id,
                a.username,
                a.display_name,
                a.avatar_url,
                gm.nickname,
                gm.role::text AS role
             FROM guild_members gm
             JOIN adventurers a ON a.id = gm.adventurer_id
             WHERE gm.guild_id = ANY($1)
             ORDER BY gm.joined_at ASC",
        )
        .bind(&guild_ids)
        .fetch_all(state.db.as_ref())
        .await
        .map_err(|err| Error::new(err.to_string()))?;

        let rifts = sqlx::query_as::<_, Rift>(
            "SELECT *
             FROM rifts
             WHERE guild_id = ANY($1)
             ORDER BY position ASC, created_at ASC",
        )
        .bind(&guild_ids)
        .fetch_all(state.db.as_ref())
        .await
        .map_err(|err| Error::new(err.to_string()))?;

        let mut members_by_guild: HashMap<Uuid, Vec<GuildMemberNode>> = HashMap::new();
        for member in member_rows {
            members_by_guild
                .entry(member.guild_id)
                .or_default()
                .push(member.into());
        }

        let mut rifts_by_guild: HashMap<Uuid, Vec<RiftNode>> = HashMap::new();
        for rift in rifts {
            rifts_by_guild
                .entry(rift.guild_id)
                .or_default()
                .push(rift.into());
        }

        Ok(guilds
            .into_iter()
            .map(|guild| {
                let guild_id = guild.id;
                let mut node = GuildNode::from(guild);
                node.members = members_by_guild.remove(&guild_id).unwrap_or_default();
                node.rifts = rifts_by_guild.remove(&guild_id).unwrap_or_default();
                node
            })
            .collect())
    }
}
