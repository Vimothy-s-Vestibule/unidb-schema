pub mod models;
pub mod scoring_schema;

use serde::{Deserialize, Serialize};

use crate::models::{BehavioralTraits, HexacoTraits};

pub use scoring_schema::scoring_prompt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiScoreResponse {
    pub username: String,
    pub user_id: i64,
    pub hexaco: HexacoTraits,
    pub behavioral: BehavioralTraits,
    pub activities: Vec<String>,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NatsMessagePayload {
    pub message_id: i64,
    pub is_intro: bool,
}
