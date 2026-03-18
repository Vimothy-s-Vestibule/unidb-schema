pub mod diesel_schema;
pub mod models;

use serde::{Deserialize, Serialize};

use crate::models::{
    CommunicationTraits, PersonalityInterests, PersonalityTraits, PersonalityValues,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiScoreResponse {
    pub username: String,
    pub user_id: String,
    pub personality: PersonalityTraits,
    pub communication: CommunicationTraits,
    pub values: PersonalityValues,
    pub interests: PersonalityInterests,
    pub introduction_embedding: Option<Vec<f32>>,
}
