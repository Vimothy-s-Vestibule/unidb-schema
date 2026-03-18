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
    // All major Acitivites/intersts over time, more general than per message, changing over time (TODO cronjob?)
    pub score_id: Option<String>,
    pub status: RecordStatus,
}

#[derive(
    Debug, Clone, Queryable, QueryableByName, Selectable, Insertable, AsChangeset, Default,
)]
#[diesel(table_name = crate::diesel_schema::scores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ScoreRecord {
    pub score_id: String,

    #[diesel(embed)]
    pub personality: PersonalityTraits,
    #[diesel(embed)]
    pub communication: CommunicationTraits,
    #[diesel(embed)]
    pub values: Values,
    #[diesel(embed)]
    pub interests: Interests,

    pub embedding: Option<pgvector::Vector>,
    pub intro_diagram: Option<Vec<u8>>,
    pub current_diagram: Option<Vec<u8>>,
}
