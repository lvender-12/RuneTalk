use http::StatusCode;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AuthError {
    #[error("invalid email or password")]
    InvalidCredentials,

    #[error("email not verified")]
    EmailNotVerified(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("Not Found")]
    NotFound,

    #[error("otp invalid or expired")]
    InvalidOtp,

    #[error("account already exists")]
    AccountExists,

    #[error("email already registered")]
    EmailAlreadyExists(String),

    #[error("username already taken")]
    UsernameAlreadyExists,

    #[error("email not verified")]
    NotVerified(String),

    #[error("invalid password")]
    InvalidPassword,

    #[error("token expired")]
    TokenExpired,

    #[error("invalid token")]
    InvalidToken,

    #[error("refresh token expired or invalid")]
    InvalidRefreshToken,

    #[error("invalid api secret key")]
    InvalidApiKey,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials
            | Self::Unauthorized
            | Self::InvalidPassword
            | Self::InvalidToken
            | Self::InvalidRefreshToken
            | Self::InvalidApiKey => StatusCode::UNAUTHORIZED,

            Self::Forbidden | Self::NotVerified(_) | Self::EmailNotVerified(_) => {
                StatusCode::FORBIDDEN
            }

            Self::TokenExpired => StatusCode::GONE,

            Self::InvalidOtp => StatusCode::BAD_REQUEST,

            Self::NotFound => StatusCode::NOT_FOUND,

            Self::AccountExists | Self::EmailAlreadyExists(_) | Self::UsernameAlreadyExists => {
                StatusCode::CONFLICT
            }
        }
    }

    pub fn to_data(&self) -> serde_json::Value {
        match self {
            Self::EmailAlreadyExists(email)
            | Self::NotVerified(email)
            | Self::EmailNotVerified(email) => {
                serde_json::json!({ "email": email })
            }
            _ => serde_json::Value::Null,
        }
    }
}
