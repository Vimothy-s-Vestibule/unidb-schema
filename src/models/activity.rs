//! User activity model (LLM-extracted events).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Activity extracted from messages or external content.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,

    /// Type: run, cycle, git_commit, took_job, etc.
    pub activity_type: String,
    /// Human-readable: "10km marathon", "Senior Engineer at Google".
    pub label: String,

    /// When the activity occurred (not extracted).
    pub occurred_at: Option<DateTime<Utc>>,

    // Source (at least one required)
    pub external_content_id: Option<Uuid>,
    pub message_id: Option<i64>,

    /// LLM reasoning.
    pub reasoning: Option<String>,
}
