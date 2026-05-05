use chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};

/// Discord message with processing pipeline metadata.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "messages")]
pub struct Message {
    #[ormlite(primary_key)]
    pub message_id: i64,
    pub channel_id: i64,
    pub sent_by: i64,

    pub content: String,
    pub sent_at: DateTime<Utc>,
    pub last_edited: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,

    // Threads & normal replies
    pub in_reply_to: Option<i64>,

    pub added_at: DateTime<Utc>,
}
