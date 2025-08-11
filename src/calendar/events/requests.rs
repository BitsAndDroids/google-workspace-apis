use crate::{
    auth::client::GoogleClient,
    calendar::events::types::{CreateEventRequest, EventDateTime},
    utils::request::{PaginationRequestTrait, Request, TimeRequestTrait},
};

use anyhow::{anyhow, Error};
use chrono::DateTime;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use super::types::{
    BirthdayProperties, Event, EventAttendee, EventList, EventReminders, EventSource,
    OutOfOfficeProperties, PatchEventRequest, WorkingLocationProperties,
};

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
pub struct EventDeleteMode;
/// Indicates that the request builder is initialized for inserting events.
/// This struct determines which filters can be applied to the request.
pub struct EventInsertMode;

pub struct EventPatchMode;

#[derive(Serialize)]
#[serde(untagged)]
pub enum EventRequest {
    Create(CreateEventRequest),
    Patch(PatchEventRequest),
}

/// The generic type parameter `T` determines the mode of operation for this client,
/// which affects which methods are available and what parameters can be set.
pub struct CalendarEventsClient<'a, T = Uninitialized> {
    pub(super) request: Request<'a>,
    pub(super) event: Option<EventRequest>,
    pub(super) _mode: std::marker::PhantomData<T>,
}

/// Implementation for the uninitialized event client.
/// This provides the entry points to initialize the client for specific operations.
impl<'a> CalendarEventsClient<'a, Uninitialized> {
    /// Creates a new calendar events client using the provided Google client for authentication.
    pub fn new(client: &'a mut GoogleClient) -> Self {
        Self {
            request: Request::new(client),
            event: None,
            _mode: std::marker::PhantomData,
        }
    }
    /// Get a list of events from the specified calendar.
    /// # Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ``` rust
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
    pub fn get_events(self, calendar_id: &str) -> CalendarEventsClient<'a, EventListMode> {
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
    ///
    /// #Examples
    ///  
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///  
    /// ```rust
    /// pub async fn insert_new_event(State(state): State<AppState>) {
    ///     let mut google_client_guard = state.google_client.lock().await;
    ///     let client = google_client_guard.as_mut().unwrap();
    ///     let start: EventDateTime = EventDateTime {
    ///         date: Some("2025-07-28".to_string()),
    ///         date_time: None,
    ///         time_zone: None,
    ///     };
    ///     let end: EventDateTime = EventDateTime {
    ///         date: Some("2025-07-28".to_string()),
    ///         date_time: None,
    ///         time_zone: None,
    ///     };
    ///     CalendarEventsClient::new(client)
    ///         .insert_event("calendar_id", start, end)
    ///         .set_summary("test_insert")
    ///         .set_description("new event")
    ///         .request()
    ///         .await;
    /// }
    pub fn insert_event(
        self,
        calendar_id: &str,
        start: EventDateTime,
        end: EventDateTime,
    ) -> CalendarEventsClient<'a, EventInsertMode> {
        let mut builder = CalendarEventsClient {
            request: self.request,
            event: Some(EventRequest::Create(CreateEventRequest::new(start, end))),
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events",);
        builder.request.method = Method::POST;
        builder
    }

