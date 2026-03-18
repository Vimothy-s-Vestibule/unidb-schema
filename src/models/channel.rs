use diesel::prelude::*;

use crate::models::ChannelType;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Default, AsChangeset, Identifiable)]
#[diesel(primary_key(channel_id))]
#[diesel(table_name = crate::diesel_schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub channel_id: String,
    pub name: String,
    pub channel_type: ChannelType,
}
