//! The Firebase Auth REST API impelemntations.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth).

pub mod change_email;
pub mod change_password;
pub mod confirm_email_verification;
pub mod confirm_password_reset;
pub mod delete_account;
pub mod exchange_custom_token_for_an_id_and_refresh_token;
pub mod exchange_refresh_token;
pub mod fetch_providers_for_email;
pub mod get_user_data;
pub mod link_with_email_password;
pub mod link_with_oauth_credential;
pub mod send_email_verification;
pub mod send_password_reset_email;
pub mod sign_in_anonymously;
pub mod sign_in_with_email_password;
pub mod sign_in_with_oauth_credential;
pub mod sign_up_with_email_password;
pub mod unlink_provider;
pub mod update_profile;
pub mod verify_password_reset_code;
