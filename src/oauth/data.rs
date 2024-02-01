use crate::oauth::OAuthError;
use crate::oauth::OAuthResult;
use std::collections::HashSet;
use std::env::VarError;

/// The PKCE code challenge option.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum PkceOption {
    /// (Recommended) S256 (SHA-256) code challenge method.
    S256,
    /// (Not recommended) Plain code challenge method.
    NotSupported,
}

/// The client ID of the OAuth 2.0.
pub struct ClientId {
    inner: oauth2::ClientId,
}

impl ClientId {
    pub fn new<S>(client_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::ClientId::new(client_id.into()),
        }
    }

    /// Loads from specified environment variable.
    pub fn from_env(key: &str) -> std::result::Result<Self, VarError> {
        let id = std::env::var(key)?;

        Ok(Self::new(id))
    }

    pub(crate) fn inner(&self) -> &oauth2::ClientId {
        &self.inner
    }
}

/// The client secret of the OAuth 2.0.
pub struct ClientSecret {
    inner: oauth2::ClientSecret,
}

impl ClientSecret {
    pub fn new<S>(client_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::ClientSecret::new(client_id.into()),
        }
    }

    /// Loads from specified environment variable.
    pub fn from_env(key: &str) -> std::result::Result<Self, VarError> {
        let secret = std::env::var(key)?;

        Ok(Self::new(secret))
    }

    pub(crate) fn inner(&self) -> &oauth2::ClientSecret {
        &self.inner
    }
}

/// The authorization endpoint of the OAuth 2.0.
#[derive(Clone)]
pub struct AuthorizeEndpoint {
    inner: oauth2::AuthUrl,
}

impl AuthorizeEndpoint {
    pub fn new<S>(url: S) -> OAuthResult<Self>
    where
        S: Into<String> + Clone,
    {
        Ok(Self {
            inner: oauth2::AuthUrl::new(url.clone().into())
                .map_err(|_| OAuthError::InvalidAuthUrl(url.into()))?,
        })
    }

    pub(crate) fn inner(&self) -> &oauth2::AuthUrl {
        &self.inner
    }
}

/// The device endpoint of the OAuth 2.0.
#[derive(Clone)]
pub struct DeviceEndpoint {
    inner: oauth2::DeviceAuthorizationUrl,
}

impl DeviceEndpoint {
    pub fn new<S>(url: S) -> OAuthResult<Self>
    where
        S: Into<String> + Clone,
    {
        Ok(Self {
            inner: oauth2::DeviceAuthorizationUrl::new(url.clone().into())
                .map_err(|_| OAuthError::InvalidAuthUrl(url.into()))?,
        })
    }

    pub(crate) fn inner(&self) -> &oauth2::DeviceAuthorizationUrl {
        &self.inner
    }
}

/// The token endpoint of the OAuth 2.0.
#[derive(Clone)]
pub struct TokenEndpoint {
    inner: oauth2::TokenUrl,
}

impl TokenEndpoint {
    pub fn new<S>(url: S) -> OAuthResult<Self>
    where
        S: Into<String> + Clone,
    {
        Ok(Self {
            inner: oauth2::TokenUrl::new(url.clone().into())
                .map_err(|_| OAuthError::InvalidTokenUrl(url.into()))?,
        })
    }

    pub(crate) fn inner(&self) -> &oauth2::TokenUrl {
        &self.inner
    }
}

/// The redirect URL of the OAuth 2.0.
#[derive(Clone)]
pub struct RedirectUrl {
    inner: oauth2::RedirectUrl,
}

impl RedirectUrl {
    pub fn new<S>(url: S) -> OAuthResult<Self>
    where
        S: Into<String> + Clone,
    {
        Ok(Self {
            inner: oauth2::RedirectUrl::new(url.clone().into())
                .map_err(|_| OAuthError::InvalidRedirectUrl(url.into()))?,
        })
    }

    pub(crate) fn inner(&self) -> &oauth2::RedirectUrl {
        &self.inner
    }
}

/// The scope of the OAuth 2.0.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct OAuthScope {
    inner: oauth2::Scope,
}

impl OAuthScope {
    pub fn new<S>(scope: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::Scope::new(scope.into()),
        }
    }

    pub(crate) fn inner(&self) -> &oauth2::Scope {
        &self.inner
    }

    /// The "openid" scope for the OpenID Connect.
    pub fn open_id() -> Self {
        Self::new("openid")
    }

    /// The "email" scope for the OpenID Connect.
    pub fn open_id_email() -> Self {
        Self::new("email")
    }

    /// The "profile" scope for the OpenID Connect.
    pub fn open_id_profile() -> Self {
        Self::new("profile")
    }

    /// The "offline_access" scope for the OpenID Connect.
    pub fn open_id_offline_access() -> Self {
        Self::new("offline_access")
    }

    /// The "address" scope for the OpenID Connect.
    pub fn open_id_address() -> Self {
        Self::new("address")
    }

    /// The "phone" scope for the OpenID Connect.
    pub fn open_id_phone() -> Self {
        Self::new("phone")
    }
}

/// The authorize request URL of the OAuth 2.0.
#[derive(Clone)]
pub struct AuthorizeUrl {
    inner: String,
}

impl AuthorizeUrl {
    pub(crate) fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: url.into(),
        }
    }

    /// Returns the URL as a string representation.
    pub fn inner(&self) -> &str {
        &self.inner
    }
}

/// The authorization code of the OAuth 2.0.
#[derive(Clone)]
pub struct AuthorizationCode {
    inner: oauth2::AuthorizationCode,
}

impl AuthorizationCode {
    pub fn new<S>(code: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::AuthorizationCode::new(code.into()),
        }
    }

    pub(crate) fn inner(&self) -> &oauth2::AuthorizationCode {
        &self.inner
    }
}

/// The CSRF state of the OAuth 2.0.
pub struct CsrfState {
    inner: String,
}

impl CsrfState {
    pub fn new<S>(state: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: state.into(),
        }
    }

    pub(crate) fn inner(&self) -> &str {
        &self.inner
    }
}

/// The verification URI of the OAuth 2.0 Device Code Grant type.
#[derive(Clone)]
pub struct VerificationUri {
    pub(crate) inner: oauth2::EndUserVerificationUrl,
}

impl VerificationUri {
    pub fn inner(&self) -> &str {
        &self.inner
    }
}

/// The verification URI complete of the OAuth 2.0 Device Code Grant type.
#[derive(Clone)]
pub struct VerificationUriComplete {
    pub(crate) inner: oauth2::VerificationUriComplete,
}

impl VerificationUriComplete {
    pub fn inner(&self) -> &str {
        &self.inner.secret()
    }
}

/// The device code of the OAuth 2.0 Device Code Grant type.
#[derive(Clone)]
pub struct UserCode {
    pub(crate) inner: oauth2::UserCode,
}

impl UserCode {
    pub fn inner(&self) -> &str {
        &self.inner.secret()
    }
}

/// The access token of the OAuth 2.0.
#[derive(Clone)]
pub struct AccessToken {
    inner: String,
}

impl AccessToken {
    pub(crate) fn new<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: token.into(),
        }
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}

/// The refresh token of the OAuth 2.0.
#[derive(Clone)]
pub struct RefreshToken {
    inner: String,
}

impl RefreshToken {
    pub(crate) fn new<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: token.into(),
        }
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}
