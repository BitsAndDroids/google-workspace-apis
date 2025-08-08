#![allow(clippy::unwrap_used)]

use crate::{
    auth::client::{get_validity_token_secs, AccessToken, ClientCredentials, GoogleClient},
    calendar::{
        events::{
            requests::EventRequest,
            types::{EventAttendee, EventList},
        },
        prelude::{EventOrderBy, EventType},
    },
    utils::request::TimeRequestTrait,
};

use super::{requests::CalendarEventsClient, types::EventDateTime};
use anyhow::Error;
use chrono::{Duration, Utc};
use reqwest::Method;

fn dummy_creds() -> ClientCredentials {
    ClientCredentials {
        client_id: "cid".into(),
        client_secret: "secret".into(),
        redirect_uri: "https://example.com/cb".into(),
        refresh_token: "rtok".into(),
    }
}

fn dummy_access(expires_in_secs: i64) -> AccessToken {
    AccessToken {
        token_type: "Bearer".into(),
        access_token: "atok".into(),
        expires_in: expires_in_secs,
        refresh_token: "rtok".into(),
        refresh_token_expires_in: 3600,
        scope: "scope".into(),
    }
}

fn dummy_google_client_valid() -> GoogleClient {
    // long validity -> won't try to refresh during tests
    GoogleClient::new(
        dummy_creds(),
        dummy_access(60 * 60),
        /*auto_refresh_token=*/ false,
    )
}

fn sample_dt(date: &str) -> EventDateTime {
    EventDateTime {
        date: Some(date.to_string()),
        date_time: None,
        time_zone: None,
    }
}

#[test]
fn event_order_by_as_str() {
    assert_eq!(EventOrderBy::StartTime.as_str(), "startTime");
    assert_eq!(EventOrderBy::Updated.as_str(), "updated");
}

#[test]
fn event_type_as_str() {
    assert_eq!(EventType::Birthday.as_str(), "birthday");
    assert_eq!(EventType::Default.as_str(), "default");
    assert_eq!(EventType::FocusTime.as_str(), "focusTime");
    assert_eq!(EventType::FromGmail.as_str(), "fromGmail");
    assert_eq!(EventType::OutOfOffice.as_str(), "outOfOffice");
    assert_eq!(EventType::WorkingLocation.as_str(), "workingLocation");
}

#[test]
fn get_validity_token_secs_works_for_past_and_future() {
    let future = (Utc::now() + Duration::seconds(120)).to_rfc3339();
    let past = (Utc::now() - Duration::seconds(120)).to_rfc3339();

    assert!(get_validity_token_secs(&future) > 0);
    assert!(get_validity_token_secs(&past) < 0);
}

#[test]
fn google_client_validity_flag() {
    let gc = dummy_google_client_valid();
    assert!(gc.is_access_token_valid());
}

#[test]
fn get_events_sets_url_method_and_params() {
    let mut gc = dummy_google_client_valid();

    let builder = CalendarEventsClient::new(&mut gc)
        .get_events("primary")
        .single_events(true)
        .event_type(EventType::Birthday)
        .order_by(EventOrderBy::StartTime)
        .max_attendees(5)
        .show_hidden_invitations(true)
        .query("hello world");

    // We’re in the same module (sibling tests.rs), so we can examine private fields
    assert_eq!(
        builder.request.url,
        "https://www.googleapis.com/calendar/v3/calendars/primary/events"
    );
    assert_eq!(builder.request.method, Method::GET);

    let p = &builder.request.params;
    assert_eq!(p.get("singleEvents").map(String::as_str), Some("true"));
    assert_eq!(p.get("eventTypes").map(String::as_str), Some("birthday"));
    assert_eq!(p.get("orderBy").map(String::as_str), Some("startTime"));
    assert_eq!(p.get("maxAttendees").map(String::as_str), Some("5"));
    assert_eq!(
        p.get("showHiddenInvitations").map(String::as_str),
        Some("true")
    );
    assert_eq!(p.get("q").map(String::as_str), Some("hello world"));
}

#[test]
fn time_filters_are_serialized_as_rfc3339() {
    let mut gc = dummy_google_client_valid();

    let now = Utc::now();
    let later = now + Duration::days(1);

    let builder = CalendarEventsClient::new(&mut gc)
        .get_events("primary")
        .time_min(now)
        .time_max(later);

    let p = &builder.request.params;
    assert_eq!(p.get("timeMin").unwrap(), &now.to_rfc3339());
    assert_eq!(p.get("timeMax").unwrap(), &later.to_rfc3339());
}

