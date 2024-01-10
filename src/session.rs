//! Authentication session for a user of the Firebase Auth.
//!
//! ## Features
//! Provides APIs that require an ID token.
//!
//! A session ([`crate::Session`]) is provided by a siging in method of config ([`crate::Config`]).
//!
//! See also [`crate::config`].  
//!
//! ## NOTE
//! ID token in a session ([`crate::Session`]) has expiration date.
//!
//! API calling through a session ([`crate::Session`]) automatically refresh an ID token by the [refresh token API](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token) when the ID token has been expired.
//!
//! All APIs through session cosume session and return new session that has same ID token or refreshed one except for the [delete account API](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
//!
//! Therefore you have to **update** session every time you use APIs through a session by returned new session.
//!
//! ## Supported APIs
//! Supported APIs are as follows:
//!
//! - [Change email](`crate::Session::change_email`)
//! - [Change password](`crate::Session::change_password`)
//! - [Update profile](`crate::Session::update_profile`)
//! - [Delete profile](`crate::Session::delete_profile`)
//! - [Get user data](`crate::Session::get_user_data`)
//! - [Link with email and password](`crate::Session::link_with_email_password`)
//! - [Link with OAuth credential](`crate::Session::link_with_oauth_credential`)
//! - [Unlink provider](`crate::Session::unlink_provider`)
//! - [Send email verification](`crate::Session::send_email_verification`)
//! - [Delete account](`crate::Session::delete_account`)
//! - [Refresh token](`crate::Session::refresh_token`)
//!
//! ## Examples
//! An example to get user data through a session with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use fars::Config;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create config.
//!     let config = Config::new(
//!         "your-firebase-project-api-key".to_string(),
//!     );
//!
//!     // Sign in with email and password.
//!     let session = config.sign_in_with_email_password(
//!         "user@example".to_string(),
//!         "password".to_string(),
//!     ).await?;
//!
//!     // Get user data.
//!     let (new_session, user_data) = session.get_user_data().await?;
//!     
//!     // Do something with user data.
//!     println!("User data: {:?}", user_data);
//!
//!     Ok(())
//! }
//! ```

use std::collections::HashSet;

use crate::api;
use crate::ApiKey;
use crate::Client;
use crate::DeleteAttribute;
use crate::DisplayName;
use crate::Email;
use crate::Error;
use crate::ExpiresIn;
use crate::IdToken;
use crate::IdpPostBody;
use crate::LanguageCode;
use crate::OAuthRequestUri;
use crate::Password;
use crate::PhotoUrl;
use crate::ProviderId;
use crate::RefreshToken;
use crate::Result;
use crate::UserData;

/// Authentication session for a user of the Firebase Auth.
///
/// ## Example
/// ```
/// use fars::Config;
///
/// let config = Config::new(
///     "your-firebase-project-api-key".to_string(),
/// );
///
/// let session = config.sign_in_with_email_password(
///     "user@example".to_string(),
///     "password".to_string(),
/// ).await?;
/// ```
#[derive(Clone, Debug)]
pub struct Session {
    /// HTTP client.
    pub(crate) client: Client,
    /// Firebase project API key.
    pub(crate) api_key: ApiKey,
    /// Firebase Auth ID token.
    pub id_token: IdToken,
    /// The number of seconds in which the ID token expires.
    pub expires_in: ExpiresIn,
    /// Firebase Auth refresh token.
    pub refresh_token: RefreshToken,
}

// Defines macros for calling APIs with refreshing tokens.

