//! Implements the link with OAuth credential API of the Firebase Auth.
//!
//! You can link an OAuth credential to a user by issuing an HTTP POST request to the Auth verifyAssertion endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential).

use serde::{Deserialize, Serialize};

use crate::ApiKey;
use crate::Client;
use crate::Endpoint;
use crate::IdpPostBody;
use crate::Result;

/// Request body payload for the link with OAuth credential API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential).
#[derive(Serialize)]
pub struct LinkWithOAuthCredentialRequestBodyPayload {
    /// The Firebase ID token of the account you are trying to link the credential to.
    #[serde(rename = "idToken")]
    id_token: String,
    /// The URI to which the IDP redirects the user back.
    #[serde(rename = "requestUri")]
    request_uri: String,
    /// Contains the OAuth credential (an ID token or access token) and provider ID which issues the credential.
    #[serde(rename = "postBody")]
    post_body: String,
    /// Whether or not to return an ID and refresh token. Should always be true.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
    /// Whether to force the return of the OAuth credential on the following errors: FEDERATED_USER_ID_ALREADY_LINKED and EMAIL_EXISTS.
    #[serde(rename = "returnIdpCredential")]
    return_idp_credential: bool,
}

impl LinkWithOAuthCredentialRequestBodyPayload {
    /// Creates a new request body payload for the link with OAuth credential API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential).
    ///
    /// ## Arguments
    /// - `id_token` - The Firebase ID token of the account you are trying to link the credential to.
    /// - `request_uri` - The URI to which the IDP redirects the user back.
    /// - `post_body` - Contains the OAuth credential (an ID token or access token) and provider ID which issues the credential.
    /// - `return_secure_token` - Whether or not to return an ID and refresh token. Should always be true.
    pub fn new(
        id_token: String,
        request_uri: String,
        post_body: IdpPostBody,
        return_idp_credential: bool,
    ) -> Self {
        Self {
            id_token,
            request_uri,
            post_body: post_body.query,
            return_secure_token: true,
            return_idp_credential,
        }
    }
}

/// Response payload for the link with OAuth credential API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential).
#[derive(Deserialize, Debug)]
pub struct LinkWithOAuthCredentialResponsePayload {
    /// The unique ID identifies the IdP account.
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    /// The linked provider ID (e.g. "google.com" for the Google provider).
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// The uid of the authenticated user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// Whether the signin email is verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    /// The email of the account.
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
    /// The photo url for the account.
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
}

/// Links the authenticated user with a federated OAuth credential.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential).
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
/// - `Error::InvalidIdToken` - Invalid ID token.
/// - `Error::ApiError` - API error on the Firebase Auth.
///
/// ## Common error codes
/// - OPERATION_NOT_ALLOWED: The corresponding provider is disabled for this project.
/// - INVALID_IDP_RESPONSE: The supplied auth credential is malformed or has expired.
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
/// - EMAIL_EXISTS: The email address is already in use by another account.
/// - FEDERATED_USER_ID_ALREADY_LINKED: This credential is already associated with a different user account.
///
/// ## Example
/// ```
/// use std::collections::HashMap;
/// use fars::api;
/// use fars::IdpPostBody;
/// use fars::ProviderId;
/// use fars::Client;
/// use fars::ApiKey;
///
/// let request_payload = api::LinkWithOAuthCredentialRequestBodyPayload::new(
///     "id-token".to_string(),
///     "request-uri".to_string(),
///     IdpPostBody::new(
///         ProviderId::Google,
///         HashMap::from([(
///             "access_token",
///             "google-access-token".to_string(),
///         )]),
///     )?,
///     true,
/// );
///
/// let response_payload = api::link_with_oauth_credential(
///     Client::new(),
///     ApiKey::new("your-firebase-project-api-key"),
///     request_payload,
/// ).await?;
/// ```
pub async fn link_with_oauth_credential(
    client: &Client,
    api_key: &ApiKey,
    request_payload: LinkWithOAuthCredentialRequestBodyPayload,
) -> Result<LinkWithOAuthCredentialResponsePayload> {
    client.send_post::<
        LinkWithOAuthCredentialRequestBodyPayload,
        LinkWithOAuthCredentialResponsePayload,
    >(
        Endpoint::SignInWithIdp,
        api_key,
        request_payload,
        None,
    )
    .await
}
