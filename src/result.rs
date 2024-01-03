//! The result type in this crate.

/// The result type in this crate.
///
/// Please handle the error case by [`crate::Error`].
pub type Result<T> = std::result::Result<T, crate::Error>;
