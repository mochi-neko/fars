use oauth2::RequestTokenError;

/// The error type for OAuth 2.0 operations.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    /// Invalid auth URL.
    #[error("Invalid auth URL: {0}")]
    InvalidAuthUrl(String),
    /// Invalid Token URL.
    #[error("Invalid token URL: {0}")]
    InvalidTokenUrl(String),
    /// Invalid redirect URL.
    #[error("Invalid redirect URL: {0}")]
    InvalidRedirectUrl(String),
    /// Invalid revocation URL.
    #[error("Invalid revocation URL: {0}")]
    InvalidRevocationUrl(String),
    /// State mismatch.
    #[error("State mismatch")]
    StateMismatch,
    /// Exchange token failed.
    #[error("Exchange token failed: {0}")]
    ExchangeTokenFailed(
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<
                oauth2::basic::BasicErrorResponseType,
            >,
        >,
    ),
}
