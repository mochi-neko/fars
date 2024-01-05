//! Result types in this crate.

/// The result type for APIs.
///
/// Please handle error case by [`crate::Error`].
pub type Result<T> = std::result::Result<T, crate::Error>;
