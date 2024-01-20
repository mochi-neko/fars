use std::collections::HashSet;

use crate::oauth::AuthorizeEndpoint;
use crate::oauth::AuthorizationCodeClient;
use crate::oauth::ClientId;
use crate::oauth::ClientSecret;
use crate::oauth::PkceOption;
use crate::oauth::RedirectUrl;
use crate::oauth::OAuthResult;
use crate::oauth::Scope;
use crate::oauth::AuthorizationCodeSession;
use crate::oauth::TokenEndpoint;

/// The OAuth client for Facebook.
///
/// ## NOTE
/// This is only available when the feature "oauth" is enabled.
///
/// ## Example
pub struct OAuthFacebookClient {
    inner: AuthorizationCodeClient,
}

impl OAuthFacebookClient {
    /// Creates a new OAuth client for Facebook.
    ///
    /// ## Arguments
    /// - `client_id` - Client ID of.
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> OAuthResult<Self> {
        let client = AuthorizationCodeClient::new(
            client_id,
            Some(client_secret),
            AuthorizeEndpoint::new("https://www.facebook.com/v18.0/dialog/oauth")?,
            Some(TokenEndpoint::new(
                "https://graph.facebook.com/v18.0/oauth/access_token",
            )?),
            redirect_url,
            PkceOption::S256, // https://developers.facebook.com/docs/facebook-login/guides/advanced/oidc-token
        )?;

        Ok(Self {
            inner: client,
        })
    }

    ///
    /// https://developers.facebook.com/docs/facebook-login/guides/permissions
    pub fn generate_authorization_session(
        &self,
        scopes: HashSet<Scope>,
    ) -> AuthorizationCodeSession {
        self.inner
            .generate_session(scopes)
    }
}
