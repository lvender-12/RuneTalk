#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("jwt error: {0}")]
    Failed(#[from] jsonwebtoken::errors::Error),

    #[error("token expired")]
    Expired,

    #[error("invalid token")]
    Invalid,

    #[error("missing token")]
    Missing,
}
