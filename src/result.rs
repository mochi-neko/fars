//! A result type for the Firebase Auth API.

/// Result type for the Firebase Auth API.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
