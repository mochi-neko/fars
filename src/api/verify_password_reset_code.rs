//! Implements the verify password reset code API of the Firebase Auth.
//!
//! You can verify a password reset code by issuing an HTTP POST request to the Auth resetPassword endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code)
use serde::{Deserialize, Serialize};

use crate::client;
use crate::result::Result;

/// Request body payload for the verify password reset code API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
#[derive(Serialize)]
pub struct VerifyPasswordResetCodeRequestBodyPayload {
    /// The email action code sent to the user's email for resetting the password.
    #[serde(rename = "oobCode")]
    oob_code: String,
}

impl VerifyPasswordResetCodeRequestBodyPayload {
    /// Creates a new request body payload for the verify password reset code API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
    ///
    /// ## Arguments
    /// - `oob_code` - The email action code sent to the user's email for resetting the password.
    pub fn new(oob_code: String) -> Self {
        Self {
            oob_code,
        }
    }
}

/// Response payload for the verify password reset code API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
#[derive(Deserialize)]
pub struct VerifyPasswordResetCodeResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Type of the email action code. Should be "PASSWORD_RESET".
    #[serde(rename = "requestType")]
    pub request_type: String,
}

/// Verifies the password reset code sent to the user's email for resetting the password.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code).
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
///
/// ## Example
/// ```
/// use fars::api::verify_password_reset_code::{
///     VerifyPasswordResetCodeRequestBodyPayload,
///     verify_password_reset_code,
/// };
///
/// let request_payload = VerifyPasswordResetCodeRequestBodyPayload::new(
///     "oob-code".to_string(),
/// );
///
/// let response_payload = verify_password_reset_code(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn verify_password_reset_code(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: VerifyPasswordResetCodeRequestBodyPayload,
) -> Result<VerifyPasswordResetCodeResponsePayload> {
    client::send_post::<
        VerifyPasswordResetCodeRequestBodyPayload,
        VerifyPasswordResetCodeResponsePayload,
    >(
        client,
        "accounts:resetPassword",
        api_key,
        request_payload,
        None,
    )
    .await
}
