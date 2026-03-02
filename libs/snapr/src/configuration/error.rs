use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Failed to parse configuration: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),
}
