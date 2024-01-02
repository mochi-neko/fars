//! Implements the sign in with OAuth credential API of Firebase Auth.
//!
//! You can sign in a user with an OAuth credential by issuing an HTTP POST request to the Auth verifyAssertion endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::IdpPostBody;
use crate::result::Result;

/// Request body payload for the sign in with OAuth credential API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
#[derive(Serialize)]
pub struct SignInWithOAuthCredentialRequestBodyPayload {
    /// The URI to which the IDP redirects the user back.
    #[serde(rename = "requestUri")]
    request_uri: String,
    /// Contains the OAuth credential (an ID token or access token) and provider ID which issues the credential.
    #[serde(rename = "postBody")]
    post_body: IdpPostBody,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
    /// Whether to force the return of the OAuth credential on the following errors: FEDERATED_USER_ID_ALREADY_LINKED and EMAIL_EXISTS.
    #[serde(rename = "returnIdpCredential")]
    return_ipd_credential: bool,
}

impl SignInWithOAuthCredentialRequestBodyPayload {
    /// Creates a new request body payload for the sign in with OAuth credential API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
    ///
    /// ## Arguments
    /// - `request_uri` - The URI to which the IDP redirects the user back.
    /// - `post_body` - Contains the OAuth credential (an ID token or access token) and provider ID which issues the credential.
    /// - `return_ipd_credential` - Whether to force the return of the OAuth credential on the following errors: FEDERATED_USER_ID_ALREADY_LINKED and EMAIL_EXISTS.
    pub fn new(
        request_uri: String,
        post_body: IdpPostBody,
        return_ipd_credential: bool,
    ) -> Self {
        Self {
            request_uri,
            post_body,
            return_secure_token: true,
            return_ipd_credential,
        }
    }
}

/// Response payload for the sign in with OAuth credential API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
#[derive(Deserialize)]
pub struct SignInWithOAuthCredentialResponsePayload {
    /// The unique ID identifies the IdP account.
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    /// The linked provider ID (e.g. "google.com" for the Google provider).
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// The uid of the authenticated user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// Whether the sign-in email is verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    /// The email for the authenticated user.
    #[serde(rename = "email")]
    pub email: String,
    /// The OIDC id token if available.
    #[serde(rename = "oauthIdToken")]
    pub oauth_id_token: Option<String>,
    /// The OAuth access token if available.
    #[serde(rename = "oauthAccessToken")]
    pub oauth_access_token: Option<String>,
    /// The OAuth 1.0 token secret if available.
    #[serde(rename = "oauthTokenSecret")]
    pub oauth_token_secret: Option<String>,
    /// The stringified JSON response containing all the IdP data corresponding to the provided OAuth credential.
    #[serde(rename = "rawUserInfo")]
    pub raw_user_info: String,
    /// The first name for the account.
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    /// The last name for the account.
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    /// The full name for the account.
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo Url for the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// A Firebase Auth ID token for the authenticated user.
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// A Firebase Auth refresh token for the authenticated user.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// Whether another account with the same credential already exists.
    /// The user will need to sign in to the original account and then link the current credential to it.
    #[serde(rename = "needConfirmation")]
    pub need_confirmation: Option<bool>,
    /// Kind.
    #[serde(rename = "kind")]
    pub kind: Option<String>,
}

/// Signs in a user with the given OAuth credential.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
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
/// - OPERATION_NOT_ALLOWED: The corresponding provider is disabled for this project.
/// - INVALID_IDP_RESPONSE: The supplied auth credential is malformed or has expired.
///
/// ## Example
/// ```
/// use fars::api::sign_in_with_oauth_credential::{
///     SignInWithOAuthCredentialRequestBodyPayload,
///     sign_in_with_oauth_credential,
/// };
/// use fars::data::idp_post_body::IdpPostBody;
///
/// let request_payload = SignInWithOAuthCredentialRequestBodyPayload::new(
///     "request-uri".to_string(),
///     IdpPostBody::Google{ id_token: "google-oauth-open-id-token".to_string() },
///     false,
/// );
///
/// let response_payload = sign_in_with_oauth_credential(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await.unwrap();
///
/// // Do something with the response payload.
/// ```
pub async fn sign_in_with_oauth_credential(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: SignInWithOAuthCredentialRequestBodyPayload,
) -> Result<SignInWithOAuthCredentialResponsePayload> {
    client::send_post::<
        SignInWithOAuthCredentialRequestBodyPayload,
        SignInWithOAuthCredentialResponsePayload,
    >(
        client,
        "accounts:signInWithIdp",
        api_key,
        request_payload,
        None,
    )
    .await
}
