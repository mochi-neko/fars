//! Verification of an ID token of the Firebase Auth.
//!
//! See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::VerificationError;
use crate::result::VerificationResult;

/// ID token payload claims for the Firebase Auth.
///
/// See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).
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
    /// Must be "https://securetoken.google.com/<projectId>", where <projectId> is the same project ID used for aud above.
    pub iss: String,
    /// Subject.
    /// Must be a non-empty string and must be the uid of the user or device.
    pub sub: String,
    /// Authentication time.
    /// Must be in the past.
    /// The time when the user authenticated.
    pub auth_time: u64,
}

/// Verify an ID token of the Firebase Auth.
///
/// See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).
///
/// ## Arguments
/// - `client` - A HTTP client.
/// - `id_token` - An ID token of the Firebase Auth.
/// - `project_id` - Your project ID of the Firebase project.
///
/// ## Returns
/// ID token payload claims if the ID token is valid.
///
/// ## Errors
/// [`crate::error::VerificationError`] if the ID token is invalid.
///
/// ## Examples
/// ```
/// let client = reqwest::Client::new();
/// let id_token = "id-token".to_string();
/// let project_id = "project-id".to_string();
///
/// match fars::verification::verify_id_token(&client, &id_token, &project_id).await? {
///     Ok(claims) => {
///         // Verify succeeded.
///     }
///     Err(error) => {
///         // Verify failed.
///     }
/// }
/// ```
#[allow(clippy::ptr_arg)]
pub async fn verify_id_token(
    client: &reqwest::Client,
    id_token: &String,
    project_id: &String,
) -> VerificationResult {
    // Decode header of the ID token.
    let header = jsonwebtoken::decode_header(id_token)
        .map_err(VerificationError::DecodeTokenHeaderFailed)?;

    // Verify type of the token in the header.
    if header.typ != Some("JWT".to_string()) {
        return Err(VerificationError::InvalidTokenType(
            header.typ,
        ));
    }

    // Verify algorithm of the token in the header.
    if header.alg != jsonwebtoken::Algorithm::RS256 {
        return Err(VerificationError::InvalidAlgorithm(
            header.alg,
        ));
    }

    // Get kid from the header.
    let kid = header
        .kid
        .ok_or(VerificationError::KidNotFound)?;

    // Get public key list from the Google API.
    let response = client.get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
      .send()
      .await
      .map_err(VerificationError::HttpRequestError)?;

    // Verify status code of the response.
    if response.status() != reqwest::StatusCode::OK {
        return Err(
            VerificationError::InvalidResponseStatusCode(response.status()),
        );
    }

    // Deserialize the response JSON.
    let key_map = response
        .json::<HashMap<String, String>>()
        .await
        .map_err(|error| {
            VerificationError::DeserializeResponseJsonFailed(error)
        })?;

    // Find public key from the key map by kid.
    let key = key_map
        .get(&kid)
        .ok_or(VerificationError::PublicKeyNotFound(
            kid,
        ))?;

    // Get decoding key from the public key.
    let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(key.as_bytes())
        .map_err(VerificationError::GetDecodingKeyFailed)?;

    // Create validation for the ID token.
    let mut validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&[project_id.clone()]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        project_id.clone()
    )]);
    validation.set_required_spec_claims(&[
        "exp",
        "iat",
        "aud",
        "iss",
        "sub",
        "auth_time",
    ]);

    // Decode and verify the ID token.
    let decoded = jsonwebtoken::decode::<IdTokenPayloadClaims>(
        id_token,
        &decoding_key,
        &validation,
    )
    .map_err(VerificationError::DecodeTokenFailed)?;

    let time_stamp = jsonwebtoken::get_current_timestamp();

    // Verify expiration time.
    if decoded.claims.exp < time_stamp {
        return Err(VerificationError::TokenExpired(
            decoded.claims.exp,
        ));
    }

    // Verify issued-at time.
    if decoded.claims.iat > time_stamp {
        return Err(
            VerificationError::TokenIssuedInTheFuture(decoded.claims.iat),
        );
    }

    Ok(decoded.claims)
}
