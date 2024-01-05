//! # fars
//! An unofficial Rust client for the Firebase Auth REST API.
//!
//! ## Features
//! 1. APIs via session-based interfaces. See [`crate::Config`] and [`crate::Session`].
//! 2. (Optional) APIs via raw interfaces in the future "raw". See [`crate::api`].
//! 3. (Optional) ID token verification in the feature "verify". See [`crate::verification`].

// public modules
pub mod data;
pub mod error;

// Internal modules
pub(crate) mod api;
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

// Feature "raw"
#[cfg(feature = "raw")]
pub mod api;
#[cfg(feature = "raw")]
pub use reqwest; // Re-export reqwest for the feature "raw" because raw APIs depend on reqwest.

// Feature "verify"
#[cfg(feature = "verify")]
pub mod verification;
