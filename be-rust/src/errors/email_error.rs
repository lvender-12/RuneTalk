#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("invalid email address: {0}")]
    Address(#[from] lettre::address::AddressError),

    #[error("failed to build email: {0}")]
    Build(#[from] lettre::error::Error),

    #[error("failed to send email: {0}")]
    Send(#[from] lettre::transport::smtp::Error),
}
