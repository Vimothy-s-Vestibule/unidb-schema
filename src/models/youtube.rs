use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ormlite::Model;
use uuid::Uuid;

use super::enums::YoutubeVideoBroadcastStatus;

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
pub struct YoutubeVideo {
    #[ormlite(primary_key)]
    pub video_id: String,
    pub channel_vestibule_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub published_at: DateTime<Utc>,
    pub total_views: i64,
    pub total_likes: i64,
    pub total_comments: i64,

    pub duration_seconds: i32,
    pub broadcast_status: YoutubeVideoBroadcastStatus,

    pub thumbnail_asset_id: Option<Uuid>,
    pub transcript_asset_id: Option<Uuid>,
    pub audio_asset_id: Option<Uuid>,
    pub keyword_tags: Option<Vec<String>>,

    pub added_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
pub struct YoutubeComment {
    #[ormlite(primary_key)]
    pub comment_id: String,
    pub video_id: String,

    pub author_raw_channel_id: String,
    pub author_channel_id: Option<Uuid>,

    pub content: String,
    pub like_count: i32,
    pub published_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,

    pub in_reply_to: Option<String>,
    pub added_at: Option<DateTime<Utc>>,
}
