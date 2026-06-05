#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("task join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}
