//! User presence and activity tracking models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::{ActivityType, PresenceStatus};

/// Discord online/offline status change (event-sourced).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Presence {
    pub id: Uuid,
    pub user_id: i64,

    /// Overall status: online, idle, dnd, offline, invisible.
    pub status: PresenceStatus,

    /// Per-client status breakdown.
    pub desktop_status: Option<PresenceStatus>,
    pub mobile_status: Option<PresenceStatus>,
    pub web_status: Option<PresenceStatus>,

    /// Time range this status was active.
    pub started_at: DateTime<Utc>,
    /// NULL = current status.
    pub ended_at: Option<DateTime<Utc>>,
}

/// Rich presence activity (games, Spotify, streaming).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PresenceActivity {
    pub id: Uuid,
    pub user_id: i64,

    pub activity_type: ActivityType,

    /// Game name, song title, custom status, etc.
    pub name: String,
    /// Secondary line (e.g., "In Menu").
    pub details: Option<String>,
    /// Third line (e.g., "Playing Solo").
    pub state: Option<String>,

    /// Stream or Spotify URL.
    pub url: Option<String>,

    /// Activity images.
    pub large_image_url: Option<String>,
    pub small_image_url: Option<String>,

    /// Time range.
    pub started_at: DateTime<Utc>,
    /// NULL = still active.
    pub ended_at: Option<DateTime<Utc>>,

    /// Extra data for future use.
    pub raw_data: Option<serde_json::Value>,
}

/// Evidence of activity while showing offline/invisible.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ForcedOnlineEvidence {
    pub id: Uuid,
    pub user_id: i64,
    pub message_id: i64,

    /// Status shown when message was sent.
    pub displayed_status: PresenceStatus,
    /// Link to presence record at that time.
    pub presence_id: Option<Uuid>,

    pub detected_at: DateTime<Utc>,
}
