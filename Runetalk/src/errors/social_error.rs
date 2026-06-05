#[derive(Debug, thiserror::Error)]
pub enum SocialError {
    #[error("friend request not found")]
    PledgeNotFound,

    #[error("friend request already sent")]
    PledgeAlreadySent,

    #[error("already friends")]
    AlreadyAllies,

    #[error("cannot friend yourself")]
    SelfRequest,
}
