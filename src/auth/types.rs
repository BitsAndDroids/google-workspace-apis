use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub token_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub scope: String,
}

#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct ClientCredentials {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub client_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub client_secret: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub redirect_uri: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Default)]
pub struct GoogleClient {
    pub client_credentials: ClientCredentials,
    pub access_token: Option<AccessToken>,
    pub client: reqwest::Client,
}

impl GoogleClient {
    pub fn from_env() -> Result<Self, String> {
        println!("Attempting to create GoogleClient from environment variables");
        let client_credentials = ClientCredentials {
            client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            redirect_uri: std::env::var("GOOGLE_REDIRECT_URI").unwrap_or_default(),
            refresh_token: std::env::var("GOOGLE_REFRESH_TOKEN").unwrap_or_default(),
        };

        let access_token = AccessToken {
            token_type: "Bearer".to_string(),
            access_token: std::env::var("GOOGLE_ACCESS_TOKEN").unwrap_or_default(),
            expires_in: 3600, // Default to 1 hour
            refresh_token: client_credentials.refresh_token.clone(),
            refresh_token_expires_in: 3600, // Default to 1 hour
            scope: "https://www.googleapis.com/auth/userinfo.email".to_string(), // Example scope
        };

        if client_credentials.client_id.is_empty() || client_credentials.client_secret.is_empty() {
            return Err(
                "Missing required environment variables for Google API credentials".to_string(),
            );
        }

        Ok(Self::new(client_credentials, access_token))
    }
}

impl GoogleClient {
    pub fn new(client_credentials: ClientCredentials, access_token: AccessToken) -> Self {
        println!("Creating GoogleClient with provided credentials and access token");
        println!(
            "Client ID: {}, Access Token: {}",
            client_credentials.client_id, access_token.access_token
        );
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token.access_token)
                .parse()
                .unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client_credentials,
            access_token: Some(access_token),
            client,
        }
    }

    #[cfg(feature = "fs")]
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let data = serde_json::json!({
            "client_credentials": self.client_credentials,
            "access_token": self.access_token,
        });

        let serialized = serde_json::to_string_pretty(&data)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    #[cfg(feature = "fs")]
    // Add new method to load credentials from a file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data: serde_json::Value = serde_json::from_str(&contents)?;

        let client_credentials: ClientCredentials =
            serde_json::from_value(data["client_credentials"].clone())?;
        let access_token: AccessToken = serde_json::from_value(data["access_token"].clone())?;

        Ok(Self::new(client_credentials, access_token))
    }
}
