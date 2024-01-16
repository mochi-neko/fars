use std::collections::HashSet;

use crate::oauth::OAuthAuthUrl;
use crate::oauth::OAuthClient;
use crate::oauth::OAuthClientId;
use crate::oauth::OAuthClientSecret;
use crate::oauth::OAuthCodeChallengeOption;
use crate::oauth::OAuthRedirectUrl;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::OAuthSession;
use crate::oauth::OAuthTokenUrl;

/// The OAuth client for Facebook.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
pub struct OAuthFacebookClient {
    inner: OAuthClient,
}

impl OAuthFacebookClient {
    /// Creates a new OAuth client for Facebook.
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
            OAuthAuthUrl::new("https://www.facebook.com/v18.0/dialog/oauth")?,
            Some(OAuthTokenUrl::new(
                "https://graph.facebook.com/v18.0/oauth/access_token",
            )?),
            redirect_url,
            None,
            OAuthCodeChallengeOption::S256, // https://developers.facebook.com/docs/facebook-login/guides/advanced/oidc-token
        )?;

        Ok(Self {
            inner: client,
        })
    }

    ///
    /// https://developers.facebook.com/docs/facebook-login/guides/permissions
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthSession {
        self.inner
            .generate_authorization_session(scopes)
    }
}
