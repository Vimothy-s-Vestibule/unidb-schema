//! Discord channel model.

use ormlite::Model;
use serde::{Deserialize, Serialize};

use super::enums::DiscordChannelType;

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "discord_channels")]
pub struct DiscordChannel {
    #[ormlite(primary_key)]
    pub channel_id: i64,
    pub name: String,
    pub channel_type: DiscordChannelType,
    pub parent_channel_id: Option<i64>,
}
