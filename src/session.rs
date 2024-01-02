//! Authentication session for a user of the Firebase Auth.

use std::collections::HashSet;

use crate::data::{ProviderId, UserData};
use crate::error::Error;
use crate::result::Result;

/// Authentication session for a user of the Firebase Auth.
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
#[derive(Clone)]
pub struct AuthSession {
    /// HTTP client.
    pub(crate) client: reqwest::Client,
    /// Firebase project API key.
    pub(crate) api_key: String,
    /// Firebase Auth ID token.
    pub id_token: String,
    /// The number of seconds in which the ID token expires.
    #[allow(dead_code)] // NOTE: This field may be used in the future.
    pub(crate) expires_in: u64,
    /// Firebase Auth refresh token.
    pub(crate) refresh_token: String,
}

// Defines macros for calling APIs with refreshing tokens.

/// Calls an API with refreshing tokens then return value with new `AuthSession``.
macro_rules! call_api_with_refreshing_tokens_with_return_value {
    // Has arguments and return value with Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr), *) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call(&auth, $($api_call_args), *).await {
                    Ok(result) => return Ok((auth, result)),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return value with Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr,) => {{
        call_api_with_refreshing_tokens_with_return_value!($auth, $api_call, $retry_count, ())
    }};
}

/// Calls an API with refreshing tokens then return not value with new `AuthSession`.
macro_rules! call_api_with_refreshing_tokens_without_return_value {
    // Has arguments and return only Auth.
    ($auth:expr, $api_call_unit:expr, $retry_count:expr, $($api_call_args:expr), *) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call_unit(&auth, $($api_call_args), *).await {
                    Ok(_) => return Ok(auth),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return only Auth.
    ($auth:expr, $api_call_unit:expr, $retry_count:expr,) => {{
        call_api_with_refreshing_tokens_without_return_value!($auth, $api_call_unit, $retry_count, ())
    }};
}

/// Calls an API with refreshing tokens then return new `AuthSession`.
macro_rules! call_api_with_refreshing_tokens_with_return_auth {
    // Has arguments and return Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr),*) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call(&auth, $($api_call_args),*).await {
                    Ok(new_auth) => return Ok(new_auth),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr) => {{
        call_api_with_refreshing_tokens_with_return_auth!($auth, $api_call, $retry_count, )
    }};
}

/// Calls an API with refreshing tokens then return no `AuthSession`.
macro_rules! call_api_with_refreshing_tokens_without_auth {
    // Has arguments and return no Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr),*) => {{
        async move {
            let mut auth = $auth;
            let mut attempts = 0;
            loop {
                match $api_call(&auth, $($api_call_args),*).await {
                    Ok(_) => return Ok(()),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdTokenError if attempts < $retry_count => {
                            match auth.refresh_tokens().await {
                                Ok(new_auth) => {
                                    auth = new_auth;
                                    attempts += 1;
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        _ => return Err(error),
                    },
                }
            }
        }
    }};

    // Has no arguments and return no Auth.
    ($auth:expr, $api_call:expr, $retry_count:expr) => {{
        call_api_with_refreshing_tokens_without_auth!($auth, $api_call, $retry_count, )
    }};
}

