//! Implements the exchange custom token for an ID and refresh token API of Firebase Auth.
//!
//! You can exchange a custom Auth token for an ID and refresh token by issuing an HTTP POST request to the Auth verifyCustomToken endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).

use serde::{Deserialize, Serialize};

use crate::ApiKey;
use crate::Client;
use crate::Endpoint;
use crate::Result;

/// Request body payload for the exchange custom token for an ID and refresh token API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
#[derive(Serialize)]
pub struct ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload {
    /// A Firebase Auth custom token from which to create an ID and refresh token pair.
    #[serde(rename = "token")]
    token: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload {
    /// Creates a new request body payload for the exchange custom token for an ID and refresh token API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
    ///
    /// ## Arguments
    /// - `token` - A Firebase Auth custom token from which to create an ID and refresh token pair.
    pub fn new(token: String) -> Self {
        Self {
            token,
            return_secure_token: true,
        }
    }
}

/// Response payload for the exchange custom token for an ID and refresh token API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
#[derive(Deserialize, Debug)]
pub struct ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload {
    /// A Firebase Auth ID token generated from the provided custom token.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// A Firebase Auth refresh token generated from the provided custom token.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
}

/// Exchanges a custom token for an ID and refresh token.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `api_key` - Your Firebase project's API key.
/// - `request_payload` - Request body payload.
///
/// ## Errors
/// - `Error::HttpRequestError` - Failed to send a request.
/// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
/// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
/// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
/// - `Error::ApiError` - API error on the Firebase Auth.
///
/// ## Common error codes
/// - INVALID_CUSTOM_TOKEN: The custom token format is incorrect or the token is invalid for some reason (e.g. expired, invalid signature etc.)
/// - CREDENTIAL_MISMATCH: The custom token corresponds to a different Firebase project.
///
/// ## Example
/// ```
/// use fars::api;
/// use fars::Client;
/// use fars::ApiKey;
///
/// let request_payload = api::ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload::new(
///    "your-custom-token".to_string(),
/// );
///
/// let response_payload = api::exchange_custom_token_for_an_id_and_refresh_token(
///     Client::new(),
///     ApiKey::new("your-firebase-project-api-key"),
///     request_payload,
/// ).await?;
/// ```
pub async fn exchange_custom_token_for_an_id_and_refresh_token(
    client: &Client,
    api_key: &ApiKey,
    request_payload: ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload,
) -> Result<ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload> {
    client.send_post::<
        ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload,
        ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload,
    >(
        Endpoint::SignInWithCustomToken,
        api_key,
        request_payload,
        None,
    )
    .await
}
