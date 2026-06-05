#[derive(Debug, thiserror::Error)]
pub enum SerdeError {
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
