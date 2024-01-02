//! Implements the exchange refresh token API of the Firebase Auth.
//!
//! You can refresh a Firebase ID token by issuing an HTTP POST request to the securetoken.googleapis.com endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token)

use serde::{Deserialize, Serialize};

use crate::client;
use crate::result::Result;

/// Request body payload for the exchange refresh token API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Serialize)]
pub struct ExchangeRefreshTokenRequestBodyPayload {
    /// The refresh token's grant type, always "refresh_token".
    #[serde(rename = "grant_type")]
    grant_type: String,
    /// A Firebase Auth refresh token.
    #[serde(rename = "refresh_token")]
    refresh_token: String,
}

impl ExchangeRefreshTokenRequestBodyPayload {
    /// Creates a new request body payload for the exchange refresh token API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
    ///
    /// ## Arguments
    /// - `refresh_token` - A Firebase Auth refresh token.
    pub fn new(refresh_token: String) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            refresh_token,
        }
    }
}

/// Response payload for the exchange refresh token API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
#[derive(Deserialize)]
pub struct ExchangeRefreshTokenResponsePayload {
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expires_in")]
    pub expires_in: String,
    /// The type of the refresh token, always "Bearer".
    #[serde(rename = "token_type")]
    pub token_type: String,
    /// The Firebase Auth refresh token provided in the request or a new refresh token.
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    /// A Firebase Auth ID token.
    #[serde(rename = "id_token")]
    pub id_token: String,
    /// The uid corresponding to the provided ID token.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// Your Firebase project ID.
    #[serde(rename = "project_id")]
    pub project_id: String,
}

/// Exchanges a refresh token for an access token and an ID token.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `api_key` - Your Firebase project's API key.
/// - `request` - Request body payload.
///
/// ## Returns
/// Result with a response payload.
///
/// ## Common error codes
/// - TOKEN_EXPIRED: The user's credential is no longer valid. The user must sign in again.
/// - USER_DISABLED: The user account has been disabled by an administrator.
/// - USER_NOT_FOUND: The user corresponding to the refresh token was not found. It is likely the user was deleted.
/// - API key not valid. Please pass a valid API key. (invalid API key provided)
/// - INVALID_REFRESH_TOKEN: An invalid refresh token is provided.
/// - Invalid JSON payload received. Unknown name \"refresh_tokens\": Cannot bind query parameter. Field 'refresh_tokens' could not be found in request message.
/// - INVALID_GRANT_TYPE: the grant type specified is invalid.
/// - MISSING_REFRESH_TOKEN: no refresh token provided.
///
/// ## Example
/// ```
/// use fars::api::exchange_refresh_token::{
///     ExchangeRefreshTokenRequestBodyPayload,
///     exchange_refresh_token,
/// };
///
/// let request_payload = ExchangeRefreshTokenRequestBodyPayload::new(
///     "refresh-token".to_string(),
/// );
///
/// let response_payload = exchange_refresh_token
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn exchange_refresh_token(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: ExchangeRefreshTokenRequestBodyPayload,
) -> Result<ExchangeRefreshTokenResponsePayload> {
    client::send_post::<
        ExchangeRefreshTokenRequestBodyPayload,
        ExchangeRefreshTokenResponsePayload,
    >(
        client,
        "token",
        api_key,
        request_payload,
        None,
    )
    .await
}
