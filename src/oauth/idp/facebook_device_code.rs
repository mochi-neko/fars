//! A client for the Facebook's Device Code grant type of the OAuth 2.0.
//! Because Facebook's Device Code API is not along with the OAuth 2.0 standard,
//! this client is implemented as a custom client, not using the `oauth2` crate.

use std::collections::HashSet;
use std::future::Future;
use std::time::{Duration, Instant};

use crate::oauth::AccessToken;
use crate::oauth::OAuthError;
use crate::oauth::OAuthResult;
use crate::oauth::OAuthScope;
use crate::oauth::OAuthToken;

/// A client for the Facebook's Device Code grant type of the OAuth 2.0.
///
/// See also [the official guide](https://developers.facebook.com/docs/facebook-login/for-devices/).
///
/// ## NOTE
/// This is only available when the feature `oauth` is enabled.
///
/// ## Recommended use cases
/// - Browserless or input-constrained devices.
///
/// ## Not recommended use cases
/// - Browser-enabled devices, use Authorization Code grant type: [`crate::oauth::FacebookAuthorizationCodeClient`] instead.
///
/// ## Example
/// ```
/// use fars::oauth::FacebookDeviceCodeClient;
/// use fars::oauth::OAuthScope;
/// use std::collections::HashSet;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = FacebookDeviceCodeClient::new(
///          "app-id".to_string(),
///          "client-token".to_string(),
///     )?;
///
///     let session = client.request_authorization(HashSet::from([
///        OAuthScope::open_id_email(),
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
#[derive(Clone)]
pub struct FacebookDeviceCodeClient {
    client: reqwest::Client,
    access_token: String, // NOTE: Not the access token of the OAuth 2.0.
}

