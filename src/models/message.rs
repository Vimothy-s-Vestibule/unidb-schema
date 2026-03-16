use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable, Insertable, Default)]
#[diesel(table_name = crate::diesel_schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DiscordMessage {
    pub message_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
