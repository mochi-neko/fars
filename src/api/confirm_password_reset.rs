//! Implements the confirm password reset API of the Firebase Auth.
//!
//! You can apply a password reset change by issuing an HTTP POST request to the Auth resetPassword endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::result::Result;

/// Request body payload for the confirm password reset API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
#[derive(Serialize)]
pub struct ConfirmPasswordResetRequestBodyPayload {
    /// The email action code sent to the user's email for resetting the password.
    #[serde(rename = "oobCode")]
    oob_code: String,
    /// The new password.
    #[serde(rename = "newPassword")]
    new_password: String,
}

impl ConfirmPasswordResetRequestBodyPayload {
    /// Creates a new request body payload for the confirm password reset API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
    ///
    /// ## Arguments
    /// - `oob_code` - The email action code sent to the user's email for resetting the password.
    /// - `new_password` - The new password.
    pub fn new(
        oob_code: String,
        new_password: String,
    ) -> Self {
        Self {
            oob_code,
            new_password,
        }
    }
}

/// Response payload for the confirm password reset API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
#[derive(Deserialize)]
pub struct ConfirmPasswordResetResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Type of the email action code. Should be "PASSWORD_RESET".
    #[serde(rename = "requestType")]
    pub request_type: String,
}

/// Confirms the password reset with the given code.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-password-reset).
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
/// - OPERATION_NOT_ALLOWED: Password sign-in is disabled for this project.
/// - EXPIRED_OOB_CODE: The action code has expired.
/// - INVALID_OOB_CODE: The action code is invalid. This can happen if the code is malformed, expired, or has already been used.
/// - USER_DISABLED: The user account has been disabled by an administrator.
///
/// ## Example
/// ```
/// use fars::api::confirm_password_reset::{
///     confirm_password_reset,
///     ConfirmPasswordResetRequestBodyPayload,
/// };
///
/// let response_payload = ConfirmPasswordResetRequestBodyPayload::new(
///     "oob-code".to_string(),
///     "new-password".to_string(),
/// );
///
/// let response_payload = confirm_password_reset(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     response_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn confirm_password_reset(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: ConfirmPasswordResetRequestBodyPayload,
) -> Result<ConfirmPasswordResetResponsePayload> {
    client::send_post::<
        ConfirmPasswordResetRequestBodyPayload,
        ConfirmPasswordResetResponsePayload,
    >(
        client,
        "accounts:resetPassword",
        api_key,
        request_payload,
        None,
    )
    .await
}
