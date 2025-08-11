use anyhow::{anyhow, Error};
use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{auth::client::GoogleClient, utils::request::Request};

use super::types::{Message, MessageList};

pub struct EmailListMode;
pub struct EmailGetMode;

pub struct GmailClient<'a, T> {
    pub(super) request: Request<'a>,
    pub(super) message: Option<Message>,
    pub(super) _mode: std::marker::PhantomData<T>,
}

impl<'a> GmailClient<'a, ()> {
    pub fn new(client: &'a mut GoogleClient) -> Self {
        GmailClient {
            request: Request::new(client),
            message: None,
            _mode: std::marker::PhantomData,
        }
    }

    pub fn get_emails(self, user_id: &str) -> GmailClient<'a, EmailListMode> {
        let mut builder = GmailClient {
            request: self.request,
            message: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://gmail.googleapis.com/gmail/v1/users/{user_id}/messages");
        builder.request.method = reqwest::Method::GET;
        builder
    }

    pub fn get_email(self, user_id: &str, email_id: &str) -> GmailClient<'a, EmailGetMode> {
        let mut builder = GmailClient {
            request: self.request,
            message: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://gmail.googleapis.com/gmail/v1/users/{user_id}/messages/{email_id}");
        builder.request.method = reqwest::Method::GET;
        builder
    }
}

impl<'a, T> GmailClient<'a, T> {
    pub(super) async fn make_request<R>(&mut self) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.request.client.refresh_acces_token_check().await?;
        match self.request.method {
            Method::GET => {
                let res = self
                    .request
                    .client
                    .req_client
                    .get(&self.request.url)
                    .query(&self.request.params)
                    .send()
                    .await?;

                if res.status().is_success() {
                    Ok(Some(res.json().await?))
                } else {
                    Ok(None)
                }
            }

            Method::POST => {
                let res = self
                    .request
                    .client
                    .req_client
                    .post(&self.request.url)
                    .body(serde_json::to_string(&self.message).unwrap())
                    .query(&self.request.params)
                    .send()
                    .await?;

                if res.status().is_success() {
                    Ok(Some(res.json().await?))
                } else {
                    Ok(None)
                }
            }

            Method::PATCH => {
                let res = self
                    .request
                    .client
                    .req_client
                    .patch(&self.request.url)
                    .body(serde_json::to_string(&self.message).unwrap())
                    .query(&self.request.params)
                    .send()
                    .await?;

                if res.status().is_success() {
                    Ok(Some(res.json().await?))
                } else {
                    Ok(None)
                }
            }
            _ => Err(anyhow!("Unsupported HTTP method")),
        }
    }
}

impl<'a> GmailClient<'a, EmailListMode> {
    pub async fn request(mut self) -> Result<Option<MessageList>, Error> {
        self.make_request().await
    }

    pub fn include_spam_trash(mut self, incl: bool) -> Self {
        self.request
            .params
            .insert("includeSpamTrash".to_string(), incl.to_string());
        self
    }

    pub fn page_token(mut self, token: i32) -> Self {
        self.request
            .params
            .insert("pageToken".to_string(), token.to_string());
        self
    }

    pub fn max_results(mut self, max: u32) -> Self {
        self.request
            .params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    pub fn query(mut self, query: &str) -> Self {
        self.request
            .params
            .insert("q".to_string(), query.to_string());
        self
    }
}

impl<'a> GmailClient<'a, EmailGetMode> {
    pub async fn request(mut self) -> Result<Option<Message>, Error> {
        self.make_request().await
    }
}
