//! Discord channel model.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::DiscordChannelType;

/// Discord channel (text, voice, forum, thread, etc.).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: i64,
    pub name: String,
    pub channel_type: DiscordChannelType,
    pub parent_channel_id: Option<i64>,
}
