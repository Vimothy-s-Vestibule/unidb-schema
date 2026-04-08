pub mod channel;
pub mod message;
pub mod newtypes;
pub mod personality;
pub mod presence;
pub mod reaction;
pub mod skill;
pub mod social;

pub use channel::*;
pub use message::*;
pub use newtypes::*;
pub use personality::*;
pub use presence::*;
pub use reaction::*;
pub use skill::*;
pub use social::*;

use chrono::{DateTime, Utc};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    #[default]
    Pending,
    Sending,
    Sent,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Default)]
pub struct VestibuleUser {
    pub discord_user_id: i64,
    pub discord_username: String,
    pub discord_display_name: String,

    pub status: UserStatus,

    pub intro_message_id: Option<i64>,
    pub score_id: Option<String>,
    pub score_last_updated: Option<DateTime<Utc>>,

    pub current_diagram: Option<Vec<u8>>,
    pub current_diagram_last_updated: Option<DateTime<Utc>>,
    pub intro_diagram: Option<Vec<u8>>,

    pub aggregate_interval_hours: Option<i32>,
    pub next_aggregate_at: Option<DateTime<Utc>>,
    pub last_aggregated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Default)]
pub struct Score {
    pub score_id: String,

    #[sqlx(flatten)]
    pub hexaco: HexacoTraits,

    #[sqlx(flatten)]
    pub behavioral: BehavioralTraits,

    #[serde(skip)]
    pub embedding: Option<Vector>,
}
