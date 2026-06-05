use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{errors::AppResult, model::config_model::DbConfig};

pub async fn load_db(conf: &DbConfig) -> AppResult<PgPool> {
    let mut url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        conf.username, conf.password, conf.host, conf.port, conf.name
    );

    if !conf.ssl_mode.is_empty() {
        url = format!("{}?sslmode={}", url, conf.ssl_mode);
    }

    let pool = PgPoolOptions::new()
        .min_connections(10)
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60 * 30))
        .test_before_acquire(true)
        .connect(&url)
        .await?;

    Ok(pool)
}
