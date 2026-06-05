#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("password hashing failed")]
    Hash(#[from] argon2::password_hash::Error),

    #[error("invalid password hash")]
    InvalidHash,

    #[error("password verification failed")]
    Verify,
}