/// Calls an API with refreshing tokens then returns new session and value.
macro_rules! call_refreshing_tokens_return_session_and_value {
    // Has arguments and returns new session and value.
    ($session:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr), *) => {{
        async move {
            let mut session = $session;
            let mut attempts = 0;
            loop {
                match $api_call(&session, $($api_call_args), *).await {
                    Ok(value) => return Ok((session, value)),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdToken if attempts < $retry_count => {
                            match session.refresh_token().await {
                                Ok(new_session) => {
                                    session = new_session;
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

    // Has no arguments and returns new session and value.
    ($session:expr, $api_call:expr, $retry_count:expr,) => {{
        call_refreshing_tokens_return_session_and_value!($session, $api_call, $retry_count, ())
    }};
}

/// Calls an API with refreshing tokens without value then returns new session.
macro_rules! call_refreshing_tokens_without_value_return_session {
    // Has arguments and returns new session.
    ($session:expr, $api_call_unit:expr, $retry_count:expr, $($api_call_args:expr), *) => {{
        async move {
            let mut session = $session;
            let mut attempts = 0;
            loop {
                match $api_call_unit(&session, $($api_call_args), *).await {
                    Ok(_) => return Ok(session),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdToken if attempts < $retry_count => {
                            match session.refresh_token().await {
                                Ok(new_session) => {
                                    session = new_session;
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

    // Has no arguments and returns new session.
    ($session:expr, $api_call_unit:expr, $retry_count:expr,) => {{
        call_refreshing_tokens_without_value_return_session!($session, $api_call_unit, $retry_count, ())
    }};
}

/// Calls an API with refreshing tokens then returns new session.
#[allow(unused_macros)]
macro_rules! call_refreshing_tokens_return_session {
    // Has arguments and returns new session.
    ($session:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr),*) => {{
        async move {
            let mut session = $session;
            let mut attempts = 0;
            loop {
                match $api_call(&session, $($api_call_args),*).await {
                    Ok(new_session) => return Ok(new_session),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdToken if attempts < $retry_count => {
                            match session.refresh_token().await {
                                Ok(new_session) => {
                                    session = new_session;
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

    // Has no arguments and returns new session.
    ($session:expr, $api_call:expr, $retry_count:expr) => {{
        call_refreshing_tokens_return_session!($session, $api_call, $retry_count, )
    }};
}

/// Calls an API with refreshing tokens then returns nothing.
macro_rules! call_refreshing_tokens_return_nothing {
    // Has arguments and returns nothing.
    ($session:expr, $api_call:expr, $retry_count:expr, $($api_call_args:expr),*) => {{
        async move {
            let mut session = $session;
            let mut attempts = 0;
            loop {
                match $api_call(&session, $($api_call_args),*).await {
                    Ok(_) => return Ok(()),
                    Err(error) => match error {
                        // NOTE: Retry for invalid ID token error.
                        Error::InvalidIdToken if attempts < $retry_count => {
                            match session.refresh_token().await {
                                Ok(new_session) => {
                                    session = new_session;
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

    // Has no arguments and returns nothing.
    ($session:expr, $api_call:expr, $retry_count:expr) => {{
        call_refreshing_tokens_return_nothing!($session, $api_call, $retry_count, )
    }};
}

// Implements public API callings for an `Session` with automatic refreshing tokens.
impl Session {
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
    /// ## Errors
    /// - `Error::InvalidHeaderValue` - Invalid header value.
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.change_email(
    ///     "new-user@example".to_string(),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn change_email(
        self,
        new_email: Email,
        locale: Option<LanguageCode>,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::change_email_internal,
            1,
            new_email.clone(),
            locale
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
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    /// - `Error::ParseExpriesInFailed` - Failed to parse the expires in value.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.change_password(
    ///     "new-password".to_string(),
    /// ).await?;
    /// ```
    pub async fn change_password(
        self,
        new_password: Password,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::change_password_internal,
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
    /// - `display_name` - (Optional) The display name for the account.
    /// - `photo_url` - (Optional) The photo url of the account.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.update_profile(
    ///     "new-display-name".to_string(),
    ///     "new-photo-url".to_string(),
    /// ).await?;
    /// ```
    pub async fn update_profile(
        self,
        display_name: Option<DisplayName>,
        photo_url: Option<PhotoUrl>,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::update_profile_internal,
            1,
            display_name.clone(),
            photo_url.clone()
        )
        .await
    }

    /// Deletes the user profile information.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Arguments
    /// - `delete_attribute` - The attributes that should be deleted from the account.
    ///
    /// ## Returns
    /// New session to replace the consumed session.
    ///
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::data::DeleteAttribute;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.delete_profile(
    ///     [DeleteAttribute::DisplayName, DeleteAttribute::PhotoUrl]
    ///         .iter()
    ///         .cloned()
    ///         .collect(),
    /// ).await?;
    /// ```
    pub async fn delete_profile(
        self,
        delete_attribute: HashSet<DeleteAttribute>,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::delete_profile_internal,
            1,
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
    /// ## Errors
    /// - `Error::InvalidHeaderValue` - Invalid header value.
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    /// - `Error::NotFoundAnyUserData` - Not found any user data.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let (new_session, user_data) = session.get_user_data().await?;
    /// ```
    pub async fn get_user_data(self) -> Result<(Session, UserData)> {
        call_refreshing_tokens_return_session_and_value!(
            self,
            Session::get_user_data_internal,
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
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    /// - `Error::ParseExpriesInFailed` - Failed to parse the expires in value.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::data::IdpPostBody;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_oauth_credencial(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-oauth-open-id-token".to_string(),
    ///     },
    /// ).await?;
    ///
    /// let new_session = session.link_with_email_password(
    ///    "new-user@example".to_string(),
    ///    "new-password".to_string(),
    /// ).await?;
    /// ```
    pub async fn link_with_email_password(
        self,
        email: Email,
        password: Password,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::link_with_email_password_internal,
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
    /// ## Errors
    /// - `Error::InvalidHeaderValue` - Invalid header value.
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    /// - `Error::ParseExpriesInFailed` - Failed to parse the expires in value.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::data::IdpPostBody;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.link_with_oauth_credential(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-id-token-got-from-google-oauth-api".to_string(),
    ///     },
    /// ).await?;
    /// ```
    pub async fn link_with_oauth_credential(
        self,
        request_uri: OAuthRequestUri,
        post_body: IdpPostBody,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::link_with_oauth_credential_internal,
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
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::data::ProviderId;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.unlink_provider(
    ///    [ProviderId::Google].iter().cloned().collect(),
    /// ).await?;
    /// ```
    pub async fn unlink_provider(
        self,
        delete_provider: HashSet<ProviderId>,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::unlink_provider_internal,
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
    /// ## Errors
    /// - `Error::InvalidHeaderValue` - Invalid header value.
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// let new_session = session.send_email_verification(
    ///     None,
    /// ).await?;
    /// ```
    pub async fn send_email_verification(
        self,
        locale: Option<LanguageCode>,
    ) -> Result<Session> {
        call_refreshing_tokens_without_value_return_session!(
            self,
            Session::send_email_verification_internal,
            1,
            locale
        )
        .await
    }

    /// Deletes the user account.
    ///
    /// Automatically refreshes tokens if needed.
    ///
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::InvalidIdToken` - Invalid ID token.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// session.delete_account().await?;
    /// ```
    pub async fn delete_account(self) -> Result<()> {
        call_refreshing_tokens_return_nothing!(
            self,
            Session::delete_account_internal,
            1,
        )
        .await
    }

    /// Refreshes the ID token.
    ///
    /// See also [API reference](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token).
    ///
    /// ## Returns
    /// New session with refreshed ID token.
    ///
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    /// - `Error::ParseExpriesInFailed` - Failed to parse the expires in value.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_in_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    ///
    /// // Expire the ID token.
    ///
    /// let new_session = session.refresh_token().await?;
    /// ```
    pub async fn refresh_token(self) -> Result<Self> {
        // Create request payload.
        let request_payload = api::ExchangeRefreshTokenRequestBodyPayload::new(
            self.refresh_token.inner,
        );

        // Send request.
        let response_payload = api::exchange_refresh_token(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create tokens.
        Ok(Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
        })
    }
}

// Implements internal API callings for an `Session`.
impl Session {
    async fn change_email_internal(
        &self,
        new_email: Email,
        locale: Option<LanguageCode>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload = api::ChangeEmailRequestBodyPayload::new(
            self.id_token.inner.clone(),
            new_email.inner,
            false,
        );

        // Send request.
        api::change_email(
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
        new_password: Password,
    ) -> Result<()> {
        // Create request payload.
        let request_payload = api::ChangePasswordRequestBodyPayload::new(
            self.id_token.inner.clone(),
            new_password.inner,
            false,
        );

        // Send request.
        api::change_password(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn update_profile_internal(
        &self,
        display_name: Option<DisplayName>,
        photo_url: Option<PhotoUrl>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload = api::UpdateProfileRequestBodyPayload::new(
            self.id_token.inner.clone(),
            display_name.map(|display_name| display_name.inner.clone()),
            photo_url.map(|photo_url| photo_url.inner.clone()),
            None,
            false,
        );

        // Send request.
        api::update_profile(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn delete_profile_internal(
        &self,
        delete_attribute: HashSet<DeleteAttribute>,
    ) -> Result<()> {
        // Format delete attributes.
        let delete_attribute = delete_attribute
            .iter()
            .copied()
            .collect();

        // Create request payload.
        let request_payload = api::UpdateProfileRequestBodyPayload::new(
            self.id_token.inner.clone(),
            None,
            None,
            Some(delete_attribute),
            false,
        );

        // Send request.
        api::update_profile(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn get_user_data_internal(&self) -> Result<UserData> {
        // Create request payload.
        let request_payload = api::GetUserDataRequestBodyPayload::new(
            self.id_token.inner.clone(),
        );

        // Send request.
        let response_payload = api::get_user_data(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Take the first user from vector.
        let user = response_payload
            .users
            .first()
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
            password_updated_at: user.password_updated_at,
            valid_since: user.valid_since.clone(),
            disabled: user.disabled,
            last_login_at: user.last_login_at.clone(),
            created_at: user.created_at.clone(),
            last_refresh_at: user.last_refresh_at.clone(),
            custom_auth: user.custom_auth,
        })
    }

    async fn link_with_email_password_internal(
        &self,
        email: Email,
        password: Password,
    ) -> Result<Self> {
        // Create request payload.
        let request_payload = api::LinkWithEmailPasswordRequestBodyPayload::new(
            self.id_token.inner.clone(),
            email.inner,
            password.inner,
        );

        // Send request.
        let response_payload = api::link_with_email_password(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Update tokens.
        Ok(Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
        })
    }

    async fn link_with_oauth_credential_internal(
        &self,
        request_uri: OAuthRequestUri,
        post_body: IdpPostBody,
    ) -> Result<Self> {
        // Create request payload.
        let request_payload =
            api::LinkWithOAuthCredentialRequestBodyPayload::new(
                self.id_token.inner.clone(),
                request_uri.inner,
                post_body,
                false,
            );

        // Send request.
        let response_payload = api::link_with_oauth_credential(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Update tokens.
        Ok(Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
        })
    }

    async fn unlink_provider_internal(
        &self,
        delete_provider: HashSet<ProviderId>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload = api::UnlinkProviderRequestBodyPayload::new(
            self.id_token.inner.clone(),
            delete_provider,
        );

        // Send request.
        api::unlink_provider(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }

    async fn send_email_verification_internal(
        &self,
        locale: Option<LanguageCode>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload = api::SendEmailVerificationRequestBodyPayload::new(
            self.id_token.inner.clone(),
        );

        // Send request.
        api::send_email_verification(
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
        let request_payload = api::DeleteAccountRequestBodyPayload::new(
            self.id_token.inner.clone(),
        );

        // Send request.
        api::delete_account(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        Ok(())
    }
}
