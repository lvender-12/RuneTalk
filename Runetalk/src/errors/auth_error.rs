#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid email or password")]
    InvalidCredentials,

    #[error("email not verified")]
    EmailNotVerified,

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("otp invalid or expired")]
    InvalidOtp,

    #[error("account already exists")]
    AccountExists,

    #[error("email already registered")]
    EmailAlreadyExists,

    #[error("username already taken")]
    UsernameAlreadyExists,
}
