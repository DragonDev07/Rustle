use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Unknown error")]
    Unknown,
}
