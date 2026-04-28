//! Discord message edit model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ormlite::Model;
use uuid::Uuid;

/// History of message edits.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
pub struct MessageEdit {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub message_id: i64,
    pub content: String,
    pub edited_at: DateTime<Utc>,
}
