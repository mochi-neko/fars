//! Implements the sign in with email password API of Firebase Auth.
//!
//! You can sign in a user with an email and password by issuing an HTTP POST request to the Auth verifyPassword endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).

use serde::{Deserialize, Serialize};

use crate::ApiKey;
use crate::Client;
use crate::Endpoint;
use crate::Result;

/// Request body payload for the sign in with email password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Serialize)]
pub struct SignInWithEmailPasswordRequestBodyPayload {
    /// The email the user is sign in with.
    #[serde(rename = "email")]
    email: String,
    /// The password for the account.
    #[serde(rename = "password")]
    password: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl SignInWithEmailPasswordRequestBodyPayload {
    /// Creates a new request body payload for the sign in with email password API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
    ///
    /// ## Arguments
    /// - `email` - The email the user is sign in with.
    /// - `password` - The password for the account.
    pub fn new(
        email: String,
        password: String,
    ) -> Self {
        Self {
            email,
            password,
            return_secure_token: true,
        }
    }
}

/// Response payload for the sign in with email password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
#[derive(Deserialize, Debug)]
pub struct SignInWithEmailPasswordResponsePayload {
    /// A Firebase Auth ID token for the authenticated user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// The email for the authenticated user.
    #[serde(rename = "email")]
    pub email: String,
    /// A Firebase Auth refresh token for the authenticated user.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// The uid of the authenticated user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// Whether the email is for an existing account.
    #[serde(rename = "registered")]
    pub registered: bool,
}

/// Signs in a user with the given email address and password.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password).
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
/// - EMAIL_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
/// - INVALID_PASSWORD: The password is invalid or the user does not have a password.
/// - USER_DISABLED: The user account has been disabled by an administrator.
///
/// ## Example
/// ```
/// use fars::api;
/// use fars::Client;
/// use fars::ApiKey;
///
/// let request_payload = api::SignInWithEmailPasswordRequestBodyPayload::new(
///     "email".to_string(),
///     "password".to_string(),
/// );
///
/// let response_payload = api::sign_in_with_email_password(
///     Client::new(),
///     ApiKey::new("your-firebase-project-api-key"),
///     request_payload,
/// ).await?;
/// ```
pub async fn sign_in_with_email_password(
    client: &Client,
    api_key: &ApiKey,
    request_payload: SignInWithEmailPasswordRequestBodyPayload,
) -> Result<SignInWithEmailPasswordResponsePayload> {
    client.send_post::<
        SignInWithEmailPasswordRequestBodyPayload,
        SignInWithEmailPasswordResponsePayload,
    >(
        Endpoint::SignInWithPassword,
        api_key,
        request_payload,
        None,
    )
    .await
}
