//! User presence and activity tracking models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::PresenceStatus;

/// Discord online/offline status change (event-sourced).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Presence {
    pub id: Uuid,
    pub user_id: i64,

    /// Overall status.
    pub status: PresenceStatus,

    /// The first time the bot picked up that this status was happening (TODO can we get actual activity begin time from serenity)
    pub started_at: DateTime<Utc>,
    /// NULL = current status.
    pub ended_at: Option<DateTime<Utc>>,
}

/// Rich presence activity (games, Spotify, streaming).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PresenceActivity {
    pub id: Uuid,
    pub user_id: i64,

    pub activity_type: String,

    /// Game name, song title, etc.
    pub name: String,
    /// Secondary line (e.g., "In Menu").
    pub details: Option<String>,
    /// Third line (e.g., "Playing Solo").
    pub state: Option<String>,

    /// Stream or Spotify URL
    pub url: Option<String>,

    /// Time range.
    pub started_at: DateTime<Utc>,
    /// None (NULL) = still ongoing
    pub ended_at: Option<DateTime<Utc>>,

    /// Raw data extracted from discord (might be useful later)
    pub raw_data: Option<serde_json::Value>,
}

/// Detects and logs as ForcedOnline using messages the user sent while showing as offline/invisible. TODO For how long does the user get forced online, and TODO can we make it so when they join a VC while being set to offline it also sets them to ForcedOnline?
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ForcedOnlineEvidence {
    pub id: Uuid,
    pub user_id: i64,
    pub message_id: i64,
    pub presence_id: Uuid,
    pub detected_at: DateTime<Utc>,
}
