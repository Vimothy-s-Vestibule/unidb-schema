use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct HexacoTraits {
    pub honesty: f64,
    pub emotionality: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub conscientiousness: f64,
    pub openness_to_experience: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
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
