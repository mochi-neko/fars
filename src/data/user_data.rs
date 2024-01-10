use serde::Deserialize;

use crate::ProviderUserInfo;

/// User data of the Firebase Auth.
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct UserData {
    /// The uid of the current user.
    #[serde(rename = "localId")]
    pub local_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Whether or not the account's email has been verified.
    #[serde(rename = "emailVerified")]
    pub email_verified: Option<bool>,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// List of all linked provider objects which contain "providerId" and "federatedId".
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Option<Vec<ProviderUserInfo>>,
    /// The photo url of the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// Hash version of password.
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    /// The timestamp, in milliseconds, that the account password was last changed.
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: Option<f64>,
    /// The timestamp, in seconds, which marks a boundary, before which Firebase ID token are considered revoked.
    #[serde(rename = "validSince")]
    pub valid_since: Option<String>,
    /// Whether the account is disabled or not.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The timestamp, in milliseconds, that the account last logged in at.
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: String,
    /// The timestamp, in milliseconds, that the account was created at.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The timestamp, in milliseconds, that the account was last refreshed at.
    #[serde(rename = "lastResheshAt")]
    pub last_refresh_at: Option<String>,
    /// Whether the account is authenticated by the developer.
    #[serde(rename = "customAuth")]
    pub custom_auth: Option<bool>,
}
