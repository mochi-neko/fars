use std::collections::HashSet;

use crate::oauth::AuthorizeEndpoint;
use crate::oauth::AuthorizationCodeClient;
use crate::oauth::ClientId;
use crate::oauth::ClientSecret;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::OAuthResult;
use crate::oauth::Scope;
use crate::oauth::AuthorizationCodeSession;
use crate::oauth::TokenEndpoint;

/// The OAuth client for GitHub.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
pub struct OAuthGitHubClient {
    inner: AuthorizationCodeClient,
}

impl OAuthGitHubClient {
    /// Creates a new OAuth client for GitHub.
    ///
    /// ## Arguments
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> OAuthResult<Self> {
        let client = AuthorizationCodeClient::new(
            client_id,
            Some(client_secret),
            AuthorizeEndpoint::new("https://github.com/login/oauth/authorize")?,
            Some(TokenEndpoint::new(
                "https://github.com/login/oauth/access_token",
            )?),
            redirect_url,
            PkceOption::NotSupported,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    ///
    /// [scopes](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps)
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<Scope>,
    ) -> AuthorizationCodeSession {
        self.inner
            .generate_session(scopes)
    }
}
