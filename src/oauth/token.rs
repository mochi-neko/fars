use std::collections::HashMap;
use std::time::Duration;

use crate::oauth::AccessToken;
use crate::oauth::RefreshToken;
use crate::IdpPostBody;
use crate::ProviderId;

/// The OAuth 2.0 token set.
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
/// use fars::oauth::Scope;
/// use fars::oauth::AuthorizationCode;
/// use fars::oauth::State;
///
/// let client = AuthorizationCodeClient::new(
///     ClientId::new("client-id"),
///     Some(ClientSecret::new("client-secret")),
///     AuthorizeEndpoint::new("https://example.com/auth")?,
///     Some(TokenEndpoint::new("https://example.com/token")?),
///     RedirectUrl::new("https://my.app.com/callback")?,
///     PkceOption::S256,
/// )?;
///
/// let session = client.generate_session(HashSet::from([
///     Scope::new("scope1"),
///     Scope::new("scope2"),
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
///     State::new(state),
/// )?;
/// ```
pub struct OAuthToken {
    /// The access token.
    pub(crate) access_token: AccessToken,
    /// The refresh token.
    pub(crate) refresh_token: Option<RefreshToken>,
    /// The expiration time.
    pub(crate) expires_in: Option<Duration>,
}

impl OAuthToken {
    /// Returns the access token.
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    /// Returns the refresh token.
    pub fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    /// Returns the expiration time.
    pub fn expires_in(&self) -> Option<Duration> {
        self.expires_in
    }

    /// Creates a new post body with access token and provider ID to sign in.
    ///
    /// ## Arguments
    /// - `provider_id` - The provider ID.
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
    /// use fars::oauth::Scope;
    /// use fars::oauth::AuthorizationCode;
    /// use fars::oauth::State;
    /// use fars::ProviderId;
    ///
    /// let client = AuthorizationCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     Some(ClientSecret::new("client-secret")),
    ///     AuthorizeEndpoint::new("https://example.com/auth")?,
    ///     Some(TokenEndpoint::new("https://example.com/token")?),
    ///     RedirectUrl::new("https://my.app.com/callback")?,
    ///     PkceOption::S256,
    /// )?;
    ///
    /// let session = client.generate_session(HashSet::from([
    ///     Scope::new("scope1"),
    ///     Scope::new("scope2"),
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
    ///     State::new(state),
    /// )?;
    ///
    /// let idp_post_body = token.create_idp_post_body(
    ///     ProviderId::Custom("custom-provider-id".to_string()),
    /// )?;
    /// ```
    pub fn create_idp_post_body(
        self,
        provider_id: ProviderId,
    ) -> crate::Result<IdpPostBody> {
        IdpPostBody::new(
            provider_id,
            HashMap::from([(
                "access_token",
                self.access_token
                    .inner()
                    .to_owned(),
            )]),
        )
    }
}
