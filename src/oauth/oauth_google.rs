use super::oauth_client::{OAuthClient, OAuthResult, OAuthSession};

pub struct OAuthGoogleClient {
    inner: OAuthClient,
}

impl OAuthGoogleClient {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> OAuthResult<Self> {
        let client = OAuthClient::new(
            client_id,
            client_secret,
            "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            "https://www.googleapis.com/oauth2/v3/token".to_string(),
            redirect_url,
            Some("https://oauth2.googleapis.com/revoke".to_string()),
        )?;

        Ok(Self {
            inner: client,
        })
    }

    pub fn generate_authorization_url(
        &self,
        scopes: Vec<String>,
    ) -> OAuthSession {
        self.inner
            .generate_authorization_url(scopes)
    }
}
