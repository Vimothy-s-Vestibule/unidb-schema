use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::social::ProcessingStatus;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Default)]
pub struct DiscordMessage {
    pub message_id: i64,
    pub channel_id: i64,
    pub user_id: i64,

    pub content: String,

    pub sent_at: DateTime<Utc>,
    pub added_at: DateTime<Utc>,
    pub last_edited: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub in_reply_to: Option<i64>,
    pub score_id: Option<String>,

    pub triage_status: Option<ProcessingStatus>,
    pub is_significant: Option<bool>,
    pub skill_status: Option<ProcessingStatus>,
    pub personality_status: Option<ProcessingStatus>,
    pub processed_at: Option<DateTime<Utc>>,
}
