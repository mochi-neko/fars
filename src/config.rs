//! Configuration for the Firebase Auth.
//!
//! ## Features
//! 1. Provides a session ([`crate::Session`]) via sigining in (or equivalent) methods.
//! 2. Provides APIs that do not require any ID token.
//!
//! See aslo [`crate::session`] for APIs that require an ID token.
//!
//! ## 1. Siging in methods
//! Supported sigining in methods are as follows:
//!
//! - [Sign up with email and password](`crate::Config::sign_up_with_email_password`)
//! - [Sign in with email and password](`crate::Config::sign_in_with_email_password`)
//! - [Sign in with OAuth credential](`crate::Config::sign_in_with_oauth_credential`)
//! - [Sign in anounymously](`crate::Config::sign_in_anonymously`)
//! - [Exchange a refresh token to an ID token](`crate::Config::exchange_refresh_token`)
//!
//! ## 2. Supported APIs that do not require an ID token
//! Supported APIs that do not require an ID token are as follows:
//!
//! - [Fetch providers for email](`crate::Config::fetch_providers_for_email`)
//! - [Send password reset email](`crate::Config::send_reset_password_email`)
//!
//! ## Supported OAuth ID providers
//! Supported OAuth ID provides are as follows:
//!
//! - [ ] (Not tested) Apple (`apple.com`)
//! - [ ] (Not tested) Apple Game Center (`gc.apple.com`)
//! - [x] Facebook (`facebook.com`)
//! - [x] GitHub (`github.com`)
//! - [x] Google (`google.com`)
//! - [ ] (Not tested) Google Play Games (`playgames.google.com`)
//! - [ ] (Not tested) Microsoft (`microsoft.com`)
//! - [ ] (Not tested) Twitter (`twitter.com`)
//! - [ ] (Not tested) Yahoo (`yahoo.com`)
//! - [ ] (Not tested) Custom (`{custom-provider-id}`)
//!
//! See also [`crate::oauth]` (optional feature: `oauth`) that provides OAuth client methods to get an OAuth access token.
//!
//! ## Examples
//!
//! ### Sign in with email / password
//! An example of sign in with email and password with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use fars::Config;
//! use fars::ApiKey;
//! use fars::Email;
//! use fars::Password;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a config.
//!     let config = Config::new(
//!         ApiKey::new("your-firebase-project-api-key"),
//!     );
//!
//!     // Get a session by signing in with email and password.
//!     let session = config.sign_in_with_email_password(
//!         Email::new("user@example"),
//!         Password::new("password"),
//!     ).await?;
//!
//!     // Do something with the session.
//!     println!(
//!         "Succeeded to sign in with email and password: {:?}",
//!         session
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Sign in with Google OAuth credential
//! An example of sign in with Google OAuth credential with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use std::collections::HashMap;
//! use fars::Config;
//! use fars::ApiKey;
//! use fars::OAuthRequestUri;
//! use fars::IdpPostBody;
//! use fars::ProviderId;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a config.
//!     let config = Config::new(
//!         ApiKey::new("your-firebase-project-api-key"),
//!     );
//!
//!     // Get a credential for Google OAuth by any method.
//!     let google_open_id_token = "user-google-oauth-open-id-token".to_string();
//!
//!     // Get a session by signing in with Google OAuth credential.
//!     let session = config.sign_in_with_oauth_credential(
//!         OAuthRequestUri::new("https://your-app.com/redirect/path/auth/handler"),
//!         IdpPostBody::new(
//!             ProviderId::Google,
//!             HashMap::from([(
//!                 "access_token",
//!                 "google-access-token".to_string(),
//!             )]),
//!         )?,
//!     ).await?;
//!
//!     // Do something with the session.
//!     println!(
//!         "Succeeded to sign in with Google OAuth credential: {:?}",
//!         session
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Send password reset email
//! An example of sending password reset email with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use fars::Config;
//! use fars::ApiKey;
//! use fars::Email;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a config.
//!     let config = Config::new(
//!         ApiKey::new("your-firebase-project-api-key"),
//!     );
//!
//!     // Send reset password email to specified email.
//!     config.send_reset_password_email(
//!         Email::new("user@example"),
//!     ).await?;
//!
//!     // Do something with the resutl.
//!     println!("Succeeded to send reset password email");
//!
//!     Ok(())
//! }
//! ```

