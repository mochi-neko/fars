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

/// A client for the Google's Authorization Code grant type of the OAuth 2.0.
///
/// See also [the official guide](https://developers.google.com/identity/protocols/oauth2/web-server).
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Recommended use cases
/// - Web-Server apps (= Confidential Clients) with PKCE **and client secret**.
///
/// ## Not recommended use cases
/// - Public Clients, such as Web-Client, Mobile and Desktop apps.
///
/// ## Not supported use cases
/// - Any clients with PKCE **without client secret**.
///
/// ## Example
/// ```
/// use fars::oauth::GoogleAuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::ClientSecret;
/// use fars::oauth::RedirectUrl;
/// use fars::oauth::Scope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::State;
/// use std::collections::HashSet;
///
/// let client = GoogleAuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     ClientSecret::new("client-secret"),
///     RedirectUrl::new("https://my.app.com/callback")?,
/// )?;
///
/// let session = client.generate_session(HashSet::from([
///    Scope::open_id(),
///    Scope::open_id_email(),
///    Scope::open_id_profile()
/// ]));
///
/// let authorize_url = session.authorize_url.inner();
///
/// // Redirect the user to the authorize URL and get the code and state from fragments.
/// let code = "code";
/// let state = "state";
///
/// let token = session.exchange_code_into_token(
///     AuthorizationCode::new(code),
///     State::new(state),
/// )?;
///
/// let access_token = token.access_token.inner();
/// ```
pub struct GoogleAuthorizationCodeClient {
    inner: AuthorizationCodeClient,
}

impl GoogleAuthorizationCodeClient {
    /// Creates a new client for the Google's Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the Google Cloud Platform.
    /// - `client_secret` - Client secret of the Google Cloud Platform.
    /// - `redirect_url` - Redirect URL of your app.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GoogleAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::RedirectUrl;
    ///
    /// let client = GoogleAuthorizationCodeClient::new(
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
            AuthorizeEndpoint::new("https://accounts.google.com/o/oauth2/v2/auth")?,
            Some(TokenEndpoint::new(
                "https://www.googleapis.com/oauth2/v4/token",
            )?),
            redirect_url,
            PkceOption::S256,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    /// Generates a new session of the Google's Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `scopes` - Scopes to request authorization.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GoogleAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::Scope;
    /// use std::collections::HashSet;
    ///
    /// let client = GoogleAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     ClientSecret::new("client-secret"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    ///
    /// let session = client.generate_session(HashSet::from([
    ///    Scope::open_id(),
    ///    Scope::open_id_email(),
    ///    Scope::open_id_profile()
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner();
    ///
    /// // Redirect the user to the authorize URL and get the code and state from fragments.
    /// ```
    pub fn generate_session(
        &self,
        scopes: HashSet<Scope>,
    ) -> AuthorizationCodeSession {
        self.inner
            .generate_session(scopes)
    }
}
