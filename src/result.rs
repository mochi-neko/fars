//! Result types in this crate.

/// The result type for APIs.
///
/// Please handle error case by [`crate::Error`].
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The result type for ID token verification.
///
/// Please handle error case by [`crate::error::VerificationError`].
pub type VerificationResult = std::result::Result<
    crate::verification::IdTokenPayloadClaims,
    crate::error::VerificationError,
>;
