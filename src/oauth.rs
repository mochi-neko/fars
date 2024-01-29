//! Provides OAuth 2.0 client.
//!
//! ## NOTE
//! This is only available when the feature "oauth" is enabled.
//!
//! ## Supported identity providers and grant types
//!
//! - [Google](https://developers.google.com/identity/protocols/oauth2)
//!     - [x] Authorization Code grant type with client secret and PKCE for Web-Server apps.
//!     - [x] Device grant type for limited-input devices.
//! - [Facebook](https://developers.facebook.com/docs/facebook-login/guides/advanced/oidc-token)
//!     - [x] Authorization Code grant type with PKCE for Web-Server, Web-Client, Mobile and Desktop apps.
//! - [GitHub](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)
//!     - [x] Authorization Code grant type with client secret for Web-Server apps.
//!     - [ ] Device grant type for limited-input devices.
//! - [Twitter (X)](https://developer.twitter.com/en/docs/authentication/oauth-2-0)
//!     - [ ] Authorization Code grant type with PKCE for Web-Server, Web-Client, Mobile, and Desktop apps.
//!         - Implemented but may not be supported by the Firebase Auth.
//! - [Microsoft](https://learn.microsoft.com/en-us/entra/identity-platform/v2-app-types)
//!     - [ ] Authorization Code grant type with PKCE for Web-Server, Web-Client, Mobile, and Desktop apps.
//!         - Implemented but may not be supported by the Firebase Auth.
//!     - [ ] Device grant type for limited-input devices.
//! - [ ] Yahoo
//! - [ ] Apple
//! - [ ] Google Play Games
//! - [ ] Apple Game Center
//!
//! ## Examples
//!
//!

mod auth_code_client;
mod auth_code_session;
mod data;
mod device_code_client;
mod device_code_session;
mod error;
mod idp;
mod result;
mod token;

pub use auth_code_client::AuthorizationCodeClient;
pub use auth_code_session::AuthorizationCodeSession;
pub use data::AccessToken;
pub use data::AuthorizationCode;
pub use data::AuthorizeEndpoint;
pub use data::AuthorizeUrl;
pub use data::ClientId;
pub use data::ClientSecret;
pub use data::CsrfState;
pub use data::DeviceEndpoint;
pub use data::OAuthScope;
pub use data::PkceOption;
pub use data::RedirectUrl;
pub use data::RefreshToken;
pub use data::TokenEndpoint;
pub use data::UserCode;
pub use data::VerificationUri;
pub use data::VerificationUriComplete;
pub use device_code_client::DeviceCodeClient;
pub use device_code_session::DeviceCodeSession;
pub use error::OAuthError;
pub use idp::facebook_auth_code::FacebookAuthorizationCodeClient;
pub use idp::github_auth_code::GitHubAuthorizationCodeClient;
pub use idp::google_auth_code::GoogleAuthorizationCodeClient;
pub use idp::google_device_code::GoogleDeviceCodeClient;
pub use idp::microsoft_auth_code::MicrosoftAuthorizationCodeClient;
pub use idp::microsoft_issuer::MicrosoftIssuer;
pub use idp::twitter_auth_code::TwitterAuthorizationCodeClient;
pub use result::OAuthResult;
pub use token::OAuthToken;
