use chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// History of message edits.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "message_edits")]
pub struct MessageEdit {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub message_id: i64,
    pub old_content: String,
    pub edited_at: DateTime<Utc>,
}
