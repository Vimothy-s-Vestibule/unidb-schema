use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DiscordChannelType {
    #[default]
    Text,
    TextThread,
    ForumPost,
    Voice,
    Forum,
    Stage,
    Category,
}

impl TryFrom<serenity::model::channel::ChannelType> for DiscordChannelType {
    type Error = String;

    fn try_from(value: serenity::model::channel::ChannelType) -> Result<Self, Self::Error> {
        match value {
            serenity::model::channel::ChannelType::Text => Ok(DiscordChannelType::Text),
            serenity::model::channel::ChannelType::Voice => Ok(DiscordChannelType::Voice),
            serenity::model::channel::ChannelType::Forum => Ok(DiscordChannelType::Forum),
            serenity::model::channel::ChannelType::Category => Ok(DiscordChannelType::Category),
            serenity::model::channel::ChannelType::Stage => Ok(DiscordChannelType::Stage),
            other => Err(format!(
                "cannot convert unsupported serenity discord channel type: {:?}",
                other
            )),
        }
    }
}

impl std::fmt::Display for DiscordChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscordChannelType::Text => write!(f, "text"),
            DiscordChannelType::TextThread => write!(f, "text_thread"),
            DiscordChannelType::ForumPost => write!(f, "forum_post"),
            DiscordChannelType::Voice => write!(f, "voice"),
            DiscordChannelType::Forum => write!(f, "forum"),
            DiscordChannelType::Stage => write!(f, "stage"),
            DiscordChannelType::Category => write!(f, "category"),
        }
    }
}
