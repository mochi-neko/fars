//! Implements the confirm email verification API of the Firebase Auth API.
//!
//! You can confirm an email verification code by issuing an HTTP POST request to the Auth setAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification).

use serde::{Deserialize, Serialize};

use crate::ApiKey;
use crate::Client;
use crate::Endpoint;
use crate::ProviderUserInfo;
use crate::Result;

/// Request body payload for the confirm email verification API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification).
#[derive(Serialize)]
pub struct ConfirmEmailVerificationRequestBodyPayload {
    /// The action code sent to user's email for email verification.
    #[serde(rename = "oobCode")]
    oob_code: String,
}

impl ConfirmEmailVerificationRequestBodyPayload {
    /// Creates a new request body payload for the confirm email verification API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification).
    ///
    /// ## Arguments
    /// - `oob_code` - The action code sent to user's email for email verification.
    pub fn new(oob_code: String) -> Self {
        Self {
            oob_code,
        }
    }
}

/// Response payload for the confirm email verification API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification).
#[derive(Deserialize, Debug)]
pub struct ConfirmEmailVerificationResponsePayload {
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo url for the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// The password hash.
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Vec<ProviderUserInfo>,
    /// Whether or not the account's email has been verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
}

/// Confirms the email verification for the given user.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification).
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
/// - EXPIRED_OOB_CODE: The action code has expired.
/// - INVALID_OOB_CODE: The action code is invalid. This can happen if the code is malformed, expired, or has already been used.
/// - USER_DISABLED: The user account has been disabled by an administrator.
/// - EMAIL_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
///
/// ## Example
/// ```
/// use fars::api;
/// use fars::Client;
/// use fars::ApiKey;
///
/// let request_payload = api::ConfirmEmailVerificationRequestBodyPayload::new(
///     "oob-code".to_string(),
/// );
///
/// let response_payload = api::confirm_email_verification(
///     Client::new(),
///     ApiKey::new("your-firebase-project-api-key"),
///     request_payload,
/// ).await?;
/// ```
pub async fn confirm_email_verification(
    client: &Client,
    api_key: &ApiKey,
    request_payload: ConfirmEmailVerificationRequestBodyPayload,
) -> Result<ConfirmEmailVerificationResponsePayload> {
    client.send_post::<
        ConfirmEmailVerificationRequestBodyPayload,
        ConfirmEmailVerificationResponsePayload,
    >(
        Endpoint::Update,
        api_key,
        request_payload,
        None,
    )
    .await
}
