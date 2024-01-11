//! Implements an internal API client for the Firebase Auth.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth).

use serde::{de::DeserializeOwned, Serialize};

use crate::error::{ApiErrorResponse, CommonErrorCode};
use crate::ApiKey;
use crate::Error;
use crate::LanguageCode;
use crate::Result;

/// HTTP client.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) inner: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Creates a new HTTP client.
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    /// Creates a new HTTP client with a custom instance.
    ///
    /// ## NOTE
    /// This method requires the `custom_client` feature.
    ///
    /// ## Arguments
    /// - `client` - A custom HTTP client instance.
    ///
    /// ## Example
    /// ```
    /// use fars::Client;
    /// use std::time::Duration;
    ///
    /// // Create a custom reqwest client with timeout.
    /// let client = fars::reqwest::ClientBuilder::new()
    ///     .timeout(Duration::from_secs(60))
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .build()?;
    ///
    /// // Customize HTTP client.
    /// let client = Client::custom(client);
    /// ```
    #[cfg(feature = "custom_client")]
    pub fn custom(client: crate::reqwest::Client) -> Self {
        Self {
            inner: client,
        }
    }

    /// Sends a POST request to the Firebase Auth API.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
    ///
    /// ## Arguments
    /// - `endpoint` - The endpoint to send the request to.
    /// - `api_key` - The Firebase project's API key.
    /// - `request_payload` - The request body payload.
    /// - `locale` - The BCP 47 language code, eg: en-US.
    ///
    /// ## Returns
    /// The result with the response payload of the API.
    ///
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    pub(crate) async fn send_post<T, U>(
        &self,
        endpoint: Endpoint,
        api_key: &ApiKey,
        request_payload: T,
        locale: Option<LanguageCode>,
    ) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        // Build a request URL.
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/{}?key={}",
            endpoint.format(),
            api_key.inner()
        );

        // Create request builder and set method and payload.
        let mut builder = self
            .inner
            .post(url)
            .json(&request_payload);

        // Set optional headers if some are provided.
        if let Some(locale) = locale {
            builder = builder.headers(optional_locale_header(locale)?);
        }

        // Send a request.
        let response = builder
            .send()
            .await
            .map_err(Error::HttpRequestError)?;

        // Check the response status code.
        let status_code = response.status();

        // Read the response body as text.
        let response_text = response
            .text()
            .await
            .map_err(|error| Error::ReadResponseTextFailed {
                error,
            })?;

        // Successful response.
        if status_code.is_success() {
            // Deserialize the response text to a payload.
            serde_json::from_str::<U>(&response_text).map_err(|error| {
                Error::DeserializeResponseJsonFailed {
                    error,
                    json: response_text,
                }
            })
        }
        // Error response.
        else {
            // Deserialize the response text to the error payload.
            let error_response =
                serde_json::from_str::<ApiErrorResponse>(&response_text)
                    .map_err(|error| {
                        Error::DeserializeErrorResponseJsonFailed {
                            error,
                            json: response_text,
                        }
                    })?;

            // Check error message and create error code.
            let error_code: CommonErrorCode = error_response
                .error
                .message
                .clone()
                .into();

            match error_code {
                // Take invalid ID token error as special case.
                | CommonErrorCode::InvalidIdToken => Err(Error::InvalidIdToken),
                | _ => Err(Error::ApiError {
                    status_code,
                    error_code,
                    response: error_response,
                }),
            }
        }
    }
}

/// The endpoint to send the request to.
pub(crate) enum Endpoint {
    /// accounts:signInWithCustomToken
    SignInWithCustomToken,
    /// token
    Token,
    /// accounts:signUp
    SignUp,
    /// accounts:signInWithPassword
    SignInWithPassword,
    /// accounts:signInWithIdp
    SignInWithIdp,
    /// accounts:createAuthUri
    CreateAuthUri,
    /// accounts:sendOobCode
    SendOobCode,
    /// accounts:resetPassword
    ResetPassword,
    /// accounts:update
    Update,
    /// accounts:lookup
    Lookup,
    /// accounts:delete
    Delete,
}

impl Endpoint {
    /// Formats the endpoint to a string.
    pub(crate) fn format(self) -> &'static str {
        match self {
            | Endpoint::SignInWithCustomToken => {
                "accounts:signInWithCustomToken"
            },
            | Endpoint::Token => "token",
            | Endpoint::SignUp => "accounts:signUp",
            | Endpoint::SignInWithPassword => "accounts:signInWithPassword",
            | Endpoint::SignInWithIdp => "accounts:signInWithIdp",
            | Endpoint::CreateAuthUri => "accounts:createAuthUri",
            | Endpoint::SendOobCode => "accounts:sendOobCode",
            | Endpoint::ResetPassword => "accounts:resetPassword",
            | Endpoint::Update => "accounts:update",
            | Endpoint::Lookup => "accounts:lookup",
            | Endpoint::Delete => "accounts:delete",
        }
    }
}

/// Creates optional headers for the locale.
///
/// ## Arguments
/// - `locale` - The BCP 47 language code, eg: en-US.
///
/// ## Returns
/// Optional headers for the locale if some locale is provided.
///
/// ## Errors
/// - `Error::InvalidHeaderValue` - Invalid header value.
fn optional_locale_header(
    locale: LanguageCode
) -> Result<reqwest::header::HeaderMap> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        "X-Firebase-Locale",
        reqwest::header::HeaderValue::from_str(locale.format()).map_err(
            |error| Error::InvalidHeaderValue {
                key: "X-Firebase-Locale",
                error,
            },
        )?,
    );

    Ok(headers)
}
