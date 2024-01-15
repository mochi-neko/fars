use std::collections::HashSet;

use oauth2::basic::BasicClient;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;

use crate::OAuthAuthUrl;
use crate::OAuthAuthorizeUrl;
use crate::OAuthClientId;
use crate::OAuthClientSecret;
use crate::OAuthRedirectUrl;
use crate::OAuthResult;
use crate::OAuthRevocationUrl;
use crate::OAuthScope;
use crate::OAuthSession;
use crate::OAuthTokenUrl;

/// The OAuth2 client.
#[derive(Clone)]
pub struct OAuthClient {
    pub(crate) inner: BasicClient,
}

impl OAuthClient {
    /// Creates a new [`OAuthClient`].
    pub fn new(
        client_id: OAuthClientId,
        client_secret: Option<OAuthClientSecret>,
        auth_url: OAuthAuthUrl,
        token_url: Option<OAuthTokenUrl>,
        redirect_url: OAuthRedirectUrl,
        revocation_url: Option<OAuthRevocationUrl>,
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
        })
    }

    /// Generates an authorization session with authorization URL.
    pub fn generate_authorization_url(
        &self,
        scopes: HashSet<OAuthScope>,
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
                request.add_scope(scope.inner().to_owned())
            });

        // Generate an authorize URL.
        let (authorize_url, csrf_state) = request.url();

        OAuthSession {
            url: OAuthAuthorizeUrl::new(authorize_url),
            client: self.clone(),
            pkce_code_verifier: pkce_code_verifier
                .secret()
                .to_owned(),
            csrf_state,
        }
    }
}
