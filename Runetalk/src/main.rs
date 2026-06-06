use async_trait::async_trait;
use std::sync::Arc;

use runetalk::{
    app::{AppState, create_app},
    config::{config::load_config, db::load_db, redis::load_redis},
};
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct DummyAuthService;

#[async_trait]
impl runetalk::modules::auth::service::AuthService for DummyAuthService {
    async fn register_service(
        &self,
        _: runetalk::modules::auth::dto::RegisterDto,
    ) -> anyhow::Result<runetalk::common::response::ApiResponse> {
        todo!()
    }
    async fn send_verification_otp(
        &self,
        _: &str,
    ) -> anyhow::Result<runetalk::common::response::ApiResponse> {
        todo!()
    }
    async fn verification_otp(
        &self,
        _: runetalk::modules::auth::dto::VerifyOtpDto,
    ) -> anyhow::Result<runetalk::common::response::ApiResponse> {
        todo!()
    }
    async fn login_service(
        &self,
        _: runetalk::modules::auth::dto::LoginDto,
    ) -> anyhow::Result<runetalk::common::response::ApiResponse> {
        todo!()
    }
}

#[derive(Clone)]
struct DummyUserService;

#[async_trait]
impl runetalk::modules::user::service::UserService for DummyUserService {
    async fn get_user_by_id(
        &self,
        _: uuid::Uuid,
    ) -> anyhow::Result<Option<runetalk::entity::Adventurer>> {
        todo!()
    }
    async fn get_user_by_username(
        &self,
        _: &str,
    ) -> anyhow::Result<Option<runetalk::entity::Adventurer>> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")))
        .init();

    let conf = load_config().expect("config error");
    let db = load_db(&conf.db).await.expect("Failed to connect db");
    let redis = load_redis(&conf.redis)
        .await
        .expect("Failed to connect redis");

    debug!("config : {:?}", conf);

    let host = format!("{}:{}", conf.app.host, conf.app.port);

    let state_for_repo = AppState {
        db: Arc::new(db.clone()),
        redis: redis.clone(),
        config: Arc::new(conf.clone()),
        auth_service: Arc::new(DummyAuthService),
        user_service: Arc::new(DummyUserService),
    };

    let auth_repo = Arc::new(runetalk::modules::auth::repository::AuthRepositoryImpl {
        state: state_for_repo.clone(),
    });

    let auth_service_impl = Arc::new(runetalk::modules::auth::service::AuthServiceImpl::new(
        auth_repo,
        state_for_repo.clone(),
    ));

    let user_repo = Arc::new(
        runetalk::modules::user::repository::UserRepositoryImpl::new(state_for_repo.clone()),
    );

    let user_service_impl = Arc::new(runetalk::modules::user::service::UserServiceImpl::new(
        user_repo,
    ));

    let state = AppState {
        db: Arc::new(db),
        redis: redis,
        config: Arc::new(conf),
        auth_service: auth_service_impl,
        user_service: user_service_impl,
    };

    debug!("State : {:?}", state);

    let app = create_app(state);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .expect("Failed to bind TCP Listener");

    info!("Listening on {}", host);
    axum::serve(listener, app).await.expect("Failed to serve");
}
