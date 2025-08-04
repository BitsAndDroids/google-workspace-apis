use crate::{
    auth::types::GoogleClient,
    calendar::events::types::{CreateEventRequest, EventDateTime},
    utils::request::{PaginationRequestTrait, Request, TimeRequestTrait},
};

use anyhow::{anyhow, Error};
use chrono::DateTime;
use reqwest::Method;
use serde::de::DeserializeOwned;

use super::types::{BirthdayProperties, Event, EventAttendee, EventList, OutOfOfficeProperties};

/// Indicates that the request builder is not yet initialized with a specific mode.
pub struct Uninitialized;
/// Indicates that the request builder is initialized for retrieving single events.
/// This struct determines which filters can be applied to the request.
pub struct EventGetMode;
/// Indicates that the request builder is initialized for retrieving a list of events.
/// This struct determines which filters can be applied to the request.
pub struct EventListMode;
/// Indicates that the request builder is initialized for inserting events.
/// This struct determines which filters can be applied to the request.
pub struct EventInsertMode;

/// The generic type parameter `T` determines the mode of operation for this client,
/// which affects which methods are available and what parameters can be set.
pub struct CalendarEventsClient<T = Uninitialized> {
    request: Request,
    event: Option<CreateEventRequest>,
    _mode: std::marker::PhantomData<T>,
}

/// Implementation for the uninitialized event client.
/// This provides the entry points to initialize the client for specific operations.
impl CalendarEventsClient<Uninitialized> {
    /// Creates a new calendar events client using the provided Google client for authentication.
    pub fn new(client: &GoogleClient) -> Self {
        Self {
            request: Request::new(client),
            event: None,
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
    pub fn get_events(self, calendar_id: &str) -> CalendarEventsClient<EventListMode> {
        let mut builder = CalendarEventsClient {
            request: self.request,
            event: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = "https://www.googleapis.com/calendar/v3/calendars/".to_string()
            + calendar_id
            + "/events";
        builder.request.method = reqwest::Method::GET;
        builder
    }

    /// Creates a new event in the specified calendar.
    ///
    /// # Arguments
    ///
    /// * `calendar_id` - The ID of the calendar where the event will be created
    /// * `start` - The start time information for the event
    /// * `end` - The end time information for the event
    ///
    /// # Returns
    ///
    /// A builder configured for inserting a new event
    pub fn insert_event(
        self,
        calendar_id: &str,
        start: EventDateTime,
        end: EventDateTime,
    ) -> CalendarEventsClient<EventInsertMode> {
        let mut builder = CalendarEventsClient {
            request: self.request,
            event: Some(CreateEventRequest::new(start, end)),
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events",);
        builder.request.method = Method::POST;
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

impl PaginationRequestTrait for CalendarEventsClient<EventListMode> {
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

impl TimeRequestTrait for CalendarEventsClient<EventListMode> {
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

impl CalendarEventsClient<EventListMode> {
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
    pub async fn request(&mut self) -> Result<Option<EventList>, Error> {
        self.make_request().await
    }
}

impl<T> CalendarEventsClient<T> {
    async fn make_request<R>(&mut self) -> Result<Option<R>, Error>
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
                    .body(serde_json::to_string(&self.event).unwrap())
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

impl CalendarEventsClient<EventInsertMode> {
    /// Sets the summary (title) of the event being created.
    ///
    /// # Arguments
    ///
    /// * `summary` - The summary text to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_event_summary(mut self, summary: &str) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.summary = Some(summary.to_string());
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the location for the event.
    ///
    /// # Arguments
    ///
    /// * `location` - The location text to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_event_location(mut self, location: &str) -> Self {
        match self.event {
            Some(ref mut event) => event.location = Some(location.to_string()),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the attendees for the event.
    ///
    /// # Arguments
    ///
    /// * `attendees` - A vector of EventAttendee objects representing the event attendees
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_event_attendees(mut self, attendees: Vec<EventAttendee>) -> Self {
        match self.event {
            Some(ref mut event) => event.attendees = attendees,
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the type of event.
    ///
    /// # Arguments
    ///
    /// * `type_` - The EventType to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_event_type(mut self, type_: EventType) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.event_type = Some(type_.as_str().to_string());
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the birthday properties for the event.
    ///
    /// # Arguments
    ///
    /// * `birtday_properties` - The BirthdayProperties to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_birtday_properties(mut self, birtday_properties: BirthdayProperties) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.birthday_properties = Some(birtday_properties);
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the color ID for the event.
    ///
    /// # Arguments
    ///
    /// * `color_id` - The color ID to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_color_id(mut self, color_id: &str) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.color_id = Some(color_id.to_string());
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets whether guests can invite others to the event.
    ///
    /// # Arguments
    ///
    /// * `can_invite` - Boolean indicating if guests can invite others
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_guests_can_invite_others(mut self, can_invite: bool) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.guests_can_invite_others = Some(can_invite);
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets whether guests can modify the event.
    ///
    /// # Arguments
    ///
    /// * `can_modify` - Boolean indicating if guests can modify the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_guests_can_modify(mut self, can_modify: bool) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.guests_can_modify = Some(can_modify);
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets whether guests can see other guests in the event.
    ///
    /// # Arguments
    ///
    /// * `can_see` - Boolean indicating if guests can see other guests
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_guests_can_see_other_guests(mut self, can_see: bool) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.guests_can_see_other_guests = Some(can_see);
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the ID for the event.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_id(mut self, id: &str) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.id = Some(id.to_string());
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the out of office properties for the event.
    ///
    /// # Arguments
    ///
    /// * `out_of_office_properties` - The OutOfOfficeProperties to set for the event
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_out_of_office_properties(
        mut self,
        out_of_office_properties: OutOfOfficeProperties,
    ) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.out_of_office_properties = Some(out_of_office_properties);
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the recurrence rules for the event.
    ///
    /// # Arguments
    ///
    /// * `recurrence` - A vector of strings containing the recurrence rules in iCalendar RFC 5545 format
    ///
    /// # Panics
    ///
    /// Panics if the event has not been initialized for insertion
    pub fn set_recurrence(mut self, recurrence: Vec<String>) -> Self {
        match self.event {
            Some(ref mut event) => {
                event.recurrence = recurrence;
            }
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Executes the request to create the event.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Event))` - The created event if successful
    /// * `Ok(None)` - If the request was unsuccessful
    /// * `Err` - If there was an error making the request
    pub async fn request(&mut self) -> Result<Option<Event>, Error> {
        self.make_request().await
    }
}
