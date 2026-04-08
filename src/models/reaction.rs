use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MessageReaction {
    pub id: Uuid,
    pub message_id: i64,
    pub user_id: i64,

    /// Unicode emoji string or custom emoji snowflake ID
    pub emoji: String,
    /// Display name (useful for custom emojis)
    pub emoji_name: Option<String>,
    /// Whether this is a custom emoji (or unicode)
    pub is_custom: bool,
    /// Whether custom emoji is animated (GIF or PNG,  only relevant when is_custom = true)
    pub is_animated: bool,
    /// CDN URL for custom emojis (NULL for unicode emojis)
    /// Format: https://cdn.discordapp.com/emojis/{id}.{png|gif}
    pub emoji_url: Option<String>,

    pub reacted_at: DateTime<Utc>,
}
