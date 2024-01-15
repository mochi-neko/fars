use oauth2::{CsrfToken, PkceCodeVerifier, TokenResponse};

use crate::OAuthAccessToken;
use crate::OAuthAuthorizationCode;
use crate::OAuthAuthorizationState;
use crate::OAuthAuthorizeUrl;
use crate::OAuthClient;
use crate::OAuthError;
use crate::OAuthRefreshToken;
use crate::OAuthResult;
use crate::OAuthToken;

/// The OAuth2 session for authorization.
pub struct OAuthSession {
    /// The authorization URL.
    pub url: OAuthAuthorizeUrl,
    /// The OAuth client.
    pub(crate) client: OAuthClient,
    /// The PKCE code verifier.
    pub(crate) pkce_code_verifier: String,
    /// The CSRF state.
    pub(crate) csrf_state: CsrfToken,
}

impl OAuthSession {
    /// Exchanges an authorization code into an access token.
    pub async fn exchange_code_into_token(
        &self,
        code: OAuthAuthorizationCode,
        state: OAuthAuthorizationState,
    ) -> OAuthResult<OAuthToken> {
        // Check the CSRF state.
        if state
            .inner()
            .ne(self.csrf_state.secret())
        {
            return Err(OAuthError::StateMismatch);
        }

        // Exchange the authorization code into an access token.
        let token_response = self
            .client
            .inner
            .exchange_code(code.inner().to_owned())
            .set_pkce_verifier(PkceCodeVerifier::new(
                self.pkce_code_verifier
                    .clone(),
            ))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(OAuthError::ExchangeTokenFailed)?;

        Ok(OAuthToken {
            access_token: OAuthAccessToken::new(
                token_response
                    .access_token()
                    .secret(),
            ),
            refresh_token: token_response
                .refresh_token()
                .map(|token| OAuthRefreshToken::new(token.secret())),
            expires_in: token_response.expires_in(),
        })
    }
}
