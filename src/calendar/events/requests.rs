use std::collections::HashMap;

use crate::utils::request::{PaginationRequestTrait, TimeRequestTrait};

use chrono::DateTime;
use reqwest::{Error, Method};

use super::types::EventList;

pub trait EventRequestBuilderTrait: PaginationRequestTrait + TimeRequestTrait {
    type EventRequestBuilder;

    fn request_events(self, calendar_id: &str, client: reqwest::Client) -> Self;
    fn event_type(self, type_: EventType) -> Self;
    fn order_by(self, by: EventOrderBy) -> Self;
    fn max_attendees(self, max: i64) -> Self;
    fn single_events(self, single: bool) -> Self;
    fn show_hidden_invitations(self, max: bool) -> Self;
    fn query(self, query_str: &str) -> Self;
    fn request(self) -> impl Future<Output = Result<Option<EventList>, Error>>;
}

pub enum EventOrderBy {
    StartTime,
    Updated,
}
impl EventOrderBy {
    pub fn as_str(&self) -> &str {
        match self {
            EventOrderBy::StartTime => "startTime",
            EventOrderBy::Updated => "updated",
        }
    }
}

pub enum EventType {
    Birthday,
    Default,
    FocusTime,
    FromGmail,
    OutOfOffice,
    WorkingLocation,
}
impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            EventType::Birthday => "birthday",
            EventType::Default => "default",
            EventType::FocusTime => "focusTime",
            EventType::FromGmail => "fromGmail",
            EventType::OutOfOffice => "outOfOffice",
            EventType::WorkingLocation => "workingLocation",
        }
    }
}

#[derive(Default, Clone)]
pub struct EventRequestBuilder {
    client: reqwest::Client,
    url: String,
    params: HashMap<String, String>,
}

impl PaginationRequestTrait for EventRequestBuilder {
    fn max_results(mut self, max: i64) -> Self {
        self.params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    fn page_token(mut self, token: &str) -> Self {
        self.params
            .insert("pageToken".to_string(), token.to_string());
        self
    }
}

impl TimeRequestTrait for EventRequestBuilder {
    fn time_min(mut self, time_min: DateTime<chrono::Utc>) -> Self {
        self.params
            .insert("timeMin".to_string(), time_min.to_rfc3339());
        self
    }

    fn time_max(mut self, time_max: DateTime<chrono::Utc>) -> Self {
        self.params
            .insert("timeMax".to_string(), time_max.to_rfc3339());
        self
    }
}

impl EventRequestBuilderTrait for EventRequestBuilder {
    type EventRequestBuilder = EventRequestBuilder;

    fn request_events(mut self, calendar_id: &str, client: reqwest::Client) -> Self {
        self.client = client;
        self.url = "https://www.googleapis.com/calendar/v3/calendars/".to_string()
            + calendar_id
            + "/events";
        self
    }
    fn event_type(mut self, type_: EventType) -> Self {
        self.params
            .insert("eventTypes".to_string(), type_.as_str().to_string());
        self
    }

    fn order_by(mut self, by: EventOrderBy) -> Self {
        self.params
            .insert("orderBy".to_string(), by.as_str().to_string());
        self
    }
    fn max_attendees(mut self, max: i64) -> Self {
        self.params
            .insert("maxAttendees".to_string(), max.to_string());
        self
    }

    fn single_events(mut self, single: bool) -> Self {
        self.params
            .insert("singleEvents".to_string(), single.to_string());
        self
    }

    fn show_hidden_invitations(mut self, max: bool) -> Self {
        self.params
            .insert("showHiddenInvitations".to_string(), max.to_string());
        self
    }

    fn query(mut self, query_str: &str) -> Self {
        self.params.insert("q".to_string(), query_str.to_string());
        self
    }

    async fn request(self) -> Result<Option<EventList>, Error> {
        let response = self
            .client
            .request(Method::GET, self.url)
            .query(&self.params)
            .send()
            .await?;
        let url = &response.url().clone();
        println!("Requesting URL: {url}");
        let url_status = &response.status();
        let calendar_res: Option<EventList> = match response.json().await {
            Ok(res) => res,
            Err(e) => {
                println!("{e}");
                println!("URL {url} Status {url_status}");
                None
            }
        };

        Ok(calendar_res)
    }
}
