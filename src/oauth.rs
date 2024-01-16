//! Provides OAuth 2.0 client.
//!
//! ## NOTE
//!
//!
//! ## Supported ID providers
//!
//!
//! ## Examples
//!
//!

mod client;
mod data;
mod error;
mod idp;
mod result;
mod session;
mod token;

pub use client::OAuthClient;
pub use data::OAuthAccessToken;
pub use data::OAuthAuthUrl;
pub use data::OAuthAuthorizationCode;
pub use data::OAuthAuthorizationState;
pub use data::OAuthAuthorizeUrl;
pub use data::OAuthClientId;
pub use data::OAuthClientSecret;
pub use data::OAuthCodeChallengeOption;
pub use data::OAuthRedirectUrl;
pub use data::OAuthRefreshToken;
pub use data::OAuthRevocationUrl;
pub use data::OAuthScope;
pub use data::OAuthTokenUrl;
pub use error::OAuthError;
pub use idp::github::OAuthGitHubClient;
pub use idp::google::OAuthGoogleClient;
pub use result::OAuthResult;
pub use session::OAuthSession;
pub use token::OAuthToken;
