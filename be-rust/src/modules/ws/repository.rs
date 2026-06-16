use crate::{
    app::AppState,
    entity::{Echo, Presence, PresenceStatus, Rift, Scroll, Whisper},
    errors::{AppError, AppResult, DbError},
    modules::ws::dto::{SendEchoDto, SendWhisperDto},
};
use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait WsRepository: Send + Sync {
    async fn find_rift_repo(&self, rift_id: Uuid) -> AppResult<Rift>;
    async fn find_scroll_repo(&self, scroll_id: Uuid) -> AppResult<Scroll>;
    async fn create_echo_repo(
        &self,
        dto: SendEchoDto,
        adventurer_id: Uuid,
    ) -> AppResult<Echo>;
    async fn create_whisper_repo(
        &self,
        dto: SendWhisperDto,
        sender_id: Uuid,
    ) -> AppResult<Whisper>;
    async fn upsert_presence_repo(
        &self,
        user_id: Uuid,
        status: PresenceStatus,
    ) -> AppResult<Presence>;
}

pub struct WsRepositoryImpl {
    pub state: AppState,
}

impl WsRepositoryImpl {
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
                Some("23503") => return DbError::not_found(entity).into(),
                _ => {}
            }
        }
        err.into()
    }
}

#[async_trait]
impl WsRepository for WsRepositoryImpl {
    async fn find_rift_repo(&self, rift_id: Uuid) -> AppResult<Rift> {
        sqlx::query_as::<_, Rift>("SELECT * FROM rifts WHERE id = $1")
            .bind(rift_id)
            .fetch_optional(self.db())
            .await?
            .ok_or_else(|| DbError::not_found("Rift").into())
    }

    async fn find_scroll_repo(&self, scroll_id: Uuid) -> AppResult<Scroll> {
        sqlx::query_as::<_, Scroll>("SELECT * FROM scrolls WHERE id = $1")
            .bind(scroll_id)
            .fetch_optional(self.db())
            .await?
            .ok_or_else(|| DbError::not_found("Scroll").into())
    }

    async fn create_echo_repo(
        &self,
        dto: SendEchoDto,
        adventurer_id: Uuid,
    ) -> AppResult<Echo> {
        sqlx::query_as::<_, Echo>(
            "INSERT INTO echoes (rift_id, adventurer_id, reply_to_id, content, message_type)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(dto.rift_id)
        .bind(adventurer_id)
        .bind(dto.reply_to_id)
        .bind(dto.content)
        .bind(dto.message_type)
        .fetch_one(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Echo"))
    }

    async fn create_whisper_repo(
        &self,
        dto: SendWhisperDto,
        sender_id: Uuid,
    ) -> AppResult<Whisper> {
        sqlx::query_as::<_, Whisper>(
            "INSERT INTO whispers (scroll_id, sender_id, reply_to_id, content, message_type)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(dto.scroll_id)
        .bind(sender_id)
        .bind(dto.reply_to_id)
        .bind(dto.content)
        .bind(dto.message_type)
        .fetch_one(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Whisper"))
    }

    async fn upsert_presence_repo(
        &self,
        user_id: Uuid,
        status: PresenceStatus,
    ) -> AppResult<Presence> {
        sqlx::query_as::<_, Presence>(
            "INSERT INTO presence (adventurer_id, status, last_seen)
             VALUES ($1, $2, NOW())
             ON CONFLICT (adventurer_id)
             DO UPDATE SET status = EXCLUDED.status, last_seen = NOW()
             RETURNING *",
        )
        .bind(user_id)
        .bind(status)
        .fetch_one(self.db())
        .await
        .map_err(|err| Self::map_db_error(err, "Presence"))
    }
}
