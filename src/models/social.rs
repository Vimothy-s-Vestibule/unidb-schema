use diesel::prelude::*;
use uuid::Uuid;

use crate::models::message::DiscordMessage;
use crate::models::VestibuleUserRecord;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = crate::diesel_schema::social_platforms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SocialPlatform {
    pub id: Uuid,
    pub platform_name: String,
    pub url: String,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, Associations, Identifiable,
)]
#[diesel(table_name = crate::diesel_schema::user_platform_association)]
#[diesel(belongs_to(VestibuleUserRecord, foreign_key = vestibule_user_id))]
#[diesel(belongs_to(SocialPlatform, foreign_key = platform_id))]
#[diesel(belongs_to(DiscordMessage, foreign_key = platform_association_mention_message_id))]
#[diesel(primary_key(vestibule_user_id, platform_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserPlatformAssociation {
    pub vestibule_user_id: String,
    pub platform_id: Uuid,
    pub platform_username: String,
    pub platform_display_name: String,
    pub profile_url: Option<String>,
    pub platform_association_mention_message_id: String,
    pub reasoning: String,
}
