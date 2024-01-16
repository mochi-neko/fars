use oauth2::{CsrfToken, PkceCodeVerifier, TokenResponse};

use crate::oauth::OAuthAccessToken;
use crate::oauth::OAuthAuthorizationCode;
use crate::oauth::OAuthAuthorizationState;
use crate::oauth::OAuthAuthorizeUrl;
use crate::oauth::OAuthClient;
use crate::oauth::OAuthError;
use crate::oauth::OAuthRefreshToken;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthToken;

/// The OAuth 2.0 session with authorize URL.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
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
/// use fars::oauth::OAuthScope;
///
/// let client = OAuthClient::new(
///     OAuthClientId::new("client-id"),
///     Some(OAuthClientSecret::new("client-secret")),
///     OAuthAuthUrl::new("https://example.com/auth").unwrap(),
///     Some(OAuthTokenUrl::new("https://example.com/token").unwrap()),
///     OAuthRedirectUrl::new("https://my.app.com/callback").unwrap(),
///     Some(OAuthRevocationUrl::new("https://example.com/revoke").unwrap()),
/// )?;
///
/// let session = client.generate_authorization_session(HashSet::from([
///     OAuthScope::new("scope1"),
///     OAuthScope::new("scope2"),
/// ]));
/// ```
pub struct OAuthSession {
    /// The authorize URL.
    pub authorize_url: OAuthAuthorizeUrl,
    /// The OAuth client.
    pub(crate) client: OAuthClient,
    /// The PKCE code verifier.
    pub(crate) pkce_code_verifier: Option<String>,
    /// The CSRF state.
    pub(crate) csrf_state: CsrfToken,
}

impl OAuthSession {
    /// Exchanges an authorization code into an access token.
    ///
    /// ## Arguments
    /// - `code` - The authorization code returned from authorization server.
    /// - `state` - The state of the authorization session.
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
    /// use fars::oauth::OAuthAuthorizationCode;
    /// use fars::oauth::OAuthAuthorizationState;
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
    ///
    /// let session = client.generate_authorization_session(HashSet::from([
    ///     OAuthScope::new("scope1"),
    ///     OAuthScope::new("scope2"),
    /// ]));
    ///
    /// let authorize_url = session.authorize_url.inner().clone();
    ///
    /// // Redirect the user to the authorize URL and get the code and state.
    /// let code = "code";
    /// let state = "state";
    ///
    /// let token = session.exchange_code_into_token(
    ///     OAuthAuthorizationCode::new(code),
    ///     OAuthAuthorizationState::new(state),
    /// )?;
    /// ```
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

        // Create a request
        let mut request = self
            .client
            .inner
            .exchange_code(code.inner().to_owned());

        // Set the PKCE code verifier if it exists.
        if let Some(verifier) = &self.pkce_code_verifier {
            request = request.set_pkce_verifier(PkceCodeVerifier::new(
                verifier.to_owned(),
            ));
        }

        // Exchange the authorization code into an access token.
        let token_response = request
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
