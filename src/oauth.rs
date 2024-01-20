//! Provides OAuth 2.0 client.
//!
//! ## NOTE
//! This is only available when the feature "oauth" is enabled.
//!
//! ## Supported identity providers and grant types
//!
//! - [Google](https://developers.google.com/identity/protocols/oauth2)
//!     - [x] Authorization Code grant type with client secret and PKCE for Web-Server apps.
//!     - [ ] (Not recommended) Authorization Code grant type with **client secret** and PKCE for Mobile, and Desktop apps.
//!     - [ ] (Not recommended) **Implicit** grant type  for Web-Client apps.
//!     - [ ] Device grant type for limited-input devices.
//! - [Facebook](https://developers.facebook.com/docs/facebook-login/guides/advanced/oidc-token)
//!     - [ ] Authorization Code grant type with PKCE for Web-Server, Web-Client, Mobile and Desktop apps.
//! - [GitHub](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)
//!     - [x] Authorization Code grant type with client secret for Web-Server apps.
//!     - [ ] (Not recommended) Authorization Code grant type with **client secret** for Web-Client, Mobile, and Desktop apps.
//!     - [ ] Device grant type for limited-input devices.
//! - [Twitter (X)](https://developer.twitter.com/en/docs/authentication/oauth-2-0)
//!     - [ ] Authorization Code grant type with PKCE for Web-Server, Web-Client, Mobile, and Desktop apps.
//!     - [ ] Client Credentials grant type for Server-to-Server apps.
//! - [ ] Apple
//! - [ ] Microsoft
//! - [ ] Yahoo
//! - [ ] Google Play Games
//! - [ ] Apple Game Center
//!
//! ## Examples
//!
//!

mod auth_code_client;
mod data;
mod error;
mod idp;
mod result;
mod auth_code_session;
mod token;

pub use auth_code_client::AuthorizationCodeClient;
pub use data::AccessToken;
pub use data::AuthorizeEndpoint;
pub use data::AuthorizationCode;
pub use data::CsrfState;
pub use data::AuthorizeUrl;
pub use data::ClientId;
pub use data::ClientSecret;
pub use data::PkceOption;
pub use data::RedirectUrl;
pub use data::RefreshToken;
pub use data::Scope;
pub use data::TokenEndpoint;
pub use error::OAuthError;
pub use idp::facebook::OAuthFacebookClient;
pub use idp::github_auth_code::GitHubAuthorizationCodeClient;
pub use idp::google_auth_code::GoogleAuthorizationCodeClient;
pub use result::OAuthResult;
pub use auth_code_session::AuthorizationCodeSession;
pub use token::OAuthToken;
