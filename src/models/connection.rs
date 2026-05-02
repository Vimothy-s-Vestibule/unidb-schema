//! External platform connection models.

use chrono::{DateTime, Utc};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use ormlite::Model;
use uuid::Uuid;

use super::enums::PlatformAccess;

/// An external platform that users can connect to (e.g., GitHub, Spotify, Strava).
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "social_platforms")]
pub struct Platform {
    pub id: Uuid,
    pub platform_name: String,
    pub homepage: String,
    pub access_type: PlatformAccess,
    pub logo: Option<Uuid>,
}

/// A user's linked account on a specific platform.
/// Sync state is tracked exclusively via the `jobs` table.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "connected_accounts")]
pub struct ConnectedAccount {
    pub id: Uuid,
    pub vestibule_user_id: Uuid,
    pub platform_id: Uuid,

    pub platform_user_id: Option<String>,
    pub platform_display_name: String,
    pub platform_username: String,

    pub bio: Option<String>,
    pub bio_additional_info: Option<String>,
    pub profile_picture: Option<Uuid>,
    pub name_embedding: Option<Vector>,

    /// Message where this link was discovered.
    pub mention_message_id: Option<i64>,
    /// Why we linked this account.
    pub reasoning: String,
}

/// Raw content fetched from an external platform via a connected account.
/// YouTube content should NOT go here — use youtube_videos/youtube_comments instead.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "external_content")]
pub struct ExternalContent {
    pub id: Uuid,
    pub account_id: Uuid,

    /// Content type: github_repo, strava_activity, etc.
    /// Must NOT be youtube_video, youtube_comment, or youtube_channel.
    pub content_type: String,

    /// Full API response.
    pub raw_data: serde_json::Value,

    /// Hash for change detection.
    pub content_hash: Option<String>,

    pub fetched_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
