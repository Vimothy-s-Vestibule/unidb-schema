//! User presence tracking models.
//! Unified table for both online/offline status changes and rich presence activities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::PresenceStatus;

/// A single presence record — either a status change or a rich presence activity.
///
/// When `presence_type` is `"status"`: `status` is set, activity fields are NULL.
/// When `presence_type` is `"activity"`: `activity_type` + `name` are set, `status` is NULL.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Presence {
    pub id: Uuid,
    pub user_id: i64,

    /// `"status"` or `"activity"`.
    pub presence_type: String,

    /// Online/offline status (only for presence_type = "status").
    pub status: Option<PresenceStatus>,

    /// Maps to serenity's ActivityType: Playing, Streaming, Listening, Watching, Custom, Competing
    pub activity_type: Option<String>,

    /// Game name, song title, etc. (only for activities).
    pub name: Option<String>,
    /// Secondary line (e.g., "In Menu", "by Artist").
    pub details: Option<String>,
    /// Third line (e.g., "Playing Solo").
    pub state: Option<String>,
    /// Stream URL, Spotify link, etc.
    pub url: Option<String>,

    /// When this presence started.
    pub started_at: DateTime<Utc>,
    /// NULL = current/still active.
    pub ended_at: Option<DateTime<Utc>>,
}

/// Detects and logs as ForcedOnline using messages the user sent while showing as offline/invisible.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ForcedOnlineEvidence {
    pub id: Uuid,
    pub user_id: i64,
    pub message_id: i64,
    pub presence_id: Uuid,
    pub detected_at: DateTime<Utc>,
}
