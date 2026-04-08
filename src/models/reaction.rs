//! Message reaction model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Emoji reaction on a Discord message.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Reaction {
    pub id: Uuid,
    pub message_id: i64,
    pub user_id: i64,

    /// Unicode emoji or custom emoji snowflake ID.
    pub emoji: String,
    /// Display name (for custom emojis).
    pub emoji_name: Option<String>,
    /// True if custom emoji (vs unicode).
    pub is_custom: bool,
    /// True if animated GIF (only for custom).
    pub is_animated: bool,
    /// CDN URL for custom emojis.
    pub emoji_url: Option<String>,

    pub reacted_at: DateTime<Utc>,
}
