use std::sync::Arc;

use axum::{Router, routing::get};
use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;

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
    Router::new()
        .route("/", get(test_handler))
        .with_state(state)
}
