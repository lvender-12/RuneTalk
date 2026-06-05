#[derive(Debug, thiserror::Error)]
pub enum RedisError {
    #[error("redis error: {0}")]
    Operation(#[from] ::redis::RedisError),

    #[error("cache key not found: {0}")]
    KeyNotFound(String),
}
