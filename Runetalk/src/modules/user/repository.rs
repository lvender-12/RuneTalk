use crate::{
    app::AppState, entity::Adventurer, errors::AppResult, modules::user::dto::EditUserResponseDto,
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Adventurer>;
    async fn edit_user_repo(&self, dto: Adventurer) -> AppResult<EditUserResponseDto>;
}

pub struct UserRepositoryImpl {
    pub state: AppState,
}

impl UserRepositoryImpl {
    pub fn new(state: AppState) -> Self {
        Self { state }
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
}
