//! The error types in this crate.

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
    ParseExpriesInFailed {
        error: std::num::ParseIntError,
    },
    /// Not found any user data in a response.
    #[error("Not found any user data in a response")]
    NotFoundAnyUserData,
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
    OperationNotAllowed,
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

        match val.as_str() {
            | "OPERATION_NOT_ALLOWED" => CommonErrorCode::OperationNotAllowed,
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

/// The error type for ID token verification.
#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    /// Decode ID token header failed.
    #[error("Decode ID token header failed: {0:?}")]
    DecodeTokenHeaderFailed(jsonwebtoken::errors::Error),
    /// Invalid type in ID token header.
    /// Must be "JWT".
    #[error("Invalid type in ID token header: {0:?}")]
    InvalidTokenType(Option<String>),
    /// Invalid algorithm in ID token header
    /// Must be "RS256".
    #[error("Invalid algorithm in ID token header: {0:?}")]
    InvalidAlgorithm(jsonwebtoken::Algorithm),
    /// No kid in the ID token header.
    #[error("No kid in the ID token header")]
    KidNotFound,
    /// HTTP request error to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com.
    #[error("HTTP request error to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com: {0:?}")]
    HttpRequestError(reqwest::Error),
    /// Invalid response status code to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com.
    #[error("Invalid response status code to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com: {0:?}")]
    InvalidResponseStatusCode(reqwest::StatusCode),
    /// Deserialize response JSON to hash map failed to get public key from "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com.
    #[error("Deserialize response JSON to hash map failed to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")]
    DeserializeResponseJsonFailed(reqwest::Error),
    /// Target public key specified by kid not found in key map.
    #[error("Target public key specified by kid not found in key map: {0:?}")]
    PublicKeyNotFound(String),
    /// Get decoding key failed.
    #[error("Get decoding key failed: {0:?}")]
    GetDecodingKeyFailed(jsonwebtoken::errors::Error),
    /// Decode or verify ID token failed.
    #[error("Decode ID token failed: {0:?}")]
    DecodeTokenFailed(jsonwebtoken::errors::Error),
    /// The ID token is expired.
    #[error("The ID token is expired at {0:?}")]
    TokenExpired(u64),
    /// The ID token is issued in the future.
    #[error("The ID token is issued in the future at {0:?}")]
    TokenIssuedInTheFuture(u64),
}
