use chrono::{DateTime, Utc};
use ormlite::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "topic")]
pub struct Topic {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "topic_message_relation")]
pub struct TopicMessageRelation {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub message_id: i64,
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "message_classification_attempts")]
pub struct MessageClassificationAttempt {
    #[ormlite(primary_key)]
    pub message_id: i64,
    pub attempted_at: DateTime<Utc>,
}
