use http::StatusCode;

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

impl JwtError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Expired => StatusCode::GONE,
            Self::Invalid | Self::Missing => StatusCode::UNAUTHORIZED,
            Self::Failed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
