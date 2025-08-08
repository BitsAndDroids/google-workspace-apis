use crate::{auth::client::GoogleClient, utils::request::Request};

use super::types::Message;

pub struct EmailListMode;

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
}
