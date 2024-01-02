//! Implements the change password API of the Firebase Auth.
//!
//! You can change a user's password by issuing an HTTP POST request to the Auth setAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-password).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::ProviderUserInfo;
use crate::result::Result;

/// Request body payload for the change password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-password).
#[derive(Serialize)]
pub struct ChangePasswordRequestBodyPayload {
    /// A Firebase Auth ID token for the user.
    #[serde(rename = "idToken")]
    id_token: String,
    /// The user's new password.
    #[serde(rename = "password")]
    password: String,
    /// Whether or not to return an ID and refresh token.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl ChangePasswordRequestBodyPayload {
    /// Creates a new request body payload for the `setAccountInfo` endpoint.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-password).
    ///
    /// ## Arguments
    /// - `id_token` - A Firebase Auth ID token for the user.
    /// - `password` - The user's new password.
    /// - `return_secure_token` - Whether or not to return an ID and refresh token.
    pub fn new(
        id_token: String,
        password: String,
        return_secure_token: bool,
    ) -> Self {
        Self {
            id_token,
            password,
            return_secure_token,
        }
    }
}

/// Response payload for the `setAccountInfo` endpoint.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-password).
#[derive(Deserialize)]
pub struct ChangePasswordResponsePayload {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Hash version of password.
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Vec<ProviderUserInfo>,
    /// New Firebase Auth ID token for user.
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    /// A Firebase Auth refresh token.
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: Option<String>,
}

/// Changes the password associated with the user account.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-password).
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
/// - WEAK_PASSWORD: The password must be 6 characters long or more.
///
/// # Example
/// ```
/// use fars::api::change_password::{
///     change_password, ChangePasswordRequestBodyPayload,
/// };
///
/// let request_payload = ChangePasswordRequestBodyPayload::new(
///     id_token,
///     password,
///     false,
/// );
///
/// let resopnse_payload = change_password(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn change_password(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: ChangePasswordRequestBodyPayload,
) -> Result<ChangePasswordResponsePayload> {
    client::send_post::<
        ChangePasswordRequestBodyPayload,
        ChangePasswordResponsePayload,
    >(
        client,
        "accounts:update",
        api_key,
        request_payload,
        None,
    )
    .await
}
