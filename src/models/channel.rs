use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::DiscordChannelType;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Default)]
pub struct DiscordChannel {
    pub channel_id: i64,
    pub name: String,
    pub channel_type: DiscordChannelType,
    pub parent_channel_id: Option<i64>,
}
