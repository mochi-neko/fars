//! Implements the unlink provider API of the Firebase Auth.
//!
//! You can unlink a provider from a current user by issuing an HTTP POST request to the Auth setAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider).

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::{ProviderId, ProviderUserInfo};
use crate::result::Result;

/// Request body payload for the unlink provider API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider).
#[derive(Serialize)]
pub struct UnlinkProviderRequestBodyPayload {
    /// The Firebase ID token of the account.
    #[serde(rename = "idToken")]
    id_token: String,
    /// The list of provider IDs to unlink, eg: 'google.com', 'password', etc.
    #[serde(rename = "deleteProvider")]
    delete_provider: Vec<String>,
}

impl UnlinkProviderRequestBodyPayload {
    /// Creates a new request body payload for the unlink provider API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider).
    ///
    /// ## Arguments
    /// - `id_token` - The Firebase ID token of the account.
    /// - `delete_provider` - The list of provider IDs to unlink, eg: 'google.com', 'password', etc.
    pub fn new(
        id_token: String,
        delete_provider: HashSet<ProviderId>,
    ) -> Self {
        Self {
            id_token,
            delete_provider: delete_provider
                .into_iter()
                .map(|provider_id| provider_id.to_string())
                .collect(),
        }
    }
}

/// Response payload for the unlink provider API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider).
#[derive(Deserialize)]
pub struct UnlinkProviderResponsePayload {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo url for the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// Hash version of the password.
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Option<Vec<ProviderUserInfo>>,
    /// Whether or not the account's email has been verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
}

/// Unlinks a provider from a user account.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider).
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
/// - INVALID_ID_TOKEN: The user's credential is no longer valid. The user must sign in again.
///
/// ## Example
/// ```
/// use fars::api::unlink_provider::{
///     UnlinkProviderRequestBodyPayload, unlink_provider,
/// };
/// use fars::data::ProviderId;
///
/// let request_payload = UnlinkProviderRequestBodyPayload::new(
///     "id-token".to_string(),
///     [ProviderId::Google].iter().cloned().collect(),
/// );
///
/// let response_payload = unlink_provider(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn unlink_provider(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: UnlinkProviderRequestBodyPayload,
) -> Result<UnlinkProviderResponsePayload> {
    client::send_post::<
        UnlinkProviderRequestBodyPayload,
        UnlinkProviderResponsePayload,
    >(
        client,
        "accounts:update",
        api_key,
        request_payload,
        None,
    )
    .await
}
