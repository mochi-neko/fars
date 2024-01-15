use std::collections::HashMap;

use crate::verification::IdTokenPayloadClaims;
use crate::verification::VerificationError;
use crate::verification::VerificationResult;
use crate::Client;
use crate::IdToken;
use crate::ProjectId;

/// Configuration for the ID token verification.
///
/// ## NOTE
/// This is only available when the feature "verify" is enabled.
///
/// ## Examples
/// ```
/// use fars::verification::VerificationConfig;
/// use fars::ProjectId;
///
/// let config = VerificationConfig::new(
///     ProjectId::new("firebase-project-id"),
/// );
/// ```
pub struct VerificationConfig {
    /// A HTTP client.
    client: Client,
    /// Your project ID of the Firebase project.
    project_id: ProjectId,
}

impl VerificationConfig {
    /// Creates a new configuration for the ID token verification.
    ///
    /// ## NOTE
    /// This is only available when the feature "verify" is enabled.
    ///
    /// ## Arguments
    /// - `project_id` - Your project ID of the Firebase project.
    ///
    /// ## Examples
    /// ```
    /// use fars::verification::VerificationConfig;
    /// use fars::ProjectId;
    ///
    /// let config = VerificationConfig::new(
    ///     ProjectId::new("firebase-project-id"),
    /// );
    /// ```
    pub fn new(project_id: ProjectId) -> Self {
        Self {
            client: Client::new(),
            project_id,
        }
    }

    /// Creates a new configuration for the ID token verification with a custom HTTP client.
    ///
    /// ## NOTE
    /// This is only available when the features "verify" and "custom_client" are enabled.
    ///
    /// ## Arguments
    /// - `client` - A HTTP client.
    /// - `project_id` - Your project ID of the Firebase project.
    ///
    /// ## Examples
    /// ```
    /// use fars::verification::VerificationConfig;
    /// use fars::ProjectId;
    /// use std::time::Duration;
    ///
    /// let client = fars::reqwest::Client::builder()
    ///     .timeout(Duration::from_secs(60))
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .build()?;
    ///
    /// let config = VerificationConfig::custom(
    ///     client,
    ///     ProjectId::new("firebase-project-id"),
    /// );
    /// ```
    #[cfg(feature = "custom_client")]
    pub fn custom(
        client: Client,
        project_id: ProjectId,
    ) -> Self {
        Self {
            client,
            project_id,
        }
    }

    /// Verifies an ID token of the Firebase Auth.
    ///
    /// See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).
    ///
    /// ## NOTE
    /// This is only available when the feature "verify" is enabled.
    ///
    /// ## Arguments
    /// - `id_token` - An ID token of the Firebase Auth.
    ///
    /// ## Returns
    /// Decoded ID token payload claims if the ID token is valid.
    ///
    /// ## Errors
    /// [`VerificationError`] if the ID token is invalid.
    ///
    /// ## Example
    /// ```
    /// use fars::verification::VerificationConfig;
    /// use fars::ProjectId;
    /// use fars::IdToken;
    ///
    /// let config = VerificationConfig::new(
    ///     ProjectId::new("firebase-project-id"),
    /// );
    ///
    /// let claims = config.verify_id_token(
    ///     &IdToken::new("id-token"),
    /// ).await?;
    /// ```
    pub async fn verify_id_token(
        &self,
        id_token: &IdToken,
    ) -> VerificationResult {
        verify_id_token(&self.client, id_token, &self.project_id).await
    }
}

/// Verify an ID token of the Firebase Auth.
///
/// See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).
///
/// ## NOTE
/// This is only available when the feature "verify" is enabled.
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
/// [`VerificationError`] if the ID token is invalid.
async fn verify_id_token(
    client: &Client,
    id_token: &IdToken,
    project_id: &ProjectId,
) -> VerificationResult {
    // Decode header of the ID token.
    let header = jsonwebtoken::decode_header(id_token.inner())
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
    let response = client
        .inner()
        .get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
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
    validation.set_audience(&[project_id.inner()]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        project_id.inner()
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
        id_token.inner(),
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