// Implements public API callings for an `AuthSession` with automatic refreshing tokens.
impl AuthSession {
    /// Changes the email for the user.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `new_email` - The new email address of the user.
    /// - `locale` - The optional language code corresponding to the user's locale.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let new_session = session.change_email(
    ///     "new-user@example".to_string(),
    ///     None,
    /// ).await.unwrap();
    ///
    /// // Do something with new_session.
    /// ```
    pub async fn change_email(
        self,
        new_email: String,
        locale: Option<String>,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            AuthSession::change_email_internal,
            1,
            new_email.clone(),
            locale.clone()
        )
        .await
    }

    /// Changes the password for the user.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `new_password` - The new password of the user.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let new_session = session.change_password(
    ///     "new-password".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with new session.
    /// ```
    pub async fn change_password(
        self,
        new_password: String,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            AuthSession::change_password_internal,
            1,
            new_password.clone()
        )
        .await
    }

    /// Updates the user profile information.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `display_name` - The display name for the account.
    /// - `photo_url` - The photo url of the account.
    /// - `delete_attribute` - The attributes that should be deleted from the account.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let new_session = session.update_profile(
    ///     "new-display-name".to_string(),
    ///     "new-photo-url".to_string(),
    ///     Vec::new(),
    /// ).await.unwrap();
    ///
    /// // Do something with new_session.
    /// ```
    pub async fn update_profile(
        self,
        display_name: String,
        photo_url: String,
        delete_attribute: Vec<crate::api::update_profile::DeleteAttribute>,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            AuthSession::update_profile_internal,
            1,
            display_name.clone(),
            photo_url.clone(),
            delete_attribute.clone()
        )
        .await
    }

    /// Gets the user data.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Returns
    /// 1. New session to replace the consumed session.
    /// 2. The user data.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let (new_session, user_data) = session.get_user_data().await.unwrap();
    ///
    /// // Do something with new_session and user_data.
    /// ```
    pub async fn get_user_data(self) -> Result<(AuthSession, UserData)> {
        call_api_with_refreshing_tokens_with_return_value!(
            self,
            AuthSession::get_user_data_internal,
            1,
        )
        .await
    }

    /// Links the user with the given email and password.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to link.
    /// - `password` - The password of the user to link.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    /// use fars::api::sign_in_with_oauth_credential::IdpPostBody;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_oauth_credencial(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-oauth-open-id-token".to_string(),
    ///     },
    /// ).await.unwrap();
    ///
    /// let new_session = session.link_with_email_password(
    ///    "new-user@example".to_string(),
    ///    "new-password".to_string(),
    /// ).await.unwrap();
    ///
    /// // Do something with new_session.
    /// ```
    pub async fn link_with_email_password(
        self,
        email: String,
        password: String,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_with_return_auth!(
            self,
            AuthSession::link_with_email_password_internal,
            1,
            email.clone(),
            password.clone()
        )
        .await
    }

    /// Links the user with the given OAuth credential.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `request_uri` - The URI to which the IDP redirects the user back.
    /// - `post_body` - The POST body passed to the IDP containing the OAuth credential and provider ID.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    /// use fars::data::IdpPostBody;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let new_session = session.link_with_oauth_credential(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-id-token-got-from-google-oauth-api".to_string(),
    ///     },
    /// ).await.unwrap();
    ///
    /// // Do something with new_session.
    /// ```
    pub async fn link_with_oauth_credential(
        self,
        request_uri: String,
        post_body: crate::data::IdpPostBody,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_with_return_auth!(
            self,
            AuthSession::link_with_oauth_credential_internal,
            1,
            request_uri.clone(),
            post_body.clone()
        )
        .await
    }

    /// Unlinks the user with the given provider.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `delete_provider` - The provider IDs to unlink.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    /// use fars::data::ProviderId;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let new_session = session.unlink_provider(
    ///    [ProviderId::Google].iter().cloned().collect(),
    /// ).await.unwrap();
    ///
    /// // Do something with new_session.
    /// ```
    pub async fn unlink_provider(
        self,
        delete_provider: HashSet<ProviderId>,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            AuthSession::unlink_provider_internal,
            1,
            delete_provider.clone()
        )
        .await
    }

    /// Sends an email verification to the user.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `locale` - The optional language code corresponding to the user's locale.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();
    ///
    /// let new_session = session.send_email_verification(
    ///     None,
    /// ).await.unwrap();
    ///
    /// // Do something with new_session.
    /// ```
    pub async fn send_email_verification(
        self,
        locale: Option<String>,
    ) -> Result<AuthSession> {
        call_api_with_refreshing_tokens_without_return_value!(
            self,
            AuthSession::send_email_verification_internal,
            1,
            locale.clone()
        )
        .await
    }

    /// Deletes the user account.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Example
    /// ```
    /// use fars::config::AuthConfig;
    ///
    /// let config = AuthConfig::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await.unwrap();;
    ///
    /// session.delete_account().await.unwrap();
    /// ```
    pub async fn delete_account(self) -> Result<()> {
        call_api_with_refreshing_tokens_without_auth!(
            self,
            AuthSession::delete_account_internal,
            1,
        )
        .await
    }
}

// Implements internal API callings for an `AuthSession`.
impl AuthSession {
    async fn refresh_tokens(self) -> Result<Self> {
        // Create request payload.
        let request_payload = crate::api::exchange_refresh_token::ExchangeRefreshTokenRequestBodyPayload::new(
            self.refresh_token.clone(),
        );

        // Send request.
        let response =
            crate::api::exchange_refresh_token::exchange_refresh_token(
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Create tokens.
        Ok(Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: response.id_token,
            expires_in: response
                .expires_in
                .parse()
                .map_err(|error| Error::NumberParseError {
                    error,
                })?,
            refresh_token: response.refresh_token,
        })
    }

