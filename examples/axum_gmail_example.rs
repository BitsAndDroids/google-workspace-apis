/// This example demonstrates how to use the `google_workspace_apis` crate
/// with Axum as a web server.
/// In the Google cloud developer portal make sure to create a new application and add all the
/// required API's.
/// For more informatieon on how to set up the Google Cloud project visit [the getting started page of the Workspace API](https://developers.google.com/workspace/guides/get-started).
///
/// Make sure to do it in this order:
/// - Start the server
/// - Navigate to localhost:8080/api/v1/google/auth
/// - Go to the url in your browser
/// - Authorize the application
/// - Your token is now stored in the GoogleClient in the server state
/// - Navigate to localhost:8080/api/v1/google/calendar/events
/// - See your upcomming events
///  
use google_workspace_apis::{
    gmail::{helpers::MessageInput, requests::GmailClient, types::Draft},
    utils::request::ApiError,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, sync::Arc};
use tokio::sync::Mutex;

use axum::{
    extract::{Query, State},
    Json, Router,
};
use google_workspace_apis::auth::{
    client::{ClientCredentials, GoogleClient},
    scopes::Scope,
};
use reqwest::StatusCode;

#[derive(Clone)]
pub struct AppState {
    pub google_client: Arc<Mutex<Option<GoogleClient>>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    // We use this to reuse the same client over multiple requests
    let state = AppState {
        google_client: Arc::new(Mutex::new(None)),
    };
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .nest("/api/v1/google/", google_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct StoredToken {
    pub refresh_token: String,
    pub access_token: String,
    pub valid_until: Option<String>,
}

pub fn save_refresh_token(
    token: &str,
    acces_token: &str,
    valid_until: Option<chrono::DateTime<chrono::Utc>>,
) -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    let data = StoredToken {
        refresh_token: token.to_string(),
        access_token: acces_token.to_string(),
        valid_until: valid_until.map(|time| time.to_string()),
    };
    let json = serde_json::to_string_pretty(&data)?;
    fs::write("token.json", json)?;
    Ok(())
}

pub fn load_refresh_token() -> std::io::Result<StoredToken> {
    let json = fs::read_to_string("token.json")?;
    let data: StoredToken = serde_json::from_str(&json)?;
    Ok(data)
}

pub async fn get_auth_url_workspace() -> String {
    let scopes: Vec<Scope> = vec![Scope::MailReadonly, Scope::MailModify];

    google_workspace_apis::auth::get_oauth_url(
        dotenvy::var("google_client_id").unwrap().as_str(),
        dotenvy::var("google_redirect_uri").unwrap().as_str(),
        scopes,
    )
}

pub async fn handle_google_oauth_redirect(
    params: Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> StatusCode {
    dotenvy::dotenv().unwrap();
    let code = params.get("code").cloned().unwrap_or("".to_string());

    let access_token = google_workspace_apis::auth::get_acces_token(
        &code,
        dotenvy::var("google_client_secret").unwrap().as_str(),
        dotenvy::var("google_client_id").unwrap().as_str(),
        dotenvy::var("google_redirect_uri").unwrap().as_str(),
    )
    .await
    .unwrap();

    let valid_until =
        chrono::Utc::now() + chrono::Duration::seconds(access_token.expires_in as i64);
    save_refresh_token(
        &access_token.refresh_token,
        &access_token.access_token,
        Some(valid_until),
    )
    .expect("Failed to save refresh token");

    let client_credentials = ClientCredentials {
        redirect_uri: dotenvy::var("google_redirect_uri").unwrap(),
        client_id: dotenvy::var("google_client_id").unwrap(),
        client_secret: dotenvy::var("google_client_secret").unwrap(),
        refresh_token: access_token.refresh_token.clone(),
    };

    let new_client = GoogleClient::new(client_credentials, access_token, true);
    let mut guard = state.google_client.lock().await;
    *guard = Some(new_client);
    println!("Client stored");
    StatusCode::OK
}

async fn get_drafts(State(state): State<AppState>) -> Json<Vec<Draft>> {
    // Create the request builder and immediately drop the lock
    let mut google_client_guard = state.google_client.lock().await;
    let client = google_client_guard.as_mut().unwrap();
    let drafts = GmailClient::new(client)
        .list_drafts("me")
        .request()
        .await
        .unwrap();
    println!("{}", drafts.result.clone().result_size_estimate);

    Json(drafts.result.drafts)
}

async fn create_draft(State(state): State<AppState>) {
    let mut google_client_guard = state.google_client.lock().await;
    let client = google_client_guard.as_mut().unwrap();
    let draft: MessageInput = MessageInput {
        from: "daveriedel93@gmail.com".to_string(),
        to: "test@gmail.com".to_string(),
        subject: "test subject".to_string(),
        body: "test body".to_string(),
        attachments: Vec::new(),
    };
    let res: Result<Draft, ApiError> = match GmailClient::new(client)
        .create_draft("me", draft)
        .request()
        .await
    {
        Ok(res) => Ok(res.result),
        Err(err) => {
            println!("failed {}", err);
            Err(err)
        }
    };
}

pub fn google_router() -> Router<AppState> {
    Router::new()
        // .route("/gmail/email", axum::routing::get(get_emails))
        .route("/gmail/drafts/create", axum::routing::get(create_draft))
        // .route("/calendar/drafts/send", axum::routing::get(send_draft))
        .route("/gmail/drafts", axum::routing::get(get_drafts))
        .route("/auth", axum::routing::get(get_auth_url_workspace))
        .route(
            "/oauth2callback",
            axum::routing::get(handle_google_oauth_redirect),
        )
}
