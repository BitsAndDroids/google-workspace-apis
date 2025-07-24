/// Module for the Google Calendar API interactions.
#[cfg(feature = "calendar")]
pub mod calendar;

/// Module for Google Tasks API interactions.
#[cfg(feature = "tasks")]
pub mod tasks;

/// Module for authentication and authorization
pub mod auth;

/// Helper module for utility functions
pub mod utils;
