//! Implements the verify password reset code API of the Firebase Auth.
//!
//! You can verify a password reset code by issuing an HTTP POST request to the Auth resetPassword endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code)

use serde::{Deserialize, Serialize};

use crate::client::Endpoint;
use crate::ApiKey;
use crate::Client;
use crate::Result;

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
#[derive(Deserialize, Debug)]
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
/// ## Errors
/// - `Error::HttpRequestError` - Failed to send a request.
/// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
/// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
/// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
/// - `Error::ApiError` - API error on the Firebase Auth.
///
/// ## Common error codes
/// - OPERATION_NOT_ALLOWED: Password sign-in is disabled for this project.
/// - EXPIRED_OOB_CODE: The action code has expired.
/// - INVALID_OOB_CODE: The action code is invalid. This can happen if the code is malformed, expired, or has already been used.
///
/// ## Example
/// ```
/// use fars::api;
/// use fars::Client;
/// use fars::ApiKey;
///
/// let request_payload = api::VerifyPasswordResetCodeRequestBodyPayload::new(
///     "oob-code".to_string(),
/// );
///
/// let response_payload = api::verify_password_reset_code(
///     Client::new(),
///     ApiKey::new("your-firebase-project-api-key"),
///     request_payload,
/// ).await?;
/// ```
pub async fn verify_password_reset_code(
    client: &Client,
    api_key: &ApiKey,
    request_payload: VerifyPasswordResetCodeRequestBodyPayload,
) -> Result<VerifyPasswordResetCodeResponsePayload> {
    client.send_post::<
        VerifyPasswordResetCodeRequestBodyPayload,
        VerifyPasswordResetCodeResponsePayload,
    >(
        Endpoint::ResetPassword,
        api_key,
        request_payload,
        None,
    )
    .await
}
