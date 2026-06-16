use axum::{Router, middleware::from_fn_with_state};
use http::HeaderValue;
use redis::aio::MultiplexedConnection;
use std::fmt;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{
    middleware::{
        api_keys::api_keys_middleware, method_not_allowed::method_not_allowed,
        not_found::not_found_middleware,
    },
    model::config_model::ConfigModel,
    modules::{
        auth::service::AuthService, socials::service::SocialService,
        sse::hub::SseHub,
        user::service::UserService, ws::{hub::WsHub, service::WsService},
    },
    routes::{auth_route::auth_routes, protected::protected_routes},
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<sqlx::PgPool>,
    pub redis: Arc<MultiplexedConnection>,
    pub config: Arc<ConfigModel>,
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
    pub social_service: Arc<dyn SocialService>,
    pub ws_service: Arc<dyn WsService>,
    pub ws_hub: WsHub,
    pub sse_hub: SseHub,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("db", &self.db)
            .field("redis", &"RedisConnection")
            .field("config", &self.config)
            .field("auth_service", &"Arc<dyn AuthService>")
            .field("user_service", &"Arc<dyn UserService>")
            .field("social_service", &"Arc<dyn SocialService>")
            .field("ws_service", &"Arc<dyn WsService>")
            .field("ws_hub", &"WsHub")
            .field("sse_hub", &"SseHub")
            .finish()
    }
}

pub fn create_app(state: AppState) -> Router {
    let allowed_origin_vec: Vec<String> = state.config.allowed_origins.clone();

    let origins: Vec<HeaderValue> = allowed_origin_vec
        .into_iter()
        .map(|src| src.parse::<HeaderValue>().expect("allowed origin failed"))
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PATCH,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::UPGRADE,
            http::header::CONNECTION,
            http::header::HeaderName::from_static("x-api-secret"),
            http::header::HeaderName::from_static("sec-websocket-key"),
            http::header::HeaderName::from_static("sec-websocket-version"),
        ])
        .allow_credentials(true);

    Router::new()
        .merge(auth_routes(state.clone()))
        .merge(protected_routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), api_keys_middleware))
        .method_not_allowed_fallback(method_not_allowed)
        .fallback(not_found_middleware)
        .with_state(state)
        .layer(cors)
}
