use crate::{
    app::AppState,
    entity::{Adventurer, Pledge},
    errors::{AppError, AppResult, DbError},
    modules::user::dto::{EditUserResponseDto, FriendRequest},
};
use async_trait::async_trait;
use mockall::automock;
use tracing::debug;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Adventurer>;
    async fn edit_user_repo(&self, dto: Adventurer) -> AppResult<EditUserResponseDto>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<Adventurer>>;
    async fn add_friend(&self, from: Uuid, to: Uuid) -> AppResult<FriendRequest>;
    async fn list_incoming_requests(&self, user_id: Uuid) -> AppResult<Vec<FriendRequest>>;
    async fn find_pledge(&self, from: Uuid, to: Uuid) -> AppResult<Option<Pledge>>;
    async fn delete_pledge(&self, from: Uuid, to: Uuid) -> AppResult<()>;
    async fn accept_friend(&self, from: Uuid, to: Uuid) -> AppResult<()>;
    async fn block_user(&self, from: Uuid, to: Uuid) -> AppResult<()>;
    async fn is_ally(&self, user1: Uuid, user2: Uuid) -> AppResult<bool>;
    async fn remove_ally(&self, user1: Uuid, user2: Uuid) -> AppResult<()>;
}

pub struct UserRepositoryImpl {
    pub state: AppState,
}

impl UserRepositoryImpl {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    fn ally_ids(user1: Uuid, user2: Uuid) -> (Uuid, Uuid) {
        if user1 < user2 {
            (user1, user2)
        } else {
            (user2, user1)
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Adventurer> {
        Ok(
            sqlx::query_as::<_, Adventurer>("select * from adventurers where id = $1")
                .bind(id)
                .fetch_one(self.state.db.as_ref())
                .await?,
        )
    }

    async fn edit_user_repo(&self, dto: Adventurer) -> AppResult<EditUserResponseDto> {
        let update = sqlx::query_as::<_, EditUserResponseDto>(
            "UPDATE adventurers SET display_name = $1, avatar_url = $2, banner_url = $3, bio = $4
                     WHERE id = $5 RETURNING *",
        )
        .bind(dto.display_name)
        .bind(dto.avatar_url)
        .bind(dto.banner_url)
        .bind(dto.bio)
        .bind(dto.id)
        .fetch_one(self.state.db.as_ref())
        .await?;
        Ok(update)
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<Adventurer>> {
        debug!("Repo : {}", username);
        Ok(
            sqlx::query_as::<_, Adventurer>("select * from adventurers where username = $1")
                .bind(username)
                .fetch_optional(self.state.db.as_ref())
                .await?,
        )
    }

    async fn add_friend(&self, from: Uuid, to: Uuid) -> AppResult<FriendRequest> {
        sqlx::query("INSERT INTO pledges (from_id, to_id, status) VALUES ($1, $2, 'pending')")
            .bind(from)
            .bind(to)
            .execute(self.state.db.as_ref())
            .await?;

        sqlx::query_as::<_, FriendRequest>(
            "SELECT p.id, p.from_id, a.username, a.display_name, a.avatar_url, p.created_at
             FROM pledges p
             JOIN adventurers a ON a.id = p.from_id
             WHERE p.from_id = $1 AND p.to_id = $2 AND p.status = 'pending'",
        )
        .bind(from)
        .bind(to)
        .fetch_one(self.state.db.as_ref())
        .await
        .map_err(Into::into)
    }

    async fn list_incoming_requests(&self, user_id: Uuid) -> AppResult<Vec<FriendRequest>> {
        Ok(sqlx::query_as::<_, FriendRequest>(
            "SELECT p.id, p.from_id, a.username, a.display_name, a.avatar_url, p.created_at
             FROM pledges p
             JOIN adventurers a ON a.id = p.from_id
             WHERE p.to_id = $1 AND p.status = 'pending'
             ORDER BY p.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(self.state.db.as_ref())
        .await?)
    }

    async fn find_pledge(&self, from: Uuid, to: Uuid) -> AppResult<Option<Pledge>> {
        Ok(
            sqlx::query_as::<_, Pledge>("SELECT * FROM pledges WHERE from_id = $1 AND to_id = $2")
                .bind(from)
                .bind(to)
                .fetch_optional(self.state.db.as_ref())
                .await?,
        )
    }

    async fn delete_pledge(&self, from: Uuid, to: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM pledges WHERE from_id = $1 AND to_id = $2")
            .bind(from)
            .bind(to)
            .execute(self.state.db.as_ref())
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Db(DbError::not_found("Friend request")));
        }
        Ok(())
    }

    async fn accept_friend(&self, from: Uuid, to: Uuid) -> AppResult<()> {
        let (id_1, id_2) = Self::ally_ids(from, to);

        let mut tx = self.state.db.begin().await?;

        let deleted = sqlx::query(
            "DELETE FROM pledges WHERE from_id = $1 AND to_id = $2 AND status = 'pending'",
        )
        .bind(from)
        .bind(to)
        .execute(&mut *tx)
        .await?;

        if deleted.rows_affected() == 0 {
            return Err(AppError::Db(DbError::not_found("Friend request")));
        }

        sqlx::query("INSERT INTO allies (id_1, id_2) VALUES ($1, $2)")
            .bind(id_1)
            .bind(id_2)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn block_user(&self, from: Uuid, to: Uuid) -> AppResult<()> {
        let (id_1, id_2) = Self::ally_ids(from, to);

        let mut tx = self.state.db.begin().await?;

        sqlx::query(
            "DELETE FROM pledges WHERE (from_id = $1 AND to_id = $2) OR (from_id = $2 AND to_id = $1)",
        )
        .bind(from)
        .bind(to)
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM allies WHERE id_1 = $1 AND id_2 = $2")
            .bind(id_1)
            .bind(id_2)
            .execute(&mut *tx)
            .await?;

        sqlx::query("INSERT INTO pledges (from_id, to_id, status) VALUES ($1, $2, 'blocked')")
            .bind(from)
            .bind(to)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn is_ally(&self, user1: Uuid, user2: Uuid) -> AppResult<bool> {
        let (id_1, id_2) = Self::ally_ids(user1, user2);
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM allies WHERE id_1 = $1 AND id_2 = $2)",
        )
        .bind(id_1)
        .bind(id_2)
        .fetch_one(self.state.db.as_ref())
        .await?;
        Ok(exists)
    }

    async fn remove_ally(&self, user1: Uuid, user2: Uuid) -> AppResult<()> {
        let (id_1, id_2) = Self::ally_ids(user1, user2);
        sqlx::query("DELETE FROM allies WHERE id_1 = $1 AND id_2 = $2")
            .bind(id_1)
            .bind(id_2)
            .execute(self.state.db.as_ref())
            .await?;
        Ok(())
    }
}
