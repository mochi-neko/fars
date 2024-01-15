//! # fars
//! An unofficial Rust client for [Firebase Auth REST API](https://firebase.google.com/docs/reference/rest/auth).
//!
//! ## Features
//! - default
//!     - APIs via session-based interfaces. See [`crate::config`] and [`crate::session`].
//!     - APIs via raw interfaces. See [`crate::api`].
//! - (Optional) `verify`
//!     - ID token verification. See [`crate::verification`].
//! - (Optional) `custom_client`
//!     - HTTP client customization. See [`crate::client`].

// public modules
pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod session;

// Internal modules
pub(crate) mod endpoint;

// Private modules
mod data;
mod result;

// Re-exports
pub use crate::client::Client;
pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::result::Result;
pub use crate::session::Session;

// Re-exports for internal modules
pub(crate) use crate::endpoint::Endpoint;

// Re-exports for data module
pub use crate::data::api_key::ApiKey;
pub use crate::data::delete_attribute::DeleteAttribute;
pub use crate::data::display_name::DisplayName;
pub use crate::data::email::Email;
pub use crate::data::expires_in::ExpiresIn;
pub use crate::data::id_token::IdToken;
pub use crate::data::idp_post_body::IdpPostBody;
pub use crate::data::language_code::LanguageCode;
pub use crate::data::oauth_continue_uri::OAuthContinueUri;
pub use crate::data::oauth_request_uri::OAuthRequestUri;
pub use crate::data::password::Password;
pub use crate::data::photo_url::PhotoUrl;
pub use crate::data::project_id::ProjectId;
pub use crate::data::provider_id::ProviderId;
pub use crate::data::provider_user_info::ProviderUserInfo;
pub use crate::data::refresh_token::RefreshToken;
pub use crate::data::user_data::UserData;

// Feature "verify"
pub mod verification;
#[cfg(feature = "verify")]
pub use crate::verification::IdTokenPayloadClaims;
#[cfg(feature = "verify")]
pub use crate::verification::VerificationConfig;
#[cfg(feature = "verify")]
pub use crate::verification::VerificationError;
#[cfg(feature = "verify")]
pub use crate::verification::VerificationResult;

// Feature "custom_client"
// Re-export reqwest for the feature "custom_client" to customize the HTTP client.
#[cfg(feature = "custom_client")]
pub use reqwest;

// Feature "oauth"
#[cfg(feature = "oauth")]
pub mod oauth;
