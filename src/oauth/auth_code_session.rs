use oauth2::{CsrfToken, PkceCodeVerifier, TokenResponse};

use crate::oauth::AccessToken;
use crate::oauth::AuthorizationCode;
use crate::oauth::AuthorizationCodeClient;
use crate::oauth::AuthorizeUrl;
use crate::oauth::CsrfState;
use crate::oauth::OAuthError;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthToken;
use crate::oauth::RefreshToken;

/// A session published by ['crate::oauth::AuthorizationCodeClient'].
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
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
/// ```
pub struct AuthorizationCodeSession {
    /// The authorize URL.
    pub authorize_url: AuthorizeUrl,
    /// The OAuth client.
    pub(crate) client: AuthorizationCodeClient,
    /// The PKCE code verifier.
    pub(crate) pkce_code_verifier: Option<String>,
    /// The CSRF state.
    pub(crate) csrf_state: CsrfToken,
}

impl AuthorizationCodeSession {
    /// Exchanges an authorization code into an access token.
    ///
    /// ## Arguments
    /// - `code` - The authorization code returned from authorization server.
    /// - `state` - The state of the authorization session.
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
    /// use fars::oauth::AuthorizationCode;
    /// use fars::oauth::CsrfState;
    ///
    /// let client = AuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     Some(ClientSecret::new("client-secret")),
    ///     AuthorizeEndpoint::new("https://example.com/auth")?,
    ///     TokenEndpoint::new("https://example.com/token")?,
    ///     RedirectUrl::new("https://my.app.com/callback")?,
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
    /// // Redirect the user to the authorize URL and get the code and state.
    /// let code = "code";
    /// let state = "state";
    ///
    /// let token = session.exchange_code_into_token(
    ///     AuthorizationCode::new(code),
    ///     CsrfState::new(state),
    /// )?;
    /// ```
    pub async fn exchange_code_into_token(
        self,
        code: AuthorizationCode,
        state: CsrfState,
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
            .client
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
            .map_err(OAuthError::AuthCodeExchangeTokenFailed)?;

        Ok(OAuthToken {
            access_token: AccessToken::new(
                token_response
                    .access_token()
                    .secret(),
            ),
            refresh_token: token_response
                .refresh_token()
                .map(|token| RefreshToken::new(token.secret())),
            expires_in: token_response.expires_in(),
        })
    }
}
