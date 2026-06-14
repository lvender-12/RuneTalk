use super::{
    auth_error::AuthError, config_error::ConfigError, db_error::DbError, email_error::EmailError,
    hash_error::HashError, http_error::HttpError, jwt_error::JwtError, redis_error::RedisError,
    serde_error::SerdeError, task_error::TaskError, time_error::TimeError, uuid_error::UuidError,
    validation_error::ValidationError,
};
use axum::{
    Json,
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Multipart error: {0}")]
    Multipart(MultipartError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Db(#[from] DbError),

    #[error(transparent)]
    Email(#[from] EmailError),

    #[error(transparent)]
    Hash(#[from] HashError),

    #[error(transparent)]
    Http(#[from] HttpError),

    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error(transparent)]
    Redis(#[from] RedisError),

    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Task(#[from] TaskError),

    #[error(transparent)]
    Time(#[from] TimeError),

    #[error(transparent)]
    Uuid(#[from] UuidError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error("internal server error")]
    Internal(#[source] anyhow::Error),
}

impl AppError {
    pub fn internal(err: impl Into<anyhow::Error>) -> Self {
        Self::Internal(err.into())
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Multipart(_) => StatusCode::BAD_REQUEST,
            AppError::Auth(err) => err.status_code(),
            AppError::Jwt(err) => err.status_code(),
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Db(err) => err.status_code(),
            AppError::Redis(_)
            | AppError::Config(_)
            | AppError::Email(_)
            | AppError::Hash(_)
            | AppError::Http(_)
            | AppError::Serde(_)
            | AppError::Task(_)
            | AppError::Time(_)
            | AppError::Uuid(_)
            | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_data(&self) -> serde_json::Value {
        match self {
            AppError::Auth(err) => err.to_data(),
            _ => serde_json::Value::Null,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.to_string();
        let data = self.to_data();

        let body = if !data.is_null() {
            crate::common::response::ApiResponse::error_with_data(&message, data)
        } else {
            crate::common::response::ApiResponse::error(&message)
        };

        (status, Json(body)).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::internal(err)
    }
}

macro_rules! impl_app_error_from {
    ($source:ty => $variant:ident($domain:ty)) => {
        impl From<$source> for AppError {
            fn from(err: $source) -> Self {
                Self::$variant(<$domain>::from(err))
            }
        }
    };
}

impl_app_error_from!(::config::ConfigError => Config(ConfigError));
impl_app_error_from!(sqlx::Error => Db(DbError));
impl_app_error_from!(::redis::RedisError => Redis(RedisError));
impl_app_error_from!(serde_json::Error => Serde(SerdeError));
impl_app_error_from!(argon2::password_hash::Error => Hash(HashError));

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            JwtErrorKind::ExpiredSignature => Self::Jwt(JwtError::Expired),
            JwtErrorKind::InvalidToken
            | JwtErrorKind::InvalidSignature
            | JwtErrorKind::InvalidAlgorithm
            | JwtErrorKind::InvalidAlgorithmName
            | JwtErrorKind::InvalidAudience
            | JwtErrorKind::InvalidIssuer
            | JwtErrorKind::InvalidSubject
            | JwtErrorKind::ImmatureSignature
            | JwtErrorKind::MissingAlgorithm
            | JwtErrorKind::MissingRequiredClaim(_)
            | JwtErrorKind::InvalidClaimFormat(_) => Self::Jwt(JwtError::Invalid),
            _ => Self::Jwt(JwtError::Failed(err)),
        }
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        Self::Multipart(err)
    }
}

impl_app_error_from!(http::Error => Http(HttpError));
impl_app_error_from!(http::header::InvalidHeaderName => Http(HttpError));
impl_app_error_from!(http::header::InvalidHeaderValue => Http(HttpError));
impl_app_error_from!(http::method::InvalidMethod => Http(HttpError));
impl_app_error_from!(http::status::InvalidStatusCode => Http(HttpError));
impl_app_error_from!(http::uri::InvalidUri => Http(HttpError));
impl_app_error_from!(lettre::address::AddressError => Email(EmailError));
impl_app_error_from!(lettre::error::Error => Email(EmailError));
impl_app_error_from!(lettre::transport::smtp::Error => Email(EmailError));
impl_app_error_from!(std::io::Error => Task(TaskError));
impl_app_error_from!(tokio::task::JoinError => Task(TaskError));
impl_app_error_from!(chrono::ParseError => Time(TimeError));
impl_app_error_from!(uuid::Error => Uuid(UuidError));
impl_app_error_from!(validator::ValidationErrors => Validation(ValidationError));
