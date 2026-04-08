//! UniDB - Database models for a Discord bot application.

pub mod models;
pub mod scoring_schema;

pub use models::*;
pub use scoring_schema::scoring_prompt;

use serde::{Deserialize, Serialize};

/// LLM scoring response for a user's intro
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiScoreResponse {
    pub username: String,
    pub user_id: i64,
    pub hexaco: HexacoTraits,
    pub behavioral: BehavioralTraits,
    pub activities: Vec<String>,
    pub embedding: Option<Vec<f32>>,
}

/// NATS message payload for processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsMessagePayload {
    pub message_id: i64,
    pub is_intro: bool,
}
