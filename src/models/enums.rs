use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "discord_channel_type", rename_all = "snake_case")]
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
    PublicThread,
}

impl std::fmt::Display for DiscordChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Text => "text",
            Self::TextThread => "text_thread",
            Self::ForumPost => "forum_post",
            Self::Voice => "voice",
            Self::Forum => "forum",
            Self::Stage => "stage",
            Self::Category => "category",
            Self::PublicThread => "public_thread",
        };
        f.write_str(s)
    }
}

impl TryFrom<serenity::model::channel::ChannelType> for DiscordChannelType {
    type Error = String;

    fn try_from(value: serenity::model::channel::ChannelType) -> Result<Self, Self::Error> {
        use serenity::model::channel::ChannelType as CT;
        match value {
            CT::Text => Ok(Self::Text),
            CT::Voice => Ok(Self::Voice),
            CT::Forum => Ok(Self::Forum),
            CT::Category => Ok(Self::Category),
            CT::Stage => Ok(Self::Stage),
            CT::PublicThread => Ok(Self::PublicThread),
            other => Err(format!("unsupported channel type: {other:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "activity_record_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ActivityRecordType {
    Activity,
    Fact,
    Skill,
    Emotion,
}

/// Where a user activity originated from
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "activity_source", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ActivitySource {
    /// LLM-extracted from a message, external content, etc.
    LlmExtraction,
    /// Obtained from a connected_accounts account
    ExternalContent,
    /// Manually added by an admin
    Manual,
}

/// Discord online status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "presence_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PresenceStatus {
    Online,
    ForcedOnline,
    Absent,
    DoNotDisturb,
    Offline,
}

/// How external platform data can be accessed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "platform_access", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PlatformAccess {
    /// Public API to get user data, no auth needed
    Public,
    /// Requires OAuth token
    OauthRequired,
    /// No API available but user(s) still mentioned an account, TODO implement scraper workers
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "job_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobType {
    YoutubeChannelRetrieval,
    DiscordChannelSync,
    StravaRetrieval,
    LinkedinScrape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(
    type_name = "youtube_video_broadcast_status",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum YoutubeVideoBroadcastStatus {
    Video,
    CurrentLive,
    PastLive,
    ScheduledLive,
    None,
}
