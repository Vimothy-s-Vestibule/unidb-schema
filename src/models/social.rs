use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    #[default]
    Idle,
    Running,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PlatformDataAccessPolicy {
    #[default]
    Public,
    OauthRequired,
    // Means: There is no API but people still mentioned an account they have on this platform
    Unavailable,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SocialPlatform {
    pub id: Uuid,
    pub platform_name: String,
    pub homepage: String,
    pub access: PlatformDataAccessPolicy,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserPlatformAssociation {
    pub id: Uuid,
    pub vestibule_user_id: i64,
    pub platform_id: Uuid,

    pub platform_username: String,
    pub platform_display_name: String,
    pub profile_url: Option<String>,

    pub platform_association_mention_message_id: i64,
    pub reasoning: String,

    // Sync scheduling
    pub sync_interval_hours: Option<i32>,
    pub next_sync_at: Option<DateTime<Utc>>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub sync_status: Option<SyncStatus>,
    pub last_sync_error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProcessingStatus {
    #[default]
    Pending,
    Processing,
    Complete,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ExternalContent {
    pub id: Uuid,
    pub platform_association_id: Uuid,

    pub content_type: String,
    pub external_id: String,
    pub raw_data: serde_json::Value,
    pub content_hash: Option<String>,

    pub fetched_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub triage_status: Option<ProcessingStatus>,
    pub is_significant: Option<bool>,
    pub skill_status: Option<ProcessingStatus>,
    pub personality_status: Option<ProcessingStatus>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Uuid,
    pub user_id: i64,

    pub activity_type: String,
    pub label: String,
    pub data: Option<serde_json::Value>,
    pub occurred_at: Option<DateTime<Utc>>,

    pub external_content_id: Option<Uuid>,
    pub message_id: Option<i64>,

    pub confidence: Option<f32>,
    pub reasoning: Option<String>,
    pub extracted_at: Option<DateTime<Utc>>,
}
