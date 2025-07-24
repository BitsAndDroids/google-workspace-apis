use crate::utils::request::{PaginationRequestTrait, Request, TimeRequestTrait};

use chrono::DateTime;
use reqwest::{Error, Method};

use super::types::EventList;

pub struct Uninitialized;
pub struct EventGetMode;
pub struct EventListMode;

pub trait EventListRequestBuilderTrait: PaginationRequestTrait + TimeRequestTrait {
    type EventRequestBuilder;
}

pub struct EventRequestBuilder<T = Uninitialized> {
    pub request: Request,
    _mode: std::marker::PhantomData<T>,
}

impl EventRequestBuilder<Uninitialized> {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            request: Request::new(client),
            _mode: std::marker::PhantomData,
        }
    }
    pub fn get_events(self, calendar_id: &str) -> EventRequestBuilder<EventListMode> {
        let mut builder = EventRequestBuilder {
            request: self.request,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = "https://www.googleapis.com/calendar/v3/calendars/".to_string()
            + calendar_id
            + "/events";
        builder
    }
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

impl PaginationRequestTrait for EventRequestBuilder<EventListMode> {
    fn max_results(mut self, max: i64) -> Self {
        self.request
            .params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    fn page_token(mut self, token: &str) -> Self {
        self.request
            .params
            .insert("pageToken".to_string(), token.to_string());
        self
    }
}

impl TimeRequestTrait for EventRequestBuilder<EventListMode> {
    fn time_min(mut self, time_min: DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("timeMin".to_string(), time_min.to_rfc3339());
        self
    }

    fn time_max(mut self, time_max: DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("timeMax".to_string(), time_max.to_rfc3339());
        self
    }
}

impl EventRequestBuilder<EventListMode> {
    pub fn event_type(mut self, type_: EventType) -> Self {
        self.request
            .params
            .insert("eventTypes".to_string(), type_.as_str().to_string());
        self
    }

    pub fn order_by(mut self, by: EventOrderBy) -> Self {
        self.request
            .params
            .insert("orderBy".to_string(), by.as_str().to_string());
        self
    }
    pub fn max_attendees(mut self, max: i64) -> Self {
        self.request
            .params
            .insert("maxAttendees".to_string(), max.to_string());
        self
    }

    pub fn single_events(mut self, single: bool) -> Self {
        self.request
            .params
            .insert("singleEvents".to_string(), single.to_string());
        self
    }

    pub fn show_hidden_invitations(mut self, max: bool) -> Self {
        self.request
            .params
            .insert("showHiddenInvitations".to_string(), max.to_string());
        self
    }

    pub fn query(mut self, query_str: &str) -> Self {
        self.request
            .params
            .insert("q".to_string(), query_str.to_string());
        self
    }

    pub async fn request(self) -> Result<Option<EventList>, Error> {
        let response = self
            .request
            .client
            .request(Method::GET, self.request.url)
            .query(&self.request.params)
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
