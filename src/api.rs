//! The Firebase Auth REST API impelemntations.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
//!
//! ## NOTE
//! This feature is only available when the feature "raw" is enabled.
//!
//! ## Supported APIs
//!
//! - [x] [Exchange custom token for an ID and refresh token](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token)
//! - [x] [Exchange a refresh token for an ID token](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token)
//! - [x] [Sign up with email / password](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password)
//! - [x] [Sign in with email / password](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password)
//! - [x] [Sign in anonymously](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously)
//! - [x] [Sign in with OAuth credential](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential)
//! - [x] [Fetch providers for email](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email)
//! - [x] [Send password reset email](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email)
//! - [ ] (Not tested) [Verify password reset code](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code)
//! - [ ] (Not tested) [Confirm password reset](https://firebase.google.com/docs/reference/rest/auth#section-confirm-reset-password)
//! - [x] [Change email](https://firebase.google.com/docs/reference/rest/auth#section-change-email)
//! - [x] [Change password](https://firebase.google.com/docs/reference/rest/auth#section-change-password)
//! - [x] [Update profile](https://firebase.google.com/docs/reference/rest/auth#section-update-profile)
//! - [x] [Get user data](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info)
//! - [x] [Link with email/password](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password)
//! - [x] [Link with OAuth credential](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential)
//! - [x] [Unlink provider](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider)
//! - [x] [Send email verification](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification)
//! - [ ] (Not tested) [Confirm email verification](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification)
//! - [x] [Delete account](https://firebase.google.com/docs/reference/rest/auth#section-delete-account)
//!
//! ## NOTE
//! Unsupported APIs have already been implemented but not tested.
//!
//! ## Examples
//!
//! ### Sign up with email / password
//! An example of sign up with email and password with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use fars::api;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a request payload specifying the email and password.
//!     let request_payload = api::SignUpWithEmailPasswordRequestBodyPayload::new(
//!         "user@example.com".to_string(),
//!         "password".to_string(),
//!     );
//!   
//!     // Send a request and receive a response payload.
//!     let response_payload = api::sign_up_with_email_password(
//!         reqwest::Client::new(),
//!         "your-firebase-project-api-key".to_string(),
//!         request_payload,
//!     ).await?;
//!
//!     // Do something with the response payload.
//!     println!("Response payload: {:?}", response_payload);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get user data
//! An example of getting user data with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows;
//!
//! ```rust
//! use fars::api;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a request payload specifying the ID token.
//!     let request_payload = api::GetUserDataRequestBodyPayload::new(
//!         "id-token".to_string(),
//!     );
//!
//!     // Send a request and receive a response payload.
//!     let response_payload = api::get_user_data(
//!         reqwest::Client::new(),
//!         "your-firebase-project-api-key".to_string(),
//!         request_payload,
//!     ).await?;
//!
//!     // Do something with the response payload.
//!     println!("Response payload: {:?}", response_payload);
//!
//!     Ok(())
//! }
//! ```

#![allow(unused_imports)]

// Private modules
mod change_email;
mod change_password;
mod confirm_email_verification;
mod confirm_password_reset;
mod delete_account;
mod exchange_custom_token_for_an_id_and_refresh_token;
mod exchange_refresh_token;
mod fetch_providers_for_email;
mod get_user_data;
mod link_with_email_password;
mod link_with_oauth_credential;
mod send_email_verification;
mod send_password_reset_email;
mod sign_in_anonymously;
mod sign_in_with_email_password;
mod sign_in_with_oauth_credential;
mod sign_up_with_email_password;
mod unlink_provider;
mod update_profile;
mod verify_password_reset_code;

