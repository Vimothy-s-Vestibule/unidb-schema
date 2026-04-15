use chrono::{DateTime, Utc};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::{ProcessingStatus, YoutubeVideoBroadcastStatus};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct YoutubeChannel {
    pub channel_id: String,
    pub display_name: String,
    pub username: String,
    pub bio_location: Option<String>,
    pub vestibule_user_id: Option<String>,
    pub bio: Option<String>,
    pub profile_picture_asset_id: Option<Uuid>,
    pub name_embedding: Option<Vector>,
    pub score_id: Option<Uuid>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct YoutubeVideo {
    pub video_id: String,
    pub channel_id: String,
    pub title: String,
    pub description: Option<String>,
    pub published_at: DateTime<Utc>,
    pub total_views: i64,
    pub total_likes: i64,
    pub total_comments: i64,

    pub duration_seconds: i32,
    pub broadcast_status: YoutubeVideoBroadcastStatus,

    pub thumbnail_asset_id: Option<Uuid>,
    pub keyword_tags: Option<Vec<String>>,

    pub added_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct YoutubeComment {
    pub comment_id: String,
    pub video_id: String,

    pub author_raw_channel_id: String,
    pub author_channel_id: Option<String>,

    pub content: String,
    pub like_count: i32,
    pub published_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    pub in_reply_to: Option<String>,

    pub triage_status: ProcessingStatus,
    pub is_significant: bool,
    pub skill_status: Option<ProcessingStatus>,
    pub personality_status: Option<ProcessingStatus>,
    pub processed_at: Option<DateTime<Utc>>,
}
