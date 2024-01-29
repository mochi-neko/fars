use std::collections::HashSet;

use crate::oauth::AuthorizationCodeClient;
use crate::oauth::AuthorizationCodeSession;
use crate::oauth::ClientId;
use crate::oauth::MicrosoftIssuer;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::TokenEndpoint;
use crate::oauth::{AuthorizeEndpoint, ClientSecret};

/// A client for the Microsoft's Authorization Code grant type with PKCE of the OAuth 2.0.
///
/// See also [the official document](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow).
///
/// ## NOTE
/// This is only available when the feature `oauth` is enabled.
///
/// ## Recommended use cases
/// - Web-Server, Web-Client, Mobile and Desktop apps with PKCE.
///
/// ## Example
/// ```
/// use fars::oauth::MicrosoftAuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::RedirectUrl;
/// use fars::oauth::MicrosoftIssuer;
/// use std::collections::HashSet;
/// use fars::oauth::OAuthScope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::CsrfState;
///
/// let client = MicrosoftAuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     None,
///     RedirectUrl::new("https://my.app.com/callback")?,
///     MicrosoftIssuer::Common,
/// )?;
///
/// let session = client.generate_authorization_session(HashSet::from([
///     OAuthScope::open_id(),
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
pub struct MicrosoftAuthorizationCodeClient {
    inner: AuthorizationCodeClient,
}

impl MicrosoftAuthorizationCodeClient {
    /// Creates a new client for the Microsoft's Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the Microsoft.
    /// - `redirect_url` - Redirect URL of your app.
    /// - `issuer` - Target type of account.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::MicrosoftAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::MicrosoftIssuer;
    ///
    /// let client = MicrosoftAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     None,
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    ///     MicrosoftIssuer::Common,
    /// )?;
    /// ```
    pub fn new(
        client_id: ClientId,
        client_secret: Option<ClientSecret>,
        redirect_url: RedirectUrl,
        issuer: MicrosoftIssuer,
    ) -> OAuthResult<Self> {
        let client = AuthorizationCodeClient::new(
            client_id,
            client_secret,
            AuthorizeEndpoint::new(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                issuer.format()
            ))?,
            TokenEndpoint::new(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                issuer.format()
            ))?,
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
    /// - `scopes` - The scopes to request authorization defined at [here](https://learn.microsoft.com/en-us/entra/identity-platform/permissions-consent-overview).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::MicrosoftAuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::MicrosoftIssuer;
    /// use std::collections::HashSet;
    /// use fars::oauth::OAuthScope;
    ///
    /// let client = MicrosoftAuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     None,
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    ///     MicrosoftIssuer::Common,
    /// )?;
    ///
    /// let session = client.generate_authorization_session(HashSet::from([
    ///     OAuthScope::open_id(),
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
