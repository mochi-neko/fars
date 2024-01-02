//! Shared data structures for the Firebase Auth API.

use serde::{Deserialize, Serialize};

use std::fmt::Display;

/// User data of the Firebase Auth.
#[derive(Deserialize, PartialEq, Clone)]
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

/// Provider user information.
#[derive(Deserialize, PartialEq, Clone)]
pub struct ProviderUserInfo {
    /// The provider identifier.
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// The display name for the account.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The photo url of the account.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    /// The federated identifier.
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,
    /// The raw identifier of the account.
    #[serde(rename = "rawId")]
    pub raw_id: Option<String>,
    /// The screen name of the account.
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
}

/// ID provider identifiers defined at [document](https://firebase.google.com/docs/projects/provisioning/configure-oauth#add-idp).
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum ProviderId {
    /// Password,
    Password,
    /// Apple.
    Apple,
    /// Apple Game Center.
    AppleGameCenter,
    /// Facebook.
    Facebook,
    /// GitHub.
    GitHub,
    /// Google.
    Google,
    /// Google Play Games.
    GooglePlayGames,
    /// LinkedIn.
    LinkedIn,
    /// Microsoft.
    Microsoft,
    /// Twitter (X).
    Twitter,
    /// Yahoo.
    Yahoo,
}

impl Display for ProviderId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ProviderId::Password => write!(f, "password"),
            | ProviderId::Apple => write!(f, "apple.com"),
            | ProviderId::AppleGameCenter => write!(f, "gc.apple.com"),
            | ProviderId::Facebook => write!(f, "facebook.com"),
            | ProviderId::GitHub => write!(f, "github.com"),
            | ProviderId::Google => write!(f, "google.com"),
            | ProviderId::GooglePlayGames => write!(f, "playgames.google.com"),
            | ProviderId::LinkedIn => write!(f, "linkedin.com"),
            | ProviderId::Microsoft => write!(f, "microsoft.com"),
            | ProviderId::Twitter => write!(f, "twitter.com"),
            | ProviderId::Yahoo => write!(f, "yahoo.com"),
        }
    }
}

impl ProviderId {
    /// Converts the provider ID to a string representation.
    ///
    /// ## Returns
    /// String representation of the provider ID.
    pub fn to_string(&self) -> String {
        match self {
            | ProviderId::Password => "password".to_string(),
            | ProviderId::Apple => "apple.com".to_string(),
            | ProviderId::AppleGameCenter => "gc.apple.com".to_string(),
            | ProviderId::Facebook => "facebook.com".to_string(),
            | ProviderId::GitHub => "github.com".to_string(),
            | ProviderId::Google => "google.com".to_string(),
            | ProviderId::GooglePlayGames => "playgames.google.com".to_string(),
            | ProviderId::LinkedIn => "linkedin.com".to_string(),
            | ProviderId::Microsoft => "microsoft.com".to_string(),
            | ProviderId::Twitter => "twitter.com".to_string(),
            | ProviderId::Yahoo => "yahoo.com".to_string(),
        }
    }

    /// Tries to parse a string to a provider ID.
    ///
    /// ## Arguments
    /// - `string` - String to parse.
    ///
    /// ## Returns
    /// Provider ID if the string is a valid provider ID, otherwise None.
    pub fn try_parse(string: String) -> Option<Self> {
        match string.as_str() {
            | "password" => Some(ProviderId::Password),
            | "apple.com" => Some(ProviderId::Apple),
            | "gc.apple.com" => Some(ProviderId::AppleGameCenter),
            | "facebook.com" => Some(ProviderId::Facebook),
            | "github.com" => Some(ProviderId::GitHub),
            | "google.com" => Some(ProviderId::Google),
            | "playgames.google.com" => Some(ProviderId::GooglePlayGames),
            | "linkedin.com" => Some(ProviderId::LinkedIn),
            | "microsoft.com" => Some(ProviderId::Microsoft),
            | "twitter.com" => Some(ProviderId::Twitter),
            | "yahoo.com" => Some(ProviderId::Yahoo),
            | _ => None,
        }
    }
}

/// Post body for ID providers contains the OAuth credential and provider ID.
#[derive(Clone)]
pub enum IdpPostBody {
    /// Google OAuth.
    Google {
        id_token: String,
    },
    /// Facebook OAuth.
    Facebook {
        access_token: String,
    },
    /// Twitter OAuth.
    Twitter {
        access_token: String,
        oauth_token_secret: String,
    },
}

impl Serialize for IdpPostBody {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            | IdpPostBody::Google {
                id_token,
            } => {
                let post_body = format!(
                    "id_token={id_token}&providerId=google.com",
                    id_token = id_token
                );
                serializer.serialize_str(post_body.as_str())
            },
            | IdpPostBody::Facebook {
                access_token,
            } => {
                let post_body = format!(
                    "access_token={access_token}&providerId=facebook.com",
                    access_token = access_token
                );
                serializer.serialize_str(post_body.as_str())
            },
            | IdpPostBody::Twitter {
                access_token,
                oauth_token_secret,
            } => {
                let post_body = format!(
                    "access_token={access_token}&oauth_token_secret={oauth_token_secret}&providerId=twitter.com",
                    access_token = access_token, oauth_token_secret = oauth_token_secret
                );
                serializer.serialize_str(post_body.as_str())
            },
        }
    }
}
