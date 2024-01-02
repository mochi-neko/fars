//! Implements the change email API of the Firebase Auth.
//!
//! You can change a user's email by issuing an HTTP POST request to the Auth setAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-email).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::ProviderUserInfo;
use crate::result::Result;

/// Request body payload for the change email API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-email).
#[derive(Serialize)]
pub struct ChangeEmailRequestBodyPayload {
    /// A Firebase Auth ID token for the user.
    #[serde(rename = "idToken")]
    id_token: String,
    /// The user's new email.
    #[serde(rename = "email")]
    email: String,
    /// Whether or not to return an ID and refresh token.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl ChangeEmailRequestBodyPayload {
    /// Creates a new request body payload for the change email API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-email).
    ///
    /// ## Arguments
    /// - `id_token` - A Firebase Auth ID token for the user.
    /// - `email` - The user's new email.
    /// - `return_secure_token` - Whether or not to return an ID and refresh token.
    pub fn new(
        id_token: String,
        email: String,
        return_secure_token: bool,
    ) -> Self {
        Self {
            id_token,
            email,
            return_secure_token,
        }
    }
}

/// Response payload for the the change email API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-email).
#[derive(Deserialize)]
pub struct ChangeEmailResponsePayload {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Hash version of the password.
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

/// Changes the email address associated with the user account.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-change-email).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `api_key` - Your Firebase project's API key.
/// - `request_payload` - Request body payload.
/// - `locale` - (Optional) The BCP 47 language code, eg: en-US.
///
/// ## Returns
/// Result with a response payload.
///
/// ## Common error codes
/// - EMAIL_EXISTS: The email address is already in use by another account.
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
///
/// ## Example
/// ```
/// use fars::api::change_email::{
///     change_email, ChangeEmailRequestBodyPayload,
/// };
///
/// let request_payload = ChangeEmailRequestBodyPayload::new(
///     id_token,
///     email,
///     true,
/// );
///
/// let resopnse_payload = change_email(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
///     None,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn change_email(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: ChangeEmailRequestBodyPayload,
    locale: Option<String>,
) -> Result<ChangeEmailResponsePayload> {
    let optional_headers = client::optional_locale_header(locale)?;

    client::send_post::<
        ChangeEmailRequestBodyPayload,
        ChangeEmailResponsePayload,
    >(
        client,
        "accounts:update",
        api_key,
        request_payload,
        optional_headers,
    )
    .await
}
