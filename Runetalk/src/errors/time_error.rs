#[derive(Debug, thiserror::Error)]
pub enum TimeError {
    #[error("date/time parse error: {0}")]
    Parse(#[from] chrono::ParseError),
}
