use anyhow::{anyhow, Error};
use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{auth::client::GoogleClient, utils::request::Request};

use super::types::{Message, MessageList};

pub struct EmailListMode;
pub struct EmailGetMode;
pub struct EmailDeleteMode;
pub struct TrashEmailMode;

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

    /// Get a list of emails from the specified user_id.
    /// # Examples
    /// ``` rust
    /// #[axum::debug_handler]
    /// pub async fn get_birtday_events(State(state): State<AppState>) -> Json<EventResponse> {
    ///     //GoogleClient is stored in the AppState wrapped in a Arc<Mutex>
    ///     let google_client_guard = state.google_client.lock().await;
    ///     let client = google_client_guard.as_ref().unwrap();
    ///     let events = GmailClient::new(client)
    ///         .get_events("primary")
    ///         .single_events(true)
    ///         .event_type(EventType::Birthday)
    ///         .max_results(10)
    ///         .order_by(google_workspace_apis::calendar::events::requests::EventOrderBy::StartTime)
    ///         //To avoid retrieving past events we set the time_min to now
    ///         .time_min(chrono::Utc::now())
    ///         //Since we retrieve single events get all birthdays for the next year
    ///         //To avoid retrieving the same birthday multiple times
    ///         .time_max(chrono::Utc::now() + chrono::Duration::days(365))
    ///         .request()
    ///         .await
    ///         .unwrap();
    ///
    ///     Json(events.unwrap().items.into())
    /// }
    /// ```
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

    pub fn delete_email(self, user_id: &str, email_id: &str) -> GmailClient<'a, EmailDeleteMode> {
        let mut builder = GmailClient {
            request: self.request,
            message: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://gmail.googleapis.com/gmail/v1/users/{user_id}/messages/{email_id}");
        builder.request.method = reqwest::Method::DELETE;
        builder
    }

    pub fn trash_email(self, user_id: &str, email_id: &str) -> GmailClient<'a, EmailDeleteMode> {
        let mut builder = GmailClient {
            request: self.request,
            message: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/{user_id}/messages/{email_id}/trash"
        );
        builder.request.method = reqwest::Method::POST;
        builder
    }

    pub fn untrash_email(self, user_id: &str, email_id: &str) -> GmailClient<'a, EmailDeleteMode> {
        let mut builder = GmailClient {
            request: self.request,
            message: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/{user_id}/messages/{email_id}/untrash"
        );
        builder.request.method = reqwest::Method::POST;
        builder
    }
}

impl<'a, T> GmailClient<'a, T> {
    pub(super) async fn delete_request(&mut self) -> Result<(), Error> {
        self.request.client.refresh_acces_token_check().await?;
        let res = self
            .request
            .client
            .req_client
            .delete(&self.request.url)
            .query(&self.request.params)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to delete email: {}", res.status()))
        }
    }

    pub(super) async fn trash_request(&mut self) -> Result<(), Error> {
        self.request.client.refresh_acces_token_check().await?;
        let res = self
            .request
            .client
            .req_client
            .post(&self.request.url)
            .query(&self.request.params)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to trash email: {}", res.status()))
        }
    }

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
}

impl<'a> GmailClient<'a, EmailGetMode> {
    pub async fn request(mut self) -> Result<Option<Message>, Error> {
        self.make_request().await
    }
}

impl<'a> GmailClient<'a, EmailDeleteMode> {
    pub async fn request(mut self) -> Result<(), Error> {
        self.delete_request().await
    }
}

impl<'a> GmailClient<'a, TrashEmailMode> {
    pub async fn request(mut self) -> Result<(), Error> {
        self.trash_request().await
    }
}
