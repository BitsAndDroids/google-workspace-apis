use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Tasks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub kind: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub etag: String,
    /**
     * Token used to access the next page of this result.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
    /**
     * List of tasks of the authenticated user.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub items: Vec<Task>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Task {
    /**
     * Output only. Type of the resource. This is always "tasks#task".
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub kind: String,
    /**
     * Task identifier.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub id: String,
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
     * Title of the task. Maximum length allowed: 1024 characters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub title: String,
    /**
     * Output only. Last modification time of the task (as a RFC 3339 timestamp).
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize",
        serialize_with = "crate::utils::serialize::deserialize_date_time_format::serialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Output only. URL pointing to this task. Used to retrieve, update, or delete this task.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "selfLink"
    )]
    pub self_link: String,
    /**
     * Output only. Parent task identifier. This field is omitted if it is a top-level task.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub parent: String,
    /**
     * Output only. String indicating the position of the task among its sibling tasks.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub position: String,
    /**
     * Notes describing the task. Maximum length allowed: 8192 characters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub notes: String,
    /**
     * Status of the task. This is either "needsAction" or "completed".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub status: String,
    /**
     * Due date of the task (as a RFC 3339 timestamp).
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize",
        serialize_with = "crate::utils::serialize::deserialize_date_time_format::serialize"
    )]
    pub due: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Completion date of the task (as a RFC 3339 timestamp).
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize",
        serialize_with = "crate::utils::serialize::deserialize_date_time_format::serialize"
    )]
    pub completed: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Flag indicating whether the task has been deleted.
     */
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub deleted: bool,
    /**
     * Flag indicating whether the task is hidden.
     */
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
    /**
     * Output only. Collection of links. This collection is read-only.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub links: Vec<TaskLink>,
    /**
     * Output only. An absolute link to the task in the Google Tasks Web UI.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "webViewLink"
    )]
    pub web_view_link: String,
    /**
     * Output only. Context information for assigned tasks.
     */
    #[serde(default, skip_serializing, rename = "assignmentInfo")]
    pub assignment_info: Option<AssignmentInfo>,
}
impl Task {
    pub fn new() -> Self {
        Task {
            kind: String::new(),
            id: String::new(),
            etag: String::new(),
            title: String::new(),
            updated: None,
            self_link: String::new(),
            parent: String::new(),
            position: String::new(),
            notes: String::new(),
            status: "needsAction".to_string(), // Default status
            due: None,
            completed: None,
            deleted: false,
            hidden: false,
            links: Vec::new(),
            web_view_link: String::new(),
            assignment_info: None,
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TaskLink {
    /**
     * Type of the link, e.g. "email", "generic", "chat_message", "keep_note".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub r#type: String,
    /**
     * The description (might be empty).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub description: String,
    /**
     * The URL.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub link: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AssignmentInfo {
    /**
     * Output only. An absolute link to the original task in the surface of assignment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "linkToTask"
    )]
    pub link_to_task: String,
    /**
     * Output only. The type of surface this assigned task originates from.
     */
    #[serde(rename = "surfaceType")]
    pub surface_type: ContextType,
    /**
     * Output only. Information about the Drive file where this task originates from.
     */
    #[serde(default, skip_serializing, rename = "driveResourceInfo")]
    pub drive_resource_info: Option<DriveResourceInfo>,
    /**
     * Output only. Information about the Chat Space where this task originates from.
     */
    #[serde(default, skip_serializing, rename = "spaceInfo")]
    pub space_info: Option<SpaceInfo>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ContextType {
    #[serde(rename = "CONTEXT_TYPE_UNSPECIFIED")]
    ContextTypeUnspecified,
    #[serde(rename = "GMAIL")]
    Gmail,
    #[serde(rename = "DOCUMENT")]
    Document,
    #[serde(rename = "SPACE")]
    Space,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DriveResourceInfo {
    /**
     * Output only. Identifier of the file in the Drive API.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "driveFileId"
    )]
    pub drive_file_id: String,
    /**
     * Output only. Resource key required to access files shared via a shared link.
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "resourceKey"
    )]
    pub resource_key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SpaceInfo {
    /**
     * Output only. The Chat space where this task originates from. The format is "spaces/{space}".
     */
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub space: String,
}
