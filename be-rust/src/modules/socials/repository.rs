use crate::{
    app::AppState,
    entity::{Guild, GuildRole, Rift},
    errors::{AppError, AppResult, DbError},
    modules::socials::dto::{CreateRiftDto, EditGuildDto, EditRiftDto, GuildDto},
};
use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait SocialRepository: Send + Sync {
    async fn create_guild_repo(
        &self,
        dto: GuildDto,
        owner_id: Uuid,
        invite_code: String,
    ) -> AppResult<Guild>;
    async fn find_guild_by_id_repo(&self, guild_id: Uuid) -> AppResult<Guild>;
    async fn find_guild_by_invite_code_repo(&self, invite_code: &str) -> AppResult<Guild>;
    async fn edit_guild_repo(&self, guild_id: Uuid, dto: EditGuildDto) -> AppResult<Guild>;
    async fn delete_guild_repo(&self, guild_id: Uuid) -> AppResult<()>;
    async fn get_member_role_repo(
        &self,
        guild_id: Uuid,
        adventurer_id: Uuid,
    ) -> AppResult<Option<GuildRole>>;
    async fn add_guild_member_repo(
        &self,
        guild_id: Uuid,
        adventurer_id: Uuid,
        role: GuildRole,
    ) -> AppResult<()>;
    async fn update_invite_code_repo(
        &self,
        guild_id: Uuid,
        invite_code: String,
    ) -> AppResult<Guild>;
    async fn create_rift_repo(&self, guild_id: Uuid, dto: CreateRiftDto) -> AppResult<Rift>;
    async fn find_rift_repo(&self, guild_id: Uuid, rift_id: Uuid) -> AppResult<Rift>;
    async fn edit_rift_repo(
        &self,
        guild_id: Uuid,
        rift_id: Uuid,
        dto: EditRiftDto,
    ) -> AppResult<Rift>;
    async fn delete_rift_repo(&self, guild_id: Uuid, rift_id: Uuid) -> AppResult<()>;
}

pub struct SocialRepositoryImpl {
    pub state: AppState,
}

impl SocialRepositoryImpl {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    fn db(&self) -> &PgPool {
        self.state.db.as_ref()
    }

    fn map_db_error(err: sqlx::Error, entity: &'static str) -> AppError {
        if let sqlx::Error::Database(db_err) = &err {
            match db_err.code().as_deref() {
                Some("23505") => return DbError::conflict(entity).into(),
                Some("23503") => return DbError::not_found("Adventurer").into(),
                _ => {}
            }
        }
        err.into()
    }
}

