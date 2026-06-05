use super::{
    adventurer_error::AdventurerError, auth_error::AuthError, config_error::ConfigError,
    db_error::DbError, email_error::EmailError, guild_error::GuildError, hash_error::HashError,
    http_error::HttpError, jwt_error::JwtError, message_error::MessageError,
    redis_error::RedisError, serde_error::SerdeError, social_error::SocialError,
    task_error::TaskError, time_error::TimeError, uuid_error::UuidError,
    validation_error::ValidationError,
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Adventurer(#[from] AdventurerError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Db(#[from] DbError),

    #[error(transparent)]
    Email(#[from] EmailError),

    #[error(transparent)]
    Guild(#[from] GuildError),

    #[error(transparent)]
    Hash(#[from] HashError),

    #[error(transparent)]
    Http(#[from] HttpError),

    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error(transparent)]
    Message(#[from] MessageError),

    #[error(transparent)]
    Redis(#[from] RedisError),

    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Social(#[from] SocialError),

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
