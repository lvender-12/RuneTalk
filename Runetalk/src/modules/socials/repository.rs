use crate::{app::AppState, entity::Adventurer, errors::AppResult};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SocialRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Adventurer>;
}

pub struct SocialRepositoryImpl {
    pub state: AppState,
}

impl SocialRepositoryImpl {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl SocialRepository for SocialRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Adventurer> {
        let adventurer = sqlx::query_as::<_, Adventurer>("SELECT * FROM adventurers WHERE id = $1")
            .bind(id)
            .fetch_one(self.state.db.as_ref())
            .await?;
        Ok(adventurer)
    }
}
