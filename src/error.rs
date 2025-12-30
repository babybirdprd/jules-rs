use thiserror::Error;

#[derive(Debug, Error)]
pub enum JulesError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API Error (Status: {status}): {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Invalid resource name: {0}")]
    InvalidResourceName(String),
}

pub type Result<T> = std::result::Result<T, JulesError>;
