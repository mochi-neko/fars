use std::collections::HashSet;

use crate::oauth::AuthorizationCodeClient;
use crate::oauth::AuthorizationCodeSession;
use crate::oauth::AuthorizeEndpoint;
use crate::oauth::ClientId;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::TokenEndpoint;

/// A client for the Facebook's Authorization Code grant type with PKCE of the OAuth 2.0.
///
/// See also [the official document](https://developers.facebook.com/docs/facebook-login/guides/advanced/oidc-token).
///
/// ## NOTE
/// This is only available when the feature `oauth` is enabled.
///
/// ## Recommended use cases
/// - Confidential and Public Clients (Web-Server, Web-Client, Mobile and Desktop apps) with PKCE.
///
/// ## Example
/// ```
/// use fars::oauth::FacebookAuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::RedirectUrl;
/// use std::collections::HashSet;
/// use fars::oauth::OAuthScope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::CsrfState;
///
/// let client = FacebookAuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     RedirectUrl::new("https://my.app.com/callback")?,
/// )?;
///
/// let session = client.generate_authorization_session(HashSet::from([
///     OAuthScope::new("email"),
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
pub struct FacebookAuthorizationCodeClient {
    inner: AuthorizationCodeClient,
}

impl FacebookAuthorizationCodeClient {
    /// Creates a new client for the Facebook's Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the Facebook.
    /// - `redirect_url` - Redirect URL of your app.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::FacebookAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::RedirectUrl;
    ///
    /// let client = FacebookAuthorizationCodeClient::new(
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
            AuthorizeEndpoint::new(
                "https://www.facebook.com/v18.0/dialog/oauth",
            )?,
            TokenEndpoint::new(
                "https://graph.facebook.com/v18.0/oauth/access_token",
            )?,
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
    /// - `scopes` - The scopes to request authorization defined at [here](https://developers.facebook.com/docs/permissions).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::FacebookAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::RedirectUrl;
    /// use std::collections::HashSet;
    /// use fars::oauth::OAuthScope;
    ///
    /// let client = FacebookAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    ///
    /// let session = client.generate_authorization_session(HashSet::from([
    ///     OAuthScope::new("email"),
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
