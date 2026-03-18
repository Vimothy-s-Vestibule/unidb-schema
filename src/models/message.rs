use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{ScoreRecord, VestibuleUserRecord};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    Default,
    AsChangeset,
    Associations,
    Identifiable,
)]
#[diesel(primary_key(message_id))]
#[diesel(table_name = crate::diesel_schema::messages)]
#[diesel(belongs_to(VestibuleUserRecord, foreign_key = user_id))]
#[diesel(belongs_to(ScoreRecord, foreign_key = score_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DiscordMessage {
    pub message_id: String,
    pub user_id: String,
    pub content: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub score_id: Option<String>,
}
