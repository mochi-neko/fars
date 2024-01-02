//! # fars
//! An unofficial Rust client for the Firebase Auth REST API.
//!
//! ## Usages
//! 1. Use APIs directry by `fars::api::*`.
//! 2. Use APIs via session-based interface by `fars::config::AuthConfig` and `fars::session::AuthSession`.

pub mod api;
pub mod config;
pub mod data;
pub mod error;
pub mod result;
pub mod session;

pub(crate) mod client;
