#[derive(Debug, thiserror::Error)]
pub enum GuildError {
    #[error("guild not found")]
    NotFound,

    #[error("guild name already exists")]
    NameTaken,

    #[error("invalid invite code")]
    InvalidInvite,

    #[error("not a guild member")]
    NotMember,

    #[error("insufficient guild permissions")]
    Forbidden,
}
