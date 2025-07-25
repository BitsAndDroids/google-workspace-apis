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
