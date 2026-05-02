//! User facts and activity model (LLM-extracted events & knowledge).

use chrono::{DateTime, Utc};
use ormlite::Model;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::enums::{ActivityRecordType, ActivitySource};

/// Activity or Fact extracted from messages or external content.
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "user_facts_and_activities")]
pub struct UserFactAndActivity {
    pub id: Uuid,
    pub user_id: Uuid,

    pub record_type: ActivityRecordType, // 'activity', 'fact', or 'skill'
    pub source: ActivitySource,
    #[ormlite(column = "type")]
    pub type_str: String, // e.g. skill name
    pub value: String, // e.g. proficiency description
    pub confidence: f32,
    pub level: Option<i16>, // 0-10 for skills
    pub score_id: Option<Uuid>,

    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub is_current: Option<bool>,

    pub created_at: DateTime<Utc>,

    #[serde(skip)]
    pub type_value_embedding: Option<Vector>,
}
