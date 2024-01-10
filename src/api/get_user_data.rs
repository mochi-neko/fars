//! Implements the get user data API of the Firebase Auth.
//!
//! You can get a user's data by issuing an HTTP POST request to the Auth getAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info)

use serde::{Deserialize, Serialize};

use crate::client;
use crate::ApiKey;
use crate::Result;
use crate::UserData;

/// Request body payload for the get user data API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Serialize)]
pub struct GetUserDataRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl GetUserDataRequestBodyPayload {
    /// Creates a new request body payload for the get user data API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
    ///
    /// ## Arguments
    /// - `id_token` - The Firebase ID token of the account.
    pub fn new(id_token: String) -> Self {
        Self {
            id_token,
        }
    }
}

/// Response payload for the get user data API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
#[derive(Deserialize, Debug)]
pub struct GetUserDataResponsePayload {
    /// The account associated with the given Firebase ID token.
    #[serde(rename = "users")]
    pub users: Vec<UserData>,
}

/// Gets the user data.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info).
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
/// - `Error::InvalidIdToken` - Invalid ID token.
/// - `Error::ApiError` - API error on the Firebase Auth.
///
/// ## Common error codes
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
/// - USER_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
///
/// ## Example
/// ```
/// use fars::api;
///
/// let request_payload = api::GetUserDataRequestBodyPayload::new(
///     "id-token".to_string(),
/// );
///
/// let response_payload = api::get_user_data(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await?;
/// ```
pub async fn get_user_data(
    client: &reqwest::Client,
    api_key: &ApiKey,
    request_payload: GetUserDataRequestBodyPayload,
) -> Result<GetUserDataResponsePayload> {
    client::send_post::<
        GetUserDataRequestBodyPayload,
        GetUserDataResponsePayload,
    >(client, client::Endpoint::Lookup, api_key, request_payload, None,)
    .await
}
