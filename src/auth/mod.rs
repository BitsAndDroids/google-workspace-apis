use anyhow::Error;
use scopes::Scope;
use types::{AccessToken, ClientCredentials, GoogleClient};

pub mod scopes;
pub mod types;

pub fn get_oauth_url(client_id: &str, redirect_uri: &str, scopes: Vec<Scope>) -> String {
    let base_url = "https://accounts.google.com/o/oauth2/auth";
    format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        base_url,
        client_id,
        redirect_uri,
        scopes
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    )
}

pub async fn get_acces_token(
    code: &str,
    client_secret: &str,
    client_id: &str,
    redirect_uri: &str,
) -> Result<AccessToken, Error> {
    let url = "https://oauth2.googleapis.com/token";
    let params = [
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let client = reqwest::Client::new();
    let res = client.post(url).form(&params).send().await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await?;
                Ok(
                    serde_json::from_value(json.clone()).unwrap_or_else(|_| AccessToken {
                        token_type: json["token_type"].as_str().unwrap_or_default().to_string(),
                        access_token: json["access_token"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        expires_in: json["expires_in"].as_i64().unwrap_or(0),
                        refresh_token: json["refresh_token"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        refresh_token_expires_in: json["x_refresh_token_expires_in"]
                            .as_i64()
                            .unwrap_or(0),
                        scope: json["scope"].as_str().unwrap_or_default().to_string(),
                    }),
                )
            } else {
                Err(anyhow::anyhow!(
                    "Failed to retrieve access token: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub async fn refresh_acces_token(client_credentials: ClientCredentials) -> Result<String, String> {
    let url = "https://oauth2.googleapis.com/token";
    let params = [
        ("client_id", client_credentials.client_id),
        ("client_secret", client_credentials.client_secret),
        ("refresh_token", client_credentials.refresh_token),
        ("grant_type", "refresh_token".to_string()),
    ];

    let client = reqwest::Client::new();
    let res = client.post(url).form(&params).send();

    match res.await {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await.unwrap();
                Ok(json["access_token"].as_str().unwrap().to_string())
            } else {
                Err(format!("Failed to refresh token: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Request error: {e}")),
    }
}

pub async fn get_google_client(
    token: AccessToken,
    client_credentials: ClientCredentials,
) -> Result<GoogleClient, anyhow::Error> {
    let client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token.access_token).parse()?,
            );
            headers
        })
        .build()?;
    Ok(GoogleClient {
        client_credentials,
        client,
    })
}
