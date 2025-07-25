//! This crate provides a Rust API client for interacting with Google Workspace APIs,
//! The main goal is to provide a unified interface for Google Workspace APIs
//! It's build on the reqwest crate
//!
//! The crate includes several examples in the `examples/` directory:
//! - `axum_calendar_example.rs`: Demonstrates how to set up authentication using axum and make basic API calls.
//!   This example requires the `calendar` feature to be enabled. Make sure to add the correct
//!   config fields like client_id, client_secret, and redirect_uri to your `Config` struct.
//! - Run examples with `cargo run --example axum_calendar_example --features calendar`
//!

/// Module for the Google Calendar API interactions.
/// This requires the `calendar` feature to be enabled.
#[cfg(feature = "calendar")]
pub mod calendar;

/// Module for Google Tasks API interactions.
/// This requires the `tasks` feature to be enabled.
#[cfg(feature = "tasks")]
pub mod tasks;

/// Module for authentication and authorization
pub mod auth;

/// Helper module for utility functions
pub mod utils;
