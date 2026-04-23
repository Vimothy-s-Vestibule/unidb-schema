//! User facts and activity model (LLM-extracted events & knowledge).

use chrono::{DateTime, Utc};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::enums::ActivitySource;

/// Activity or Fact extracted from messages or external content.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserFactAndActivity {
    pub id: Uuid,
    pub user_id: Uuid,

    pub record_type: String, // 'activity', 'fact', or 'skill'
    pub source: ActivitySource,
    pub r#type: String, // e.g. skill name
    pub value: String, // e.g. proficiency description
    pub confidence: f32,
    pub level: Option<i16>, // 0-10 for skills

    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub is_current: Option<bool>,

    pub created_at: DateTime<Utc>,

    #[serde(skip)]
    pub type_value_embedding: Option<Vector>,
}
