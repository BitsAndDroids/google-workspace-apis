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
    ///  
    /// # Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ``` rust
    /// pub async fn get_emails(State(state): State<AppState>) -> Json<MessageList> {
    ///     //GoogleClient is stored in the AppState wrapped in a Arc<Mutex>
    ///     let google_client_guard = state.google_client.lock().await;
    ///     let client = google_client_guard.as_ref().unwrap();
    ///     let res = GmailClient::new(client)
    ///         // "me" is a special value that refers to the authenticated user when used as user_id
    ///         .get_emails("me")
    ///         .max_results(10)
    ///         .request()
    ///         .await
    ///         .unwrap();
    ///
    ///     Json(emails.unwrap().items.into())
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

    /// Get a specific email by user_id and email_id.
    ///  
    /// # Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ```rust
    /// pub async fn get_email(State(state): State<AppState>, Path((user_id, email_id)):
    /// Path<(String, String)>) -> Json<Message> {
    ///   let google_client_guard = state.google_client.lock().await;
    ///   let client = google_client_guard.as_ref().unwrap();
    ///   let res = GmailClient::new(client)
    ///   // "me" is a special value that refers to the authenticated user when used as user_id
    ///   .get_email(user_id, &email_id)
    ///   .request()
    ///   .await.unwrap();
    ///    
    ///   json!(res.unwrap())
    /// }
    /// ```
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

    /// Delete a specific email by user_id and email_id.
    /// This will completely remove the email from the user's mailbox (not moved to trash).
    /// Use trash_email instead if you want to move it to the trash.
    ///  
    /// # Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ```rust
    /// pub async fn delete_email(State(state): State<AppState>, Path((user_id, email_id)):
    /// Path<(String, String)>) -> Json<()> {
    ///
    ///   let google_client_guard = state.google_client.lock().await;
    ///   let client = google_client_guard.as_ref().unwrap();
    ///    
    ///   let res = GmailClient::new(client)
    ///   // "me" is a special value that refers to the authenticated user when used as user_id
    ///   .delete_email(&user_id, &email_id)
    ///   .request().await.unwrap();
    /// }
    ///```
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

    /// Trash a specific email by user_id and email_id.
    /// This will move the email to the trash folder, allowing it to be restored later.
    /// # Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ```rust
    /// pub async fn trash_email(State(state): State<AppState>, Path((user_id, email_id)):
    /// Path<(String, String)>) -> Json<()> {
    ///
    ///   let google_client_guard = state.google_client.lock().await;
    ///   let client = google_client_guard.as_ref().unwrap();
    ///    
    ///   let res = GmailClient::new(client)
    ///   // "me" is a special value that refers to the authenticated user when used as user_id
    ///   .trash_email(&user_id, &email_id)
    ///   .request().await.unwrap();
    /// }
    ///```
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

    /// Untrash a specific email by user_id and email_id.
    /// This will move the email from the trash folder, restoring it.
    ///  
    /// # Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ```rust
    /// pub async fn untrash_email(State(state): State<AppState>, Path((user_id, email_id)):
    /// Path<(String, String)>) -> Json<()> {
    ///
    ///   let google_client_guard = state.google_client.lock().await;
    ///   let client = google_client_guard.as_ref().unwrap();
    ///    
    ///   let res = GmailClient::new(client)
    ///   // "me" is a special value that refers to the authenticated user when used as user_id
    ///   .untrash_email(&user_id, &email_id)
    ///   .request().await.unwrap();
    /// }
    ///```
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

    /// Include messages from SPAM and TRASH in the results.
    pub fn include_spam_trash(mut self, incl: bool) -> Self {
        self.request
            .params
            .insert("includeSpamTrash".to_string(), incl.to_string());
        self
    }

    /// Page token to retrieve a specific page of results in the list.
    pub fn page_token(mut self, token: i32) -> Self {
        self.request
            .params
            .insert("pageToken".to_string(), token.to_string());
        self
    }

    /// Maximum number of messages to return. This field defaults to 100. The maximum allowed value for this field is 500.
    pub fn max_results(mut self, max: u32) -> Self {
        self.request
            .params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    /// Only return messages matching the specified query.
    /// Supports the same query format as the Gmail search box.
    /// For example, "from:someuser@example.com rfc822msgid:<somemsgid@example.com> is:unread".
    /// Parameter cannot be used when accessing the api using the gmail.metadata scope.
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
