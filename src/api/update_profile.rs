//! Implements the update profile API of the Firebase Auth API.
//!
//! You can update a user's profile (display name / photo URL) by issuing an HTTP POST request to the Auth setAccountInfo endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-update-profile).
use serde::{Deserialize, Serialize};

use crate::client;
use crate::data::{DeleteAttribute, ProviderUserInfo};
use crate::Result;

/// Request body payload for the update profile API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-update-profile).
#[derive(Serialize)]
pub struct UpdateProfileRequestBodyPayload {
    /// A Firebase Auth ID token for the user.
    #[serde(rename = "idToken")]
    id_token: String,
    /// The user's new display name.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// The user's new photo url.
    #[serde(rename = "photoUrl")]
    photo_url: Option<String>,
    /// List of attributes to delete, "DISPLAY_NAME" or "PHOTO_URL". This will nullify these values.
    #[serde(rename = "deleteAttribute")]
    delete_attribute: Option<Vec<String>>,
    /// Whether or not to return an ID and refresh token.
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl UpdateProfileRequestBodyPayload {
    /// Creates a new request body payload for the update profile API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-update-profile).
    ///
    /// ## Arguments
    /// - `id_token` - A Firebase Auth ID token for the user.
    /// - `display_name` - The user's new display name.
    /// - `photo_url` - The user's new photo url.
    /// - `delete_attribute` - List of attributes to delete.
    /// - `return_secure_token` - Whether or not to return an ID and refresh token.
    pub fn new(
        id_token: String,
        display_name: Option<String>,
        photo_url: Option<String>,
        delete_attribute: Option<Vec<DeleteAttribute>>,
        return_secure_token: bool,
    ) -> Self {
        let delete_attribute = match delete_attribute {
            | Some(delete_attribute) => Some(
                delete_attribute
                    .into_iter()
                    .map(|attribute| match attribute {
                        | DeleteAttribute::DisplayName => {
                            "DISPLAY_NAME".to_string()
                        },
                        | DeleteAttribute::PhotoUrl => "PHOTO_URL".to_string(),
                    })
                    .collect(),
            ),
            | None => None,
        };

        Self {
            id_token,
            display_name,
            photo_url,
            delete_attribute,
            return_secure_token,
        }
    }
}

/// Response payload for the update profile API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-update-profile).
#[derive(Deserialize, Debug)]
pub struct UpdateProfileResponsePayload {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// User's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// User's new display name.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// User's new photo url.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// Hash version of password.
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Vec<ProviderUserInfo>,
    /// New Firebase Auth ID token for user.
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    /// A Firebase Auth refresh token.
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    /// The number of seconds in which the ID token expires.
    #[serde(rename = "expiresIn")]
    pub expires_in: Option<String>,
}

/// Updates a user's profile information.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-update-profile).
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
/// - INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
///
/// ## Example
/// ```
/// use fars::api;
///
/// let request_payload = api::UpdateProfileRequestBodyPayload::new(
///     "id-token".to_string(),
///     Some("new-display-name".to_string()),
///     Some("new-photo-url".to_string()),
///     None,
/// );
///
/// let response_payload = api::update_profile(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await?;
/// ```
pub async fn update_profile(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: UpdateProfileRequestBodyPayload,
) -> Result<UpdateProfileResponsePayload> {
    client::send_post::<
        UpdateProfileRequestBodyPayload,
        UpdateProfileResponsePayload,
    >(
        client,
        "accounts:update",
        api_key,
        request_payload,
        None,
    )
    .await
}
