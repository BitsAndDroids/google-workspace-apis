use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventDefaultReminder {
    /**
     *The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. do not have reminders.useDefault set to True).
     */

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    method: String,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize"
    )]
    minutes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Event {
    /**
     * The kind of the event. Always calendar#event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub kind: String,

    /**
     * ETag of the resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub etag: String,

    /**
     * Identifier of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub id: String,

    /**
     * Status of the event. Optional. Possible values are: "confirmed", "tentative", "cancelled".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub status: String,

    /**
     * HTML link to the event in the Google Calendar web UI.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "htmlLink"
    )]
    pub html_link: String,

    /**
     * Creation time of the event (as a RFC3339 timestamp).
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,

    /**
     * Last modification time of the event (as a RFC3339 timestamp).
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,

    /**
     * Title of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub summary: String,

    /**
     * Description of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub description: String,

    /**
     * Geographic location of the event as free-form text.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub location: String,

    /**
     * The color of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "colorId"
    )]
    pub color_id: String,

    /**
     * The creator of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<EventPerson>,

    /**
     * The organizer of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer: Option<EventPerson>,

    /**
     * The start time of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<EventDateTime>,

    /**
     * The end time of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<EventDateTime>,

    /**
     * Whether the end time is unspecified.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "endTimeUnspecified"
    )]
    pub end_time_unspecified: Option<bool>,

    /**
     * List of RRULE, EXRULE, RDATE, and EXDATE lines for a recurring event.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub recurrence: Vec<String>,

    /**
     * For an instance of a recurring event, the ID of the recurring event to which this instance belongs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "recurringEventId"
    )]
    pub recurring_event_id: String,

    /**
     * The original start time of the instance in case of a recurring event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "originalStartTime"
    )]
    pub original_start_time: Option<EventDateTime>,

    /**
     * Transparency of the event. Optional. Possible values are: "opaque", "transparent".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub transparency: String,

    /**
     * Visibility of the event. Optional. Possible values are: "default", "public", "private", "confidential".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub visibility: String,

    /**
     * Event unique identifier as defined in RFC5545.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "iCalUID"
    )]
    pub ical_uid: String,

    /**
     * Sequence number as defined in iCalendar.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize"
    )]
    pub sequence: i64,

    /**
     * Attendees of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub attendees: Vec<EventAttendee>,

    /**
     * Whether attendees may have been omitted from the event's representation.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "attendeesOmitted"
    )]
    pub attendees_omitted: Option<bool>,

    /**
     * Extended properties of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "extendedProperties"
    )]
    pub extended_properties: Option<EventExtendedProperties>,

    /**
     * An absolute link to the Google Hangout associated with this event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "hangoutLink"
    )]
    pub hangout_link: String,

    /**
     * Information about the conference-related information.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "conferenceData"
    )]
    pub conference_data: Option<EventConferenceData>,

    /**
     * A gadget that extends this event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gadget: Option<EventGadget>,

    /**
     * Whether anyone can invite themselves to the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "anyoneCanAddSelf"
    )]
    pub anyone_can_add_self: Option<bool>,

    /**
     * Whether guests can invite others to the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "guestsCanInviteOthers"
    )]
    pub guests_can_invite_others: Option<bool>,

    /**
     * Whether guests can modify the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "guestsCanModify"
    )]
    pub guests_can_modify: Option<bool>,

    /**
     * Whether guests can see other guests' responses.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "guestsCanSeeOtherGuests"
    )]
    pub guests_can_see_other_guests: Option<bool>,

    /**
     * Whether this is a private event copy.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "privateCopy"
    )]
    pub private_copy: Option<bool>,

    /**
     * Whether this is a locked event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    /**
     * Information about the event's reminders.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<EventReminders>,

    /**
     * Source from which the event was created.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,

    /**
     * Working location properties of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "workingLocationProperties"
    )]
    pub working_location_properties: Option<WorkingLocationProperties>,

    /**
     * Out of office properties for the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "outOfOfficeProperties"
    )]
    pub out_of_office_properties: Option<OutOfOfficeProperties>,

    /**
     * Focus time properties of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "focusTimeProperties"
    )]
    pub focus_time_properties: Option<FocusTimeProperties>,

    /**
     * File attachments for the event.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub attachments: Vec<EventAttachment>,

    /**
     * Birthday event properties.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "birthdayProperties"
    )]
    pub birthday_properties: Option<BirthdayProperties>,

    /**
     * Event type. The possible values are: "default", "outOfOffice", "focusTime", "workingLocation".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "eventType"
    )]
    pub event_type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventPerson {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub email: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct EventDateTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize",
        rename = "dateTime"
    )]
    pub date_time: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventAttendee {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub email: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "self")]
    pub self_: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "responseStatus"
    )]
    pub response_status: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub comment: String,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize",
        rename = "additionalGuests"
    )]
    pub additional_guests: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventExtendedProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<std::collections::HashMap<String, String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventConferenceData {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "createRequest"
    )]
    pub create_request: Option<ConferenceRequestStatus>,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize",
        rename = "entryPoints"
    )]
    pub entry_points: Vec<EntryPoint>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "conferenceSolution"
    )]
    pub conference_solution: Option<ConferenceSolution>,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "conferenceId"
    )]
    pub conference_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub signature: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub notes: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConferenceRequestStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "requestId"
    )]
    pub request_id: String,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "conferenceSolutionKey"
    )]
    pub conference_solution_key: Option<ConferenceSolutionKey>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ConferenceStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConferenceSolutionKey {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConferenceStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "statusCode"
    )]
    pub status_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EntryPoint {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "entryPointType"
    )]
    pub entry_point_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub uri: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub label: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub pin: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "accessCode"
    )]
    pub access_code: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "meetingCode"
    )]
    pub meeting_code: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub passcode: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConferenceSolution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<ConferenceSolutionKey>,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub name: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "iconUri"
    )]
    pub icon_uri: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventGadget {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub r#type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub title: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub link: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "iconLink"
    )]
    pub icon_link: String,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize"
    )]
    pub width: i64,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize"
    )]
    pub height: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub display: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventReminders {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "useDefault"
    )]
    pub use_default: Option<bool>,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub overrides: Vec<EventDefaultReminder>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventSource {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub url: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WorkingLocationProperties {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub r#type: String,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "homeOffice"
    )]
    pub home_office: Option<serde_json::Value>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "customLocation"
    )]
    pub custom_location: Option<CustomLocation>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "officeLocation"
    )]
    pub office_location: Option<OfficeLocation>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CustomLocation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub label: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OfficeLocation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "buildingId"
    )]
    pub building_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "floorId"
    )]
    pub floor_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "floorSectionId"
    )]
    pub floor_section_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "deskId"
    )]
    pub desk_id: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub label: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OutOfOfficeProperties {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "autoDeclineMode"
    )]
    pub auto_decline_mode: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "declineMessage"
    )]
    pub decline_message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FocusTimeProperties {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "autoDeclineMode"
    )]
    pub auto_decline_mode: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "declineMessage"
    )]
    pub decline_message: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "chatStatus"
    )]
    pub chat_status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventAttachment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "fileUrl"
    )]
    pub file_url: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub title: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "mimeType"
    )]
    pub mime_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "iconLink"
    )]
    pub icon_link: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "fileId"
    )]
    pub file_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BirthdayProperties {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub contact: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub r#type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "customTypeName"
    )]
    pub custom_type_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventList {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "accessRole"
    )]
    pub access_role: String,
    /**
     * The default reminders that the authenticated user has for this calendar.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize",
        rename = "defaultReminders"
    )]
    pub default_reminders: Vec<EventDefaultReminder>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of events on the calendar.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub items: Vec<Event>,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub kind: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "nextSyncToken"
    )]
    pub next_sync_token: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub summary: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "timeZone"
    )]
    pub time_zone: String,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize",
        serialize_with = "crate::utils::serialize::deserialize_date_time_format::serialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct PatchEventRequest {
    /// The (exclusive) end time of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<EventDateTime>,

    /// The (inclusive) start time of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<EventDateTime>,

    /// Whether anyone can invite themselves to the event (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anyone_can_add_self: Option<bool>,

    /// The attendees of the event
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attendees: Vec<EventAttendee>,

    /// Birthday event properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_properties: Option<BirthdayProperties>,

    /// The color ID of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,

    /// Conference-related information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<ConferenceData>,

    /// Description of the event (can contain HTML)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Event type (default, focusTime, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Extended properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,

    /// Focus time properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_time_properties: Option<FocusTimeProperties>,

    /// Gadget information (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gadget: Option<EventGadget>,

    /// Whether attendees can invite others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_invite_others: Option<bool>,

    /// Whether attendees can modify the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_modify: Option<bool>,

    /// Whether attendees can see other guests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_see_other_guests: Option<bool>,

    /// Opaque identifier of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Geographic location of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Out of office properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_office_properties: Option<OutOfOfficeProperties>,

    /// Recurrence rules
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub recurrence: Vec<String>,

    /// Reminder settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<EventReminders>,

    /// Sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    /// Source information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,

    /// Event status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Event summary/title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Whether the event blocks time on the calendar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<String>,

    /// Visibility of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,

    /// Working location properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_location_properties: Option<WorkingLocationProperties>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct CreateEventRequest {
    /// Required: The (exclusive) end time of the event
    pub end: EventDateTime,

    /// Required: The (inclusive) start time of the event
    pub start: EventDateTime,

    /// Whether anyone can invite themselves to the event (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anyone_can_add_self: Option<bool>,

    /// The attendees of the event
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attendees: Vec<EventAttendee>,

    /// Birthday event properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_properties: Option<BirthdayProperties>,

    /// The color ID of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,

    /// Conference-related information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<ConferenceData>,

    /// Description of the event (can contain HTML)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Event type (default, focusTime, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Extended properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,

    /// Focus time properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_time_properties: Option<FocusTimeProperties>,

    /// Gadget information (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gadget: Option<EventGadget>,

    /// Whether attendees can invite others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_invite_others: Option<bool>,

    /// Whether attendees can modify the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_modify: Option<bool>,

    /// Whether attendees can see other guests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_see_other_guests: Option<bool>,

    /// Opaque identifier of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Geographic location of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Out of office properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_office_properties: Option<OutOfOfficeProperties>,

    /// Recurrence rules
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub recurrence: Vec<String>,

    /// Reminder settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<EventReminders>,

    /// Sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    /// Source information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,

    /// Event status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Event summary/title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Whether the event blocks time on the calendar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<String>,

    /// Visibility of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,

    /// Working location properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_location_properties: Option<WorkingLocationProperties>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConferenceData {
    /// Conference solution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_solution: Option<ConferenceSolution>,

    /// Entry points
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub entry_points: Vec<EntryPoint>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExtendedProperties {
    /// Properties private to this calendar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<std::collections::HashMap<String, String>>,

    /// Properties shared with other calendars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<std::collections::HashMap<String, String>>,
}

impl CreateEventRequest {
    /// Creates a new instance of `ExtendedProperties` with empty maps
    pub fn new(start: EventDateTime, end: EventDateTime) -> Self {
        CreateEventRequest {
            start,
            end,
            anyone_can_add_self: None,
            attendees: vec![],
            birthday_properties: None,
            color_id: None,
            conference_data: None,
            description: None,
            event_type: None,
            extended_properties: None,
            focus_time_properties: None,
            gadget: None,
            guests_can_invite_others: None,
            guests_can_modify: None,
            guests_can_see_other_guests: None,
            id: None,
            location: None,
            out_of_office_properties: None,
            recurrence: vec![],
            reminders: None,
            sequence: None,
            source: None,
            status: None,
            summary: None,
            transparency: None,
            visibility: None,
            working_location_properties: None,
        }
    }
}
