#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("http error: {0}")]
    Build(#[from] http::Error),

    #[error("invalid header name: {0}")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),

    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error("invalid http method: {0}")]
    InvalidMethod(#[from] http::method::InvalidMethod),

    #[error("invalid status code: {0}")]
    InvalidStatusCode(#[from] http::status::InvalidStatusCode),

    #[error("invalid uri: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
}
