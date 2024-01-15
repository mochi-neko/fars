use std::time::Duration;

use crate::OAuthAccessToken;
use crate::OAuthRefreshToken;

/// The OAuth2 token.
pub struct OAuthToken {
    /// The access token.
    pub access_token: OAuthAccessToken,
    /// The refresh token.
    pub refresh_token: Option<OAuthRefreshToken>,
    /// The expiration time.
    pub expires_in: Option<Duration>,
}
