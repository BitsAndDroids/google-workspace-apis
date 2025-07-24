use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TaskLists {
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
     * List of tasklists of the authenticated user.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub items: Vec<TaskList>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TaskList {
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
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub etag: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize::deserialize_date_time_format::deserialize",
        serialize_with = "crate::utils::serialize::deserialize_date_time_format::serialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "selfLink"
    )]
    pub self_link: String,
}
