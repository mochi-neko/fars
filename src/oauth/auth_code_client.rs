use std::collections::HashSet;

use oauth2::basic::BasicClient;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;

use crate::oauth::AuthorizationCodeSession;
use crate::oauth::AuthorizeEndpoint;
use crate::oauth::AuthorizeUrl;
use crate::oauth::ClientId;
use crate::oauth::ClientSecret;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::TokenEndpoint;

/// A client for the Authorization Code grant type of the OAuth 2.0.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Recommended use cases
/// - Confidential clients (Web-Server apps) with PKCE and/or client secret.
/// - Public clients (Web-Client, Mobile and Desktop apps) with PKCE **without client secret**.
///
/// ## Not recommended use cases
/// - Public clients (Web-Client, Mobile and Desktop apps) with **client secret**, because secret is no longer secret in public clients.
///
/// ## Not supported use cases
/// - Browserless or input-constrained devices, use Device Code grant type: [`crate::oauth::DeviceCodeClient`] instead.
///
/// ## Example
/// ```
/// use fars::oauth::AuthorizationCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::ClientSecret;
/// use fars::oauth::AuthorizeEndpoint;
/// use fars::oauth::TokenEndpoint;
/// use fars::oauth::RedirectUrl;
/// use fars::oauth::PkceOption;
///
/// let client = AuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     Some(ClientSecret::new("client-secret")),
///     AuthorizeEndpoint::new("https://example.com/auth")?,
///     TokenEndpoint::new("https://example.com/token")?,
///     RedirectUrl::new("https://my.app.com/callback")?,
///     PkceOption::S256,
/// )?;
/// ```
#[derive(Clone)]
pub struct AuthorizationCodeClient {
    pub(crate) client: BasicClient,
    pub(crate) pkce_option: PkceOption,
}

impl AuthorizationCodeClient {
    /// Creates a new client for the Authorization Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID.
    /// - `client_secret` - Client secret.
    /// - `authorize_endpoint` - Authorization API URL.
    /// - `token_endpoint` - Token API URL.
    /// - `redirect_url` - Redirect URL to receive authorization code.
    /// - `pkce_option` - The PKCE code challenge option.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::AuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::AuthorizeEndpoint;
    /// use fars::oauth::TokenEndpoint;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::PkceOption;
    ///
    /// let client = AuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     Some(ClientSecret::new("client-secret")),
    ///     AuthorizeEndpoint::new("https://example.com/auth")?,
    ///     TokenEndpoint::new("https://example.com/token")?,
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    ///     PkceOption::S256,
    /// )?;
    /// ```
    pub fn new(
        client_id: ClientId,
        client_secret: Option<ClientSecret>,
        authorize_endpoint: AuthorizeEndpoint,
        token_endpoint: TokenEndpoint,
        redirect_url: RedirectUrl,
        pkce_option: PkceOption,
    ) -> OAuthResult<Self> {
        let client_secret = client_secret.map(|client_secret| {
            client_secret
                .inner()
                .to_owned()
        });

        // Create an internal OAuth client with settings.
        let client = BasicClient::new(
            client_id.inner().to_owned(),
            client_secret,
            authorize_endpoint
                .inner()
                .to_owned(),
            Some(
                token_endpoint
                    .inner()
                    .to_owned(),
            ),
        )
        .set_redirect_uri(
            redirect_url
                .inner()
                .to_owned(),
        );

        Ok(Self {
            client,
            pkce_option,
        })
    }

    /// Generates an Authorization Code flow session with authorize URL.
    ///
    /// ## Arguments
    /// - `scopes` - Scopes to request authorization.
    ///
    /// ## Example
    /// ```
    /// use std::collections::HashSet;
    /// use fars::oauth::AuthorizationCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::AuthorizeEndpoint;
    /// use fars::oauth::TokenEndpoint;
    /// use fars::oauth::RedirectUrl;
    /// use fars::oauth::PkceOption;
    /// use fars::oauth::OAuthScope;
    ///
    /// let client = AuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     Some(ClientSecret::new("client-secret")),
    ///     AuthorizeEndpoint::new("https://example.com/auth").unwrap(),
    ///     TokenEndpoint::new("https://example.com/token").unwrap(),
    ///     RedirectUrl::new("https://my.app.com/callback").unwrap(),
    ///     PkceOption::S256,
    /// )?;
    ///
    /// let session = client.generate_session(HashSet::from([
    ///     OAuthScope::new("scope1"),
    ///     OAuthScope::new("scope2"),
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner().clone();
    ///
    /// // Redirect the user to the authorize URL.
    /// ```
    pub fn generate_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> AuthorizationCodeSession {
        // Generate an authorization request.
        let mut request = self
            .client
            .authorize_url(CsrfToken::new_random);

        // Add a PKCE code challenge and verifier if supported.
        let code_verifier;
        match self.pkce_option {
            | PkceOption::S256 => {
                // Generate a PKCE code challenge and verifier.
                let (pkce_code_challenge, pkce_code_verifier) =
                    PkceCodeChallenge::new_random_sha256();

                request = request.set_pkce_challenge(pkce_code_challenge);

                code_verifier = Some(
                    pkce_code_verifier
                        .secret()
                        .to_owned(),
                );
            },
            | PkceOption::NotSupported => {
                code_verifier = None;
            },
        };

        // Set scopes.
        let request = scopes
            .iter()
            .fold(request, |request, scope| {
                request.add_scope(scope.inner().to_owned())
            });

        // Generate an authorize URL with state.
        let (authorize_url, csrf_state) = request.url();

        AuthorizationCodeSession {
            authorize_url: AuthorizeUrl::new(authorize_url),
            client: self.clone(),
            pkce_code_verifier: code_verifier,
            csrf_state,
        }
    }
}
