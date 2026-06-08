use axum::extract::multipart::MultipartError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultipartErrorWrapper {
    #[error("Multipart error: {0}")]
    Extract(#[from] MultipartError),
}
