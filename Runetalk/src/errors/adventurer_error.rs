#[derive(Debug, thiserror::Error)]
pub enum AdventurerError {
    #[error("adventurer not found")]
    NotFound,

    #[error("username already taken")]
    UsernameTaken,

    #[error("email already registered")]
    EmailTaken,
}
