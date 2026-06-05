#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("database error: {0}")]
    Query(#[from] sqlx::Error),

    #[error("{entity} not found")]
    NotFound { entity: &'static str },

    #[error("{entity} already exists")]
    Conflict { entity: &'static str },
}

impl DbError {
    pub fn not_found(entity: &'static str) -> Self {
        Self::NotFound { entity }
    }

    pub fn conflict(entity: &'static str) -> Self {
        Self::Conflict { entity }
    }
}
