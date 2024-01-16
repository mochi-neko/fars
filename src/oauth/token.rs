use std::collections::HashMap;
use std::time::Duration;

use crate::oauth::OAuthAccessToken;
use crate::oauth::OAuthRefreshToken;
use crate::IdpPostBody;
use crate::ProviderId;

/// The OAuth 2.0 token.
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
pub struct OAuthToken {
    /// The access token.
    pub(crate) access_token: OAuthAccessToken,
    /// The refresh token.
    pub(crate) refresh_token: Option<OAuthRefreshToken>,
    /// The expiration time.
    pub(crate) expires_in: Option<Duration>,
}

impl OAuthToken {
    /// Returns the access token.
    pub fn access_token(&self) -> &OAuthAccessToken {
        &self.access_token
    }

    /// Returns the refresh token.
    pub fn refresh_token(&self) -> Option<&OAuthRefreshToken> {
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
    /// use fars::ProviderId;
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
