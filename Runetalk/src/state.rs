use crate::{
    app::AppState,
    config::{db::load_db, redis::load_redis},
    model::config_model::ConfigModel,
    modules::{
        auth::{repository::AuthRepositoryImpl, service::AuthServiceImpl},
        socials::{repository::SocialRepositoryImpl, service::SocialServiceImpl},
        user::{repository::UserRepositoryImpl, service::UserServiceImpl},
    },
};
use std::sync::Arc;

impl AppState {
    pub async fn init(conf: ConfigModel) -> Self {
        let db = load_db(&conf.db).await.expect("Failed to connect db");
        let redis = load_redis(&conf.redis)
            .await
            .expect("Failed to connect redis");

        let db_arc = Arc::new(db);
        let config_arc = Arc::new(conf);

        let mut state = Self {
            db: db_arc.clone(),
            redis,
            config: config_arc.clone(),
            auth_service: Arc::new(DummyAuthService),
            user_service: Arc::new(DummyUserService),
            social_service: Arc::new(DummySocialService),
        };

        let auth_repo = Arc::new(AuthRepositoryImpl {
            state: state.clone(),
        });
        let auth_service_impl = Arc::new(AuthServiceImpl::new(auth_repo, config_arc.clone()));

        let user_repo = Arc::new(UserRepositoryImpl::new(state.clone()));
        let user_service_impl = Arc::new(UserServiceImpl::new(user_repo));

        let social_repo = Arc::new(SocialRepositoryImpl {
            state: state.clone(),
        });
        let social_service_impl = Arc::new(SocialServiceImpl::new(social_repo));

        state.auth_service = auth_service_impl;
        state.user_service = user_service_impl;
        state.social_service = social_service_impl;

        state
    }
}

#[derive(Clone)]
struct DummyAuthService;

#[async_trait::async_trait]
impl crate::modules::auth::service::AuthService for DummyAuthService {
    async fn register_service(
        &self,
        _: crate::modules::auth::dto::RegisterDto,
    ) -> crate::errors::AppResult<crate::common::response::ApiResponse> {
        todo!()
    }
    async fn send_verification_otp(
        &self,
        _: &str,
    ) -> crate::errors::AppResult<crate::common::response::ApiResponse> {
        todo!()
    }
    async fn verification_otp(
        &self,
        _: crate::modules::auth::dto::VerifyOtpDto,
    ) -> crate::errors::AppResult<crate::common::response::ApiResponse> {
        todo!()
    }
    async fn login_service(
        &self,
        _: crate::modules::auth::dto::LoginDto,
    ) -> crate::errors::AppResult<String> {
        todo!()
    }
}

#[derive(Clone)]
struct DummyUserService;

#[async_trait::async_trait]
impl crate::modules::user::service::UserService for DummyUserService {
    async fn edit_user_service(
        &self,
        _state: &AppState,
        _multipart: axum::extract::Multipart,
        _id: uuid::Uuid,
    ) -> crate::errors::AppResult<crate::modules::user::dto::EditUserResponseDto> {
        todo!()
    }
    async fn profile_user(
        &self,
        _id: uuid::Uuid,
    ) -> crate::errors::AppResult<crate::modules::user::dto::ProfileUser> {
        todo!()
    }

    async fn add_friend_service(
        &self,
        _username: &str,
        _id: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }

    async fn list_incoming_requests_service(
        &self,
        _user_id: uuid::Uuid,
    ) -> crate::errors::AppResult<Vec<crate::modules::user::dto::FriendRequest>> {
        todo!()
    }

    async fn accept_friend_service(
        &self,
        _from: uuid::Uuid,
        _to: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }

    async fn reject_friend_service(
        &self,
        _from: uuid::Uuid,
        _to: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }

    async fn block_friend_service(
        &self,
        _blocker: uuid::Uuid,
        _blocked: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }

    async fn is_ally_service(
        &self,
        _user1: uuid::Uuid,
        _user2: uuid::Uuid,
    ) -> crate::errors::AppResult<bool> {
        todo!()
    }

    async fn remove_ally_service(
        &self,
        _user1: uuid::Uuid,
        _user2: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }
}

#[derive(Clone)]
struct DummySocialService;

#[async_trait::async_trait]
impl crate::modules::socials::service::SocialService for DummySocialService {
    async fn create_guild_service(
        &self,
        _dto: crate::modules::socials::dto::GuildDto,
        _id: uuid::Uuid,
    ) -> crate::errors::AppResult<crate::entity::Guild> {
        todo!()
    }

    async fn get_guild_service(
        &self,
        _guild_id: uuid::Uuid,
        _user_id: uuid::Uuid,
    ) -> crate::errors::AppResult<crate::entity::Guild> {
        todo!()
    }

    async fn edit_guild_service(
        &self,
        _guild_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _dto: crate::modules::socials::dto::EditGuildDto,
    ) -> crate::errors::AppResult<crate::entity::Guild> {
        todo!()
    }

    async fn delete_guild_service(
        &self,
        _guild_id: uuid::Uuid,
        _user_id: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }

    async fn create_rift_service(
        &self,
        _guild_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _dto: crate::modules::socials::dto::CreateRiftDto,
    ) -> crate::errors::AppResult<crate::entity::Rift> {
        todo!()
    }

    async fn edit_rift_service(
        &self,
        _guild_id: uuid::Uuid,
        _rift_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _dto: crate::modules::socials::dto::EditRiftDto,
    ) -> crate::errors::AppResult<crate::entity::Rift> {
        todo!()
    }

    async fn delete_rift_service(
        &self,
        _guild_id: uuid::Uuid,
        _rift_id: uuid::Uuid,
        _user_id: uuid::Uuid,
    ) -> crate::errors::AppResult<()> {
        todo!()
    }

    async fn join_guild_service(
        &self,
        _invite_code: &str,
        _user_id: uuid::Uuid,
    ) -> crate::errors::AppResult<crate::entity::Guild> {
        todo!()
    }

    async fn get_invite_link_service(
        &self,
        _guild_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _origin: &str,
    ) -> crate::errors::AppResult<crate::modules::socials::dto::InviteLinkResponse> {
        todo!()
    }

    async fn regenerate_invite_service(
        &self,
        _guild_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _origin: &str,
    ) -> crate::errors::AppResult<crate::modules::socials::dto::InviteLinkResponse> {
        todo!()
    }
}
