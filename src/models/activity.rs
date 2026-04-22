//! User activity model (LLM-extracted events).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::ActivitySource;

/// Activity extracted from messages or external content.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,

    pub source: ActivitySource,

    /// Type: run, cycle, git_commit, took_job, etc.
    pub activity_type: String,
    /// Human-readable summary: "10km marathon", "Senior Engineer at Google".
    pub label: String,

    pub started_at: Option<DateTime<Utc>>,
    /// When the activity ended (NULL for point-in-time or still ongoing).
    pub ended_at: Option<DateTime<Utc>>,

    // Source (at least one required)
    pub external_content_id: Option<Uuid>,
    pub message_id: Option<i64>,
    pub youtube_comment_id: Option<String>,
    pub discord_presence_id: Option<Uuid>,

    /// The LLM inferred the activity
    pub reasoning: Option<String>,
}