// Re-exports
pub use change_email::change_email;
pub use change_email::ChangeEmailRequestBodyPayload;
pub use change_email::ChangeEmailResponsePayload;
pub use change_password::change_password;
pub use change_password::ChangePasswordRequestBodyPayload;
pub use change_password::ChangePasswordResponsePayload;
pub use confirm_email_verification::confirm_email_verification;
pub use confirm_email_verification::ConfirmEmailVerificationRequestBodyPayload;
pub use confirm_email_verification::ConfirmEmailVerificationResponsePayload;
pub use confirm_password_reset::confirm_password_reset;
pub use confirm_password_reset::ConfirmPasswordResetRequestBodyPayload;
pub use confirm_password_reset::ConfirmPasswordResetResponsePayload;
pub use delete_account::delete_account;
pub use delete_account::DeleteAccountRequestBodyPayload;
pub use delete_account::DeleteAccountResponsePayload;
pub use exchange_custom_token_for_an_id_and_refresh_token::exchange_custom_token_for_an_id_and_refresh_token;
pub use exchange_custom_token_for_an_id_and_refresh_token::ExchangeCustomTokenForAnIdAndRefreshTokenRequestBodyPayload;
pub use exchange_custom_token_for_an_id_and_refresh_token::ExchangeCustomTokenForAnIdAndRefreshTokenResponsePayload;
pub use exchange_refresh_token::exchange_refresh_token;
pub use exchange_refresh_token::ExchangeRefreshTokenRequestBodyPayload;
pub use exchange_refresh_token::ExchangeRefreshTokenResponsePayload;
pub use fetch_providers_for_email::fetch_providers_for_email;
pub use fetch_providers_for_email::FetchProvidersForEmailRequestBodyPayload;
pub use fetch_providers_for_email::FetchProvidersForEmailResponsePayload;
pub use get_user_data::get_user_data;
pub use get_user_data::GetUserDataRequestBodyPayload;
pub use get_user_data::GetUserDataResponsePayload;
pub use link_with_email_password::link_with_email_password;
pub use link_with_email_password::LinkWithEmailPasswordRequestBodyPayload;
pub use link_with_email_password::LinkWithEmailPasswordResponsePayload;
pub use link_with_oauth_credential::link_with_oauth_credential;
pub use link_with_oauth_credential::LinkWithOAuthCredentialRequestBodyPayload;
pub use link_with_oauth_credential::LinkWithOAuthCredentialResponsePayload;
pub use send_email_verification::send_email_verification;
pub use send_email_verification::SendEmailVerificationRequestBodyPayload;
pub use send_email_verification::SendEmailVerificationResponsePayload;
pub use send_password_reset_email::send_password_reset_email;
pub use send_password_reset_email::SendPasswordResetEmailRequestBodyPayload;
pub use send_password_reset_email::SendPasswordResetEmailResponsePayload;
pub use sign_in_anonymously::sign_in_anonymously;
pub use sign_in_anonymously::SignInAnonymouslyRequestBodyPayload;
pub use sign_in_anonymously::SignInAnonymouslyResponsePayload;
pub use sign_in_with_email_password::sign_in_with_email_password;
pub use sign_in_with_email_password::SignInWithEmailPasswordRequestBodyPayload;
pub use sign_in_with_email_password::SignInWithEmailPasswordResponsePayload;
pub use sign_in_with_oauth_credential::sign_in_with_oauth_credential;
pub use sign_in_with_oauth_credential::SignInWithOAuthCredentialRequestBodyPayload;
pub use sign_in_with_oauth_credential::SignInWithOAuthCredentialResponsePayload;
pub use sign_up_with_email_password::sign_up_with_email_password;
pub use sign_up_with_email_password::SignUpWithEmailPasswordRequestBodyPayload;
pub use sign_up_with_email_password::SignUpWithEmailPasswordResponsePayload;
pub use unlink_provider::unlink_provider;
pub use unlink_provider::UnlinkProviderRequestBodyPayload;
pub use unlink_provider::UnlinkProviderResponsePayload;
pub use update_profile::update_profile;
pub use update_profile::UpdateProfileRequestBodyPayload;
pub use update_profile::UpdateProfileResponsePayload;
pub use verify_password_reset_code::verify_password_reset_code;
pub use verify_password_reset_code::VerifyPasswordResetCodeRequestBodyPayload;
pub use verify_password_reset_code::VerifyPasswordResetCodeResponsePayload;
