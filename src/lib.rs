//! # fars
//! An unofficial Rust client for the Firebase Auth REST API.
//!
//! ## Usages
//! 1. Use APIs via raw interfaces by `fars::api::*`.
//! 2. Use APIs via session-based interfaces by `fars::Config` and `fars::Session`.
//! 3. (Optional) Verify an ID token by `fars::verification::verify_id_token` in feature "verify".

// public modules
pub mod api;
pub mod data;
pub mod error;

// Internal modules
pub(crate) mod client;

// Private modules
mod config;
mod result;
mod session;

// Re-exports
pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::result::Result;
pub use crate::session::Session;

// API dependent re-exports
pub use reqwest;

// Feature "verify"
#[cfg(feature = "verify")]
pub mod verification;
