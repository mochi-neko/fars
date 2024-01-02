//! Implements the delete account API of the Firebase Auth.
//!
//! You can delete a current user by issuing an HTTP POST request to the Auth deleteAccount endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account)

use serde::{Deserialize, Serialize};

use crate::client;
use crate::result::Result;

/// Request body payload for the delete account API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
#[derive(Serialize)]
pub struct DeleteAccountRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
}

impl DeleteAccountRequestBodyPayload {
    /// Creates a new request body payload for the delete account API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
    ///
    /// ## Arguments
    /// - `id_token` - The Firebase ID token of the account.
    pub fn new(id_token: String) -> Self {
        Self {
            id_token,
        }
    }
}

/// Response payload for the delete account API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
#[derive(Deserialize)]
pub struct DeleteAccountResponsePayload {}

/// Deletes the account of the user specified by the given ID token.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
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
/// use fars::api::delete_account::{
///     DeleteAccountRequestBodyPayload,
///     delete_account,
/// };
///
/// let request_payload = DeleteAccountRequestBodyPayload::new(
///     "id-token".to_string(),
/// );
///
/// let response_payload = delete_account(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn delete_account(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: DeleteAccountRequestBodyPayload,
) -> Result<DeleteAccountResponsePayload> {
    client::send_post::<
        DeleteAccountRequestBodyPayload,
        DeleteAccountResponsePayload,
    >(
        client,
        "accounts:delete",
        api_key,
        request_payload,
        None,
    )
    .await
}
