use crate::verification::VerificationError;

/// The result type for ID token verification.
///
/// Please handle error case by [`VerificationError`].
///
/// ## NOTE
/// This is only available when the feature "verify" is enabled.
pub type VerificationResult = std::result::Result<
    crate::verification::IdTokenPayloadClaims,
    VerificationError,
>;
