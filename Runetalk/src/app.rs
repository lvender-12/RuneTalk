use std::sync::Arc;

use axum::{Router, routing::get};
use http::HeaderValue;
use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::model::config_model::ConfigModel;

pub async fn test_handler() -> &'static str {
    "Hello, World!"
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<sqlx::PgPool>,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
    pub config: Arc<ConfigModel>,
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
        .route("/", get(test_handler))
        .with_state(state)
        .layer(cors)
}
