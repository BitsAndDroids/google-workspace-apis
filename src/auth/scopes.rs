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
    //Drive
    DriveAppfolder,
    DriveInstall,
    DriveFile,
    DriveAppsReadonly,
    Drive,
    DriveReadonly,
    DriveActivity,
    DriveActivityReadonly,
    DriveMeetReadonly,
    DriveMetadata,
    DriveMetadataReadonly,
    DriveScripts,
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
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
            Scope::TasksReadOnly => "https://www.googleapis.com/auth/tasks.readonly",
            Scope::Tasks => "https://www.googleapis.com/auth/tasks",
            Scope::Mail => "https://mail.google.com",
            Scope::MailModify => "https://www.googleapis.com/auth/gmail.modify",
            Scope::MailReadonly => "https://www.googleapis.com/auth/gmail.readonly",
            Scope::MailMetadata => "https://www.googleapis.com/auth/gmail.metadata",
            Scope::DriveAppfolder => {
                "https://www.googleapis.com/auth/drive.appdata
https://www.googleapis.com/auth/drive.appfolder"
            }
            Scope::DriveInstall => "https://www.googleapis.com/auth/drive.install",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
            Scope::DriveAppsReadonly => "https://www.googleapis.com/auth/drive.apps.readonly",
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::DriveActivity => "https://www.googleapis.com/auth/drive.activity",
            Scope::DriveActivityReadonly => {
                "https://www.googleapis.com/auth/drive.activity.readonly"
            }
            Scope::DriveMeetReadonly => "https://www.googleapis.com/auth/drive.meet.readonly",
            Scope::DriveMetadata => "https://www.googleapis.com/auth/drive.metadata",
            Scope::DriveMetadataReadonly => {
                "https://www.googleapis.com/auth/drive.metadata.readonly"
            }
            Scope::DriveScripts => "https://www.googleapis.com/auth/drive.scripts",
        }
    }
}
