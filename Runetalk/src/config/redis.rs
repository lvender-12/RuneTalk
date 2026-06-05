use std::sync::Arc;

use redis::{Client, aio::MultiplexedConnection};
use tokio::sync::Mutex;

use crate::{errors::AppResult, model::config_model::RedisConfig};

pub async fn load_redis(conf: &RedisConfig) -> AppResult<Arc<Mutex<MultiplexedConnection>>> {
    let url = format!(
        "rediss://{}:{}@{}:{}",
        conf.username, conf.password, conf.host, conf.port
    );

    let client = Client::open(url)?;
    let conn = client.get_multiplexed_async_connection().await?;

    Ok(Arc::new(Mutex::new(conn)))
}
