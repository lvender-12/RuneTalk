use crate::{
    app::AppState,
    config::{db::load_db, redis::load_redis},
    model::config_model::ConfigModel,
    modules::{
        auth::{repository::AuthRepositoryImpl, service::AuthServiceImpl},
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
        };

        let auth_repo = Arc::new(AuthRepositoryImpl {
            state: state.clone(),
        });
        let auth_service_impl = Arc::new(AuthServiceImpl::new(auth_repo, config_arc.clone()));

        let user_repo = Arc::new(UserRepositoryImpl::new(state.clone()));
        let user_service_impl = Arc::new(UserServiceImpl::new(user_repo));

        state.auth_service = auth_service_impl;
        state.user_service = user_service_impl;

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
}