#[test]
fn insert_event_builds_body_and_setters_apply() {
    let mut gc = dummy_google_client_valid();

    let start = sample_dt("2025-07-28");
    let end = sample_dt("2025-07-28");
    let attendees = vec![EventAttendee {
        email: "a@example.com".into(),
        ..EventAttendee {
            id: String::new(),
            email: String::new(),
            display_name: String::new(),
            organizer: None,
            self_: None,
            resource: None,
            optional: None,
            response_status: String::new(),
            comment: String::new(),
            additional_guests: 0,
        }
    }];

    let builder = CalendarEventsClient::new(&mut gc)
        .insert_event("cal_123", start.clone(), end.clone())
        .set_summary("My Summary")
        .set_description("Desc")
        .set_location("Somewhere")
        .set_attendees(attendees.clone())
        .set_type(EventType::Default)
        .set_color_id("5")
        .set_recurrence(vec!["RRULE:FREQ=DAILY".into()]);

    // URL + method
    assert_eq!(
        builder.request.url,
        "https://www.googleapis.com/calendar/v3/calendars/cal_123/events"
    );
    assert_eq!(builder.request.method, Method::POST);

    // Body (event) contents
    match builder.event.as_ref().unwrap() {
        EventRequest::Create(payload) => {
            assert_eq!(payload.start, start);
            assert_eq!(payload.end, end);
            assert_eq!(payload.summary.as_deref(), Some("My Summary"));
            assert_eq!(payload.description.as_deref(), Some("Desc"));
            assert_eq!(payload.location.as_deref(), Some("Somewhere"));
            assert_eq!(payload.color_id.as_deref(), Some("5"));
            assert_eq!(payload.attendees, attendees);
            assert_eq!(payload.recurrence, vec!["RRULE:FREQ=DAILY".to_string()]);
            assert_eq!(payload.event_type.as_deref(), Some("default"));
        }
        _ => panic!("expected CreateEventRequest"),
    }
}

#[test]
fn patch_event_setters_apply() {
    let mut gc = dummy_google_client_valid();

    let new_start = sample_dt("2026-02-01");
    let new_end = sample_dt("2026-02-02");

    let builder = CalendarEventsClient::new(&mut gc)
        .patch_event("primary", "evt_42")
        .set_summary("New title")
        .set_description("New desc")
        .set_location("Moon base")
        .set_color_id("9")
        .set_event_type(EventType::OutOfOffice)
        .set_guests_can_invite_others(true)
        .set_guests_can_modify(false)
        .set_guests_can_see_other_guests(true)
        .set_id("new-id")
        .set_recurrence(vec!["RRULE:FREQ=WEEKLY".into()])
        .set_sequence(7)
        .set_status("tentative")
        .set_transparancy("opaque")
        .set_visibility("private")
        .set_start(new_start.clone())
        .set_end(new_end.clone())
        .set_send_updates("all")
        .set_conference_data_version(1)
        .support_attachments(true)
        .set_max_attendees(3);

    // URL + method + params
    assert_eq!(
        builder.request.url,
        "https://www.googleapis.com/calendar/v3/calendars/primary/events/evt_42"
    );
    assert_eq!(builder.request.method, Method::PATCH);

    let p = &builder.request.params;
    assert_eq!(p.get("sendUpdates").map(String::as_str), Some("all"));
    assert_eq!(
        p.get("conferenceDataVersion").map(String::as_str),
        Some("1")
    );
    assert_eq!(
        p.get("supportAttachments").map(String::as_str),
        Some("true")
    );
    assert_eq!(p.get("maxAttendees").map(String::as_str), Some("3"));

    // Body (event) contents
    match builder.event.as_ref().unwrap() {
        EventRequest::Patch(payload) => {
            assert_eq!(payload.start.as_ref().unwrap(), &new_start);
            assert_eq!(payload.end.as_ref().unwrap(), &new_end);
            assert_eq!(payload.summary.as_deref(), Some("New title"));
            assert_eq!(payload.description.as_deref(), Some("New desc"));
            assert_eq!(payload.location.as_deref(), Some("Moon base"));
            assert_eq!(payload.color_id.as_deref(), Some("9"));
            assert_eq!(payload.event_type.as_deref(), Some("outOfOffice"));
            assert_eq!(payload.guests_can_invite_others, Some(true));
            assert_eq!(payload.guests_can_modify, Some(false));
            assert_eq!(payload.guests_can_see_other_guests, Some(true));
            assert_eq!(payload.id.as_deref(), Some("new-id"));
            assert_eq!(payload.recurrence, vec!["RRULE:FREQ=WEEKLY".to_string()]);
            assert_eq!(payload.sequence, Some(7));
            assert_eq!(payload.status.as_deref(), Some("tentative"));
            assert_eq!(payload.transparency.as_deref(), Some("opaque"));
            assert_eq!(payload.visibility.as_deref(), Some("private"));
        }
        _ => panic!("expected PatchEventRequest"),
    }
}

#[tokio::test]
async fn make_request_unsupported_method_errors() {
    let mut gc = dummy_google_client_valid();
    // Start from any mode; we'll get EventListMode by calling get_events
    let mut client = CalendarEventsClient::new(&mut gc).get_events("primary");
    // Force an unsupported method to exercise the error path without any network I/O
    client.request.method = Method::DELETE;

    let res: Result<Option<EventList>, Error> = client.make_request().await;
    assert!(res.is_err());
}
