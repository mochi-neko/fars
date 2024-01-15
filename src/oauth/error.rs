use oauth2::RequestTokenError;

/// The error type for OAuth2 operations.
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("Invalid authorization URL: {0}")]
    InvalidAuthUrl(String),
    #[error("Invalid token URL: {0}")]
    InvalidTokenUrl(String),
    #[error("Invalid redirect URL: {0}")]
    InvalidRedirectUrl(String),
    #[error("Invalid revocation URL: {0}")]
    InvalidRevocationUrl(String),
    #[error("State mismatch")]
    StateMismatch,
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
