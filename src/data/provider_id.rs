//! Defines the provider ID of the Firebase Auth.

use std::fmt::Display;

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
    /// Formats the provider ID to a string representation of the Firebase Auth.
    ///
    /// ## Returns
    /// String representation of the provider ID of the Firebase Auth.
    pub fn format(&self) -> String {
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
