use crate::oauth::OAuthError;

/// The result type for OAuth2 operations.
pub type OAuthResult<T> = std::result::Result<T, OAuthError>;