#[async_trait]
impl SocialRepository for SocialRepositoryImpl {
    async fn create_guild_repo(
        &self,
        dto: GuildDto,
        owner_id: Uuid,
        invite_code: String,
    ) -> AppResult<Guild> {
        let mut tx = self.db().begin().await?;

        let guild = sqlx::query_as::<_, Guild>(
            "INSERT INTO guilds (owner_id, name, description, invite_code, is_public)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(owner_id)
        .bind(&dto.name)
        .bind(&dto.description)
        .bind(&invite_code)
        .bind(dto.is_public)
        .fetch_one(&mut *tx)
        .await
        .map_err(|err| Self::map_db_error(err, "Guild"))?;

        sqlx::query(
            "INSERT INTO guild_members (guild_id, adventurer_id, role)
             VALUES ($1, $2, 'owner')",
        )
        .bind(guild.id)
        .bind(owner_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO rifts (guild_id, name, type, position)
             VALUES ($1, 'general', 'text', 0)",
        )
        .bind(guild.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(guild)
    }

    async fn find_guild_by_id_repo(&self, guild_id: Uuid) -> AppResult<Guild> {
        sqlx::query_as::<_, Guild>("SELECT * FROM guilds WHERE id = $1")
            .bind(guild_id)
            .fetch_optional(self.db())
            .await?
            .ok_or_else(|| DbError::not_found("Guild").into())
    }

    async fn find_guild_by_invite_code_repo(&self, invite_code: &str) -> AppResult<Guild> {
        sqlx::query_as::<_, Guild>("SELECT * FROM guilds WHERE invite_code = $1")
            .bind(invite_code)
            .fetch_optional(self.db())
            .await?
            .ok_or_else(|| DbError::not_found("Invite code").into())
    }

    async fn edit_guild_repo(&self, guild_id: Uuid, dto: EditGuildDto) -> AppResult<Guild> {
        sqlx::query_as::<_, Guild>(
            "UPDATE guilds
             SET name = $1,
                 description = $2,
                 icon_url = $3,
                 banner_url = $4,
                 is_public = $5,
                 updated_at = NOW()
             WHERE id = $6
             RETURNING *",
        )
        .bind(&dto.name)
        .bind(&dto.description)
        .bind(&dto.icon_url)
        .bind(&dto.banner_url)
        .bind(dto.is_public)
        .bind(guild_id)
        .fetch_optional(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Guild"))?
        .ok_or_else(|| DbError::not_found("Guild").into())
    }

    async fn delete_guild_repo(&self, guild_id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM guilds WHERE id = $1")
            .bind(guild_id)
            .execute(self.db())
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::not_found("Guild").into());
        }

        Ok(())
    }

    async fn get_member_role_repo(
        &self,
        guild_id: Uuid,
        adventurer_id: Uuid,
    ) -> AppResult<Option<GuildRole>> {
        let role = sqlx::query_scalar::<_, GuildRole>(
            "SELECT role FROM guild_members
             WHERE guild_id = $1 AND adventurer_id = $2",
        )
        .bind(guild_id)
        .bind(adventurer_id)
        .fetch_optional(self.db())
        .await?;

        Ok(role)
    }

    async fn add_guild_member_repo(
        &self,
        guild_id: Uuid,
        adventurer_id: Uuid,
        role: GuildRole,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO guild_members (guild_id, adventurer_id, role)
             VALUES ($1, $2, $3)",
        )
        .bind(guild_id)
        .bind(adventurer_id)
        .bind(role)
        .execute(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Guild member"))?;

        Ok(())
    }

    async fn update_invite_code_repo(
        &self,
        guild_id: Uuid,
        invite_code: String,
    ) -> AppResult<Guild> {
        sqlx::query_as::<_, Guild>(
            "UPDATE guilds
             SET invite_code = $1, updated_at = NOW()
             WHERE id = $2
             RETURNING *",
        )
        .bind(&invite_code)
        .bind(guild_id)
        .fetch_optional(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Guild"))?
        .ok_or_else(|| DbError::not_found("Guild").into())
    }

    async fn create_rift_repo(&self, guild_id: Uuid, dto: CreateRiftDto) -> AppResult<Rift> {
        let position = sqlx::query_scalar::<_, i32>(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM rifts WHERE guild_id = $1",
        )
        .bind(guild_id)
        .fetch_one(self.db())
        .await?;

        sqlx::query_as::<_, Rift>(
            "INSERT INTO rifts (guild_id, name, topic, type, position, is_private)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(guild_id)
        .bind(&dto.name)
        .bind(&dto.topic)
        .bind(dto.rift_type)
        .bind(position)
        .bind(dto.is_private)
        .fetch_one(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Rift"))
    }

    async fn find_rift_repo(&self, guild_id: Uuid, rift_id: Uuid) -> AppResult<Rift> {
        sqlx::query_as::<_, Rift>("SELECT * FROM rifts WHERE id = $1 AND guild_id = $2")
            .bind(rift_id)
            .bind(guild_id)
            .fetch_optional(self.db())
            .await?
            .ok_or_else(|| DbError::not_found("Rift").into())
    }

    async fn edit_rift_repo(
        &self,
        guild_id: Uuid,
        rift_id: Uuid,
        dto: EditRiftDto,
    ) -> AppResult<Rift> {
        let current = self.find_rift_repo(guild_id, rift_id).await?;

        let name = dto.name.unwrap_or(current.name);
        let topic = dto.topic.or(current.topic);
        let rift_type = dto.rift_type.unwrap_or(current.rift_type);
        let position = dto.position.unwrap_or(current.position);
        let is_private = dto.is_private.unwrap_or(current.is_private);

        sqlx::query_as::<_, Rift>(
            "UPDATE rifts
             SET name = $1,
                 topic = $2,
                 type = $3,
                 position = $4,
                 is_private = $5
             WHERE id = $6 AND guild_id = $7
             RETURNING *",
        )
        .bind(&name)
        .bind(&topic)
        .bind(rift_type)
        .bind(position)
        .bind(is_private)
        .bind(rift_id)
        .bind(guild_id)
        .fetch_optional(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Rift"))?
        .ok_or_else(|| DbError::not_found("Rift").into())
    }

    async fn delete_rift_repo(&self, guild_id: Uuid, rift_id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM rifts WHERE id = $1 AND guild_id = $2")
            .bind(rift_id)
            .bind(guild_id)
            .execute(self.db())
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::not_found("Rift").into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{
        entity::{GuildRole, RiftType},
        modules::socials::dto::{CreateRiftDto, EditGuildDto, EditRiftDto, GuildDto},
        testing::repo_test_state,
    };
    use uuid::Uuid;

    async fn db_pool() -> Option<sqlx::PgPool> {
        let url = std::env::var("DATABASE_URL").ok()?;
        sqlx::PgPool::connect(&url).await.ok()
    }

    async fn create_adventurer(pool: &sqlx::PgPool) -> Uuid {
        let id = Uuid::new_v4();
        let suffix = &id.to_string()[..8];
        sqlx::query(
            "INSERT INTO adventurers (id, username, email, password, is_verified)
             VALUES ($1, $2, $3, $4, TRUE)",
        )
        .bind(id)
        .bind(format!("guild_test_{suffix}"))
        .bind(format!("guild_test_{suffix}@example.com"))
        .bind("hashed_password")
        .execute(pool)
        .await
        .expect("insert adventurer");

        id
    }

    async fn cleanup_adventurer(pool: &sqlx::PgPool, id: Uuid) {
        let _ = sqlx::query("DELETE FROM adventurers WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await;
    }

    async fn repo(pool: sqlx::PgPool) -> SocialRepositoryImpl {
        SocialRepositoryImpl::new(repo_test_state(pool).await)
    }

    #[tokio::test]
    async fn create_guild_repo_creates_owner_member_and_default_rift() {
        let Some(pool) = db_pool().await else {
            eprintln!("Skipping: DATABASE_URL not set or unreachable");
            return;
        };

        let owner_id = create_adventurer(&pool).await;
        let repository = repo(pool.clone()).await;
        let dto = GuildDto {
            name: "Integration Guild".to_string(),
            description: Some("integration test".to_string()),
            is_public: true,
        };

        let guild = repository
            .create_guild_repo(dto, owner_id, "TEST1234".to_string())
            .await
            .expect("create guild");

        assert_eq!(guild.owner_id, owner_id);
        assert_eq!(guild.name, "Integration Guild");
        assert_eq!(guild.invite_code, "TEST1234");

        let role = repository
            .get_member_role_repo(guild.id, owner_id)
            .await
            .expect("member role")
            .expect("owner membership");
        assert_eq!(role, GuildRole::Owner);

        let rifts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM rifts WHERE guild_id = $1")
            .bind(guild.id)
            .fetch_one(&pool)
            .await
            .expect("count rifts");
        assert_eq!(rifts, 1);

        cleanup_adventurer(&pool, owner_id).await;
    }

    #[tokio::test]
    async fn find_guild_by_invite_code_repo() {
        let Some(pool) = db_pool().await else {
            eprintln!("Skipping: DATABASE_URL not set or unreachable");
            return;
        };

        let owner_id = create_adventurer(&pool).await;
        let repository = repo(pool.clone()).await;
        let guild = repository
            .create_guild_repo(
                GuildDto {
                    name: "Invite Guild".to_string(),
                    description: None,
                    is_public: false,
                },
                owner_id,
                "INVITE99".to_string(),
            )
            .await
            .expect("create guild");

        let found = repository
            .find_guild_by_invite_code_repo("INVITE99")
            .await
            .expect("find by invite");
        assert_eq!(found.id, guild.id);

        cleanup_adventurer(&pool, owner_id).await;
    }

    #[tokio::test]
    async fn edit_and_delete_guild_repo() {
        let Some(pool) = db_pool().await else {
            eprintln!("Skipping: DATABASE_URL not set or unreachable");
            return;
        };

        let owner_id = create_adventurer(&pool).await;
        let repository = repo(pool.clone()).await;
        let guild = repository
            .create_guild_repo(
                GuildDto {
                    name: "Editable Guild".to_string(),
                    description: None,
                    is_public: false,
                },
                owner_id,
                "EDIT1234".to_string(),
            )
            .await
            .expect("create guild");

        let updated = repository
            .edit_guild_repo(
                guild.id,
                EditGuildDto {
                    name: "Updated Guild".to_string(),
                    description: Some("updated".to_string()),
                    icon_url: None,
                    banner_url: None,
                    is_public: true,
                },
            )
            .await
            .expect("edit guild");
        assert_eq!(updated.name, "Updated Guild");
        assert!(updated.is_public);

        repository
            .delete_guild_repo(guild.id)
            .await
            .expect("delete guild");

        let err = repository
            .find_guild_by_id_repo(guild.id)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            crate::errors::AppError::Db(crate::errors::DbError::NotFound { entity: "Guild" })
        ));

        cleanup_adventurer(&pool, owner_id).await;
    }

    #[tokio::test]
    async fn rift_crud_repo() {
        let Some(pool) = db_pool().await else {
            eprintln!("Skipping: DATABASE_URL not set or unreachable");
            return;
        };

        let owner_id = create_adventurer(&pool).await;
        let repository = repo(pool.clone()).await;
        let guild = repository
            .create_guild_repo(
                GuildDto {
                    name: "Rift Guild".to_string(),
                    description: None,
                    is_public: true,
                },
                owner_id,
                "RIFT1234".to_string(),
            )
            .await
            .expect("create guild");

        let rift = repository
            .create_rift_repo(
                guild.id,
                CreateRiftDto {
                    name: "voice".to_string(),
                    topic: Some("voice chat".to_string()),
                    rift_type: RiftType::Voice,
                    is_private: false,
                },
            )
            .await
            .expect("create rift");
        assert_eq!(rift.name, "voice");
        assert_eq!(rift.rift_type, RiftType::Voice);

        let updated = repository
            .edit_rift_repo(
                guild.id,
                rift.id,
                EditRiftDto {
                    name: Some("voice-updated".to_string()),
                    topic: None,
                    rift_type: Some(RiftType::Announcement),
                    position: Some(2),
                    is_private: Some(true),
                },
            )
            .await
            .expect("edit rift");
        assert_eq!(updated.name, "voice-updated");
        assert_eq!(updated.rift_type, RiftType::Announcement);
        assert_eq!(updated.position, 2);
        assert!(updated.is_private);

        repository
            .delete_rift_repo(guild.id, rift.id)
            .await
            .expect("delete rift");

        cleanup_adventurer(&pool, owner_id).await;
    }

    #[tokio::test]
    async fn add_guild_member_and_update_invite_code_repo() {
        let Some(pool) = db_pool().await else {
            eprintln!("Skipping: DATABASE_URL not set or unreachable");
            return;
        };

        let owner_id = create_adventurer(&pool).await;
        let member_id = create_adventurer(&pool).await;
        let repository = repo(pool.clone()).await;
        let guild = repository
            .create_guild_repo(
                GuildDto {
                    name: "Member Guild".to_string(),
                    description: None,
                    is_public: true,
                },
                owner_id,
                "MEMB1234".to_string(),
            )
            .await
            .expect("create guild");

        repository
            .add_guild_member_repo(guild.id, member_id, GuildRole::Member)
            .await
            .expect("add member");

        let role = repository
            .get_member_role_repo(guild.id, member_id)
            .await
            .expect("role")
            .expect("member role");
        assert_eq!(role, GuildRole::Member);

        let updated = repository
            .update_invite_code_repo(guild.id, "NEWCODE1".to_string())
            .await
            .expect("update invite");
        assert_eq!(updated.invite_code, "NEWCODE1");

        cleanup_adventurer(&pool, member_id).await;
        cleanup_adventurer(&pool, owner_id).await;
    }
}
