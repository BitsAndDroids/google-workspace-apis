use crate::auth::types::GoogleClient;

pub mod default_builder;
pub mod deserialize;
pub mod format;
pub mod request;
pub mod serialize;
pub mod validation;

pub fn get_client_from_env() -> Result<GoogleClient, String> {
    let client = GoogleClient::from_env();
    match client {
        Ok(client) => Ok(client),
        Err(e) => Err(format!("Failed to create GoogleApiClient: {}", e)),
    }
}