    /// Patches a specific event in the specified calendar.
    ///
    /// # Arguments
    ///
    /// * `calendar_id` - The ID of the calendar where the event is located
    /// * `event_id` - The ID of the event to modify
    ///
    /// # Returns
    ///
    /// A builder configured for patching an existing event
    ///  
    ///  # Examples
    ///   
    /// `Axum is used in this example, but it can be adapted to other frameworks like Actix or
    /// Rocket.`
    ///
    /// ``` rust
    ///     async fn update_event(State(state): State<AppState>) {
    ///     let mut google_client_guard = state.google_client.lock().await;
    ///     let client = google_client_guard.as_mut().unwrap();
    ///     CalendarEventsClient::new(client)
    ///      // Main callendar can be targetd by the string "primary" as id
    ///      .patch_event("calendar_id", "event_id")
    ///      .set_summary("New summary/title")
    ///      .set_description("New description")
    ///      .request()
    ///      .await
    ///      .unwrap();
    ///     }
    /// ```
    pub fn patch_event(
        self,
        calendar_id: &str,
        event_id: &str,
    ) -> CalendarEventsClient<'a, EventPatchMode> {
        let mut builder = CalendarEventsClient {
            request: self.request,
            event: Some(EventRequest::Patch(PatchEventRequest::default())),
            _mode: std::marker::PhantomData,
        };
        builder.request.url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events/{event_id}"
        );
        builder.request.method = Method::PATCH;
        builder
    }

    pub fn delete_event(
        self,
        calendar_id: &str,
        event_id: &str,
    ) -> CalendarEventsClient<'a, EventDeleteMode> {
        let mut builder = CalendarEventsClient {
            request: self.request,
            event: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events/{event_id}"
        );
        builder.request.method = Method::DELETE;
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

