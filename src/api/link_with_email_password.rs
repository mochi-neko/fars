//! Implements the link with email password API of the Firebase Auth.
//!
//! You can link an email/password to a current user by issuing an HTTP POST request to the Auth setAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::ProviderUserInfo;
use crate::result::Result;

/// Request body payload for the link with email password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password).
#[derive(Serialize)]
pub struct LinkWithEmailAndPasswordRequestBodyPayload {
    /// The Firebase ID token of the account you are trying to link the credential to.
    #[serde(rename = "idToken")]
    id_token: String,
    /// The email to link to the account.
    #[serde(rename = "email")]
    email: String,
    /// The new password of the account.
    #[serde(rename = "password")]
    password: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl LinkWithEmailAndPasswordRequestBodyPayload {
    /// Creates a new request body payload for the link with email password API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password).
    ///
    /// ## Arguments
    /// - `id_token` - The Firebase ID token of the account you are trying to link the credential to.
    /// - `email` - The email to link to the account.
    /// - `password` - The new password of the account.
    pub fn new(
        id_token: String,
        email: String,
        password: String,
    ) -> Self {
        Self {
            id_token,
            email,
            password,
            return_secure_token: true,
        }
    }
}

/// Response payload for the link with email password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password).
#[derive(Deserialize)]
pub struct LinkWithEmailAndPasswordResponsePayload {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo url for the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// Hash version of password.
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Vec<ProviderUserInfo>,
    /// Whether or not the account's email has been verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    /// New Firebase Auth ID token for user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// A Firebase Auth refresh token.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
}

/// Links the user account with the given credentials.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password).
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
/// - CREDENTIAL_TOO_OLD_LOGIN_AGAIN: The user's credential is no longer valid. The user must sign in again.
/// - TOKEN_EXPIRED: The user's credential is no longer valid. The user must sign in again.
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
/// - WEAK_PASSWORD: The password must be 6 characters long or more.
///
/// ## Example
/// ```
/// use fars::api::link_with_email_password::{
///     LinkWithEmailAndPasswordRequestBodyPayload,
///     link_with_email_password,
/// };
///
/// let request_payload = LinkWithEmailAndPasswordRequestBodyPayload::new(
///     "id-token".to_string(),
///     "email".to_string(),
///     "password".to_string(),
/// );
///
/// let response_payload = link_with_email_password(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn link_with_email_password(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: LinkWithEmailAndPasswordRequestBodyPayload,
) -> Result<LinkWithEmailAndPasswordResponsePayload> {
    client::send_post::<
        LinkWithEmailAndPasswordRequestBodyPayload,
        LinkWithEmailAndPasswordResponsePayload,
    >(
        client,
        "accounts:update",
        api_key,
        request_payload,
        None,
    )
    .await
}
