pub mod message;
pub mod newtypes;
pub mod skill;
pub mod structs;

use diesel::prelude::*;

pub use message::*;
pub use newtypes::*;
pub use skill::*;
pub use structs::*;

#[derive(
    Debug,
    Clone,
    Queryable,
    QueryableByName,
    Selectable,
    Insertable,
    AsChangeset,
    Default,
    Associations,
    Identifiable,
)]
#[diesel(table_name = crate::diesel_schema::vestibule_users)]
#[diesel(primary_key(discord_user_id))]
#[diesel(belongs_to(DiscordMessage, foreign_key = intro_message_id))]
#[diesel(belongs_to(ScoreRecord, foreign_key = score_id))]
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
    Debug,
    Clone,
    Queryable,
    QueryableByName,
    Selectable,
    Insertable,
    AsChangeset,
    Default,
    Identifiable,
)]
#[diesel(table_name = crate::diesel_schema::scores)]
#[diesel(primary_key(score_id))]
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

    // For intro messages: Embed the whole message, for normal messages: dont generate embeddings, for users: dont generate embeddings
    pub embedding: Option<pgvector::Vector>,

    pub intro_diagram: Option<Vec<u8>>,
    pub current_diagram: Option<Vec<u8>>,
}
