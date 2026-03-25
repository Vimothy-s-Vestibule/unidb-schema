use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Skill definition
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    #[serde(skip)]
    pub embedding: Option<Vector>,
}

/// User's proficiency in a skill
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSkill {
    pub id: Uuid,
    pub user_id: i64,
    pub skill_id: Uuid,
    /// Skill level 0-10
    pub level: i16,
    /// LLM reasoning context
    pub llm_context: Option<String>,
}

/// Evidence linking a message to a skill assessment
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSkillEvidence {
    pub user_skill_id: Uuid,
    pub message_id: i64,
    /// How strongly this message supports the skill assessment
    pub weight: f32,
    pub reasoning: String,
}
