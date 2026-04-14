use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Skill definition with embedding TODO generate for similarity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,

    #[serde(skip)]
    pub embedding: Option<Vector>,
}

/// The specific combination and rating and description of a user and a skill
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSkill {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,

    /// Proficiency level (0-10).
    pub level: i16,
    /// LLM reasoning for the assessment
    pub llm_context: Option<String>,
}

/// Message evidence supporting a skill assessment
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSkillEvidence {
    pub id: Uuid,
    pub user_skill_id: Uuid,
    pub message_id: Option<i64>,
    pub youtube_comment_id: Option<String>,

    /// Weight of this evidence (0.0-1.0).
    pub weight: f32,
    /// LLM reasoning.
    pub reasoning: String,
}
