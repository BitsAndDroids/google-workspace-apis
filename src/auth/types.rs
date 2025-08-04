use std::sync::Arc;

use anyhow::Error;
use chrono::{DateTime, Utc};
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

#[derive(Clone, Default)]
pub struct GoogleClient {
    pub client_credentials: ClientCredentials,
    pub access_token: Option<ClientTokenData>,
    pub req_client: reqwest::Client,
    pub auto_refresh_token: bool,
    refresh_handlers: Vec<Arc<dyn TokenRefreshHandler>>,
}

impl std::fmt::Debug for GoogleClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoogleClient")
            .field("access_token", &self.access_token)
            .field("refresh_token", &"[REDACTED]")
            .field(
                "token_expiry",
                &self.access_token.as_ref().unwrap().expires_on,
            )
            .field("client_id", &self.client_credentials.client_id)
            .field("client_secret", &"[REDACTED]")
            .field(
                "refresh_handlers",
                &format!("[{} handlers]", self.refresh_handlers.len()),
            )
            .finish()
    }
}

impl From<AccessToken> for ClientTokenData {
    fn from(token: AccessToken) -> Self {
        let now = chrono::Utc::now();
        let new_expires_on = now + chrono::Duration::seconds(token.expires_in);
        println!("NEW FROM EXPIRES ON {new_expires_on}");
        Self {
            access_token: token.access_token,
            expires_on: new_expires_on,
            refresh_token: token.refresh_token,
        }
    }
}

impl GoogleClient {
    pub fn new(
        client_credentials: ClientCredentials,
        access_token: AccessToken,
        auto_refresh_token: bool,
    ) -> Self {
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
            auto_refresh_token,
            refresh_handlers: Vec::new(),
        }
    }

    pub fn add_token_refresh_handler<H>(&mut self, handler: H)
    where
        H: TokenRefreshHandler + 'static,
    {
        self.refresh_handlers.push(Arc::new(handler));
    }

    // Modify your refresh_access_token_check method to notify handlers
    pub async fn refresh_acces_token_check(&mut self) -> Result<(), Error> {
        if self.auto_refresh_token && !self.is_access_token_valid() {
            println!(
                "expired on {}",
                self.access_token.as_ref().unwrap().expires_on
            );
            self.update_access_token().await?;
        }
        Ok(())
    }

    pub fn enable_auto_refresh(&mut self) {
        self.auto_refresh_token = true;
    }

    pub fn disable_auto_refresh(&mut self) {
        self.auto_refresh_token = false;
    }

    pub fn is_access_token_valid(&self) -> bool {
        if let Some(token_data) = &self.access_token {
            let now = chrono::Utc::now();
            println!(
                "Token validation: now={}, expires_on={}, valid={}",
                now,
                token_data.expires_on,
                now < token_data.expires_on
            );
            return now < token_data.expires_on;
        }
        false
    }

    pub async fn update_access_token(&mut self) -> Result<(), Error> {
        let new_token = refresh_acces_token(&self.client_credentials).await?;
        println!("new token {}", new_token.access_token);
        self.access_token = Some(new_token.into());
        println!(
            "new expires on {}",
            self.access_token.as_ref().unwrap().expires_on
        );

        for handler in &mut self.refresh_handlers {
            handler.on_token_refresh(
                self.access_token.as_ref().unwrap().access_token.clone(),
                self.access_token.as_ref().unwrap().refresh_token.clone(),
                self.access_token.as_ref().unwrap().expires_on,
            );
        }
        Ok(())
    }
}

pub trait TokenRefreshHandler: Send + Sync {
    fn on_token_refresh(&self, new_token: String, refresh_token: String, new_expiry: DateTime<Utc>);
}

// Implement for Fn closures
impl<F> TokenRefreshHandler for F
where
    F: Fn(String, String, DateTime<Utc>) + Send + Sync,
{
    fn on_token_refresh(
        &self,
        new_token: String,
        refresh_token: String,
        new_expiry: DateTime<Utc>,
    ) {
        self(new_token, refresh_token, new_expiry);
    }
}
