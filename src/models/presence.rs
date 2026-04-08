use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// Discord presence status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PresenceStatus {
    #[default]
    Online,
    Idle,
    Dnd,
    Offline,
    Invisible,
}

/// Discord rich presence activity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PresenceActivityType {
    #[default]
    Playing,
    Streaming,
    Listening,
    Watching,
    Competing,
    Custom,
}

/// Tracks Discord online/offline status changes (event-sourced)
/// A new row is inserted each time a user's status changes
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserPresence {
    pub id: Uuid,
    pub user_id: i64,

    /// Core status: online, idle, dnd, offline, invisible
    pub status: PresenceStatus,

    /// Per-client breakdown (if available from Discord)
    pub desktop_status: Option<PresenceStatus>,
    pub mobile_status: Option<PresenceStatus>,
    pub web_status: Option<PresenceStatus>,

    /// When this status period started
    pub started_at: DateTime<Utc>,
    /// When this status period ended (NULL = current status)
    pub ended_at: Option<DateTime<Utc>>,
}

/// Tracks rich presence / activities (games, Spotify, streaming, etc.)
/// Event-sourced: only insert when activity starts, update ended_at when it ends
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserPresenceActivity {
    pub id: Uuid,
    pub user_id: i64,

    /// Activity type: playing, streaming, listening, watching, competing, custom
    pub activity_type: PresenceActivityType,

    /// Game name, song title, custom status text, etc.
    pub name: String,
    /// Secondary line (e.g., "In Menu", "by Artist")
    pub details: Option<String>,
    /// Third line (e.g., "Playing Solo", "In a party")
    pub state: Option<String>,

    /// Stream URL, Spotify link, etc.
    pub url: Option<String>,

    /// Large image URL if available
    pub large_image_url: Option<String>,
    /// Small image URL if available
    pub small_image_url: Option<String>,

    /// When this activity started
    pub started_at: DateTime<Utc>,
    /// When this activity ended (NULL = still active)
    pub ended_at: Option<DateTime<Utc>>,

    /// Raw activity data for anything we didn't anticipate
    pub raw_data: Option<serde_json::Value>,
}

/// Evidence of a user sending a message while showing as offline/invisible
/// Links messages to the presence record at that time
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ForcedOnlineEvidence {
    pub id: Uuid,
    pub user_id: i64,
    pub message_id: i64,

    /// What status they were showing when they sent the message (offline, invisible)
    pub displayed_status: PresenceStatus,

    /// Link to the presence record at that time
    pub presence_id: Option<Uuid>,

    pub detected_at: DateTime<Utc>,
}
