use std::collections::HashMap;

use chrono::DateTime;

use crate::auth::types::GoogleClient;

pub(crate) struct Request {
    pub(crate) client: GoogleClient,
    pub(crate) url: String,
    pub(crate) method: reqwest::Method,
    pub(crate) params: HashMap<String, String>,
    pub(crate) body: Option<String>,
}

impl Request {
    pub(crate) fn new(client: &GoogleClient) -> Self {
        Self {
            client: client.clone(),
            url: "".to_string(),
            method: reqwest::Method::GET,
            params: HashMap::new(),
            body: None,
        }
    }
}

pub trait PaginationRequestTrait {
    fn max_results(self, max: i64) -> Self;
    fn page_token(self, token: &str) -> Self;
}

pub trait TimeRequestTrait {
    fn time_min(self, max: DateTime<chrono::Utc>) -> Self;
    fn time_max(self, token: DateTime<chrono::Utc>) -> Self;
}
