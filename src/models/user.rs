use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Main user entity. A user can have multiple Discord accounts.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct VestibuleUser {
    /// unique (TODO UUIDv7?) user id
    pub id: Uuid,

    // Real names extracted from messages (TODO extract, not display name)
    pub real_first: Option<String>,
    pub real_last: Option<String>,
    pub nickname: Option<String>,

    // Introduction message reference
    pub intro_message_id: Option<i64>,

    /// Aggregated personality score from all messages the user sent
    pub score_id: Option<Uuid>,
    pub score_last_updated: Option<DateTime<Utc>>,

    // HEXACO diagram images (PNG bytes)
    pub current_diagram: Option<Vec<u8>>,
    pub current_diagram_last_updated: Option<DateTime<Utc>>,

    pub intro_diagram: Option<Vec<u8>>,
}

/// Discord account linked to a VestibuleUser.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DiscordAccount {
    pub discord_user_id: i64,
    pub vestibule_user_id: Uuid,
    pub username: String,
    pub display_name: String,
}
