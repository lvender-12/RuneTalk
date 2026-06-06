use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    entity::Adventurer,
    modules::user::repository::UserRepository,
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<Adventurer>>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<Adventurer>>;
}

pub struct UserServiceImpl {
    pub repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_id(&self, _id: Uuid) -> Result<Option<Adventurer>> {
        todo!()
    }

    async fn get_user_by_username(&self, _username: &str) -> Result<Option<Adventurer>> {
        todo!()
    }
}
