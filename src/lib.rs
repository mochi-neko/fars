//! # fars
//! An unofficial Rust client for the Firebase Auth REST API.
//!
//! ## Features
//! - default
//!     -  APIs via session-based interfaces. See [`crate::config`] and [`crate::session`].
//! - (Optional) raw
//!     - APIs via raw interfaces. See [`crate::api`].
//! - (Optional) verify
//!     - ID token verification. See [`crate::verification`].

// public modules
pub mod config;
pub mod data;
pub mod error;
pub mod session;

// Internal modules
pub(crate) mod client;

// Private modules
mod result;

// Re-exports
pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::result::Result;
pub use crate::session::Session;

// Feature "raw"
#[cfg(feature = "raw")]
pub mod api;
#[cfg(not(feature = "raw"))]
pub(crate) mod api;
#[cfg(feature = "raw")]
pub use reqwest; // Re-export reqwest for the feature "raw" because raw APIs depend on reqwest.

// Feature "verify"
#[cfg(feature = "verify")]
pub mod verification;
