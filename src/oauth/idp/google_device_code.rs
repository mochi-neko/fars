use std::collections::HashSet;

use crate::oauth::ClientId;
use crate::oauth::ClientSecret;
use crate::oauth::DeviceCodeClient;
use crate::oauth::DeviceCodeSession;
use crate::oauth::DeviceEndpoint;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::TokenEndpoint;

/// A client for the Google's Device Code grant type with PKCE and Client Secret of the OAuth 2.0.
///
/// See also [the official guide](https://developers.google.com/identity/protocols/oauth2/limited-input-device).
///
/// ## NOTE
/// This is only available when the feature `oauth` is enabled.
///
/// ## Recommended use cases
/// - Limited-Input Device Clients **with Client Secret**.
///
/// ## Not recommended use cases
/// - Not Limited-Input Device Clients, use Authorization Code Grant type: [`crate::oauth::GoogleAuthorizationCodeClient`] instead.
///
/// ## Not supported use cases
/// - Any clients **without Client Secret**.
///
/// ## Example
/// ```
/// use fars::oauth::GoogleDeviceCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::ClientSecret;
/// use fars::oauth::OAuthScope;
/// use std::collections::HashSet;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = GoogleDeviceCodeClient::new(
///         ClientId::new("client-id"),
///         ClientSecret::new("client-secret"),
///     )?;
///
///     let session = client.request_authorization(HashSet::from([
///        OAuthScope::open_id(),
///        OAuthScope::open_id_email(),
///        OAuthScope::open_id_profile()
///     ]))
///     .await?;
///
///     let verification_uri = session.verification_uri();
///     let user_code = session.user_code();
///
///     // Display the verification URI and user code to the user,
///     // then authorize on another device.
///
///     let token = session.poll_exchange_token(
///         tokio::time::sleep,
///         None,
///     ).await?;
///
///     let access_token = token.access_token().inner();
/// }
/// ```
pub struct GoogleDeviceCodeClient {
    inner: DeviceCodeClient,
}

impl GoogleDeviceCodeClient {
    /// Creates a new client for the Google's Device Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the Google Cloud Platform.
    /// - `client_secret` - Client secret of the Google Cloud Platform.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GoogleDeviceCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    ///
    /// let client = GoogleDeviceCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     ClientSecret::new("client-secret"),
    /// )?;
    /// ```
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
    ) -> OAuthResult<Self> {
        let client = DeviceCodeClient::new(
            client_id,
            Some(client_secret),
            DeviceEndpoint::new("https://oauth2.googleapis.com/device/code")?,
            TokenEndpoint::new("https://www.googleapis.com/oauth2/v4/token")?,
        )?;

        Ok(Self {
            inner: client,
        })
    }

    /// Requests authorization and generates a new session of the Google's Device Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `scopes` - Scopes to request authorization defined at [here](https://developers.google.com/identity/protocols/oauth2/scopes).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::GoogleDeviceCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::ClientSecret;
    /// use fars::oauth::OAuthScope;
    /// use std::collections::HashSet;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = GoogleDeviceCodeClient::new(
    ///         ClientId::new("client-id"),
    ///         ClientSecret::new("client-secret"),
    ///     )?;
    ///
    ///     let session = client.request_authorization(HashSet::from([
    ///        OAuthScope::open_id(),
    ///        OAuthScope::open_id_email(),
    ///        OAuthScope::open_id_profile()
    ///     ]))
    ///     .await?;
    ///
    ///     let verification_uri = session.verification_uri();
    ///     let user_code = session.user_code();
    ///
    ///     // Display the verification URI and user code to the user.
    /// }
    /// ```
    pub async fn request_authorization(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthResult<DeviceCodeSession> {
        self.inner
            .request_authorization(scopes)
            .await
    }
}
