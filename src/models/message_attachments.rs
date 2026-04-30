use chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Link between messages and media assets.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
pub struct MessageAttachment {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub message_id: i64,
    pub asset_id: Uuid,

    pub added_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
