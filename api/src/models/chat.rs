use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCount {
    pub total: i64,
    pub breakdown: ChatCountBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCountBreakdown {
    #[serde(rename = "SMS")]
    pub sms: i64,
    #[serde(rename = "iMessage")]
    pub imessage: i64,
    #[serde(rename = "RCS")]
    pub rcs: i64,
}

/// Represents a chat conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    #[serde(rename = "originalROWID")]
    pub original_row_id: i64,
    pub guid: String,
    pub style: i64,
    pub chat_identifier: String,
    pub is_archived: bool,
    pub display_name: String,
    #[serde(default)]
    pub participants: Vec<Participant>,
    pub is_filtered: Option<bool>,
    pub group_id: Option<String>,
    #[serde(default)]
    pub properties: Vec<ChatProperty>,
    pub last_addressed_handle: String,
    pub last_message: Option<Message>,
    #[serde(default)]
    pub messages: Vec<Message>,
}

/// Represents a participant in a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    #[serde(rename = "originalROWID")]
    pub original_row_id: i64,
    pub address: String,
    pub service: String,
    pub uncanonicalized_id: Option<String>,
    pub country: String,
}

/// Represents properties of a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatProperty {
    pub pv: Option<i64>,
    pub number_of_times_responded_to_thread: Option<i64>,
    pub last_seen_message_guid: Option<String>,
    #[serde(rename = "LSMD")]
    pub lsmd: Option<DateTime<Utc>>,
    pub should_force_to_sms: Option<bool>,
    pub has_been_auto_spam_reported: Option<bool>,
}

/// Represents a message in a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "originalROWID")]
    pub original_row_id: i64,
    pub guid: String,
    pub text: Option<String>,
    pub attributed_body: Option<String>,
    // pub handle: Option<String>,
    pub handle_id: i64,
    pub other_handle: i64,
    pub attachments: Vec<Attachment>,
    pub subject: Option<String>,
    pub error: i64,
    pub date_created: i64,
    pub date_read: Option<i64>,
    pub date_delivered: Option<i64>,
    pub is_delivered: bool,
    pub is_from_me: bool,
    pub has_dd_results: bool,
    pub is_archived: bool,
    pub item_type: i64,
    pub group_title: Option<String>,
    pub group_action_type: i64,
    pub balloon_bundle_id: Option<String>,
    pub associated_message_guid: Option<String>,
    pub associated_message_type: Option<String>,
    pub expressive_send_style_id: Option<String>,
    pub thread_originator_guid: Option<String>,
    pub has_payload_data: bool,
    pub country: Option<String>,
    pub is_delayed: bool,
    pub is_auto_reply: bool,
    pub is_system_message: bool,
    pub is_service_message: bool,
    pub is_forward: bool,
    pub thread_originator_part: Option<String>,
    pub is_corrupt: bool,
    pub date_played: Option<i64>,
    pub cache_roomnames: Option<String>,
    pub is_spam: bool,
    pub is_expired: bool,
    pub time_expressive_send_played: Option<i64>,
    pub is_audio_message: bool,
    pub reply_to_guid: Option<String>,
    pub share_status: i64,
    pub share_direction: i64,
    pub was_delivered_quietly: bool,
    pub did_notify_recipient: bool,
    pub chats: Vec<Chat>,
    pub message_summary_info: Option<String>,
    pub payload_data: Option<String>,
    pub date_edited: Option<i64>,
    pub date_retracted: Option<i64>,
    pub part_count: i64,
}

/// Represents an attachment in a message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    // Placeholder struct - fields can be added when attachment structure is known
}
