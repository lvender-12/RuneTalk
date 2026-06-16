use crate::{
    entity::{Echo, GuildRole, Presence, PresenceStatus, Scroll, Whisper},
    errors::{AppResult, AuthError},
    modules::{
        socials::repository::SocialRepository,
        ws::{
            dto::{SendEchoDto, SendWhisperDto},
            repository::WsRepository,
        },
    },
};
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[automock]
#[async_trait]
pub trait WsService: Send + Sync {
    async fn verify_rift_access(&self, rift_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn verify_scroll_access(&self, scroll_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn send_echo_service(
        &self,
        dto: SendEchoDto,
        user_id: Uuid,
    ) -> AppResult<Echo>;
    async fn send_whisper_service(
        &self,
        dto: SendWhisperDto,
        user_id: Uuid,
    ) -> AppResult<Whisper>;
    async fn scroll_recipient_id(
        &self,
        scroll_id: Uuid,
        sender_id: Uuid,
    ) -> AppResult<Uuid>;
    async fn set_presence(
        &self,
        user_id: Uuid,
        status: PresenceStatus,
    ) -> AppResult<Presence>;
}

pub struct WsServiceImpl {
    pub ws_repo: Arc<dyn WsRepository>,
    pub social_repo: Arc<dyn SocialRepository>,
}

impl WsServiceImpl {
    pub fn new(ws_repo: Arc<dyn WsRepository>, social_repo: Arc<dyn SocialRepository>) -> Self {
        Self {
            ws_repo,
            social_repo,
        }
    }
}

#[async_trait]
impl WsService for WsServiceImpl {
    async fn verify_rift_access(&self, rift_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let rift = self.ws_repo.find_rift_repo(rift_id).await?;

        let role = self
            .social_repo
            .get_member_role_repo(rift.guild_id, user_id)
            .await?
            .ok_or(AuthError::Forbidden)?;

        if rift.is_private && !matches!(role, GuildRole::Owner | GuildRole::Admin) {
            return Err(AuthError::Forbidden.into());
        }

        Ok(())
    }

    async fn verify_scroll_access(&self, scroll_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let scroll = self.ws_repo.find_scroll_repo(scroll_id).await?;

        if scroll.initiator_id == user_id || scroll.recipient_id == user_id {
            Ok(())
        } else {
            Err(AuthError::Forbidden.into())
        }
    }

    async fn send_echo_service(&self, dto: SendEchoDto, user_id: Uuid) -> AppResult<Echo> {
        dto.validate()?;
        self.verify_rift_access(dto.rift_id, user_id).await?;
        self.ws_repo.create_echo_repo(dto, user_id).await
    }

    async fn send_whisper_service(
        &self,
        dto: SendWhisperDto,
        user_id: Uuid,
    ) -> AppResult<Whisper> {
        dto.validate()?;
        self.verify_scroll_access(dto.scroll_id, user_id).await?;
        self.ws_repo.create_whisper_repo(dto, user_id).await
    }

    async fn scroll_recipient_id(
        &self,
        scroll_id: Uuid,
        sender_id: Uuid,
    ) -> AppResult<Uuid> {
        let scroll = self.ws_repo.find_scroll_repo(scroll_id).await?;
        Ok(other_scroll_participant(&scroll, sender_id))
    }

    async fn set_presence(
        &self,
        user_id: Uuid,
        status: PresenceStatus,
    ) -> AppResult<Presence> {
        self.ws_repo.upsert_presence_repo(user_id, status).await
    }
}

fn other_scroll_participant(scroll: &Scroll, user_id: Uuid) -> Uuid {
    if scroll.initiator_id == user_id {
        scroll.recipient_id
    } else {
        scroll.initiator_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entity::{GuildRole, PresenceStatus},
        errors::{AppError, AuthError},
        modules::{
            socials::repository::MockSocialRepository,
            ws::{
                dto::{SendEchoDto, SendWhisperDto},
                repository::MockWsRepository,
            },
        },
        testing::fixtures::{dummy_echo, dummy_presence, dummy_rift, dummy_scroll, dummy_whisper},
    };
    use crate::entity::MessageType;
    use mockall::predicate::*;
    use std::sync::Arc;

    fn service(ws_repo: MockWsRepository, social_repo: MockSocialRepository) -> WsServiceImpl {
        WsServiceImpl::new(Arc::new(ws_repo), Arc::new(social_repo))
    }

    #[tokio::test]
    async fn verify_rift_access_allows_guild_member() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let rift_id = rift.id;
        let mut ws_repo = MockWsRepository::new();
        let mut social_repo = MockSocialRepository::new();

        ws_repo
            .expect_find_rift_repo()
            .with(eq(rift_id))
            .times(1)
            .returning(move |_| Ok(rift.clone()));

        social_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        let svc = service(ws_repo, social_repo);
        svc.verify_rift_access(rift_id, user_id).await.unwrap();
    }

    #[tokio::test]
    async fn verify_rift_access_forbids_private_rift_for_member() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let mut rift = dummy_rift(guild_id);
        rift.is_private = true;
        let rift_id = rift.id;
        let mut ws_repo = MockWsRepository::new();
        let mut social_repo = MockSocialRepository::new();

        ws_repo
            .expect_find_rift_repo()
            .with(eq(rift_id))
            .times(1)
            .returning(move |_| Ok(rift.clone()));

        social_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        let svc = service(ws_repo, social_repo);
        let err = svc
            .verify_rift_access(rift_id, user_id)
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }

