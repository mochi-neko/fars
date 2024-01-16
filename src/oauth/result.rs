use crate::oauth::OAuthError;

/// The result type for OAuth 2.0 operations.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
pub type OAuthResult<T> = std::result::Result<T, OAuthError>;
