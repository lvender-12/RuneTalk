use std::sync::Arc;

use redis::{Client, aio::MultiplexedConnection};

use crate::{errors::AppResult, model::config_model::RedisConfig};

pub async fn load_redis(conf: &RedisConfig) -> AppResult<Arc<MultiplexedConnection>> {
    let scheme = if conf.host == "localhost" || conf.host == "127.0.0.1" {
        "redis"
    } else {
        "rediss"
    };

    let url = match (conf.username.is_empty(), conf.password.is_empty()) {
        (true, true) => format!("{}://{}:{}", scheme, conf.host, conf.port),
        (true, false) => format!(
            "{}://:{}@{}:{}",
            scheme, conf.password, conf.host, conf.port
        ),
        (false, true) => format!("{}://{}@{}:{}", scheme, conf.username, conf.host, conf.port),
        (false, false) => format!(
            "{}://{}:{}@{}:{}",
            scheme, conf.username, conf.password, conf.host, conf.port
        ),
    };

    let client = Client::open(url)?;
    let conn = client.get_multiplexed_async_connection().await?;

    Ok(Arc::new(conn))
}
