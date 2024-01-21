use std::collections::HashSet;

use crate::oauth::AuthorizeEndpoint;
use crate::oauth::AuthorizationCodeClient;
use crate::oauth::ClientId;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::OAuthResult;
use crate::oauth::AuthScope;
use crate::oauth::AuthorizationCodeSession;
use crate::oauth::TokenEndpoint;

/// A client for the Twitter's Authorization Code grant type with PKCE of the OAuth 2.0.
///
/// See also [the official document](https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code).
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## WARNING
/// Twitter OAuth 2.0 Access Token may not be supported by the Firebase Auth.
///
/// ## Recommended use cases
/// - Web-Server, Web-Client, Mobile and Desktop apps with PKCE.
///
/// ## Example
/// ```
/// use fars::oauth::TwitterAuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::RedirectUrl;
/// use std::collections::HashSet;
/// use fars::oauth::AuthScope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::CsrfState;
///
/// let client = TwitterAuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     RedirectUrl::new("https://my.app.com/callback")?,
/// )?;
///
/// let session = client.generate_authorization_session(HashSet::from([
///     AuthScope::open_id(),
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
pub struct TwitterAuthorizationCodeClient {
    inner: AuthorizationCodeClient,
}

impl TwitterAuthorizationCodeClient {
    /// Creates a new client for the Twitter's Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the Twitter.
    /// - `redirect_url` - Redirect URL of your app.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::TwitterAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::RedirectUrl;
    ///
    /// let client = TwitterAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    /// ```
    pub fn new(
        client_id: ClientId,
        redirect_url: RedirectUrl,
    ) -> OAuthResult<Self> {
        let client = AuthorizationCodeClient::new(
            client_id,
            None,
            AuthorizeEndpoint::new("https://twitter.com/i/oauth2/authorize")?,
            Some(TokenEndpoint::new(
                "https://api.twitter.com/2/oauth2/token",
            )?),
            redirect_url,
            PkceOption::S256,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    /// Generates a new authorization session.
    ///
    /// ## Arguments
    /// - `scopes` - The scopes to request authorization defined at [here](https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::TwitterAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::RedirectUrl;
    /// use std::collections::HashSet;
    /// use fars::oauth::AuthScope;
    ///
    /// let client = TwitterAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    ///
    /// let session = client.generate_authorization_session(HashSet::from([
    ///     AuthScope::open_id(),
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner();
    /// ```
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<AuthScope>,
    ) -> AuthorizationCodeSession {
        self.inner
            .generate_session(scopes)
    }
}
