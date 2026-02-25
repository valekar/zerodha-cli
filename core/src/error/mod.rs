//! Error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZerodhaError {
    #[error("API error: {message}")]
    Api { status: u16, message: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Parse error: {0}")]
    Parse(String),
}
