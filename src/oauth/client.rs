use oauth2::basic::BasicClient;
use oauth2::AuthUrl;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::RedirectUrl;
use oauth2::RevocationUrl;
use oauth2::Scope;
use oauth2::TokenUrl;

use crate::OAuthError;
use crate::OAuthResult;
use crate::OAuthSession;

/// The OAuth2 client.
#[derive(Clone)]
pub struct OAuthClient {
    pub(crate) inner: BasicClient,
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
