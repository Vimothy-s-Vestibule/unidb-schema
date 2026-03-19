use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{newtypes::ThreadType, Channel};

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
#[diesel(primary_key(thread_id))]
#[diesel(table_name = crate::diesel_schema::threads)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DiscordThread {
    pub thread_id: String,
    pub channel_id: String,
    pub name: String,
    pub thread_type: ThreadType,
}
