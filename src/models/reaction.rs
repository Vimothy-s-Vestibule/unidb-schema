//! Message reaction and emoji models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ormlite::Model;
use uuid::Uuid;

/// Emoji reaction on a Discord message.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "message_reactions")]
pub struct MessageReaction {
    pub id: Uuid,
    pub message_id: i64,
    pub user_id: i64,
    pub emoji_id: Option<Uuid>,
    pub reacted_at: DateTime<Utc>,
}

/// A Discord emoji (unicode or custom).
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "discord_emojis")]
pub struct DiscordEmoji {
    pub id: Uuid,
    /// Unicode emoji string or custom emoji snowflake ID
    pub discord_emoji_id: String,
    /// Some string if the emoji is a custom one created by some server, otherwise NULL
    pub from_guild: Option<String>,
    /// Display name (useful for custom emojis)
    pub emoji_display_name: Option<String>,
    /// Whether custom emoji is animated (GIF vs PNG)
    pub is_animated: bool,
    /// CDN URL for custom emojis
    pub emoji_url: Option<String>,
    pub asset_id: Option<Uuid>,
}
