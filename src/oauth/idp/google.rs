use std::collections::HashSet;

use crate::oauth::OAuthAuthUrl;
use crate::oauth::OAuthClient;
use crate::oauth::OAuthClientId;
use crate::oauth::OAuthClientSecret;
use crate::oauth::OAuthCodeChallengeOption;
use crate::oauth::OAuthRedirectUrl;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthRevocationUrl;
use crate::oauth::OAuthScope;
use crate::oauth::OAuthSession;
use crate::oauth::OAuthTokenUrl;

/// The OAuth client for Google.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
pub struct OAuthGoogleClient {
    inner: OAuthClient,
}

impl OAuthGoogleClient {
    /// Creates a new OAuth client for Google.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of.
    pub fn new(
        client_id: OAuthClientId,
        client_secret: OAuthClientSecret,
        redirect_url: OAuthRedirectUrl,
    ) -> OAuthResult<Self> {
        let client = OAuthClient::new(
            client_id,
            Some(client_secret),
            OAuthAuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth")?,
            Some(OAuthTokenUrl::new(
                "https://www.googleapis.com/oauth2/v3/token",
            )?),
            redirect_url,
            Some(OAuthRevocationUrl::new(
                "https://oauth2.googleapis.com/revoke",
            )?),
            OAuthCodeChallengeOption::S256,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthSession {
        self.inner
            .generate_authorization_session(scopes)
    }
}
