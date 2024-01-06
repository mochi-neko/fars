//! Configuration for the Firebase Auth.
//!
//! ## Features
//! 1. Provides a session ([`crate::Session`]) via sigining in (or equivalent) methods.
//! 2. Provides APIs that do not require any ID token.
//!
//! About APIs that require an ID token, see [`crate::session`].
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
//! - [ ] (Not implemented) Apple (`apple.com`)
//! - [ ] (Not implemented) Apple Game Center (`gc.apple.com`)
//! - [ ] (Not tested) Facebook (`facebook.com`)
//! - [ ] (Not implemented) GitHub (`github.com`)
//! - [x] Google (`google.com`)
//! - [ ] (Not implemented) Google Play Games (`playgames.google.com`)
//! - [ ] (Not implemented) LinkedIn (`linkedin.com`)
//! - [ ] (Not implemented) Microsoft (`microsoft.com`)
//! - [ ] (Not tested) Twitter (`twitter.com`)
//! - [ ] (Not implemented) Yahoo (`yahoo.com`)
//! - [ ] (Not implemented) custom
//!
//! Unsupported providers have either not been tested or the format of [`crate::data::IdpPostBody`] is not documented at the [official API reference](https://firebase.google.com/docs/reference/rest/auth).
//!
//! ## Examples
//!
//! ### Sign in with email / password
//! An example of sign in with email and password with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use fars::Config;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a config.
//!     let config = Config::new(
//!         "your-firebase-project-api-key".to_string(),
//!     );
//!
//!     // Get a session by signing in with email and password.
//!     let session = config.sign_in_with_email_password(
//!         "user@example".to_string(),
//!         "password".to_string(),
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
//! use fars::Config;
//! use fars::data::IdpPostBody;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a config.
//!     let config = Config::new(
//!         "your-firebase-project-api-key".to_string(),
//!     );
//!
//!     // Get a credential for Google OAuth by any method.
//!     let google_open_id_token = "user-google-oauth-open-id-token".to_string();
//!
//!     // Get a session by signing in with Google OAuth credential.
//!     let session = config.sign_in_with_oauth_credential(
//!         "https://your-app.com/redirect/path/auth/handler".to_string(),
//!         IdpPostBody::Google {
//!             id_token: google_open_id_token,
//!         },
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
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a config.
//!     let config = Config::new(
//!         "your-firebase-project-api-key".to_string(),
//!     );
//!
//!     // Send reset password email to specified email.
//!     config.send_reset_password_email(
//!         "user@example".to_string(),
//!     ).await?;
//!
//!     // Do something with the resutl.
//!     println!("Succeeded to send reset password email");
//!
//!     Ok(())
//! }
//! ```

use crate::api;
use crate::data::IdpPostBody;
use crate::data::ProviderId;
use crate::Error;
use crate::Result;
use crate::Session;

/// Configuration for the Firebase Auth.
///
/// ## Example
/// ```
/// use fars::Config;
///
/// let config = Config::new(
///     "your-firebase-project-api-key".to_string(),
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Config {
    /// Firebase project API key.
    api_key: String,
}

impl Config {
    /// ## Arguments
    /// - `api_key` - Your Firebase project API key.
    ///
    /// ## Example
    /// ```
    /// use fars::Config;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    /// ```
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
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
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_up_with_email_password(
    ///     "user@example".to_string(),
    ///     "password".to_string(),
    /// ).await?;
    /// ```
    pub async fn sign_up_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<Session> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload =
            api::SignUpWithEmailPasswordRequestBodyPayload::new(
                email, password,
            );

        // Send request.
        let response_payload = api::sign_up_with_email_password(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::ParseExpriesInFailed {
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
    /// ```
    pub async fn sign_in_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<Session> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload =
            api::SignInWithEmailPasswordRequestBodyPayload::new(
                email, password,
            );

        // Send request.
        let response_payload = api::sign_in_with_email_password(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::ParseExpriesInFailed {
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
    /// let session = config.sign_in_anonymously().await?;
    /// ```
    pub async fn sign_in_anonymously(&self) -> Result<Session> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload = api::SignInAnonymouslyRequestBodyPayload::new();

        // Send request.
        let response_payload =
            api::sign_in_anonymously(&client, &self.api_key, request_payload)
                .await?;

        // Create session.
        Ok(Session {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::ParseExpriesInFailed {
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
    /// use fars::data::IdpPostBody;
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let session = config.sign_in_with_oauth_credential(
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    ///     IdpPostBody::Google {
    ///         id_token: "user-google-oauth-open-id-token".to_string(),
    ///     },
    /// ).await?;
    /// ```
    pub async fn sign_in_with_oauth_credential(
        &self,
        request_uri: String,
        post_body: IdpPostBody,
    ) -> Result<Session> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload =
            api::SignInWithOAuthCredentialRequestBodyPayload::new(
                request_uri,
                post_body,
                false,
            );

        // Send request.
        let response_payload = api::sign_in_with_oauth_credential(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::ParseExpriesInFailed {
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
    /// let session = config.exchange_refresh_token(
    ///     "user-firebase-refresh-token".to_string(),
    /// ).await?;
    /// ```
    pub async fn exchange_refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<Session> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload =
            api::ExchangeRefreshTokenRequestBodyPayload::new(refresh_token);

        // Send request.
        let response_payload = api::exchange_refresh_token(
            &client,
            &self.api_key,
            request_payload,
        )
        .await?;

        // Create session.
        Ok(Session {
            client,
            api_key: self.api_key.clone(),
            id_token: response_payload.id_token,
            expires_in: response_payload
                .expires_in
                .parse()
                .map_err(|error| Error::ParseExpriesInFailed {
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
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// let providers = config.fetch_providers_for_email(
    ///     "user@example".to_string(),
    ///     "https://your-app.com/redirect/path/auth/handler".to_string(),
    /// ).await?;
    /// ```
    pub async fn fetch_providers_for_email(
        &self,
        email: String,
        continue_uri: String,
    ) -> Result<Option<Vec<ProviderId>>> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload =
            api::FetchProvidersForEmailRequestBodyPayload::new(
                email,
                continue_uri,
            );

        // Send request.
        let response_payload = api::fetch_providers_for_email(
            &client,
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
    ///
    /// let config = Config::new(
    ///     "your-firebase-project-api-key".to_string(),
    /// );
    ///
    /// config.send_reset_password_email(
    ///     "user@example".to_string(),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn send_reset_password_email(
        &self,
        email: String,
        locale: Option<String>,
    ) -> Result<()> {
        // Create a HTTP client.
        let client = reqwest::Client::new();

        // Create request payload.
        let request_payload =
            api::SendPasswordResetEmailRequestBodyPayload::new(email);

        // Send request.
        api::send_password_reset_email(
            &client,
            &self.api_key,
            request_payload,
            locale,
        )
        .await?;

        Ok(())
    }
}
