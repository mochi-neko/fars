//! # fars
//! An unofficial Rust client for the Firebase Auth REST API.
//!
//! ## Usages
//! 1. Use APIs via raw interfaces by `fars::api::*`.
//! 2. Use APIs via session-based interfaces by `fars::Config` and `fars::Session`.
//! 3. Verify an ID token by `fars::verification::verify_id_token`.

// public modules
pub mod api;
pub mod data;
pub mod error;
pub mod verification;

// Internal modules
pub(crate) mod client;

// Private modules
mod config;
mod result;
mod session;

// Re-exports
pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::error::VerificationError;
pub use crate::result::Result;
pub use crate::result::VerificationResult;
pub use crate::session::Session;
