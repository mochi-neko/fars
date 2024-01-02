//! Configuration for the Firebase Auth.

use crate::data::IdpPostBody;
use crate::error::Error;
use crate::result::Result;
use crate::session::AuthSession;

/// Configuration for the Firebase Auth.
#[derive(Clone)]
pub struct AuthConfig {
    /// Firebase project API key.
    api_key: String,
}

impl AuthConfig {
    /// Creates a new [`AuthConfig`] instance.
    ///
    /// ## Arguments
    /// - `api_key` - Your Firebase project API key.
    ///
    /// ## Returns
    /// The [`AuthConfig`] instance.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// // Do something with config.
    /// ```
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
        }
    }

    /// Builds a new HTTP client from config.
    fn build_client(&self) -> Result<reqwest::Client> {
        // NOTE: Timeout options are not supported on WASM.
        reqwest::ClientBuilder::new()
            .build()
            .map_err(|error| Error::HttpClientBuildError(error))
    }

    /// Signs up a new user with the given email and password.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to sign up.
    /// - `password` - The password of the user to sign up.
    ///
    /// ## Returns
    /// The session for the signed up user.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_up_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_up_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_up_with_email_password::SignUpWithEmailPasswordRequestBodyPayload::new(email, password);

        // Send request.
        let response_payload =
        crate::api::sign_up_with_email_password::sign_up_with_email_password(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        })
    }

    /// Signs in a user with the given email and password.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to sign in.
    /// - `password` - The password of the user to sign in.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_in_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_in_with_email_password::SignInWithEmailPasswordRequestBodyPayload::new(email, password);

        // Send request.
        let response_payload =
        crate::api::sign_in_with_email_password::sign_in_with_email_password(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        })
    }

    /// Signs in as an anonymous user.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_in_anonymously().await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_in_anonymously(&self) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_in_anonymously::SignInAnonymouslyRequestBodyPayload::new();

        // Send request.
        let response_payload =
            crate::api::sign_in_anonymously::sign_in_anonymously(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        })
    }

    /// Signs in a user with the given OAuth credential.
    ///
    /// ## Arguments
    /// - `request_uri` - The URI to which the IDP redirects the user back.
    /// - `post_body` - The POST body passed to the IDP containing the OAuth credential and provider ID.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    /// use fars::data::IdpPostBody;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_in_oauth_credencial(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-oauth-open-id-token".to_string(),
    ///     },
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn sign_in_oauth_credencial(
        &self,
        request_uri: String,
        post_body: IdpPostBody,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::sign_in_with_oauth_credential::SignInWithOAuthCredentialRequestBodyPayload::new(
                request_uri,
                post_body,
                false,
            );

        // Send request.
        let response_payload =
            crate::api::sign_in_with_oauth_credential::sign_in_with_oauth_credential(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        })
    }

    /// Exchanges a refresh token for an ID token and new refresh token.
    ///
    /// ## Arguments
    /// - `refresh_token` - A Firebase Auth refresh token.
    ///
    /// ## Returns
    /// The session for the signed in user.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.exchange_refresh_tokens(
    ///     "user-firebase-refresh-token".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with session.
    /// ```
    pub async fn exchange_refresh_tokens(
        &self,
        refresh_token: String,
    ) -> Result<AuthSession> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload = crate::api::exchange_refresh_token::ExchangeRefreshTokenRequestBodyPayload::new(
            refresh_token,
        );

        // Send request.
        let response_payload =
            crate::api::exchange_refresh_token::exchange_refresh_token(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create session.
        Ok(AuthSession {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response_payload.refresh_token,
        })
    }

    /// Fetches the list of all IDPs for the specified email.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to fetch providers.
    /// - `continue_uri` - The URI to which the IDP redirects the user back.
    ///
    /// ## Returns
    /// The list of all IDPs for the specified email.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let providers = config.fetch_providers_for_email(
    ///     "user@example".to_string(),
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with providers.
    /// ```
    pub async fn fetch_providers_for_email(
        &self,
        email: String,
        continue_uri: String,
    ) -> Result<Vec<String>> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
        crate::api::fetch_providers_for_email::FetchProvidersForEmailRequestBodyPayload::new(
            email,
            continue_uri,
        );

        // Send request.
        let response_payload =
            crate::api::fetch_providers_for_email::fetch_providers_for_email(
                &client,
                &self.api_key,
                request_payload,
            )
            .await?;

        Ok(response_payload.all_providers)
    }

    /// Sends a password reset email to the given email address.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to send password reset email.
    /// - `locale` - The optional language code corresponding to the user's locale.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// config.send_reset_password_email(
    ///     "user@example".to_string(),
    ///     None,
    /// ).await.unwrap();
    ///
    /// // Do something.
    /// ```
    pub async fn send_reset_password_email(
        &self,
        email: String,
        locale: Option<String>,
    ) -> Result<()> {
        // Create a HTTP client.
        let client = self.build_client()?;

        // Create request payload.
        let request_payload =
            crate::api::send_password_reset_email::SendPasswordResetEmailRequestBodyPayload::new(email);

        // Send request.
        crate::api::send_password_reset_email::send_password_reset_email(
            &client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }
}
