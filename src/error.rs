//! Error types for the CLI

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CliError>;

impl From<Box<dyn std::error::Error>> for CliError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        CliError::Config(err.to_string())
    }
}

impl From<String> for CliError {
    fn from(err: String) -> Self {
        CliError::Config(err)
    }
}

impl From<&str> for CliError {
    fn from(err: &str) -> Self {
        CliError::Config(err.to_string())
    }
}

/// Main error type for the CLI
#[derive(Error, Debug)]
pub enum CliError {
    // Authentication errors
    #[error("Authentication required. Run 'kite auth login'")]
    NotAuthenticated,

    #[error("Invalid API credentials")]
    InvalidCredentials,

    #[error("Token expired. Run 'kite auth login'")]
    TokenExpired,

    // API errors
    #[error("API error: {message}")]
    ApiError { message: String, code: Option<u16> },

    #[error("Rate limit exceeded. Retrying in {seconds}s...")]
    RateLimitExceeded { seconds: u64 },

    #[error("Invalid instrument: {0}")]
    InvalidInstrument(String),

    #[error("Order rejected: {0}")]
    OrderRejected(String),

    // Network errors
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Request timed out")]
    Timeout,

    // Configuration errors
    #[error("Config error: {0}")]
    Config(String),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // CSV parsing errors
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    // JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    // Shell errors
    #[error("Shell error: {0}")]
    Shell(String),

    // Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),
}
