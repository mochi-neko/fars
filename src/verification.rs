//! Verification of an ID token of the Firebase Auth.
//!
//! See also [document](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library).
//!
//! ## NOTE
//! This feature is only available when the feature `verify` is enabled.
//!
//! ## Examples
//! An example of ID token verification with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:
//!
//! ```rust
//! use fars::verification::VerificationConfig;
//! use fars::ProjectId;
//! use fars::IdToken;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a verification config.
//!     let config = VerificationConfig::new(
//!         ProjectId::new("firebase-project-id"),
//!     );
//!
//!     // Verify the ID token.
//!     match config.verify_id_token(
//!         &IdToken::new("id-token"),
//!     ).await {
//!         Ok(claims) => {
//!             // Verification succeeded.
//!             println!("Token ID verification succeeded: {:?}", claims);
//!         },
//!         Err(error) => {
//!             // Verification failed.
//!             eprintln!("Token ID verification failed: {:?}", error);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod id_token_payload_claim;
mod verification_config;
mod verification_error;
mod verification_result;

pub use id_token_payload_claim::IdTokenPayloadClaims;
pub use verification_config::VerificationConfig;
pub use verification_error::VerificationError;
pub use verification_result::VerificationResult;
