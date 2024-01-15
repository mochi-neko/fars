use serde::{Deserialize, Serialize};

/// ID token payload claims for the Firebase Auth.
///
/// See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).
///
/// ## NOTE
/// This is only available when the feature "verify" is enabled.
#[derive(Debug, Deserialize, Serialize)]
pub struct IdTokenPayloadClaims {
    /// Expiration time.
    /// Must be in the future.
    /// The time is measured in seconds since the UNIX epoch.
    pub exp: u64,
    /// Issued-at time.
    /// Must be in the past.
    /// The time is measured in seconds since the UNIX epoch.
    pub iat: u64,
    /// Audience.
    /// Must be your Firebase project ID, the unique identifier for your Firebase project, which can be found in the URL of that project's console.
    pub aud: String,
    /// Issuer.
    /// Must be `"https://securetoken.google.com/<projectId>"`, where `<projectId>` is the same project ID used for aud above.
    pub iss: String,
    /// Subject.
    /// Must be a non-empty string and must be the uid of the user or device.
    pub sub: String,
    /// Authentication time.
    /// Must be in the past.
    /// The time when the user authenticated.
    pub auth_time: u64,
}
