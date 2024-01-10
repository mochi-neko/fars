//! Implements the send password reset email API of the Firebase Auth.
//!
//! You can send a password reset email by issuing an HTTP POST request to the Auth getOobConfirmationCode endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email)

use serde::{Deserialize, Serialize};

use crate::client;
use crate::ApiKey;
use crate::LanguageCode;
use crate::Result;

/// Request body payload for the send password reset email API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
#[derive(Serialize)]
pub struct SendPasswordResetEmailRequestBodyPayload {
    /// The kind of OOB code to return. Should be "PASSWORD_RESET" for password reset.
    #[serde(rename = "requestType")]
    request_type: String,
    /// User's email address.
    #[serde(rename = "email")]
    email: String,
}

impl SendPasswordResetEmailRequestBodyPayload {
    /// Creates a new request body payload for the send password reset email API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
    ///
    /// ## Arguments
    /// - `email` - User's email address.
    pub fn new(email: String) -> Self {
        Self {
            request_type: "PASSWORD_RESET".to_string(),
            email,
        }
    }
}

/// Response payload for the send password reset email API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
#[derive(Deserialize, Debug)]
pub struct SendPasswordResetEmailResponsePayload {
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
}

/// Sends a password reset email to the given email address.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `api_key` - Your Firebase project's API key.
/// - `request_payload` - Request body payload.
/// - `locale` - (Optional) The BCP 47 language code, eg: en-US.
///
/// ## Errors
/// - `Error::InvalidHeaderValue` - Invalid header value.
/// - `Error::HttpRequestError` - Failed to send a request.
/// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
/// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
/// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
/// - `Error::ApiError` - API error on the Firebase Auth.
///
/// ## Common error codes
/// - EMAIL_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
///
/// ## Example
/// ```
/// use fars::api;
///
/// let request_payload = api::SendPasswordResetEmailRequestBodyPayload::new(
///     "email".to_string(),
/// );
///
/// let response_payload = api::send_password_reset_email(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
///     None,
/// ).await?;
/// ```
pub async fn send_password_reset_email(
    client: &reqwest::Client,
    api_key: &ApiKey,
    request_payload: SendPasswordResetEmailRequestBodyPayload,
    locale: Option<LanguageCode>,
) -> Result<SendPasswordResetEmailResponsePayload> {
    let optional_headers = client::optional_locale_header(locale)?;

    client::send_post::<
        SendPasswordResetEmailRequestBodyPayload,
        SendPasswordResetEmailResponsePayload,
    >(
        client,
        client::Endpoint::SendOobCode,
        api_key,
        request_payload,
        optional_headers,
    )
    .await
}
