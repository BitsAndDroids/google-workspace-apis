use std::collections::HashMap;

pub struct Request {
    pub client: reqwest::Client,
    pub url: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new(url: String, client: reqwest::Client) -> Self {
        Self {
            client,
            url,
            params: HashMap::new(),
        }
    }
}

pub trait PaginationRequestTrait {
    fn max_results(self, max: i64) -> Self;
    fn page_token(self, token: &str) -> Self;
}

pub trait TimeRequestTrait {
    fn min_time(self, max: i64) -> Self;
    fn max_time(self, token: &str) -> Self;
}