use crate::api;
use crate::ApiKey;
use crate::Client;
use crate::Email;
use crate::ExpiresIn;
use crate::IdToken;
use crate::IdpPostBody;
use crate::LanguageCode;
use crate::OAuthContinueUri;
use crate::OAuthRequestUri;
use crate::Password;
use crate::ProviderId;
use crate::RefreshToken;
use crate::Result;
use crate::Session;

/// Configuration for the Firebase Auth.
///
/// ## Example
/// ```
/// use fars::Config;
/// use fars::ApiKey;
///
/// let config = Config::new(
///     ApiKey::new("your-firebase-project-api-key"),
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Config {
    /// Firebase project API key.
    api_key: ApiKey,
    /// A HTTP client.
    client: Client,
}

impl Config {
    /// Creates a new config.
    ///
    /// ## Arguments
    /// - `api_key` - Your Firebase project API key.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::ApiKey;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    /// ```
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    /// Creates a new config with a custom HTTP client.
    ///
    /// ## NOTE
    /// This method requires the `custom_client` feature.
    ///
    /// ## Arguments
    /// - `api_key` - Your Firebase project API key.
    /// - `client` - A custom HTTP client.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::ApiKey;
    /// use std::time::Duration;
    ///
    /// // Create a custom reqwest client with timeout.
    /// let client = fars::reqwest::ClientBuilder::new()
    ///     .timeout(Duration::from_secs(60))
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .build()?;
    ///
    /// // Create a custom config.
    /// let config = Config::custom(
    ///     ApiKey::new("your-firebase-project-api-key"),
    ///     Client::custom(client),
    /// );
    /// ```
    #[cfg(feature = "custom_client")]
    pub fn custom(
        api_key: ApiKey,
        client: Client,
    ) -> Self {
        Self {
            api_key,
            client,
        }
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
    /// use fars::ApiKey;
    /// use fars::Email;
    /// use fars::Password;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// let session = config.sign_up_with_email_password(
    ///     Email::new("user@example"),
    ///     Password::new("password"),
    /// ).await?;
    /// ```
    pub async fn sign_up_with_email_password(
        &self,
        email: Email,
        password: Password,
    ) -> Result<Session> {
        // Create request payload.
        let request_payload =
            api::SignUpWithEmailPasswordRequestBodyPayload::new(
                email.inner().to_string(),
                password.inner().to_string(),
            );

        // Send request.
        let response_payload = api::sign_up_with_email_password(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
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
    /// use fars::ApiKey;
    /// use fars::Email;
    /// use fars::Password;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// let session = config.sign_in_with_email_password(
    ///     Email::new("user@example"),
    ///     Password::new("password"),
    /// ).await?;
    /// ```
    pub async fn sign_in_with_email_password(
        &self,
        email: Email,
        password: Password,
    ) -> Result<Session> {
        // Create request payload.
        let request_payload =
            api::SignInWithEmailPasswordRequestBodyPayload::new(
                email.inner().to_string(),
                password.inner().to_string(),
            );

        // Send request.
        let response_payload = api::sign_in_with_email_password(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
        })
    }

    /// Signs in as an anonymous user.
    ///
    /// ## Returns
    /// The session for the signed in user.
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
    /// use fars::ApiKey;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// let session = config.sign_in_anonymously().await?;
    /// ```
    pub async fn sign_in_anonymously(&self) -> Result<Session> {
        // Create request payload.
        let request_payload = api::SignInAnonymouslyRequestBodyPayload::new();

        // Send request.
        let response_payload = api::sign_in_anonymously(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
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
    /// use std::collections::HashMap;
    /// use fars::Config;
    /// use fars::ApiKey;
    /// use fars::OAuthRequestUri;
    /// use fars::IdpPostBody;
    /// use fars::ProviderId;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// let session = config.sign_in_with_oauth_credential(
    ///     OAuthRequestUri::new("https://your-app.com/redirect/path/auth/handler"),
    ///     IdpPostBody::new(
    ///         ProviderId::Google,
    ///         HashMap::from([(
    ///             "access_token",
    ///             "google-access-token".to_string(),
    ///         )]),
    ///     )?,
    /// ).await?;
    /// ```
    pub async fn sign_in_with_oauth_credential(
        &self,
        request_uri: OAuthRequestUri,
        post_body: IdpPostBody,
    ) -> Result<Session> {
        // Create request payload.
        let request_payload =
            api::SignInWithOAuthCredentialRequestBodyPayload::new(
                request_uri
                    .inner()
                    .to_string(),
                post_body,
                false,
            );

        // Send request.
        let response_payload = api::sign_in_with_oauth_credential(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
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
    /// use fars::ApiKey;
    /// use fars::RefreshToken;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// let session = config.exchange_refresh_token(
    ///     RefreshToken::new("user-firebase-refresh-token"),
    /// ).await?;
    /// ```
    pub async fn exchange_refresh_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<Session> {
        // Create request payload.
        let request_payload = api::ExchangeRefreshTokenRequestBodyPayload::new(
            refresh_token
                .inner()
                .to_string(),
        );

        // Send request.
        let response_payload = api::exchange_refresh_token(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            id_token: IdToken::new(response_payload.id_token),
            expires_in: ExpiresIn::parse(response_payload.expires_in)?,
            refresh_token: RefreshToken::new(response_payload.refresh_token),
        })
    }

    /// Fetches the list of all IDPs for the specified email.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to fetch providers.
    /// - `continue_uri` - The URI to which the IDP redirects the user back.
    ///
    /// ## Returns
    /// - None - The email address is not registered or protected. See also the [issue](https://github.com/firebase/firebase-ios-sdk/issues/11810).
    /// - Some - The list of all IDPs for the specified email if the email is registered and not protected.
    ///
    /// ## Errors
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::ApiKey;
    /// use fars::Email;
    /// use fars::OAuthContinueUri;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// let providers = config.fetch_providers_for_email(
    ///     Email::new("user@example"),
    ///     OAuthContinueUri::new("https://your-app.com/current/path"),
    /// ).await?;
    /// ```
    pub async fn fetch_providers_for_email(
        &self,
        email: Email,
        continue_uri: OAuthContinueUri,
    ) -> Result<Option<Vec<ProviderId>>> {
        // Create request payload.
        let request_payload =
            api::FetchProvidersForEmailRequestBodyPayload::new(
                email.inner().to_string(),
                continue_uri
                    .inner()
                    .to_string(),
            );

        // Send request.
        let response_payload = api::fetch_providers_for_email(
            &self.client,
            &self.api_key,
            request_payload,
        )
        .await?;

        match response_payload.all_providers {
            | None => Ok(None),
            | Some(providers) => {
                // Parse provider IDs from string to `ProviderId`.
                let provider_ids = providers
                    .iter()
                    .map(|provider_id| ProviderId::parse(provider_id.clone()))
                    .collect();

                Ok(Some(provider_ids))
            },
        }
    }

    /// Sends a password reset email to the given email address.
    ///
    /// ## Arguments
    /// - `email` - The email of the user to send password reset email.
    /// - `locale` - The optional language code corresponding to the user's locale.
    ///
    /// ## Errors
    /// - `Error::InvalidHeaderValue` - Invalid header value.
    /// - `Error::HttpRequestError` - Failed to send a request.
    /// - `Error::ReadResponseTextFailed` - Failed to read the response body as text.
    /// - `Error::DeserializeResponseJsonFailed` - Failed to deserialize the response body as JSON.
    /// - `Error::DeserializeErrorResponseJsonFailed` - Failed to deserialize the error response body as JSON.
    /// - `Error::ApiError` - API error on the Firebase Auth.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    /// use fars::ApiKey;
    /// use fars::Email;
    ///
    /// let config = Config::new(
    ///     ApiKey::new("your-firebase-project-api-key"),
    /// );
    ///
    /// config.send_reset_password_email(
    ///     Email::new("user@example".),
    ///     None, // locale
    /// ).await?;
    /// ```
    pub async fn send_reset_password_email(
        &self,
        email: Email,
        locale: Option<LanguageCode>,
    ) -> Result<()> {
        // Create request payload.
        let request_payload =
            api::SendPasswordResetEmailRequestBodyPayload::new(
                email.inner().to_string(),
            );

        // Send request.
        api::send_password_reset_email(
            &self.client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }
}
