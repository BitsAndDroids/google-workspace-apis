use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Message {
    /**
     * The immutable ID of the message.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub id: String,

    /**
     * The ID of the thread the message belongs to.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "threadId"
    )]
    pub thread_id: String,

    /**
     * List of IDs of labels applied to this message.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize",
        rename = "labelIds"
    )]
    pub label_ids: Vec<String>,

    /**
     * A short part of the message text.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub snippet: String,

    /**
     * The ID of the last history record that modified this message.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "historyId"
    )]
    pub history_id: String,

    /**
     * The internal message creation timestamp (epoch ms), which determines ordering in the inbox.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "internalDate"
    )]
    pub internal_date: String,

    /**
     * The parsed email structure in the message parts.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<MessagePart>,

    /**
     * Estimated size in bytes of the message.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize",
        rename = "sizeEstimate"
    )]
    pub size_estimate: i64,

    /**
     * The entire email message in an RFC 2822 formatted and base64url encoded string.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub raw: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MessagePart {
    /**
     * The immutable ID of the message part.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "partId"
    )]
    pub part_id: String,

    /**
     * The MIME type of the message part.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "mimeType"
    )]
    pub mime_type: String,

    /**
     * The filename of the attachment. Only present if this message part represents an attachment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub filename: String,

    /**
     * List of headers on this message part.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub headers: Vec<Header>,

    /**
     * The message part body for this part, which may be empty for container MIME message parts.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<MessagePartBody>,

    /**
     * The child MIME message parts of this part. This only applies to container MIME message parts.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub parts: Vec<MessagePart>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Header {
    /**
     * The name of the header before the : separator. For example, To.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub name: String,

    /**
     * The value of the header after the : separator. For example, someuser@example.com.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MessagePartBody {
    /**
     * The body data of the message part as a base64url encoded string.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize"
    )]
    pub data: String,

    /**
     * Size of the body data.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize"
    )]
    pub size: i64,

    /**
     * When present, contains the ID of the attachment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "attachmentId"
    )]
    pub attachment_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MessageList {
    /**
     * List of messages.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_vec::deserialize"
    )]
    pub messages: Vec<Message>,

    /**
     * Token to retrieve the next page of results.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,

    /**
     * Estimated total number of results.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::validation::zero_i64",
        deserialize_with = "crate::utils::deserialize::deserialize_nullable_i64::deserialize",
        rename = "resultSizeEstimate"
    )]
    pub result_size_estimate: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct ModifyMessageRequest {
    /**
     * Label IDs to add to the message.
     */
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "addLabelIds")]
    pub add_label_ids: Vec<String>,

    /**
     * Label IDs to remove from the message.
     */
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        rename = "removeLabelIds"
    )]
    pub remove_label_ids: Vec<String>,
}

impl MessagePartBody {
    pub fn new() -> Self {
        MessagePartBody {
            data: String::new(),
            size: 0,
            attachment_id: String::new(),
        }
    }
}

impl Default for MessagePartBody {
    fn default() -> Self {
        Self::new()
    }
}

impl MessagePart {
    pub fn new() -> Self {
        MessagePart {
            part_id: String::new(),
            mime_type: String::new(),
            filename: String::new(),
            headers: Vec::new(),
            body: None,
            parts: Vec::new(),
        }
    }
}

impl Default for MessagePart {
    fn default() -> Self {
        Self::new()
    }
}

pub enum GetMessageFormat {}
//TODO: finish format enum https://developers.google.com/workspace/gmail/api/reference/rest/v1/Format
