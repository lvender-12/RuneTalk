#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config error: {0}")]
    Load(#[from] config::ConfigError),

    #[error("missing config field: {0}")]
    MissingField(&'static str),

    #[error("invalid config value for {field}: {reason}")]
    InvalidValue { field: &'static str, reason: String },
}