    async fn change_email_internal(
        &self,
        new_email: String,
        locale: Option<String>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::change_email::ChangeEmailRequestBodyPayload::new(
                self.id_token.clone(),
                new_email,
                false,
            );

        // Send request.
        crate::api::change_email::change_email(
            &self.client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }

    async fn change_password_internal(
        &self,
        new_password: String,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::change_password::ChangePasswordRequestBodyPayload::new(
                self.id_token.clone(),
                new_password,
                false,
            );

        // Send request.
        crate::api::change_password::change_password(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn update_profile_internal(
        &self,
        display_name: String,
        photo_url: String,
        delete_attribute: Vec<crate::api::update_profile::DeleteAttribute>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::update_profile::UpdateProfileRequestBodyPayload::new(
                self.id_token.clone(),
                display_name,
                photo_url,
                delete_attribute,
                false,
            );

        // Send request.
        crate::api::update_profile::update_profile(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn get_user_data_internal(&self) -> Result<UserData> {
        // Create request payload.
        let request_payload =
            crate::api::get_user_data::GetUserDataRequestBodyPayload::new(
                self.id_token.clone(),
            );

        // Send request.
        let response = crate::api::get_user_data::get_user_data(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Take the first user from vector.
        let user = response
            .users
            .get(0)
            .ok_or(Error::NotFoundAnyUserData)?;

        Ok(UserData {
            local_id: user.local_id.clone(),
            email: user.email.clone(),
            email_verified: user.email_verified,
            display_name: user.display_name.clone(),
            photo_url: user.photo_url.clone(),
            provider_user_info: user
                .provider_user_info
                .clone(),
            password_hash: user.password_hash.clone(),
            password_updated_at: user
                .password_updated_at
                .clone(),
            valid_since: user.valid_since.clone(),
            disabled: user.disabled.clone(),
            last_login_at: user.last_login_at.clone(),
            created_at: user.created_at.clone(),
            last_refresh_at: user.last_refresh_at.clone(),
            custom_auth: user.custom_auth,
        })
    }

    async fn link_with_email_password_internal(
        &self,
        email: String,
        password: String,
    ) -> Result<Self> {
        // Create request payload.
        let request_payload =
            crate::api::link_with_email_password::LinkWithEmailAndPasswordRequestBodyPayload::new(
                self.id_token.clone(),
                email,
                password,
            );

        // Send request.
        let response_payload =
            crate::api::link_with_email_password::link_with_email_password(
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Update tokens.
        Ok(Self {
            client: self.client.clone(),
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

    async fn link_with_oauth_credential_internal(
        &self,
        request_uri: String,
        post_body: crate::data::IdpPostBody,
    ) -> Result<Self> {
        // Create request payload.
        let request_payload =
            crate::api::link_with_oauth_credential::LinkWithOAuthCredentialRequestBodyPayload::new(
                self.id_token.clone(),
                request_uri,
                post_body,
                false,
            );

        // Send request.
        let response_payload =
            crate::api::link_with_oauth_credential::link_with_oauth_credential(
                &self.client,
                &self.api_key,
                request_payload,
            )
            .await?;

        // Update tokens.
        Ok(Self {
            client: self.client.clone(),
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

    async fn unlink_provider_internal(
        &self,
        delete_provider: HashSet<ProviderId>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::unlink_provider::UnlinkProviderRequestBodyPayload::new(
                self.id_token.clone(),
                delete_provider,
            );

        // Send request.
        crate::api::unlink_provider::unlink_provider(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn send_email_verification_internal(
        &self,
        locale: Option<String>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::send_email_verification::SendEmailVerificationRequestBodyPayload::new(
                self.id_token.clone(),
            );

        // Send request.
        crate::api::send_email_verification::send_email_verification(
            &self.client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }

    async fn delete_account_internal(&self) -> Result<()> {
        // Create request payload.
        let request_payload =
            crate::api::delete_account::DeleteAccountRequestBodyPayload::new(
                self.id_token.clone(),
            );

        // Send request.
        crate::api::delete_account::delete_account(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }
}
