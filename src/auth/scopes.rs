pub enum Scope {
    CalendarReadOnly,
    CalendarEvents,
    TasksReadOnly,
    Tasks,
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Scope::CalendarReadOnly => "https://www.googleapis.com/auth/calendar.readonly",
            Scope::CalendarEvents => "https://www.googleapis.com/auth/calendar.events",
            Scope::TasksReadOnly => "https://www.googleapis.com/auth/tasks.readonly",
            Scope::Tasks => "https://www.googleapis.com/auth/tasks",
        }
    }
}
