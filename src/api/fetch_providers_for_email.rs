//! Implements the fetch providers for email API of the Firebase Auth.
//!
//! You can look all providers associated with a specified email by issuing an HTTP POST request to the Auth createAuthUri endpoint.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).

use serde::{Deserialize, Serialize};

use crate::client;
use crate::Result;

/// Request body payload for the fetch providers for email API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
#[derive(Serialize)]
pub struct FetchProvidersForEmailRequestBodyPayload {
    /// User's email address
    #[serde(rename = "identifier")]
    identifier: String,
    /// The URI to which the IDP redirects the user back. For this use case, this is just the current URL.
    #[serde(rename = "continueUri")]
    continue_uri: String,
}

impl FetchProvidersForEmailRequestBodyPayload {
    /// Creates a new request body payload for the fetch providers for email API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
    ///
    /// ## Arguments
    /// - `identifier` - User's email address
    /// - `continue_uri` - The URI to which the IDP redirects the user back. For this use case, this is just the current URL.
    pub fn new(
        identifier: String,
        continue_uri: String,
    ) -> Self {
        Self {
            identifier,
            continue_uri,
        }
    }
}

/// Response payload for the fetch providers for email API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
#[derive(Deserialize, Debug)]
pub struct FetchProvidersForEmailResponsePayload {
    /// The list of providers that the user has previously signed in with.
    #[serde(rename = "allProviders")]
    pub all_providers: Vec<String>,
    /// Whether the email address is for an existing account.
    #[serde(rename = "registered")]
    pub registered: bool,
}

/// Fetches the list of sign-in methods available for the specified email address.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email).
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
/// - INVALID_EMAIL: The email address is badly formatted.
///
/// ## Example
/// ```
/// use fars::api;
///
/// let request_payload = api::FetchProvidersForEmailRequestBodyPayload::new(
///     "email".to_string(),
///     "continue-uri".to_string(),
/// );
///
/// let response_payload = api::fetch_providers_for_email(
///     reqwest::Client::new(),
///     "your-firebase-project-api-key".to_string(),
///     request_payload,
/// ).await?;
/// ```
pub async fn fetch_providers_for_email(
    client: &reqwest::Client,
    api_key: &String,
    request_payload: FetchProvidersForEmailRequestBodyPayload,
) -> Result<FetchProvidersForEmailResponsePayload> {
    client::send_post::<
        FetchProvidersForEmailRequestBodyPayload,
        FetchProvidersForEmailResponsePayload,
    >(
        client,
        "accounts:createAuthUri",
        api_key,
        request_payload,
        None,
    )
    .await
}
