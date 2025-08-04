use anyhow::{anyhow, Error};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::refresh_acces_token;

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
pub struct ClientTokenData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub access_token: String,

    #[serde(default)]
    pub expires_on: chrono::DateTime<chrono::Utc>,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub refresh_token: String,
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
    pub access_token: Option<ClientTokenData>,
    pub req_client: reqwest::Client,
    pub auto_refresh_token: bool,
}

impl From<AccessToken> for ClientTokenData {
    fn from(token: AccessToken) -> Self {
        Self {
            access_token: token.access_token,
            expires_on: chrono::Utc::now() + chrono::Duration::seconds(token.expires_in),
            refresh_token: token.refresh_token,
        }
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
            access_token: Some(access_token.into()),
            req_client: client,
            auto_refresh_token: false,
        }
    }

    pub async fn refresh_acces_token_check(&mut self) -> Result<(), Error> {
        if self.auto_refresh_token && !self.is_access_token_valid() {
            match self.update_access_token().await {
                Ok(_) => return Ok(()),
                Err(e) => return Err(anyhow!(e)),
            }
        }
        Ok(())
    }

    pub fn enable_auto_refresh(&mut self) {
        self.auto_refresh_token = true;
    }

    pub fn is_access_token_valid(&self) -> bool {
        if let Some(token_data) = &self.access_token {
            return chrono::Utc::now() < token_data.expires_on;
        }
        false
    }

    pub async fn update_access_token(&mut self) -> Result<(), Error> {
        let new_token = refresh_acces_token(&self.client_credentials).await.unwrap();
        self.access_token = Some(new_token.into());
        Ok(())
    }
}