impl<'a> PaginationRequestTrait for CalendarEventsClient<'a, EventListMode> {
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

impl<'a> TimeRequestTrait for CalendarEventsClient<'a, EventListMode> {
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

impl<'a> CalendarEventsClient<'a, EventListMode> {
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

impl<'a, T> CalendarEventsClient<'a, T> {
    pub(super) async fn make_delete_request(&mut self) -> Result<bool, Error> {
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
            Ok(true)
        } else {
            Ok(false)
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

            Method::PATCH => {
                let res = self
                    .request
                    .client
                    .req_client
                    .patch(&self.request.url)
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

impl<'a> CalendarEventsClient<'a, EventInsertMode> {
    /// Sets the summary (title) of the event being created.
    ///
    /// # Arguments
    ///
    /// * `summary` - The summary text to set for the event
    ///
    pub fn set_summary(self, summary: &str) -> Self {
        self.modify_event(|event| event.summary = Some(summary.to_string()))
    }

    /// Sets the description of the event being created.
    ///
    /// # Arguments
    ///
    /// * `description` - The summary text to set for the event
    pub fn set_description(self, descr: &str) -> Self {
        self.modify_event(|event| event.description = Some(descr.to_string()))
    }

    /// Sets the location for the event.
    ///
    /// # Arguments
    ///
    /// * `location` - The location text to set for the event
    pub fn set_location(self, location: &str) -> Self {
        self.modify_event(|event| event.location = Some(location.to_string()))
    }

    /// Sets the attendees for the event.
    ///
    /// # Arguments
    ///
    /// * `attendees` - A vector of EventAttendee objects representing the event attendees
    pub fn set_attendees(self, attendees: Vec<EventAttendee>) -> Self {
        self.modify_event(|event| event.attendees = attendees)
    }

    /// Sets the type of event.
    ///
    /// # Arguments
    ///
    /// * `type_` - The EventType to set for the event
    ///
    /// pub enum EventType {
    ///    Birthday,
    ///    Default,
    ///    FocusTime,
    ///    FromGmail,
    ///    OutOfOffice,
    /// }
    pub fn set_type(self, type_: EventType) -> Self {
        self.modify_event(|event| event.event_type = Some(type_.as_str().to_string()))
    }

    /// Sets the birthday properties for the event.
    ///
    /// # Arguments
    ///
    /// * `birtday_properties` - The BirthdayProperties to set for the event
    pub fn set_birtday_properties(self, birtday_properties: BirthdayProperties) -> Self {
        self.modify_event(|event| event.birthday_properties = Some(birtday_properties))
    }

    /// Sets the color ID for the event.
    ///
    /// # Arguments
    ///
    /// * `color_id` - The color ID to set for the event
    pub fn set_color_id(self, color_id: &str) -> Self {
        self.modify_event(|event| event.color_id = Some(color_id.to_string()))
    }

    /// Sets whether guests can invite others to the event.
    ///
    /// # Arguments
    ///
    /// * `can_invite` - Boolean indicating if guests can invite others
    pub fn set_guests_can_invite_others(self, can_invite: bool) -> Self {
        self.modify_event(|event| event.guests_can_invite_others = Some(can_invite))
    }

    /// Sets whether guests can modify the event.
    ///
    /// # Arguments
    ///
    /// * `can_modify` - Boolean indicating if guests can modify the event
    pub fn set_guests_can_modify(self, can_modify: bool) -> Self {
        self.modify_event(|event| event.guests_can_modify = Some(can_modify))
    }

    /// Sets whether guests can see other guests in the event.
    ///
    /// # Arguments
    ///
    /// * `can_see` - Boolean indicating if guests can see other guests
    pub fn set_guests_can_see_other_guests(self, can_see: bool) -> Self {
        self.modify_event(|event| event.guests_can_see_other_guests = Some(can_see))
    }

    /// Sets the ID for the event.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to set for the event
    pub fn set_id(self, id: &str) -> Self {
        self.modify_event(|event| event.id = Some(id.to_string()))
    }

    /// Sets the out of office properties for the event.
    ///
    /// # Arguments
    ///
    /// * `out_of_office_properties` - The OutOfOfficeProperties to set for the event
    pub fn set_out_of_office_properties(
        self,
        out_of_office_properties: OutOfOfficeProperties,
    ) -> Self {
        self.modify_event(|event| event.out_of_office_properties = Some(out_of_office_properties))
    }

    /// Sets the recurrence rules for the event.
    ///
    /// # Arguments
    ///
    /// * `recurrence` - A vector of strings containing the recurrence rules in iCalendar RFC 5545 format
    pub fn set_recurrence(self, recurrence: Vec<String>) -> Self {
        self.modify_event(|event| event.recurrence = recurrence)
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

    fn modify_event<F>(mut self, modifier: F) -> Self
    where
        F: FnOnce(&mut CreateEventRequest),
    {
        if let Some(EventRequest::Create(ref mut event)) = self.event {
            modifier(event);
        }
        self
    }
}

impl<'a> CalendarEventsClient<'a, EventPatchMode> {
    /// Patch the end of the event
    ///  
    ///  # Arguments
    ///
    ///  * `end` - new EventDateTime format
    pub fn set_end(self, end: EventDateTime) -> Self {
        self.modify_event(|event| event.end = Some(end))
    }

    /// Patch the start of the event
    ///  
    ///  # Arguments
    ///
    ///  * `start` - new EventDateTime format
    pub fn set_start(self, start: EventDateTime) -> Self {
        self.modify_event(|event| event.start = Some(start))
    }

    /// Patch the summary of the event
    ///
    /// # Arguments
    ///
    /// * `summary` - new summary of the event
    pub fn set_summary(self, summary: &str) -> Self {
        self.modify_event(|event| event.summary = Some(summary.to_string()))
    }

    /// Patch the description of the event
    ///
    /// # Arguments
    ///
    /// * `description` - new description of the event
    pub fn set_description(self, descr: &str) -> Self {
        self.modify_event(|event| event.description = Some(descr.to_string()))
    }

    /// Patch the attendees of the event
    ///
    /// # Arguments
    ///
    /// * `attendees` - Vec<EventAttendee>
    ///  
    /// This will overwrite the existing attendee list.
    /// Previous entries aren't appended
    pub fn set_attendees(self, attendees: Vec<EventAttendee>) -> Self {
        self.modify_event(|event| event.attendees = attendees)
    }

    /// Patch the color_id of the event
    ///
    /// # Arguments
    ///
    /// * `id` - &str
    pub fn set_color_id(self, id: &str) -> Self {
        self.modify_event(|event| event.color_id = Some(id.to_string()))
    }

    /// Patch the event type of the event
    ///
    /// # Arguments
    ///
    /// * `event_type` - the new EventType
    ///  
    /// pub enum EventType {
    ///    Birthday,
    ///    Default,
    ///    FocusTime,
    ///    FromGmail,
    ///    OutOfOffice,
    /// }
    pub fn set_event_type(self, event_type: EventType) -> Self {
        self.modify_event(|event| event.event_type = Some(event_type.as_str().to_string()))
    }

    /// Patch the guests_can_invite_others field
    ///
    /// # Arguments
    ///
    /// * `can_invite` - Boolean
    ///  
    /// This dictates if the guests can invite other attendees for this event
    pub fn set_guests_can_invite_others(self, can_invite: bool) -> Self {
        self.modify_event(|event| event.guests_can_invite_others = Some(can_invite))
    }

    /// Patch the guests_can_modify field
    ///
    /// # Arguments
    ///
    /// * `can_modify` - Boolean
    ///  
    /// This dictates if the guests can modify the event
    pub fn set_guests_can_modify(self, can_modify: bool) -> Self {
        self.modify_event(|event| event.guests_can_modify = Some(can_modify))
    }

    /// Patch the guests_can_see field
    ///
    /// # Arguments
    ///
    /// * `can_see` - Boolean
    ///  
    /// This dictates if the guests can see other guests
    pub fn set_guests_can_see_other_guests(self, can_see: bool) -> Self {
        self.modify_event(|event| event.guests_can_see_other_guests = Some(can_see))
    }

    /// Patch the id field
    ///
    /// # Arguments
    ///
    /// * `id` - &str
    ///  
    /// Alters the id of the event
    pub fn set_id(self, id: &str) -> Self {
        self.modify_event(|event| event.id = Some(id.to_string()))
    }

    /// Patch the location field
    ///
    /// # Arguments
    ///
    /// * `location` - &str
    ///  
    /// Location of the event
    pub fn set_location(self, location: &str) -> Self {
        self.modify_event(|event| event.location = Some(location.to_string()))
    }

    /// Patch the out of office properties field
    ///
    /// # Arguments
    ///
    /// * `properties` - OutOfOfficeProperties
    pub fn set_out_of_office_properties(self, properties: OutOfOfficeProperties) -> Self {
        self.modify_event(|event| event.out_of_office_properties = Some(properties))
    }

    /// Patch the recurrence field
    ///
    /// # Arguments
    ///
    /// * `recurrence` - Vec<String>
    ///  
    /// This will overwrite the existing attendee list.
    /// Previous entries aren't appended
    pub fn set_recurrence(self, recurrence: Vec<String>) -> Self {
        self.modify_event(|event| event.recurrence = recurrence)
    }

    /// Patch the reminders field
    ///
    /// # Arguments
    ///
    /// * `reminders` - EventReminders
    ///  
    /// This will overwrite the existing attendee list.
    /// Previous entries aren't appended
    pub fn set_reminders(self, reminders: EventReminders) -> Self {
        self.modify_event(|event| event.reminders = Some(reminders))
    }

    /// Patch the sequence field
    ///
    /// # Arguments
    ///
    /// * `sequence` - i32
    pub fn set_sequence(self, sequence: i32) -> Self {
        self.modify_event(|event| event.sequence = Some(sequence))
    }

    /// Patch the source field
    ///
    /// # Arguments
    ///
    /// * `source` - EventSource
    pub fn set_source(self, source: EventSource) -> Self {
        self.modify_event(|event| event.source = Some(source))
    }

    /// Patch the status field
    ///
    /// # Arguments
    ///
    /// * `status` - &str
    ///
    /// Options are "confirmed", "tentative", "cancelled"
    pub fn set_status(self, status: &str) -> Self {
        self.modify_event(|event| event.status = Some(status.to_string()))
    }

    /// Patch the transparancy field
    ///
    /// # Arguments
    ///
    /// * `transparancy` - &str
    ///
    /// Options are "opaque" (does block time on the calendar) or "transparant" (does not block
    /// time on the calendar)"
    pub fn set_transparancy(self, transparancy: &str) -> Self {
        self.modify_event(|event| event.transparency = Some(transparancy.to_string()))
    }

    /// Patch the visibility field
    ///
    /// # Arguments
    ///
    /// * `visibility` - &str
    ///
    /// Options are "default", "public" (all readers of the calendar), "private" (only attendees
    /// may view event details), "confidential" (same as private added compatibility reasons)
    pub fn set_visibility(self, visibility: &str) -> Self {
        self.modify_event(|event| event.visibility = Some(visibility.to_string()))
    }

    /// Patch the working location properties field
    ///
    /// # Arguments
    ///
    /// * `properties` - WorkingLocationProperties
    pub fn set_working_location_properties(self, properties: WorkingLocationProperties) -> Self {
        self.modify_event(|event| event.working_location_properties = Some(properties))
    }

    /// Set the query parameter sendUpdates
    ///
    /// # Arguments
    ///
    /// * `send` - &str
    ///
    /// options are "all", "externalOnly", "none"
    pub fn set_send_updates(mut self, send: &str) -> Self {
        self.request
            .params
            .insert("sendUpdates".to_string(), send.to_string());
        self
    }

    /// Set the conference data version query parameter
    ///  
    ///`Version number of conference data supported by the API client.
    ///Version 0 assumes no conference data support and ignores conference data in the event's body.
    ///Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData.
    ///The default is 0. Acceptable values are 0 to 1, inclusive.`
    pub fn set_conference_data_version(mut self, v: i8) -> Self {
        self.request
            .params
            .insert("conferenceDataVersion".to_string(), v.to_string());
        self
    }
    /// Set the maxAttendees query parameter
    ///  
    ///`Whether API client performing operation supports event attachments.
    ///Optional.
    ///The default is False.`
    pub fn support_attachments(mut self, support: bool) -> Self {
        self.request
            .params
            .insert("supportAttachments".to_string(), support.to_string());
        self
    }

    /// Set the maxAttendees query parameter
    /// `The maximum number of attendees to include in the response.
    /// If there are more than the specified number of attendees, only the participant is returned.
    /// Optional.`
    pub fn set_max_attendees(mut self, v: i16) -> Self {
        self.request
            .params
            .insert("maxAttendees".to_string(), v.to_string());
        self
    }

    fn modify_event<F>(mut self, modifier: F) -> Self
    where
        F: FnOnce(&mut PatchEventRequest),
    {
        if let Some(EventRequest::Patch(ref mut event)) = self.event {
            modifier(event);
        }
        self
    }

    /// Executes the request to create the event.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Event))` - The patched event if successful
    /// * `Ok(None)` - If the request was unsuccessful
    /// * `Err` - If there was an error making the request
    pub async fn request(&mut self) -> Result<Option<Event>, Error> {
        self.make_request().await
    }
}

impl<'a> CalendarEventsClient<'a, EventDeleteMode> {
    /// Executes the request to delete the event.
    ///
    /// # Returns
    ///
    /// * `Result<bool, Error>` - A result indicating whether the deletion was successful.
    pub async fn request(&mut self) -> Result<(), Error> {
        match self.make_delete_request().await {
            Ok(true) => Ok(()),
            Ok(false) => Err(anyhow::anyhow!("Failed to delete event")),
            Err(e) => Err(e),
        }
    }

    /// Guests who should receive notifications about the deletion of the event.
    /// Acceptable values are:
    ///
    /// "all": Notifications are sent to all guests.
    /// "externalOnly": Notifications are sent to non-Google Calendar guests only.
    /// "none": No notifications are sent. For calendar migration tasks, consider using the Events.import method instead.
    pub fn send_updates(mut self, send: &str) -> Self {
        self.request
            .params
            .insert("sendUpdates".to_string(), send.to_string());
        self
    }
}
