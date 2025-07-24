# Google workspace API client

This crate is an unofficial opinionated library that unifies the Google Workspace API clients.
The current workspace crate landscape is highly fragmented, with each API employing a distinct approach.
This library aims to provide a unified interface for all Google Workspace APIs.

## Supported APIs

I'm currently working on the following APIs (more will be added soon):

### Auth

- Get OAuth url
- Get Access token
- Refresh token

### Calendar (WIP)

For the API documentation, see the [Calender API documentation](https://developers.google.com/workspace/calendar/api/guides/overview).

The calls that are currently supported by this crate are:

#### GET

- Events (returns a list of events for a specified calendar)

### Tasks (WIP)

For the api documentation, see the [Tasks API documentation](https://developers.google.com/workspace/tasks/reference/rest).

The calls that are currently supported by this crate are:

#### GET

- Tasklists (returns a list of task lists for the authenticated user)

## Features

To include the correct API client, you need to define the feature in your `Cargo.toml` file:

```toml
google-workspaces-api = { version: "0.1", features = ["calendar", "tasks"] }
```
