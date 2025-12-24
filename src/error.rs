//! CLI error types with exit codes.

use std::process::ExitCode;
use thiserror::Error;

/// CLI errors mapped to specific exit codes.
#[derive(Error, Debug)]
pub enum CliError {
    /// General API or JSON error.
    #[error("{0}")]
    General(String),

    /// Invalid arguments or configuration.
    #[error("{0}")]
    Validation(String),

    /// Authentication error (invalid/missing API key).
    #[error("{0}")]
    Authentication(String),

    /// Rate limit exceeded.
    #[error("rate limit exceeded")]
    RateLimit,

    /// Resource not found.
    #[error("{0}")]
    NotFound(String),

    /// Network or timeout error.
    #[error("{0}")]
    Network(String),

    /// Config file error.
    #[error("config error: {0}")]
    Config(String),
}

impl CliError {
    /// Get the exit code for this error.
    pub fn exit_code(&self) -> ExitCode {
        match self {
            CliError::General(_) => ExitCode::from(1),
            CliError::Validation(_) | CliError::Config(_) => ExitCode::from(2),
            CliError::Authentication(_) => ExitCode::from(3),
            CliError::RateLimit => ExitCode::from(4),
            CliError::NotFound(_) => ExitCode::from(5),
            CliError::Network(_) => ExitCode::from(6),
        }
    }
}

impl From<earningsfeed::Error> for CliError {
    fn from(err: earningsfeed::Error) -> Self {
        match err {
            earningsfeed::Error::Authentication => {
                CliError::Authentication("invalid or missing API key".to_string())
            }
            earningsfeed::Error::RateLimit { .. } => CliError::RateLimit,
            earningsfeed::Error::NotFound { path } => {
                CliError::NotFound(format!("not found: {}", path))
            }
            earningsfeed::Error::Validation { message } => CliError::Validation(message),
            earningsfeed::Error::Timeout(duration) => {
                CliError::Network(format!("request timed out after {:?}", duration))
            }
            earningsfeed::Error::Http(e) => CliError::Network(e.to_string()),
            earningsfeed::Error::Api { message, .. } => CliError::General(message),
            earningsfeed::Error::Json(e) => CliError::General(e.to_string()),
            earningsfeed::Error::Config(msg) => CliError::Config(msg),
        }
    }
}

impl From<confy::ConfyError> for CliError {
    fn from(err: confy::ConfyError) -> Self {
        CliError::Config(err.to_string())
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::General(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CliError>;
