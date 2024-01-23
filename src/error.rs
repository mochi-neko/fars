//! The error types for APIs.

use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// The error type for APIs.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // HTTP client errors
    /// Invalid header value.
    #[error("Invalid header value: {key:?} - {error:?}")]
    InvalidHeaderValue {
        key: &'static str,
        error: reqwest::header::InvalidHeaderValue,
    },
    /// HTTP request error.
    #[error("HTTP request error: {0:?}")]
    HttpRequestError(reqwest::Error),

    // API errors
    /// API error on the Firebase Auth.
    #[error(
        "Firebase Auth API error: ({status_code:?}) {error_code:?} - {response:?}"
    )]
    ApiError {
        status_code: reqwest::StatusCode,
        error_code: CommonErrorCode,
        response: ApiErrorResponse,
    },
    /// Invalid ID token error.
    #[error("Invalid ID token")]
    InvalidIdToken,

    // Response errors
    /// Read response text failed.
    #[error("Read response text failed: {error:?}")]
    ReadResponseTextFailed {
        error: reqwest::Error,
    },
    /// Deserialize response JSON failed.
    #[error("Deserialize response JSON failed: {error:?} - {json:?}")]
    DeserializeResponseJsonFailed {
        error: serde_json::Error,
        json: String,
    },
    /// Deserialize error response JSON failed.
    #[error("Deserialize error response JSON failed: {error:?} - {json:?}")]
    DeserializeErrorResponseJsonFailed {
        error: serde_json::Error,
        json: String,
    },
    /// Parse `expires_in` failed.
    #[error("Parse expires_in failed: {error:?}")]
    ParseExpiresInFailed {
        error: std::num::ParseIntError,
    },
    /// Not found any user data in a response.
    #[error("Not found any user data in a response")]
    NotFoundAnyUserData,
    /// Url encode failed.
    #[error("Url encode failed: {error:?}")]
    UrlEncodeFailed {
        error: serde_urlencoded::ser::Error,
    },
}

/// Error response payload for the auth endpoints.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    #[serde(rename = "error")]
    pub error: ErrorResponse,
}

impl Display for ApiErrorResponse {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

/// Error response payload for the auth endpoints.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errors")]
    pub errors: Vec<ErrorElement>,
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "message")]
    pub message: String,
}

/// Error response payload for the auth endpoints.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-error-response).
#[derive(Debug, Deserialize)]
pub struct ErrorElement {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "message")]
    pub message: String,
}

/// Common error codes for the Firebase Auth API.
#[derive(Debug)]
pub enum CommonErrorCode {
    /// OPERATION_NOT_ALLOWED: The operation is disabled for this project.
    OperationNotAllowed(String),
    /// TOO_MANY_ATTEMPTS_TRY_LATER: We have blocked all requests from this device due to unusual activity. Try again later.
    TooManyAttemptsTryLater,
    /// INVALID_API_KEY: API key not valid. Please pass a valid API key. (invalid API key provided)
    InvalidApiKey,
    /// INVALID_CUSTOM_TOKEN: The custom token format is incorrect or the token is invalid for some reason (e.g. expired, invalid signature etc.)
    InvalidCustomToken,
    /// INVALID_ID_TOKEN:The user's credential is no longer valid. The user must sign in again.
    InvalidIdToken,
    /// INVALID_REFRESH_TOKEN: An invalid refresh token is provided.
    InvalidRefreshToken,
    /// Invalid JSON payload received. Unknown name \"refresh_tokens\": Cannot bind query parameter. Field 'refresh_tokens' could not be found in request message.
    InvalidJsonPayloadReceived(String),
    /// INVALID_GRANT_TYPE: the grant type specified is invalid.
    InvalidGrantType,
    /// INVALID_PASSWORD: The password is invalid or the user does not have a password.
    InvalidPassword,
    /// INVALID_IDP_RESPONSE: The supplied auth credential is malformed or has expired.
    InvalidIdpResponse,
    /// INVALID_CREDENTIAL_OR_PROVIDER_ID: Invalid IdP response/credential: ...
    InvalidCredentialOrProviderId(String),
    /// INVALID_EMAIL: The email address is badly formatted.
    InvalidEmail,
    /// INVALID_LOGIN_CREDENTIALS: The supplied auth credential is malformed or has expired.
    InvalidLoginCredentials,
    /// CREDENTIAL_MISMATCH: The custom token corresponds to a different Firebase project.
    CredentialMismatch,
    /// CREDENTIAL_TOO_OLD_LOGIN_AGAIN: The user's credential is no longer valid. The user must sign in again.
    CredentialTooOldLoginAgain,
    /// TOKEN_EXPIRED: The user's credential is no longer valid. The user must sign in again.
    TokenExpired,
    /// USER_DISABLED: The user account has been disabled by an administrator.
    UserDisabled,
    /// USER_NOT_FOUND: The user corresponding to the refresh token was not found. It is likely the user was deleted.
    UserNotFound,
    /// MISSING_REFRESH_TOKEN: no refresh token provided.
    MissingRefreshToken,
    /// EMAIL_EXISTS: The email address is already in use by another account.
    EmailExists,
    /// EMAIL_NOT_FOUND: There is no user record corresponding to this identifier. The user may have been deleted.
    EmailNotFound,
    /// WEAK_PASSWORD: The password must be 6 characters long or more.
    WeakPassword,
    /// FEDERATED_USER_ID_ALREADY_LINKED: This credential is already associated with a different user account.
    FederatedUserIdAlreadyLinked,
    /// EXPIRED_OOB_CODE: The action code has expired.
    ExpiredOobCode,
    /// INVALID_OOB_CODE: The action code is invalid. This can happen if the code is malformed, expired, or has already been used.
    InvalidOobCode,
    // ADMIN_ONLY_OPERATION: This operation is reserved to administrators only.
    AdminOnlyOperation,
    /// Unknown error codes.
    Unknown(String),
}

