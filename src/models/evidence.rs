use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Message evidence supporting an inferred fact or activity
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FactAndActivityEvidence {
    pub id: Uuid,
    pub fact_or_activity_id: Uuid,
    
    // The polymorphic source
    pub message_id: Option<i64>,
    pub youtube_comment_id: Option<String>,
    pub external_content_id: Option<Uuid>,
    pub discord_presence_id: Option<Uuid>,

    /// Weight of this evidence (0.0-1.0).
    pub weight: f32,
    
    /// LLM reasoning for why this specific source supports the fact.
    pub reasoning: String,
}
