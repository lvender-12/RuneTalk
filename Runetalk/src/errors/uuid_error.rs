#[derive(Debug, thiserror::Error)]
pub enum UuidError {
    #[error("invalid uuid: {0}")]
    Parse(#[from] uuid::Error),
}
