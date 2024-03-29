//! Implements the sign up with email password API of Firebase Auth.
//!
//! You can create a new email and password user by issuing an HTTP POST request to the Auth signupNewUser endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).

use serde::{Deserialize, Serialize};

use crate::ApiKey;
use crate::Client;
use crate::Endpoint;
use crate::Result;

/// Request body payload for the sign up with email password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
#[derive(Serialize)]
pub struct SignUpWithEmailPasswordRequestBodyPayload {
    /// The email for the user to create.
    #[serde(rename = "email")]
    email: String,
    /// The password for the user to create.
    #[serde(rename = "password")]
    password: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl SignUpWithEmailPasswordRequestBodyPayload {
    /// Creates a new request body payload for the sign up with email password API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
    ///
    /// ## Arguments
    /// - `email` - The email for the user to create.
    /// - `password` - The password for the user to create.
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

/// Response payload for the sign up with email password API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
#[derive(Deserialize, Debug)]
pub struct SignUpWithEmailPasswordResponsePayload {
    /// A Firebase Auth ID token for the newly created user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// The email for the newly created user.
    #[serde(rename = "email")]
    pub email: String,
    /// A Firebase Auth refresh token for the newly created user.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// The uid of the newly created user.
    #[serde(rename = "localId")]
    pub local_id: String,
}

/// Signs up a user with the given email address and password.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password).
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
/// - EMAIL_EXISTS: The email address is already in use by another account.
/// - OPERATION_NOT_ALLOWED: Password sign-in is disabled for this project.
/// - TOO_MANY_ATTEMPTS_TRY_LATER: We have blocked all requests from this device due to unusual activity. Try again later.
///
/// ## Example
/// ```
/// use fars::api;
/// use fars::Client;
/// use fars::ApiKey;
///
/// let request_payload = api::SignUpWithEmailPasswordRequestBodyPayload::new(
///     "email".to_string(),
///     "password".to_string(),
/// );
///
/// let response_payload = api::sign_up_with_email_password(
///     Client::new(),
///     ApiKey::new("your-firebase-project-api-key"),
///     request_payload,
/// ).await?;
/// ```
pub async fn sign_up_with_email_password(
    client: &Client,
    api_key: &ApiKey,
    request_payload: SignUpWithEmailPasswordRequestBodyPayload,
) -> Result<SignUpWithEmailPasswordResponsePayload> {
    client.send_post::<
        SignUpWithEmailPasswordRequestBodyPayload,
        SignUpWithEmailPasswordResponsePayload,
    >(
        Endpoint::SignUp,
        api_key,
        request_payload,
        None,
    )
    .await
}
