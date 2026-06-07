use axum::Router;
use http::HeaderValue;
use redis::aio::MultiplexedConnection;
use std::fmt;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{
    model::config_model::ConfigModel,
    modules::{auth::service::AuthService, user::service::UserService},
    routes::auth_route::auth_routes,
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<sqlx::PgPool>,
    pub redis: Arc<MultiplexedConnection>,
    pub config: Arc<ConfigModel>,
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("db", &self.db)
            .field("redis", &"RedisConnection")
            .field("config", &self.config)
            .field("auth_service", &"Arc<dyn AuthService>")
            .field("user_service", &"Arc<dyn UserService>")
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
        .with_state(state)
        .layer(cors)
}
