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
            other => Err(format!("unsupported channel type: {other:?}")),
        }
    }
}

/// Status for external platform sync jobs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PlatformSyncStatus {
    #[default]
    Idle,
    Running,
    Failed,
}

/// Discord online status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
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
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PlatformAccess {
    /// Public API to get user data, no auth needed
    Public,
    /// Requires OAuth token
    OauthRequired,
    /// No API available but user(s) still mentioned an account, TODO implement scraper workers
    Unavailable,
}

/// Status for the async processing pipeline (general triage, skills, personality). TODO include significance here?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum YoutubeVideoBroadcastStatus {
    Video,
    CurrentLive,
    PastLive,
    ScheduledLive,
    None,
}
