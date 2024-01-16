use std::collections::HashSet;

use oauth2::basic::BasicClient;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;

use crate::oauth::OAuthAuthUrl;
use crate::oauth::OAuthAuthorizeUrl;
use crate::oauth::OAuthClientId;
use crate::oauth::OAuthClientSecret;
use crate::oauth::OAuthCodeChallengeOption;
use crate::oauth::OAuthRedirectUrl;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthRevocationUrl;
use crate::oauth::OAuthScope;
use crate::oauth::OAuthSession;
use crate::oauth::OAuthTokenUrl;

/// The OAuth 2.0 client.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
/// ```
/// use fars::oauth::OAuthClient;
/// use fars::oauth::OAuthClientId;
/// use fars::oauth::OAuthClientSecret;
/// use fars::oauth::OAuthAuthUrl;
/// use fars::oauth::OAuthTokenUrl;
/// use fars::oauth::OAuthRedirectUrl;
/// use fars::oauth::OAuthRevocationUrl;
/// use fars::oauth::OAuthCodeChallengeOption;
///
/// let client = OAuthClient::new(
///     OAuthClientId::new("client-id"),
///     Some(OAuthClientSecret::new("client-secret")),
///     OAuthAuthUrl::new("https://example.com/auth")?,
///     Some(OAuthTokenUrl::new("https://example.com/token")?),
///     OAuthRedirectUrl::new("https://my.app.com/callback")?,
///     Some(OAuthRevocationUrl::new("https://example.com/revoke")?),
///     OAuthCodeChallengeOption::S256,
/// )?;
/// ```
#[derive(Clone)]
pub struct OAuthClient {
    pub(crate) inner: BasicClient,
    pub(crate) code_challenge_option: OAuthCodeChallengeOption,
}

impl OAuthClient {
    /// Creates a new [`OAuthClient`].
    ///
    /// ## Arguments
    /// - `client_id` - Client ID.
    /// - `client_secret` - Client secret.
    /// - `auth_url` - Authorization API URL.
    /// - `token_url` - Token API URL.
    /// - `redirect_url` - Redirect URL to receive authorization code.
    /// - `revocation_url` - Revocation API URL.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::OAuthClient;
    /// use fars::oauth::OAuthClientId;
    /// use fars::oauth::OAuthClientSecret;
    /// use fars::oauth::OAuthAuthUrl;
    /// use fars::oauth::OAuthTokenUrl;
    /// use fars::oauth::OAuthRedirectUrl;
    /// use fars::oauth::OAuthRevocationUrl;
    /// use fars::oauth::OAuthCodeChallengeOption;
    ///
    /// let client = OAuthClient::new(
    ///     OAuthClientId::new("client-id"),
    ///     Some(OAuthClientSecret::new("client-secret")),
    ///     OAuthAuthUrl::new("https://example.com/auth")?,
    ///     Some(OAuthTokenUrl::new("https://example.com/token")?),
    ///     OAuthRedirectUrl::new("https://my.app.com/callback")?,
    ///     Some(OAuthRevocationUrl::new("https://example.com/revoke")?),
    ///     OAuthCodeChallengeOption::S256,
    /// )?;
    /// ```
    pub fn new(
        client_id: OAuthClientId,
        client_secret: Option<OAuthClientSecret>,
        auth_url: OAuthAuthUrl,
        token_url: Option<OAuthTokenUrl>,
        redirect_url: OAuthRedirectUrl,
        revocation_url: Option<OAuthRevocationUrl>,
        code_challenge_option: OAuthCodeChallengeOption,
    ) -> OAuthResult<Self> {
        let client_secret = client_secret.map(|client_secret| {
            client_secret
                .inner()
                .to_owned()
        });
        let token_url = token_url.map(|token_url| token_url.inner().to_owned());

        // Create an internal OAuth client with settings.
        let mut client = BasicClient::new(
            client_id.inner().to_owned(),
            client_secret,
            auth_url.inner().to_owned(),
            token_url,
        )
        .set_redirect_uri(
            redirect_url
                .inner()
                .to_owned(),
        );

        if let Some(revocation_url) = revocation_url {
            client = client.set_revocation_uri(
                revocation_url
                    .inner()
                    .to_owned(),
            );
        }

        Ok(Self {
            inner: client,
            code_challenge_option,
        })
    }

    /// Generates an authorization session with authorize URL.
    ///
    /// ## Arguments
    /// - `scopes` - Scopes to request.
    ///
    /// ## Example
    /// ```
    /// use std::collections::HashSet;
    /// use fars::oauth::OAuthClient;
    /// use fars::oauth::OAuthClientId;
    /// use fars::oauth::OAuthClientSecret;
    /// use fars::oauth::OAuthAuthUrl;
    /// use fars::oauth::OAuthTokenUrl;
    /// use fars::oauth::OAuthRedirectUrl;
    /// use fars::oauth::OAuthRevocationUrl;
    /// use fars::oauth::OAuthCodeChallengeOption;
    /// use fars::oauth::OAuthScope;
    ///
    /// let client = OAuthClient::new(
    ///     OAuthClientId::new("client-id"),
    ///     Some(OAuthClientSecret::new("client-secret")),
    ///     OAuthAuthUrl::new("https://example.com/auth").unwrap(),
    ///     Some(OAuthTokenUrl::new("https://example.com/token").unwrap()),
    ///     OAuthRedirectUrl::new("https://my.app.com/callback").unwrap(),
    ///     Some(OAuthRevocationUrl::new("https://example.com/revoke").unwrap()),
    ///     OAuthCodeChallengeOption::S256,
    /// )?;
    ///
    /// let session = client.generate_authorization_session(HashSet::from([
    ///     OAuthScope::new("scope1"),
    ///     OAuthScope::new("scope2"),
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner().clone();
    ///
    /// // Redirect the user to the authorize URL.
    /// ```
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthSession {
        // Generate an authorization request.
        let mut request = self
            .inner
            .authorize_url(CsrfToken::new_random);

        // Add a PKCE code challenge and verifier if supported.
        let code_verifier;
        match self.code_challenge_option {
            | OAuthCodeChallengeOption::S256 => {
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
            | OAuthCodeChallengeOption::NotSupported => {
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

        OAuthSession {
            authorize_url: OAuthAuthorizeUrl::new(authorize_url),
            client: self.clone(),
            pkce_code_verifier: code_verifier,
            csrf_state,
        }
    }
}
