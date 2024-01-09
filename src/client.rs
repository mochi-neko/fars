//! Implements an internal API client for the Firebase Auth.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth).

use serde::{de::DeserializeOwned, Serialize};

use crate::error::{ApiErrorResponse, CommonErrorCode, Error};
use crate::{ApiKey, Result};

/// Sends a POST request to the Firebase Auth API.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
///
/// ## Arguments
/// - `client` - HTTP client.
/// - `endpoint` - The endpoint to send the request to.
/// - `api_key` - The Firebase project's API key.
/// - `request_payload` - The request body payload.
/// - `optional_headers` - Optional headers to send with the request.
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
    client: &reqwest::Client,
    endpoint: &str,
    api_key: &ApiKey,
    request_payload: T,
    optional_headers: Option<reqwest::header::HeaderMap>,
) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    // Build a request URL.
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/{}?key={}",
        endpoint, api_key.inner
    );

    // Create request builder and set method and payload.
    let mut builder = client
        .post(url)
        .json(&request_payload);

    // Set optional headers if some are provided.
    if let Some(optional_headers) = optional_headers {
        builder = builder.headers(optional_headers);
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
            serde_json::from_str::<ApiErrorResponse>(&response_text).map_err(
                |error| Error::DeserializeErrorResponseJsonFailed {
                    error,
                    json: response_text,
                },
            )?;

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

pub(crate) enum Endpoint {}

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
pub(crate) fn optional_locale_header(
    locale: Option<String>
) -> Result<Option<reqwest::header::HeaderMap>> {
    match locale {
        | Some(locale) => {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "X-Firebase-Locale",
                reqwest::header::HeaderValue::from_str(&locale).map_err(
                    |error| Error::InvalidHeaderValue {
                        key: "X-Firebase-Locale",
                        error,
                    },
                )?,
            );
            Ok(Some(headers))
        },
        | None => Ok(None),
    }
}
