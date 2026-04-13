//! Discord message model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::ProcessingStatus;

/// Discord message with processing pipeline metadata.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub channel_id: i64,
    pub sent_by: Option<i64>,

    pub content: String,

    // Timestamps
    pub sent_at: DateTime<Utc>,
    pub added_at: DateTime<Utc>,
    pub last_edited: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,

    // Threading
    pub in_reply_to: Option<i64>,

    // Per-message personality score
    pub score_id: Option<Uuid>,

    // Processing pipeline things
    pub added_at: DateTime<Utc>,
    pub triage_status: ProcessingStatus,
    pub is_significant: bool,
    pub skill_status: Option<ProcessingStatus>,
    pub personality_status: Option<ProcessingStatus>,
    pub processed_at: Option<DateTime<Utc>>,
}
