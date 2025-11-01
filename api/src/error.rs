// error.rs

use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    // === BlueBubbles API Errors ===
    // Map the API's ErrorType to specific variants
    /// Server error from BlueBubbles
    #[error("Server error: {message}")]
    ServerError { message: String },

    /// Database error from BlueBubbles
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    /// iMessage error from BlueBubbles
    #[error("iMessage error: {message}")]
    IMessageError { message: String },

    /// Socket error from BlueBubbles
    #[error("Socket error: {message}")]
    SocketError { message: String },

    /// Validation error from BlueBubbles
    #[error("Validation error: {message}")]
    ValidationError { message: String },

    /// Authentication error from BlueBubbles
    #[error("Authentication failed: {message}")]
    AuthenticationError { message: String },

    /// Gateway timeout from BlueBubbles
    #[error("Gateway timeout: {message}")]
    GatewayTimeout { message: String },

    // === HTTP/Network Errors ===
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HTTPError(#[from] reqwest::Error),

    /// JSON deserialization failed
    #[error("JSON deserialization failed: {0}")]
    DeserializationError(#[from] serde_path_to_error::Error<serde_json::Error>),

    /// API returned error without data
    #[error("API error ({status}): {message}")]
    ApiError { status: StatusCode, message: String },

    /// Unexpected response from API
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),
}

// Helper to convert a raw api error to the crate error type
impl From<crate::models::response::ErrorResponse> for Error {
    fn from(err: crate::models::response::ErrorResponse) -> Self {
        use crate::models::response::ErrorType;

        match err.error_type {
            ErrorType::ServerError => Error::ServerError {
                message: err.message,
            },
            ErrorType::DatabaseError => Error::DatabaseError {
                message: err.message,
            },
            ErrorType::IMessageError => Error::IMessageError {
                message: err.message,
            },
            ErrorType::SocketError => Error::SocketError {
                message: err.message,
            },
            ErrorType::ValidationError => Error::ValidationError {
                message: err.message,
            },
            ErrorType::AuthenticationError => Error::AuthenticationError {
                message: err.message,
            },
            ErrorType::GatewayTimeout => Error::GatewayTimeout {
                message: err.message,
            },
            ErrorType::Other(type_str) => Error::ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("{}: {}", type_str, err.message),
            },
        }
    }
}
