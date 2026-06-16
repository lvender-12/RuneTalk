pub mod fixtures;
pub mod noop;
pub mod router;

use std::sync::Arc;

use redis::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::{
    app::AppState,
    model::config_model::ConfigModel,
    modules::{
        socials::service::SocialService,
        sse::hub::SseHub,
        ws::{hub::WsHub, service::WsService},
    },
    utils::jwt::generate_jwt,
};
use uuid::Uuid;

pub async fn test_app_state(social_service: Arc<dyn SocialService>) -> AppState {
    test_app_state_with_ws(social_service, Arc::new(noop::NoopWsService)).await
}

pub async fn test_ws_app_state(ws_service: Arc<dyn WsService>) -> AppState {
    test_app_state_with_ws(Arc::new(noop::NoopSocialService), ws_service).await
}

pub async fn test_app_state_with_ws(
    social_service: Arc<dyn SocialService>,
    ws_service: Arc<dyn WsService>,
) -> AppState {
    let config = Arc::new(fixtures::dummy_config());
    let db = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy("postgres://localhost:5432/runetalk")
        .expect("lazy db pool");

    let client = Client::open("redis://127.0.0.1:6379").expect("redis client");
    let redis = client
        .get_multiplexed_async_connection()
        .await
        .expect("redis connection");

    AppState {
        db: Arc::new(db),
        redis: Arc::new(redis),
        config,
        auth_service: Arc::new(noop::NoopAuthService),
        user_service: Arc::new(noop::NoopUserService),
        social_service,
        ws_service,
        ws_hub: WsHub::new(),
        sse_hub: SseHub::new(),
    }
}

pub async fn repo_test_state(pool: PgPool) -> AppState {
    let config = Arc::new(fixtures::dummy_config());
    let client = Client::open("redis://127.0.0.1:6379").expect("redis client");
    let redis = client
        .get_multiplexed_async_connection()
        .await
        .expect("redis connection");

    AppState {
        db: Arc::new(pool),
        redis: Arc::new(redis),
        config,
        auth_service: Arc::new(noop::NoopAuthService),
        user_service: Arc::new(noop::NoopUserService),
        social_service: Arc::new(noop::NoopSocialService),
        ws_service: Arc::new(noop::NoopWsService),
        ws_hub: WsHub::new(),
        sse_hub: SseHub::new(),
    }
}

pub fn auth_token(user_id: Uuid, config: &ConfigModel) -> String {
    generate_jwt(
        user_id.to_string(),
        "test@example.com".to_string(),
        config,
    )
    .expect("jwt token")
}

pub fn auth_cookie_jar(user_id: Uuid, config: &ConfigModel) -> axum_extra::extract::CookieJar {
    use axum_extra::extract::cookie::Cookie;

    let token = auth_token(user_id, config);
    axum_extra::extract::CookieJar::new().add(Cookie::new("token", token))
}
