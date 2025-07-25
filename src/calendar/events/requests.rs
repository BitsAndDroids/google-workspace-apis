use crate::{
    auth::types::GoogleClient,
    utils::request::{PaginationRequestTrait, Request, TimeRequestTrait},
};

use chrono::DateTime;
use reqwest::{Error, Method};

use super::types::EventList;

/// Indicates that the request builder is not yet initialized with a specific mode.
pub struct Uninitialized;
/// Indicates that the request builder is initialized for retrieving single events.
/// This struct determines which filters can be applied to the request.
pub struct EventGetMode;
/// Indicates that the request builder is initialized for retrieving a list of events.
/// This struct determines which filters can be applied to the request.
pub struct EventListMode;

pub trait EventListRequestBuilderTrait: PaginationRequestTrait + TimeRequestTrait {
    type EventRequestBuilder;
}

/// The main builder for making requests to the Google Calendar API to retrieve events.
pub struct EventRequestBuilder<T = Uninitialized> {
    request: Request,
    _mode: std::marker::PhantomData<T>,
}

impl EventRequestBuilder<Uninitialized> {
    pub fn new(client: &GoogleClient) -> Self {
        Self {
            request: Request::new(client),
            _mode: std::marker::PhantomData,
        }
    }
    /// Get a list of events from the specified calendar.
    /// # Examples
    /// ```
    /// #[axum::debug_handler]
    /// pub async fn get_birtday_events(State(state): State<AppState>) -> Json<EventResponse> {
    ///     //GoogleClient is stored in the AppState wrapped in a Arc<Mutex>
    ///     let google_client_guard = state.google_client.lock().await;
    ///     let client = google_client_guard.as_ref().unwrap();
    ///     let events = EventRequestBuilder::new(client)
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
    pub fn get_events(self, calendar_id: &str) -> EventRequestBuilder<EventListMode> {
        let mut builder = EventRequestBuilder {
            request: self.request,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = "https://www.googleapis.com/calendar/v3/calendars/".to_string()
            + calendar_id
            + "/events";
        builder.request.method = reqwest::Method::GET;
        builder
    }
}

/// Event ordering options for Google Calendar events.
/// StartTime doesn't work with recurring events unless singleEvents is set to true.
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

/** Event types for Google Calendar events.
* These are used to filter events when making requests.
* See [Google Calendar API
* documentation](https://developers.google.com/calendar/api/v3/reference/events
*/
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
    /// Maximum number of results to return.
    fn max_results(mut self, max: i64) -> Self {
        self.request
            .params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    /// Page token for pagination. Works with `max_results`.
    fn page_token(mut self, token: &str) -> Self {
        self.request
            .params
            .insert("pageToken".to_string(), token.to_string());
        self
    }
}

impl TimeRequestTrait for EventRequestBuilder<EventListMode> {
    /// Minimum time for events to return. If not set, all historicall events matching the other
    /// filters are returned.
    fn time_min(mut self, time_min: DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("timeMin".to_string(), time_min.to_rfc3339());
        self
    }

    /// Maximum time for events to return. If not set, all future events matching the other filters are returned.
    fn time_max(mut self, time_max: DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("timeMax".to_string(), time_max.to_rfc3339());
        self
    }
}

impl EventRequestBuilder<EventListMode> {
    /// Set the type of events to filter by.
    pub fn event_type(mut self, type_: EventType) -> Self {
        self.request
            .params
            .insert("eventTypes".to_string(), type_.as_str().to_string());
        self
    }

    /// Order the events by the specified field.
    /// This can be either `startTime` or `updated`.
    /// The startTime value can only be used with specific event times
    /// Use this value in comgination with the singleEvents parameter set to true.
    pub fn order_by(mut self, by: EventOrderBy) -> Self {
        self.request
            .params
            .insert("orderBy".to_string(), by.as_str().to_string());
        self
    }

    /// Filter events by the amount of attendees.
    pub fn max_attendees(mut self, max: i64) -> Self {
        self.request
            .params
            .insert("maxAttendees".to_string(), max.to_string());
        self
    }

    /// Filter if set to true only returns single_events.
    pub fn single_events(mut self, single: bool) -> Self {
        self.request
            .params
            .insert("singleEvents".to_string(), single.to_string());
        self
    }

    /// Filter if set to true shows hidden invitations.
    pub fn show_hidden_invitations(mut self, max: bool) -> Self {
        self.request
            .params
            .insert("showHiddenInvitations".to_string(), max.to_string());
        self
    }

    /// Add a query string to the request.
    /// This searches for events matching the query string in the fields:
    /// location, summary, description, and attendees.
    pub fn query(mut self, query_str: &str) -> Self {
        self.request
            .params
            .insert("q".to_string(), query_str.to_string());
        self
    }

    /// Returns a request result for getting a list of events from the specified calendar.
    pub async fn request(self) -> Result<Option<EventList>, Error> {
        println!("Requesting calendar events from: {}", self.request.url);
        println!("Request parameters: {:?}", self.request.params);
        let response = self
            .request
            .client
            .request(Method::GET, self.request.url)
            .query(&self.request.params)
            .send()
            .await?;
        let url = &response.url().clone();
        let url_status = &response.status();
        let calendar_res: Option<EventList> = match response.json().await {
            Ok(res) => res,
            Err(_) => {
                println!("URL {url} Status {url_status}");
                None
            }
        };

        Ok(calendar_res)
    }
}
