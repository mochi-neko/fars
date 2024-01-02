//! Implements the get user data API of the Firebase Auth.
//!
//! You can get a user's data by issuing an HTTP POST request to the Auth getAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info)

use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::UserData;
use crate::result::Result;

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
#[derive(Deserialize)]
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
/// ## Returns
/// Result with a response payload.
///
/// ## Common error codes
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
/// - USER_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
///
/// ## Example
/// ```
/// use fars::api::get_user_data::{
///     GetUserDataRequestBodyPayload,
///     get_user_data,
/// };
///
/// let request_payload = GetUserDataRequestBodyPayload::new(
///     "id-token".to_string(),
/// );
///
/// let response_payload = get_user_data(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn get_user_data(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: GetUserDataRequestBodyPayload,
) -> Result<GetUserDataResponsePayload> {
    client::send_post::<
        GetUserDataRequestBodyPayload,
        GetUserDataResponsePayload,
    >(client, "accounts:lookup", api_key, request_payload, None,)
    .await
}
