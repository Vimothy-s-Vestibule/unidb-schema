//! External platform integration models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::{PlatformAccess, PlatformSyncStatus};

/// External social platform (GitHub, Strava, Spotify, etc.).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Platform {
    pub id: Uuid,
    pub platform_name: String,
    pub homepage: String,
    pub access_type: PlatformAccess,
}

/// Link between a user and their account on an external platform.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserPlatformLink {
    pub id: Uuid,
    pub vestibule_user_id: Uuid,
    pub platform_id: Uuid,

    pub platform_username: String,
    pub platform_display_name: String,
    pub profile_url: Option<String>,

    /// Message where this link was discovered.
    pub platform_association_mention_message_id: Option<i64>,
    /// Why we linked this account.
    pub reasoning: String,

    // Sync state
    pub last_synced_at: Option<DateTime<Utc>>,
    pub last_sync_error: Option<String>,
    pub sync_status: Option<PlatformSyncStatus>,
}

/// Raw content fetched from an external platform.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ExternalContent {
    pub id: Uuid,
    pub platform_association_id: Uuid,

    /// Content type: github_repo, strava_activity, etc.
    pub content_type: String,
    /// Platform's unique ID for this content.
    pub external_id: String,

    /// Full API response.
    pub raw_data: serde_json::Value,
    /// Hash for change detection.
    pub content_hash: Option<String>,

    pub fetched_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
