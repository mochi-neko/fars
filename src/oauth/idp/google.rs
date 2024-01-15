use std::collections::HashSet;

use crate::OAuthAuthUrl;
use crate::OAuthClient;
use crate::OAuthClientId;
use crate::OAuthClientSecret;
use crate::OAuthRedirectUrl;
use crate::OAuthResult;
use crate::OAuthRevocationUrl;
use crate::OAuthScope;
use crate::OAuthSession;
use crate::OAuthTokenUrl;

pub struct OAuthGoogleClient {
    inner: OAuthClient,
}

impl OAuthGoogleClient {
    pub fn new(
        client_id: OAuthClientId,
        client_secret: OAuthClientSecret,
        redirect_url: OAuthRedirectUrl,
    ) -> OAuthResult<Self> {
        let client = OAuthClient::new(
            client_id,
            Some(client_secret),
            OAuthAuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth")?,
            Some(OAuthTokenUrl::new(
                "https://www.googleapis.com/oauth2/v3/token",
            )?),
            redirect_url,
            Some(OAuthRevocationUrl::new(
                "https://oauth2.googleapis.com/revoke",
            )?),
        )?;

        Ok(Self {
            inner: client,
        })
    }

    pub fn generate_authorization_url(
        &self,
        scopes: HashSet<OAuthScope>,
    ) -> OAuthSession {
        self.inner
            .generate_authorization_url(scopes)
    }
}
