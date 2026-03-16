pub mod message;
pub mod newtypes;
pub mod structs;

use diesel::prelude::*;

pub use message::*;
pub use newtypes::*;
pub use structs::*;

#[derive(
    Debug, Clone, Queryable, QueryableByName, Selectable, Insertable, AsChangeset, Default,
)]
#[diesel(table_name = crate::diesel_schema::vestibule_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VestibuleUserRecord {
    pub discord_user_id: String,
    pub discord_username: String,
    pub yt_username: Option<String>,
    pub yt_display_name: Option<String>,
    pub intro_message_id: Option<String>,

    #[diesel(embed)]
    pub personality: PersonalityTraits,
    #[diesel(embed)]
    pub communication: CommunicationTraits,
    #[diesel(embed)]
    pub values: Values,
    #[diesel(embed)]
    pub interests: Interests,

    pub intro_embedding: Option<pgvector::Vector>,
    pub intro_diagram: Option<Vec<u8>>,

    pub status: RecordStatus,
}
