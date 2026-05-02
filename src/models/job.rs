use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ormlite::Model;
use uuid::Uuid;

use super::enums::{JobStatus, JobType};

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "jobs")]
pub struct Job {
    pub id: Uuid,
    pub job_type: JobType,
    pub status: JobStatus,
    pub locked_by_worker_id: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub scheduled_for: DateTime<Utc>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub last_error: Option<String>,

    // The table item this job seeks to update
    pub youtube_video_id: Option<String>,
    pub discord_channel_id: Option<i64>,
    pub connected_account_id: Option<Uuid>,
    pub media_asset_id: Option<Uuid>,
}
