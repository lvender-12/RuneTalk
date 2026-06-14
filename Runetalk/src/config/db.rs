use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{errors::AppResult, model::config_model::DbConfig};

pub async fn load_db(conf: &DbConfig) -> AppResult<PgPool> {
    let url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        conf.username, conf.password, conf.host, conf.port, conf.name
    );

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .test_before_acquire(true)
        .connect(&url)
        .await?;

    Ok(pool)
}
