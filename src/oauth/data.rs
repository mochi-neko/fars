use crate::OAuthError;
use crate::OAuthResult;

pub struct OAuthClientId {
    inner: oauth2::ClientId,
}

impl OAuthClientId {
    pub fn new<S>(client_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::ClientId::new(client_id.into()),
        }
    }

    pub(crate) fn inner(&self) -> &oauth2::ClientId {
        &self.inner
    }
}
pub struct OAuthClientSecret {
    inner: oauth2::ClientSecret,
}

impl OAuthClientSecret {
    pub fn new<S>(client_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::ClientSecret::new(client_id.into()),
        }
    }

    pub(crate) fn inner(&self) -> &oauth2::ClientSecret {
        &self.inner
    }
}

pub struct OAuthAuthUrl {
    inner: oauth2::AuthUrl,
}

impl OAuthAuthUrl {
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

pub struct OAuthTokenUrl {
    inner: oauth2::TokenUrl,
}

impl OAuthTokenUrl {
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

pub struct OAuthRedirectUrl {
    inner: oauth2::RedirectUrl,
}

impl OAuthRedirectUrl {
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

pub struct OAuthRevocationUrl {
    inner: oauth2::RevocationUrl,
}

impl OAuthRevocationUrl {
    pub fn new<S>(url: S) -> OAuthResult<Self>
    where
        S: Into<String> + Clone,
    {
        Ok(Self {
            inner: oauth2::RevocationUrl::new(url.clone().into())
                .map_err(|_| OAuthError::InvalidRevocationUrl(url.into()))?,
        })
    }

    pub(crate) fn inner(&self) -> &oauth2::RevocationUrl {
        &self.inner
    }
}

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
}

pub struct OAuthAuthorizeUrl {
    inner: String,
}

impl OAuthAuthorizeUrl {
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: url.into(),
        }
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}

pub struct OAuthAuthorizationCode {
    inner: oauth2::AuthorizationCode,
}

impl OAuthAuthorizationCode {
    pub fn new<S>(code: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: oauth2::AuthorizationCode::new(code.into()),
        }
    }

    pub fn inner(&self) -> &oauth2::AuthorizationCode {
        &self.inner
    }
}

pub struct OAuthAuthorizationState {
    inner: String,
}

impl OAuthAuthorizationState {
    pub fn new<S>(state: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: state.into(),
        }
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}

pub struct OAuthAccessToken {
    inner: String,
}

impl OAuthAccessToken {
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

pub struct OAuthRefreshToken {
    inner: String,
}

impl OAuthRefreshToken {
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
