use serde::Deserialize;

/// User information provided from an identity provider.
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ProviderUserInfo {
    /// The provider identifier.
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// The federated identifier.
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo url of the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The raw identifier of the account.
    #[serde(rename = "rawId")]
    pub raw_id: Option<String>,
    /// The screen name of the account.
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
}
