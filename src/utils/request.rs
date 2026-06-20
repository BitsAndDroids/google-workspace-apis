use chrono::DateTime;
use reqwest::StatusCode;
use std::collections::HashMap;
use thiserror::Error;

use crate::auth::client::GoogleClient;

pub struct Request<'a> {
    pub client: &'a mut GoogleClient,
    pub url: String,
    pub method: reqwest::Method,
    pub params: HashMap<String, String>,
    pub body: Option<String>,
}

impl<'a> Request<'a> {
    pub fn new(client: &'a mut GoogleClient) -> Self {
        Self {
            client,
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

pub struct SuccessResult<R> {
    pub status: reqwest::StatusCode,
    pub result: R,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP {status}: {body}")]
    HttpError { status: StatusCode, body: String },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
