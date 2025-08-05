use anyhow::{anyhow, Error};
use client::{AccessToken, ClientCredentials};
use scopes::Scope;

pub mod client;
pub mod scopes;

/// Helper function to generate the OAuth URL for Google authentication.
/// # Example:
/// ```
/// pub async fn get_auth_url_workspace() -> String {
///    let google_cfg = CONFIG;
///
///    // Define which scopes you want to request access to.
///    let scopes: Vec<Scope> = vec![
///        Scope::CalendarReadOnly,
///        Scope::CalendarEvents,
///        Scope::TasksReadOnly,
///    ];
///
///    // This will return a simple url string
///    google_workspace_apis::auth::get_oauth_url(
///        google_cfg.google_client_id,
///        google_cfg.google_redirect_uri,
///        scopes,
///    )
///}
///```
/// This returns a URL that can be used to redirect users to Google's OAuth consent screen,
/// The result has to be handled in the web application to capture the authorization code
/// Google will redirect to the specified `redirect_uri` with the authorization code as a query
/// parameter.
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

/// Helper function to retrieve an access token from Google using the authorization code.
/// # Example:
/// ```
/// pub async fn handle_google_oauth_redirect(
///    params: Query<HashMap<String, String>>,
///    State(state): State<AppState>,
///) -> StatusCode {
///    let code = params.get("code").cloned().unwrap_or("".to_string());
///
///    let google_cfg = CONFIG;
///
///    // Makes a call to Google's OAuth2 token endpoint to exchange the authorization code for an
///    access token + refresh token
///    let access_token = google_workspace_apis::auth::get_acces_token(
///        &code,
///        google_cfg.google_client_secret,
///        google_cfg.google_client_id,
///        google_cfg.google_redirect_uri,
///    )
///    .await
///    .unwrap();
///
///    let valid_until =
///        chrono::Utc::now() + chrono::Duration::seconds(access_token.expires_in as i64);
///    //Optional save the refresh token to the database or cache
///    save_refresh_token(
///        &access_token.refresh_token,
///        &access_token.access_token,
///        Some(valid_until),
///    )
///    .expect("Failed to save refresh token");
///
///    let client_credentials = ClientCredentials {
///        redirect_uri: google_cfg.google_redirect_uri.to_string(),
///        client_id: google_cfg.google_client_id.to_string(),
///        client_secret: google_cfg.google_client_secret.to_string(),
///        refresh_token: access_token.refresh_token.clone(),
///    };
///
///    let new_client = GoogleClient::new(client_credentials, access_token);
///    let mut guard = state.google_client.lock().await;
///    *guard = Some(new_client);
///    println!("Google client initialized successfully");
///    StatusCode::OK
///}
///
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

pub async fn refresh_acces_token(
    client_credentials: &ClientCredentials,
) -> Result<AccessToken, anyhow::Error> {
    let url = "https://oauth2.googleapis.com/token";
    let params = [
        ("client_id", client_credentials.client_id.clone()),
        ("client_secret", client_credentials.client_secret.clone()),
        ("refresh_token", client_credentials.refresh_token.clone()),
        ("grant_type", "refresh_token".to_string()),
    ];

    let client = reqwest::Client::new();
    let res = client.post(url).form(&params).send();

    match res.await {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await.unwrap();
                let token = AccessToken {
                    token_type: json["token_type"].as_str().unwrap_or_default().to_string(),
                    access_token: json["access_token"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    expires_in: json["expires_in"].as_i64().unwrap_or(0),
                    refresh_token: client_credentials.refresh_token.clone(),
                    refresh_token_expires_in: 0,
                    scope: json["scope"].as_str().unwrap_or_default().to_string(),
                };
                Ok(token)
            } else {
                Err(anyhow!("Failed to refresh token: {}", response.status()))
            }
        }
        Err(e) => Err(anyhow!("Request error: {e}")),
    }
}