impl From<String> for CommonErrorCode {
    fn from(val: String) -> Self {
        if val
            .as_str()
            .starts_with("Invalid JSON payload received. Unknown name")
        {
            return CommonErrorCode::InvalidJsonPayloadReceived(val);
        }

        if val
            .as_str()
            .starts_with("OPERATION_NOT_ALLOWED")
        {
            return CommonErrorCode::OperationNotAllowed(val);
        }

        if val
            .as_str()
            .starts_with("INVALID_CREDENTIAL_OR_PROVIDER_ID")
        {
            return CommonErrorCode::InvalidCredentialOrProviderId(val);
        }

        match val.as_str() {
            | "TOO_MANY_ATTEMPTS_TRY_LATER" => {
                CommonErrorCode::TooManyAttemptsTryLater
            },
            | "INVALID_API_KEY" => CommonErrorCode::InvalidApiKey,
            | "INVALID_CUSTOM_TOKEN" => CommonErrorCode::InvalidCustomToken,
            | "INVALID_ID_TOKEN" => CommonErrorCode::InvalidIdToken,
            | "INVALID_REFRESH_TOKEN" => CommonErrorCode::InvalidRefreshToken,
            | "INVALID_GRANT_TYPE" => CommonErrorCode::InvalidGrantType,
            | "INVALID_PASSWORD" => CommonErrorCode::InvalidPassword,
            | "INVALID_IDP_RESPONSE" => CommonErrorCode::InvalidIdpResponse,
            | "INVALID_EMAIL" => CommonErrorCode::InvalidEmail,
            | "INVALID_LOGIN_CREDENTIALS" => {
                CommonErrorCode::InvalidLoginCredentials
            },
            | "CREDENTIAL_MISMATCH" => CommonErrorCode::CredentialMismatch,
            | "CREDENTIAL_TOO_OLD_LOGIN_AGAIN" => {
                CommonErrorCode::CredentialTooOldLoginAgain
            },
            | "TOKEN_EXPIRED" => CommonErrorCode::TokenExpired,
            | "USER_DISABLED" => CommonErrorCode::UserDisabled,
            | "USER_NOT_FOUND" => CommonErrorCode::UserNotFound,
            | "MISSING_REFRESH_TOKEN" => CommonErrorCode::MissingRefreshToken,
            | "EMAIL_EXISTS" => CommonErrorCode::EmailExists,
            | "EMAIL_NOT_FOUND" => CommonErrorCode::EmailNotFound,
            | "WEAK_PASSWORD" => CommonErrorCode::WeakPassword,
            | "FEDERATED_USER_ID_ALREADY_LINKED" => {
                CommonErrorCode::FederatedUserIdAlreadyLinked
            },
            | "EXPIRED_OOB_CODE" => CommonErrorCode::ExpiredOobCode,
            | "INVALID_OOB_CODE" => CommonErrorCode::InvalidOobCode,
            | "ADMIN_ONLY_OPERATION" => CommonErrorCode::AdminOnlyOperation,
            | _ => CommonErrorCode::Unknown(val),
        }
    }
}