    #[tokio::test]
    async fn verify_scroll_access_allows_participant() {
        let user_id = Uuid::new_v4();
        let scroll = dummy_scroll(user_id, Uuid::new_v4());
        let scroll_id = scroll.id;
        let mut ws_repo = MockWsRepository::new();

        ws_repo
            .expect_find_scroll_repo()
            .with(eq(scroll_id))
            .times(1)
            .returning(move |_| Ok(scroll.clone()));

        let svc = service(ws_repo, MockSocialRepository::new());
        svc.verify_scroll_access(scroll_id, user_id).await.unwrap();
    }

    #[tokio::test]
    async fn verify_scroll_access_forbids_outsider() {
        let scroll = dummy_scroll(Uuid::new_v4(), Uuid::new_v4());
        let scroll_id = scroll.id;
        let outsider = Uuid::new_v4();
        let mut ws_repo = MockWsRepository::new();

        ws_repo
            .expect_find_scroll_repo()
            .with(eq(scroll_id))
            .times(1)
            .returning(move |_| Ok(scroll.clone()));

        let svc = service(ws_repo, MockSocialRepository::new());
        let err = svc
            .verify_scroll_access(scroll_id, outsider)
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Auth(AuthError::Forbidden)));
    }

    #[tokio::test]
    async fn send_echo_service_validates_and_persists() {
        let user_id = Uuid::new_v4();
        let guild_id = Uuid::new_v4();
        let rift = dummy_rift(guild_id);
        let rift_id = rift.id;
        let echo = dummy_echo(rift_id, user_id);
        let mut ws_repo = MockWsRepository::new();
        let mut social_repo = MockSocialRepository::new();

        ws_repo
            .expect_find_rift_repo()
            .with(eq(rift_id))
            .times(1)
            .returning(move |_| Ok(rift.clone()));

        social_repo
            .expect_get_member_role_repo()
            .with(eq(guild_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(Some(GuildRole::Member)));

        ws_repo
            .expect_create_echo_repo()
            .times(1)
            .returning(move |_, _| Ok(echo.clone()));

        let svc = service(ws_repo, social_repo);
        let dto = SendEchoDto {
            rift_id,
            content: "hello".to_string(),
            reply_to_id: None,
            message_type: MessageType::Text,
        };

        let result = svc.send_echo_service(dto, user_id).await.unwrap();
        assert_eq!(result.content, "hello echo");
    }

    #[tokio::test]
    async fn send_whisper_service_validates_and_persists() {
        let sender_id = Uuid::new_v4();
        let scroll = dummy_scroll(sender_id, Uuid::new_v4());
        let scroll_id = scroll.id;
        let whisper = dummy_whisper(scroll_id, sender_id);
        let mut ws_repo = MockWsRepository::new();

        ws_repo
            .expect_find_scroll_repo()
            .with(eq(scroll_id))
            .times(1)
            .returning(move |_| Ok(scroll.clone()));

        ws_repo
            .expect_create_whisper_repo()
            .times(1)
            .returning(move |_, _| Ok(whisper.clone()));

        let svc = service(ws_repo, MockSocialRepository::new());
        let dto = SendWhisperDto {
            scroll_id,
            content: "secret".to_string(),
            reply_to_id: None,
            message_type: Default::default(),
        };

        let result = svc.send_whisper_service(dto, sender_id).await.unwrap();
        assert_eq!(result.content, "hello whisper");
    }

    #[tokio::test]
    async fn scroll_recipient_id_returns_other_participant() {
        let sender_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let scroll = dummy_scroll(sender_id, recipient_id);
        let scroll_id = scroll.id;
        let mut ws_repo = MockWsRepository::new();

        ws_repo
            .expect_find_scroll_repo()
            .with(eq(scroll_id))
            .times(1)
            .returning(move |_| Ok(scroll.clone()));

        let svc = service(ws_repo, MockSocialRepository::new());
        let result = svc
            .scroll_recipient_id(scroll_id, sender_id)
            .await
            .unwrap();
        assert_eq!(result, recipient_id);
    }

    #[tokio::test]
    async fn set_presence_delegates_to_repository() {
        let user_id = Uuid::new_v4();
        let presence = dummy_presence(user_id, PresenceStatus::Online);
        let mut ws_repo = MockWsRepository::new();

        ws_repo
            .expect_upsert_presence_repo()
            .with(eq(user_id), eq(PresenceStatus::Idle))
            .times(1)
            .returning(move |_, _| Ok(presence.clone()));

        let svc = service(ws_repo, MockSocialRepository::new());
        let result = svc
            .set_presence(user_id, PresenceStatus::Idle)
            .await
            .unwrap();
        assert_eq!(result.status, PresenceStatus::Online);
    }
}
