use std::time::Duration;

use oauth2::basic::BasicClient;
use oauth2::AuthUrl;
use oauth2::AuthorizationCode;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;
use oauth2::RedirectUrl;
use oauth2::RequestTokenError;
use oauth2::RevocationUrl;
use oauth2::Scope;
use oauth2::TokenResponse;
use oauth2::TokenUrl;

/// The error type for OAuth2 operations.
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("Invalid authorization URL: {0}")]
    InvalidAuthUrl(String),
    #[error("Invalid token URL: {0}")]
    InvalidTokenUrl(String),
    #[error("Invalid redirect URL: {0}")]
    InvalidRedirectUrl(String),
    #[error("Invalid revocation URL: {0}")]
    InvalidRevocationUrl(String),
    #[error("State mismatch")]
    StateMismatch,
    #[error("Exchange token failed: {0}")]
    ExchangeTokenFailed(
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<
                oauth2::basic::BasicErrorResponseType,
            >,
        >,
    ),
}

/// The result type for OAuth2 operations.
pub type OAuthResult<T> = std::result::Result<T, OAuthError>;

/// The OAuth2 client.
#[derive(Clone)]
pub struct OAuthClient {
    inner: BasicClient,
}

/// The OAuth2 session for authorization.
pub struct OAuthSession {
    /// The authorization URL.
    pub url: String,
    /// The OAuth client.
    client: OAuthClient,
    /// The PKCE code verifier.
    pkce_code_verifier: String,
    /// The CSRF state.
    csrf_state: CsrfToken,
}

/// The OAuth2 token.
pub struct OAuthToken {
    /// The access token.
    pub access_token: String,
    /// The refresh token.
    pub refresh_token: String,
    /// The expiration time.
    pub expires_in: Option<Duration>,
}

impl OAuthClient {
    /// Creates a new [`OAuthClient`].
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
        revocation_url: Option<String>,
    ) -> OAuthResult<Self> {
        // Create an internal OAuth client with settings.
        let mut client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url.clone())
                .map_err(|_| OAuthError::InvalidAuthUrl(auth_url))?,
            Some(
                TokenUrl::new(token_url.clone())
                    .map_err(|_| OAuthError::InvalidTokenUrl(token_url))?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url.clone())
                .map_err(|_| OAuthError::InvalidRedirectUrl(redirect_url))?,
        );

        if let Some(revocation_url) = revocation_url {
            client = client.set_revocation_uri(
                RevocationUrl::new(revocation_url.clone()).map_err(|_| {
                    OAuthError::InvalidRevocationUrl(revocation_url)
                })?,
            );
        }

        Ok(Self {
            inner: client,
        })
    }

    /// Generates an authorization session with authorization URL.
    pub fn generate_authorization_url(
        &self,
        scopes: Vec<String>,
    ) -> OAuthSession {
        // Generate a PKCE code challenge and verifier.
        let (pkce_code_challenge, pkce_code_verifier) =
            PkceCodeChallenge::new_random_sha256();

        // Generate an authorization request.
        let request = self
            .inner
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge);

        // Set scopes.
        let request = scopes
            .iter()
            .fold(request, |request, scope| {
                request.add_scope(Scope::new(scope.to_owned()))
            });

        // Generate an authorization URL.
        let (authorize_url, csrf_state) = request.url();

        OAuthSession {
            url: authorize_url.to_string(),
            client: self.clone(),
            pkce_code_verifier: pkce_code_verifier
                .secret()
                .to_owned(),
            csrf_state,
        }
    }
}

impl OAuthSession {
    /// Exchanges an authorization code into an access token.
    pub async fn exchange_code_into_token(
        &self,
        code: String,
        state: String,
    ) -> OAuthResult<OAuthToken> {
        // Check the CSRF state.
        if state.ne(self.csrf_state.secret()) {
            return Err(OAuthError::StateMismatch);
        }

        // Exchange the authorization code into an access token.
        let token_response = self
            .client
            .inner
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(
                self.pkce_code_verifier
                    .clone(),
            ))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(OAuthError::ExchangeTokenFailed)?;

        Ok(OAuthToken {
            access_token: token_response
                .access_token()
                .secret()
                .to_string(),
            refresh_token: token_response
                .refresh_token()
                .map(|token| token.secret().to_string())
                .unwrap_or_default(),
            expires_in: token_response.expires_in(),
        })
    }
}
