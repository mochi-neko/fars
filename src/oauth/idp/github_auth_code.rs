use std::collections::HashSet;

use crate::oauth::AuthorizationCodeClient;
use crate::oauth::AuthorizationCodeSession;
use crate::oauth::AuthorizeEndpoint;
use crate::oauth::ClientId;
use crate::oauth::ClientSecret;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::TokenEndpoint;

/// A client for the GitHub's Authorization Code grant type with Client Secret of the OAuth 2.0.
///
/// See also [the official document](https://docs.github.com/en/developers/apps/authorizing-oauth-apps#web-application-flow).
///
/// ## NOTE
/// This is only available when the feature `oauth` is enabled.
///
/// ## IMPORTANT
/// GitHub OAuth 2.0 does not support PKCE.
///
/// ## Recommended use cases
/// - Confidential clients (Web-Server apps) **with Client Secret** without PKCE.
///
/// ## Not recommended use cases
/// - Public clients (Web-Client, Mobile and Desktop apps) because Client Secret is no longer secret in public clients.
///
/// ## Not supported use cases
/// - Any clients **with PKCE**.
///
/// ## Example
/// ```
/// use fars::oauth::GitHubAuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::ClientSecret;
/// use fars::oauth::RedirectUrl;
/// use fars::oauth::OAuthScope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::CsrfState;
/// use std::collections::HashSet;
///
/// let client = GitHubAuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     ClientSecret::new("client-secret"),
///     RedirectUrl::new("https://my.app.com/callback")?,
/// )?;
///
/// let session = client.generate_authorization_session(HashSet::from([
///     OAuthScope::new("read:user"),
///     OAuthScope::new("user:email"),
/// ]));
///
/// let authorize_url = session.authorize_url.inner();
///
/// // Redirect the user to the authorize URL and get the code and state from URL.
/// let code = "code";
/// let state = "state";
///
/// let token = session.exchange_code_into_token(
///     AuthorizationCode::new(code),
///     CsrfState::new(state),
/// )?;
///
/// let access_token = token.access_token.inner();
/// ```
pub struct GitHubAuthorizationCodeClient {
    inner: AuthorizationCodeClient,
}

impl GitHubAuthorizationCodeClient {
    /// Creates a new client for the GitHub's Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the GitHub.
    /// - `client_secret` - Client secret of the GitHub.
    /// - `redirect_url` - Redirect URL of your app.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GitHubAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::RedirectUrl;
    ///
    /// let client = GitHubAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     ClientSecret::new("client-secret"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    /// ```
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> OAuthResult<Self> {
        let client = AuthorizationCodeClient::new(
            client_id,
            Some(client_secret),
            AuthorizeEndpoint::new("https://github.com/login/oauth/authorize")?,
            TokenEndpoint::new("https://github.com/login/oauth/access_token")?,
            redirect_url,
            PkceOption::NotSupported,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    /// Generates a new authorization session.
    ///
    /// ## Arguments
    /// - `scopes` - The scopes to request authorization defined at [here](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GitHubAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::OAuthScope;
    /// use std::collections::HashSet;
    ///
    /// let client = GitHubAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     ClientSecret::new("client-secret"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    ///
    /// let session = client.generate_authorization_session(HashSet::from([
    ///     OAuthScope::new("read:user"),
    ///     OAuthScope::new("user:email"),
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner();
    /// ```
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> AuthorizationCodeSession {
        self.inner
            .generate_session(scopes)
    }
}