impl FacebookDeviceCodeClient {
    /// Creates a new client for the Facebook's Device Code grant type of the OAuth 2.0.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of the Facebook.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::FacebookDeviceCodeClient;
    ///
    /// let client = FacebookDeviceCodeClient::new(
    ///     "app-id".to_string(),
    ///     "client-token".to_string(),
    /// )?;
    /// ```
    pub fn new(
        app_id: String,
        client_token: String,
    ) -> OAuthResult<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            access_token: format!("{}|{}", app_id, client_token),
        })
    }

    /// Requests authorization and generates a new session of the Facebook's Device Code grant type of the OAuth 2.0.
    ///
    /// See also [the official guide](https://developers.facebook.com/docs/facebook-login/for-devices#tech-step1).
    ///
    /// ## Arguments
    /// - `scopes` - Scopes to request authorization defined at [here](https://developers.facebook.com/docs/permissions).
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::FacebookDeviceCodeClient;
    /// use fars::oauth::OAuthScope;
    /// use std::collections::HashSet;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = FacebookDeviceCodeClient::new(
    ///         "app-id".to_string(),
    ///         "client-token".to_string(),
    ///     )?;
    ///
    ///     let session = client.request_authorization(HashSet::from([
    ///         OAuthScope::open_id_email(),
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
    ) -> OAuthResult<FacebookDeviceCodeSession> {
        let endpoint = "https://graph.facebook.com/v2.6/device/login";
        let scopes = scopes
            .iter()
            .map(|scope| {
                scope
                    .inner()
                    .to_owned()
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(",");
        let url = format!(
            "{}?access_token={}&scope={}",
            endpoint, &self.access_token, scopes
        );

        let response = self
            .client
            .post(url)
            .send()
            .await
            .map_err(OAuthError::ReqwestError)?;

        let status = response.status();

        let response_text = response
            .text()
            .await
            .map_err(OAuthError::ReqwestError)?;

        if status.is_success() {
            let response = serde_json::from_str::<FacebookDeviceCodeResponse>(
                &response_text,
            )
            .map_err(|error| {
                OAuthError::JsonDeserializationFailed(
                    error,
                    response_text.clone(),
                )
            })?;

            Ok(FacebookDeviceCodeSession {
                client: self.clone(),
                response,
            })
        } else {
            Err(OAuthError::ManualApiCallFailed(
                status,
                response_text,
            ))
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FacebookDeviceCodeResponse {
    code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

/// A session of the Facebook's Device Code grant type of the OAuth 2.0.
pub struct FacebookDeviceCodeSession {
    client: FacebookDeviceCodeClient,
    response: FacebookDeviceCodeResponse,
}

impl FacebookDeviceCodeSession {
    /// Verification URI of the Device Code grant type to display to the user.
    pub fn verification_uri(&self) -> &str {
        &self.response.verification_uri
    }

    /// User code of the Device Code grant type to display to the user.
    pub fn user_code(&self) -> &str {
        &self.response.user_code
    }

    /// Polls the token endpoint and exchanges the device code into an access token.
    ///
    /// See also [the official guide](https://developers.facebook.com/docs/facebook-login/for-devices#tech-step3).
    ///
    /// ## Arguments
    /// - `interval_fn` - A function to sleep for the interval time, e.g. `tokio::time::sleep`.
    ///
    /// ## Example
    /// ```
    /// use fars::oauth::FacebookDeviceCodeClient;
    /// use fars::oauth::OAuthScope;
    /// use std::collections::HashSet;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = FacebookDeviceCodeClient::new(
    ///          "app-id".to_string(),
    ///          "client-token".to_string(),
    ///     )?;
    ///
    ///     let session = client.request_authorization(HashSet::from([
    ///        OAuthScope::open_id_email(),
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
    /// }
    /// ```
    pub async fn poll_exchange_token<I, IF>(
        self,
        interval_fn: I,
        timeout: Option<Duration>,
    ) -> OAuthResult<OAuthToken>
    where
        I: Fn(Duration) -> IF,
        IF: Future<Output = ()>,
    {
        let timeout = timeout.unwrap_or(Duration::from_secs(
            self.response.expires_in,
        ));
        let interval = Duration::from_secs(self.response.interval);

        let timer = Instant::now();

        while timer.elapsed() < timeout {
            match self.exchange_token().await {
                // Success
                | Ok(token) => return Ok(token),
                // Continue polling
                | Err(OAuthError::ContinuePolling) => {
                    interval_fn(interval).await;
                },
                // Error
                | Err(error) => return Err(error),
            }
        }

        Err(OAuthError::Timeout)
    }

    async fn exchange_token(&self) -> OAuthResult<OAuthToken> {
        let endpoint = "https://graph.facebook.com/v2.6/device/login_status";
        let url = format!(
            "{}?access_token={}&code={}",
            endpoint, &self.client.access_token, self.response.code
        );

        let response = self
            .client
            .client
            .post(url)
            .send()
            .await
            .map_err(OAuthError::ReqwestError)?;

        let status = response.status();

        let response_text = response
            .text()
            .await
            .map_err(OAuthError::ReqwestError)?;

        if status.is_success() {
            match serde_json::from_str::<FacebookTokenResponse>(&response_text)
                .map_err(|error| {
                    OAuthError::JsonDeserializationFailed(
                        error,
                        response_text.clone(),
                    )
                }) {
                | Ok(response) => {
                    return Ok(OAuthToken {
                        access_token: AccessToken::new(response.access_token),
                        refresh_token: None,
                        expires_in: Some(Duration::from_secs(
                            response.expires_in,
                        )),
                    })
                },
                | Err(_) => {
                    let error_response = serde_json::from_str::<
                        FacebookTokenErrorResponse,
                    >(&response_text)
                    .map_err(|error| {
                        OAuthError::JsonDeserializationFailed(
                            error,
                            response_text.clone(),
                        )
                    })?;

                    return match error_response
                        .error
                        .error_subcode
                    {
                        // Continue polling.
                        | 1349174 | 1349172 => Err(OAuthError::ContinuePolling),
                        // Other errors.
                        | _ => Err(OAuthError::ManualApiCallFailed(
                            status,
                            response_text,
                        )),
                    };
                },
            }
        } else {
            Err(OAuthError::ManualApiCallFailed(
                status,
                response_text,
            ))
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FacebookTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FacebookTokenErrorResponse {
    error: FacebookTokenError,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FacebookTokenError {
    message: String,
    code: u64,
    error_subcode: u64,
    error_user_title: String,
    error_user_msg: String,
}
