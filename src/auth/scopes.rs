pub enum Scope {
    //Calendar
    Calendar,
    CalendarEvents,
    CalendarEventsReadonly,
    CalendarReadOnly,
    CalendarAppCreated,
    CalendarEventsFreeBusy,
    CalendarEventsOwned,
    CalendarEventsOwnedReadonly,
    CalendarEventsPublicReadonly,
    //Tasks
    TasksReadOnly,
    Tasks,
    //gmail
    Mail,
    MailModify,
    MailReadonly,
    MailMetadata,
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            //Calendar
            Scope::Calendar => "https://www.googleapis.com/auth/calendar",
            Scope::CalendarEventsReadonly => {
                "https://www.googleapis.com/auth/calendar.events.readonly"
            }
            Scope::CalendarAppCreated => "https://www.googleapis.com/auth/calendar.app.created",
            Scope::CalendarEventsFreeBusy => {
                "https://www.googleapis.com/auth/calendar.events.freebusy"
            }
            Scope::CalendarEventsOwned => "https://www.googleapis.com/auth/calendar.events.owned",
            Scope::CalendarEventsOwnedReadonly => {
                "https://www.googleapis.com/auth/calendar.events.owned.readonly"
            }
            Scope::CalendarEventsPublicReadonly => {
                "https://www.googleapis.com/auth/calendar.readonly"
            }
            Scope::CalendarReadOnly => "https://www.googleapis.com/auth/calendar.readonly",
            Scope::CalendarEvents => "https://www.googleapis.com/auth/calendar.events",
            //Tasks
            Scope::TasksReadOnly => "https://www.googleapis.com/auth/tasks.readonly",
            Scope::Tasks => "https://www.googleapis.com/auth/tasks",
            //GMAIL
            Scope::Mail => "https://mail.google.com",
            Scope::MailModify => "https://www.googleapis.com/auth/gmail.modify",
            Scope::MailReadonly => "https://www.googleapis.com/auth/gmail.readonly",
            Scope::MailMetadata => "https://www.googleapis.com/auth/gmail.metadata",
        }
    }
}
