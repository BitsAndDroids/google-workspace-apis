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
use google_workspace_apis::calendar::{events::requests::CalendarEventsClient, prelude::*};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use axum::{
    Json, Router,
    extract::{Query, State},
};
use google_workspace_apis::{
    auth::{
        scopes::Scope,
        types::{ClientCredentials, GoogleClient},
    },
    calendar::events::types::Event,
};
use reqwest::StatusCode;

#[derive(Clone)]
pub struct AppState {
    pub google_client: Arc<Mutex<Option<GoogleClient>>>,
}

#[tokio::main]
async fn main() {
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

pub struct Config {
    google_client_id: &'static str,
    google_client_secret: &'static str,
    google_redirect_uri: &'static str,
}

pub async fn get_auth_url_workspace() -> String {
    let google_cfg = Config {
        google_client_id: "",
        google_client_secret: "",
        google_redirect_uri: "http://localhost:8080/api/v1/google/oauth2/redirect",
    };

    let scopes: Vec<Scope> = vec![
        Scope::CalendarReadOnly,
        Scope::CalendarEvents,
        Scope::TasksReadOnly,
    ];

    google_workspace_apis::auth::get_oauth_url(
        google_cfg.google_client_id,
        google_cfg.google_redirect_uri,
        scopes,
    )
}

pub async fn handle_google_oauth_redirect(
    params: Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> StatusCode {
    let code = params.get("code").cloned().unwrap_or("".to_string());

    //Load this config from settings using cfg-toml for example
    //Make sure to add these fields before running the example
    let google_cfg = Config {
        google_client_id: "",
        google_client_secret: "",
        google_redirect_uri: "http://localhost:8080/api/v1/google/oauth2/redirect",
    };

    let access_token = google_workspace_apis::auth::get_acces_token(
        &code,
        google_cfg.google_client_secret,
        google_cfg.google_client_id,
        google_cfg.google_redirect_uri,
    )
    .await
    .unwrap();

    let client_credentials = ClientCredentials {
        redirect_uri: google_cfg.google_redirect_uri.to_string(),
        client_id: google_cfg.google_client_id.to_string(),
        client_secret: google_cfg.google_client_secret.to_string(),
        refresh_token: access_token.refresh_token.clone(),
    };

    let new_client = GoogleClient::new(client_credentials, access_token);
    let mut guard = state.google_client.lock().await;
    *guard = Some(new_client);
    println!("Google client initialized successfully");
    StatusCode::OK
}

async fn get_calendar_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    // Create the request builder and immediately drop the lock
    let google_client_guard = state.google_client.lock().await;
    let client = google_client_guard.as_ref().unwrap();
    let events = CalendarEventsClient::new(client)
        .get_events("primary")
        .single_events(true)
        .max_results(10)
        .order_by(google_workspace_apis::calendar::events::requests::EventOrderBy::StartTime)
        .time_min(chrono::Utc::now())
        .request()
        .await
        .unwrap();

    Json(events.unwrap().items)
}

pub fn google_router() -> Router<AppState> {
    Router::new()
        .route("/calendar/events", axum::routing::get(get_calendar_events))
        .route("/auth", axum::routing::get(get_auth_url_workspace))
        // .route("/events", axum::routing::get(get_calendar_events))
        .route(
            "/oauth2callback",
            axum::routing::get(handle_google_oauth_redirect),
        )
}
