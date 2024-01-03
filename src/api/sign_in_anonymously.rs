//! Implements the sign in anonymously API of Firebase Auth.
//!
//! You can sign in a user anonymously by issuing an HTTP POST request to the Auth signupNewUser endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::Result;

/// Request body payload for the sign in anonymously API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
#[derive(Serialize)]
pub struct SignInAnonymouslyRequestBodyPayload {
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl SignInAnonymouslyRequestBodyPayload {
    /// Creates a new request body payload for the sign in anonymously API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
    pub fn new() -> Self {
        Self {
            return_secure_token: true,
        }
    }
}

impl Default for SignInAnonymouslyRequestBodyPayload {
    fn default() -> Self {
        Self::new()
    }
}

/// Response payload for the sign in anonymously API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
#[derive(Deserialize)]
pub struct SignInAnonymouslyResponsePayload {
    /// A Firebase Auth ID token for the newly created user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// A Firebase Auth refresh token for the newly created user.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// The uid of the newly created user.
    #[serde(rename = "localId")]
    pub local_id: String,
}

/// Signs in a user anonymously.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously).
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
/// - OPERATION_NOT_ALLOWED: Anonymous user sign-in is disabled for this project.
///
/// ## Example
/// ```
/// use fars::api;
///
/// let request_payload = api::SignInAnonymouslyRequestBodyPayload::new();
///
/// let response_payload = api::sign_in_anonymously(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await?;
/// ```
pub async fn sign_in_anonymously(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: SignInAnonymouslyRequestBodyPayload,
) -> Result<SignInAnonymouslyResponsePayload> {
    client::send_post::<
        SignInAnonymouslyRequestBodyPayload,
        SignInAnonymouslyResponsePayload,
    >(
        client,
        "accounts:signUp",
        api_key,
        request_payload,
        None,
    )
    .await
}
