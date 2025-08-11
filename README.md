# Google workspace API client

[![Crates.io Version](https://img.shields.io/crates/v/google-workspace-apis)](https://crates.io/crates/google-workspace-apis)

This crate is an unofficial opinionated library
that unifies the Google Workspace API clients.
The current workspace crate landscape is highly fragmented,
with each API employing a distinct approach.
This library aims to provide a unified interface for all Google Workspace APIs.

Current supported APIs include: Google Tasks, Google Calendar, and Gmail.

## Example

```rust
    // If the last parameter is set to true the client
    // will automatically refresh the access token if it expires
    let new_client = GoogleClient::new(client_credentials, access_token, true);

    // Insert a task
    match TasksClient::new(client)
        .insert_task("{TASKLIST_ID}")
        .set_task_title("test api")
        .set_task_notes("hello")
        .request()
        .await;

    //Get a list of events from the primary calendar
    let events = CalendarEventsClient::new(client)
        .get_events("primary")
        .single_events(true)
        .event_type(EventType::Birthday)
        .max_results(10)
        .order_by(google_workspace_apis::calendar::events::requests::EventOrderBy::StartTime)
        //To avoid retrieving past events we set the time_min to now
        .time_min(chrono::Utc::now())
        //Since we retrieve single events add all birthdays for the next year
        .time_max(chrono::Utc::now() + chrono::Duration::days(365))
        .request()
        .await
        .unwrap();

```

## Supported APIs

I'm currently working on the following APIs (more will be added soon):

### Auth

- Get OAuth url
- Get Access token
- Refresh token

### Calendar

For the API documentation, see the [Calender API documentation](https://developers.google.com/workspace/calendar/api/guides/overview).

The actions that are currently supported by this crate are:

#### Events (calendar API)

- Get
- List
- Patch
- Delete

### Tasks

For the API documentation, see the [Tasks API documentation](https://developers.google.com/workspace/tasks/reference/rest).

The actions that are currently supported by this crate are:

#### Tasks

- Insert
- List
- Delete
- Patch

#### Tasklists

- Get

### Gmail

For the API documentation, see the [Gmail API documentation](https://developers.google.com/workspace/gmail/api/guides).

The actions that are currently supported by this crate are:

#### User messages (emails)

- Get
- List
- Delete
- Trash
- Untrash

## Features

To include the correct API client,
you need to define the feature in your `Cargo.toml` file:

```toml
google-workspaces-api = { version: "1.2", features = ["calendar", "tasks", "gmail"] }
```
