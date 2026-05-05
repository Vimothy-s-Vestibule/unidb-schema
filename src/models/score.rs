use ormlite::Model;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// HEXACO personality dimensions (0.0 - 1.0 scale).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HexacoTraits {
    pub honesty: f64,
    pub emotionality: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub conscientiousness: f64,
    pub openness_to_experience: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "scores")]
pub struct Score {
    pub id: Uuid,

    // HEXACO traits (flattened)
    pub honesty: f64,
    pub emotionality: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub conscientiousness: f64,
    pub openness_to_experience: f64,

    // Behavioral traits (flattened)
    pub agency: f64,
    pub achievement: f64,
    pub influence: f64,
    pub sarcasm: f64,
    pub security: f64,
    pub self_reflection: f64,
    pub technical_competence: f64,
    pub busyness: f64,

    /// Vector embedding for similarity search.
    #[serde(skip)]
    pub embedding: Option<Vector>,
}
