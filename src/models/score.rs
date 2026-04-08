//! Personality scoring models (HEXACO + behavioral traits).

use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// HEXACO personality dimensions (0.0 - 1.0 scale).
#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct HexacoTraits {
    pub honesty: f64,
    pub emotionality: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub conscientiousness: f64,
    pub openness_to_experience: f64,
}

/// Custom behavioral trait dimensions.
#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct BehavioralTraits {
    pub agency: f64,
    pub achievement: f64,
    pub influence: f64,
    pub sarcasm: f64,
    pub security: f64,
    pub self_reflection: f64,
    pub technical_competence: f64,
    pub busyness: f64,
}

/// Aggregated personality score with embedding.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Score {
    pub id: Uuid,

    // HEXACO traits (flattened)
    #[sqlx(flatten)]
    pub hexaco: HexacoTraits,

    // Behavioral traits (flattened)
    #[sqlx(flatten)]
    pub behavioral: BehavioralTraits,

    /// Vector embedding for similarity search.
    #[serde(skip)]
    pub embedding: Option<Vector>,
}
