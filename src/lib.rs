//! # fars
//! An unofficial Rust client for the Firebase Auth REST API.
//!
//! ## Usages
//! 1. Use APIs directry by `fars::api::*`.
//! 2. Use APIs via session-based interface by `fars::Config` and `fars::Session`.

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
