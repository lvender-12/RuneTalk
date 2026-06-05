#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Fields(#[from] validator::ValidationErrors),
}
