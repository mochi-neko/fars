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

/// The OAuth client for GitHub.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
pub struct OAuthGitHubClient {
    inner: OAuthClient,
}

impl OAuthGitHubClient {
    /// Creates a new OAuth client for GitHub.
    ///
    /// ## Arguments
    pub fn new(
        client_id: OAuthClientId,
        client_secret: OAuthClientSecret,
        redirect_url: OAuthRedirectUrl,
    ) -> OAuthResult<Self> {
        let client = OAuthClient::new(
            client_id,
            Some(client_secret),
            OAuthAuthUrl::new("https://github.com/login/oauth/authorize")?,
            Some(OAuthTokenUrl::new(
                "https://github.com/login/oauth/access_token",
            )?),
            redirect_url,
            None,
            OAuthCodeChallengeOption::NotSupported,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    ///
    /// [scopes](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps)
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthSession {
        self.inner
            .generate_authorization_session(scopes)
    }
}
