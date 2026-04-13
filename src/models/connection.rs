//! External platform connection models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::{PlatformAccess, PlatformSyncStatus};

/// An external platform that users can connect to (e.g., GitHub, Spotify, Strava).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Platform {
    pub id: Uuid,
    pub platform_name: String,
    pub homepage: String,
    pub access_type: PlatformAccess,
}

/// A user's linked account on a specific platform.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConnectedAccount {
    pub id: Uuid,
    pub vestibule_user_id: Uuid,
    pub platform_id: Uuid,

    pub platform_username: String,
    pub platform_display_name: String,
    pub profile_url: Option<String>,

    /// Message where this link was discovered.
    pub mention_message_id: Option<i64>,
    /// Why we linked this account.
    pub reasoning: String,

    // Sync state
    pub last_synced_at: Option<DateTime<Utc>>,
    pub last_sync_error: Option<String>,
    pub sync_status: PlatformSyncStatus,
}

/// Raw content fetched from an external platform via a connected account.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ExternalContent {
    pub id: Uuid,
    pub account_id: Uuid,

    /// Content type: github_repo, strava_activity, etc.
    pub content_type: String,

    /// Full API response.
    pub raw_data: serde_json::Value,

    /// Hash for change detection.
    pub content_hash: Option<String>,

    pub fetched_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
