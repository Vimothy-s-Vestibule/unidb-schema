//! Skill tracking models.

use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Skill definition with optional embedding for similarity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,

    #[serde(skip)]
    pub embedding: Option<Vector>,
}

/// User's proficiency level in a skill.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSkill {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,

    /// Proficiency level (0-10).
    pub level: i16,
    /// LLM reasoning for the assessment.
    pub llm_context: Option<String>,
}

/// Message evidence supporting a skill assessment.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSkillEvidence {
    pub user_skill_id: Uuid,
    pub message_id: i64,

    /// Weight of this evidence (0.0-1.0).
    pub weight: f32,
    /// LLM reasoning.
    pub reasoning: String,
}
