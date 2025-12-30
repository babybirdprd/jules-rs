//! Error types for the Jules API client.
//!
//! This module provides a unified error type [`JulesError`] that covers all
//! possible error conditions when interacting with the Jules API.

use thiserror::Error;

/// The error type for Jules API operations.
///
/// This enum represents all possible errors that can occur when using the
/// [`JulesClient`](crate::JulesClient).
#[derive(Debug, Error)]
pub enum JulesError {
    /// An HTTP request failed due to network or connection issues.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Failed to serialize or deserialize JSON data.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The API returned an error response.
    ///
    /// This includes the HTTP status code and any error message from the API.
    #[error("API Error (Status: {status}): {message}")]
    Api {
        /// The HTTP status code returned by the API.
        status: reqwest::StatusCode,
        /// The error message from the API response body.
        message: String,
    },

    /// Failed to parse a URL.
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    /// An invalid resource name was provided.
    ///
    /// Resource names must follow the format `resource_type/resource_id`.
    #[error("Invalid resource name: {0}")]
    InvalidResourceName(String),
}

/// A specialized [`Result`](std::result::Result) type for Jules API operations.
///
/// This type alias provides a convenient way to return results that may fail
/// with a [`JulesError`].
pub type Result<T> = std::result::Result<T, JulesError>;
