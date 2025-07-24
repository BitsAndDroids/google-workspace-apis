use std::collections::HashMap;

use chrono::DateTime;

pub struct Request {
    pub client: reqwest::Client,
    pub url: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            url: "".to_string(),
            params: HashMap::new(),
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
