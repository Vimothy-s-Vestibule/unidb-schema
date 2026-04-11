//! Discord channel model.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::DiscordChannelType;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DiscordChannel {
    pub channel_id: i64,
    pub name: String,
    pub channel_type: DiscordChannelType,
    pub parent_channel_id: Option<i64>,
}
