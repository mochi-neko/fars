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

/// A client for the Google's Authorization Code grant type with PKCE and Client Secret of the OAuth 2.0.
///
/// See also [the official guide](https://developers.google.com/identity/protocols/oauth2/web-server).
///
/// ## NOTE
/// This is only available when the feature `oauth` is enabled.
///
/// ## Recommended use cases
/// - Confidential clients (Web-Server apps) with PKCE **and Client Secret**.
///
/// ## Not recommended use cases
/// - Public clients (Web-Client, Mobile and Desktop apps) because Client Secret is no longer secret in public clients.
/// - Browserless or input-constrained devices, use Device Code grant type: [`crate::oauth::GoogleDeviceCodeClient`] instead.
///
/// ## Not supported use cases
/// - Any clients **without Client Secret**.
///
/// ## Example
/// ```
/// use fars::oauth::GoogleAuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::ClientSecret;
/// use fars::oauth::RedirectUrl;
/// use fars::oauth::OAuthScope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::CsrfState;
/// use std::collections::HashSet;
///
/// let client = GoogleAuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     ClientSecret::new("client-secret"),
///     RedirectUrl::new("https://my.app.com/callback")?,
/// )?;
///
/// let session = client.generate_session(HashSet::from([
///    OAuthScope::open_id(),
///    OAuthScope::open_id_email(),
///    OAuthScope::open_id_profile()
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
            AuthorizeEndpoint::new(
                "https://accounts.google.com/o/oauth2/v2/auth",
            )?,
            TokenEndpoint::new("https://www.googleapis.com/oauth2/v4/token")?,
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
    /// - `scopes` - Scopes to request authorization defined at [here](https://developers.google.com/identity/protocols/oauth2/scopes).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GoogleAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::OAuthScope;
    /// use std::collections::HashSet;
    ///
    /// let client = GoogleAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     ClientSecret::new("client-secret"),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    /// )?;
    ///
    /// let session = client.generate_session(HashSet::from([
    ///    OAuthScope::open_id(),
    ///    OAuthScope::open_id_email(),
    ///    OAuthScope::open_id_profile()
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner();
    ///
    /// // Redirect the user to the authorize URL and get the code and state from URL.
    /// ```
    pub fn generate_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> AuthorizationCodeSession {
        self.inner
            .generate_session(scopes)
    }
}
