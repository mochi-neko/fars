use std::collections::HashSet;

use oauth2::basic::BasicClient;

use crate::oauth::ClientId;
use crate::oauth::ClientSecret;
use crate::oauth::DeviceCodeSession;
use crate::oauth::DeviceEndpoint;
use crate::oauth::OAuthError;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::TokenEndpoint;
use crate::oauth::UserCode;
use crate::oauth::VerificationUri;
use crate::oauth::VerificationUriComplete;

/// A client for the Device Code grant type of the OAuth 2.0.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Recommended use cases
/// - Browserless or input-constrained devices.
///
/// ## Not recommended use cases
/// - Browser-enabled devices, use Authorization Code grant type: [`crate::oauth::AuthorizationCodeClient`] instead.
///
/// ## Example
/// ```
/// use fars::oauth::DeviceCodeClient;
/// use fars::oauth::ClientId;
/// use fars::oauth::DeviceEndpoint;
/// use fars::oauth::TokenEndpoint;
///
/// let client = DeviceCodeClient::new(
///     ClientId::new("client-id"),
///     None,
///     DeviceEndpoint::new("https://example.com/device")?,
///     TokenEndpoint::new("https://example.com/token")?,
/// )?;
/// ```
#[derive(Clone)]
pub struct DeviceCodeClient {
    pub(crate) client: BasicClient,
}

impl DeviceCodeClient {
    /// Creates a new client for the Device Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID.
    /// - `client_secret` - Client secret.
    /// - `authorize_endpoint` - Authorization API URL. (This is not used in the Device Code flow.)
    /// - `device_endpoint` - Device API URL.
    /// - `token_endpoint` - Token API URL.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::DeviceCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::DeviceEndpoint;
    /// use fars::oauth::TokenEndpoint;
    ///
    /// let client = DeviceCodeClient::new(
    ///     ClientId::new("client-id"),
    ///     None,
    ///     DeviceEndpoint::new("https://example.com/device")?,
    ///     TokenEndpoint::new("https://example.com/token")?,
    /// )?;
    /// ```
    pub fn new(
        client_id: ClientId,
        client_secret: Option<ClientSecret>,
        device_endpoint: DeviceEndpoint,
        token_endpoint: TokenEndpoint,
    ) -> OAuthResult<Self> {
        let client_secret = client_secret.map(|client_secret| {
            client_secret
                .inner()
                .to_owned()
        });

        // Create an internal OAuth client with settings.
        let client = BasicClient::new(
            client_id.inner().to_owned(),
            client_secret,
            // NOTE: This is not used in the Device Code flow but required to the `oauth2` crate implementation.
            oauth2::AuthUrl::new("https://dummy.com/auth".to_string()).unwrap(),
            Some(
                token_endpoint
                    .inner()
                    .to_owned(),
            ),
        )
        .set_device_authorization_url(
            device_endpoint
                .inner()
                .to_owned(),
        )
        .set_auth_type(oauth2::AuthType::RequestBody);

        Ok(Self {
            client,
        })
    }

    /// Requests authorization and generate a Device Code flow session with verification URI and user code.
    ///
    /// ## Arguments
    /// - `scopes` - Scopes to request authorization.
    ///
    /// ## Example
    /// ```
    /// use std::collections::HashSet;
    /// use fars::oauth::DeviceCodeClient;
    /// use fars::oauth::ClientId;
    /// use fars::oauth::DeviceEndpoint;
    /// use fars::oauth::TokenEndpoint;
    /// use fars::oauth::OAuthScope;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = DeviceCodeClient::new(
    ///         ClientId::new("client-id"),
    ///         None,
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
    /// }
    /// ```
    pub async fn request_authorization(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthResult<DeviceCodeSession> {
        // Generate an authorization request.
        let request = self
            .client
            .exchange_device_code()
            .map_err(OAuthError::DeviceAuthorizationRequestError)?;

        // Set scopes.
        let request = scopes
            .iter()
            .fold(request, |request, scope| {
                request.add_scope(scope.inner().to_owned())
            });

        // Request authorization to device endpoint.
        let response = request
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(OAuthError::DeviceCodeExchangeFailed)?;

        Ok(DeviceCodeSession {
            verification_uri: VerificationUri {
                inner: response
                    .verification_uri()
                    .clone(),
            },
            verification_uri_complete: response
                .verification_uri_complete()
                .map(|complete| VerificationUriComplete {
                    inner: complete.clone(),
                }),
            user_code: UserCode {
                inner: response.user_code().clone(),
            },
            response,
            client: self.clone(),
        })
    }
}
