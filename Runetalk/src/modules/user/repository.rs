use crate::{
    app::AppState,
    entity::Adventurer,
    errors::AppResult,
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Adventurer>>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<Adventurer>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<Adventurer>>;
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
    async fn find_by_id(&self, _id: Uuid) -> AppResult<Option<Adventurer>> {
        todo!()
    }

    async fn find_by_username(&self, _username: &str) -> AppResult<Option<Adventurer>> {
        todo!()
    }

    async fn find_by_email(&self, _email: &str) -> AppResult<Option<Adventurer>> {
        todo!()
    }
}
