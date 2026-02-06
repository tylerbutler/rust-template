//! Structured error types for the application.
//!
//! Use [`AppError`] in library code for precise, typed error handling.
//! In binary code (`main.rs`), prefer [`anyhow::Result`] for ergonomic error
//! propagation and contextual messages.

use thiserror::Error;

/// Application-specific error types.
///
/// Each variant demonstrates a common `thiserror` pattern:
/// - **`Io`**: Automatic conversion from [`std::io::Error`] via `#[from]`
/// - **`Config`**: Simple string message for descriptive errors
/// - **`NotFound`**: Structured fields for machine-readable error context
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    /// An I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A configuration error occurred.
    #[error("configuration error: {0}")]
    Config(String),

    /// A required resource was not found.
    #[error("{resource} not found: {id}")]
    NotFound {
        /// The type of resource that was not found.
        resource: &'static str,
        /// The identifier that was looked up.
        id: String,
    },
}

/// A type alias for Results that use [`AppError`].
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_error_converts_automatically() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let app_err = AppError::from(io_err);
        assert!(matches!(app_err, AppError::Io(_)));
        assert!(app_err.to_string().contains("file missing"));
    }

    #[test]
    fn config_error_displays_message() {
        let err = AppError::Config("invalid value".to_string());
        assert_eq!(err.to_string(), "configuration error: invalid value");
    }

    #[test]
    fn not_found_error_includes_fields() {
        let err = AppError::NotFound {
            resource: "user",
            id: "42".to_string(),
        };
        assert_eq!(err.to_string(), "user not found: 42");
    }
}
