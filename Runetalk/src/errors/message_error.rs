#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error("rift not found")]
    RiftNotFound,

    #[error("echo not found")]
    EchoNotFound,

    #[error("scroll not found")]
    ScrollNotFound,

    #[error("whisper not found")]
    WhisperNotFound,

    #[error("not allowed to access this channel")]
    ChannelForbidden,

    #[error("not allowed to access this conversation")]
    ConversationForbidden,
}
