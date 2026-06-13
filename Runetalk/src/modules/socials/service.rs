use crate::{errors::AppResult, modules::socials::repository::SocialRepository};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

#[async_trait]
pub trait SocialService: Send + Sync {
    async fn profile_user(&self, id: Uuid) -> AppResult<()>;
}

pub struct SocialServiceImpl {
    pub repo: Arc<dyn SocialRepository>,
}

impl SocialServiceImpl {
    pub fn new(repo: Arc<dyn SocialRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl SocialService for SocialServiceImpl {
    async fn profile_user(&self, id: Uuid) -> AppResult<()> {
        let profile = self.repo.find_by_id(id).await?;
        debug!("{:?}", profile);
        Ok(())
    }
}
