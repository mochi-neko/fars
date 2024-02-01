use oauth2::{ConfigurationError, RequestTokenError};

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
    /// Exchange token failed on the authorization code flow.
    #[error("Auth code exchange token failed: {0:?}")]
    AuthCodeExchangeTokenFailed(
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<
                oauth2::basic::BasicErrorResponseType,
            >,
        >,
    ),
    /// Device authorization request error.
    #[error("Device authorization request error: {0:?}")]
    DeviceAuthorizationRequestError(ConfigurationError),
    /// Exchange device code failed on the device code flow.
    #[error("Device code exchange token failed: {0:?}")]
    DeviceCodeExchangeFailed(
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<
                oauth2::basic::BasicErrorResponseType,
            >,
        >,
    ),
    /// Exchange token failed on the device code flow.
    #[error("Device exchange token failed: {0:?}")]
    DeviceExchangeTokenFailed(
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::DeviceCodeErrorResponse,
        >,
    ),
    /// Internal reqwest error.
    #[error("Internal reqwest error: {0:?}")]
    ReqwestError(reqwest::Error),
    /// JSON deserialization failed.
    #[error("JSON deserialization failed: {0:?}, {1:?}")]
    JsonDeserializationFailed(serde_json::Error, String),
    /// Manual API call failed.
    #[error("Manual API call failed: {0:?}, {1:?}")]
    ManualApiCallFailed(reqwest::StatusCode, String),
    /// Continue polling.
    #[error("Continue polling")]
    ContinuePolling,
    /// Timeout.
    #[error("Timeout")]
    Timeout,
}
