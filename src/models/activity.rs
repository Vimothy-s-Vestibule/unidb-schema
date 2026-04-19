//! Unified user activity model.
//! Covers both LLM-extracted events and Discord rich presence activities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::ActivitySource;

/// A user activity — either extracted by LLM from content, observed via
/// Discord rich presence, or manually added by an admin.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,

    /// Where this activity came from.
    pub source: ActivitySource,

    /// For discord_presence activities: the discord account observed.
    /// NULL for llm_extraction/manual.
    pub discord_user_id: Option<i64>,

    /// For llm_extraction/manual activities: the vestibule user.
    /// NULL for discord_presence.
    pub user_id: Option<Uuid>,

    /// Type: run, cycle, git_commit, took_job, game, spotify, streaming, etc.
    pub activity_type: String,
    /// Human-readable summary: "10km marathon", "Playing Elden Ring".
    pub label: String,

    /// Rich presence detail fields (from Discord presence, NULL for LLM-extracted).
    pub details: Option<String>,
    /// Third line (e.g., "Playing Solo").
    pub state: Option<String>,
    /// Stream URL, Spotify link, etc.
    pub url: Option<String>,

    /// When the activity happened/started (not when extracted).
    pub started_at: Option<DateTime<Utc>>,
    /// When the activity ended (NULL for point-in-time or still ongoing).
    pub ended_at: Option<DateTime<Utc>>,

    // Sources for LLM-extracted activities
    pub external_content_id: Option<Uuid>,
    pub message_id: Option<i64>,
    pub youtube_comment_id: Option<String>,
    /// LLM reasoning or manual justification.
    pub reasoning: Option<String>,

    /// Raw Discord presence data (only for source = discord_presence).
    pub raw_data: Option<serde_json::Value>,
}
