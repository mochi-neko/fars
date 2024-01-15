use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier, TokenResponse};

use crate::{OAuthClient, OAuthError, OAuthResult, OAuthToken};

/// The OAuth2 session for authorization.
pub struct OAuthSession {
    /// The authorization URL.
    pub url: String,
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
