use oauth2::{StandardDeviceAuthorizationResponse, TokenResponse};
use std::time::Duration;

use crate::oauth::AccessToken;
use crate::oauth::DeviceCodeClient;
use crate::oauth::OAuthError;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthToken;
use crate::oauth::RefreshToken;
use crate::oauth::UserCode;
use crate::oauth::VerificationUri;
use crate::oauth::VerificationUriComplete;

/// A session published by ['crate::oauth::DeviceCodeClient'].
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
/// ```
/// use std::collections::HashSet;
/// use fars::oauth::DeviceCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::ClientSecret;
/// use fars::oauth::DeviceEndpoint;
/// use fars::oauth::TokenEndpoint;
/// use fars::oauth::OAuthScope;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = DeviceCodeClient::new(
///         ClientId::new("client-id"),
///         Some(ClientSecret::new("client-secret")),
///         DeviceEndpoint::new("https://example.com/device")?,
///         TokenEndpoint::new("https://example.com/token")?,
///     )?;
///
///     let session = client.request_authorization(HashSet::from([
///         OAuthScope::new("scope1"),
///         OAuthScope::new("scope2"),
///     ]))
///     .await?;
/// }
/// ```
pub struct DeviceCodeSession {
    /// The verification URI.
    pub verification_uri: VerificationUri,
    /// The verification URI complete.
    pub verification_uri_complete: Option<VerificationUriComplete>,
    /// The user code.
    pub user_code: UserCode,
    /// The authorize response.
    pub(crate) response: StandardDeviceAuthorizationResponse,
    /// The OAuth client.
    pub(crate) client: DeviceCodeClient,
}

impl DeviceCodeSession {
    /// Polls to token endpoint to exchange a device code into an access token.
    ///
    /// ## Arguments
    /// - `sleep_fn` - The function to sleep.
    /// - `timeout` - The timeout duration.
    ///
    /// ## Example
    /// ```
    /// use std::collections::HashSet;
    /// use fars::oauth::DeviceCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::DeviceEndpoint;
    /// use fars::oauth::TokenEndpoint;
    /// use fars::oauth::OAuthScope;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = DeviceCodeClient::new(
    ///         ClientId::new("client-id"),
    ///         Some(ClientSecret::new("client-secret")),
    ///         DeviceEndpoint::new("https://example.com/device")?,
    ///         TokenEndpoint::new("https://example.com/token")?,
    ///     )?;
    ///
    ///     let session = client.request_authorization(HashSet::from([
    ///         OAuthScope::new("scope1"),
    ///         OAuthScope::new("scope2"),
    ///     ]))
    ///     .await?;
    ///
    ///     let verification_uri = session.verification_uri();
    ///     let user_code = session.user_code();
    ///
    ///     // Display the verification URI and user code to the user.
    ///
    ///     let token = session.poll_exchange_token(
    ///         tokio::time::sleep,
    ///         None,
    ///     ).await?;
    /// }
    /// ```
    pub async fn poll_exchange_token<S, SF>(
        self,
        sleep_fn: S,
        timeout: Option<Duration>,
    ) -> OAuthResult<OAuthToken>
    where
        S: Fn(Duration) -> SF,
        SF: std::future::Future<Output = ()>,
    {
        // Create a request
        let request = self
            .client
            .client
            .exchange_device_access_token(&self.response);

        // Exchange the authorization code into an access token.
        let token_response = request
            .request_async(
                oauth2::reqwest::async_http_client,
                sleep_fn,
                timeout,
            )
            .await
            .map_err(OAuthError::DeviceExchangeTokenFailed)?;

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
