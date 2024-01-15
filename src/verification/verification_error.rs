/// The error type for ID token verification.
///
/// ## NOTE
/// This is only available when the feature "verify" is enabled.
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
    /// HTTP request error to get public key from [public keys list](https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com).
    #[error("HTTP request error to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com: {0:?}")]
    HttpRequestError(reqwest::Error),
    /// Invalid response status code to get public key from [public keys list](https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com).
    #[error("Invalid response status code to get public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com: {0:?}")]
    InvalidResponseStatusCode(reqwest::StatusCode),
    /// Deserialize response JSON to hash map failed to get public key from [public keys list](https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com).
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
