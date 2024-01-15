use std::time::Duration;

/// The OAuth2 token.
pub struct OAuthToken {
    /// The access token.
    pub access_token: String,
    /// The refresh token.
    pub refresh_token: String,
    /// The expiration time.
    pub expires_in: Option<Duration>,
}
