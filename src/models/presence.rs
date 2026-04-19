use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::PresenceStatus;

/// Discord online/offline status change (event-sourced).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Presence {
    pub id: Uuid,
    pub user_id: i64,

    /// Overall status.
    pub status: PresenceStatus,

    /// The first time the bot picked up that this status was happening (TODO can we get actual activity begin time from serenity)
    pub started_at: DateTime<Utc>,
    /// NULL = current status.
    pub ended_at: Option<DateTime<Utc>>,
}

/// Detects and logs as ForcedOnline using messages the user sent while showing as offline/invisible. TODO For how long does the user get forced online, and TODO can we make it so when they join a VC while being set to offline it also sets them to ForcedOnline?
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ForcedOnlineEvidence {
    pub id: Uuid,
    pub user_id: i64,
    pub message_id: i64,
    pub presence_id: Uuid,
    pub detected_at: DateTime<Utc>,
}
