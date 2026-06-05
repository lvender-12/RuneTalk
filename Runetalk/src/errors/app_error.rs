use super::{
    auth_error::AuthError, config_error::ConfigError, db_error::DbError, email_error::EmailError,
    hash_error::HashError, http_error::HttpError, jwt_error::JwtError, redis_error::RedisError,
    serde_error::SerdeError, task_error::TaskError, time_error::TimeError, uuid_error::UuidError,
    validation_error::ValidationError,
};
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
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
