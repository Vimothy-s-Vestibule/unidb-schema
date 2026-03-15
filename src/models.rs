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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiScoreResponse {
    pub username: String,
    pub user_id: String,
    pub personality: PersonalityTraits,
    pub communication: CommunicationTraits,
    pub values: Values,
    pub interests: Interests,
    pub introduction_embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeComment {
    pub id: String,
    pub parent_id: Option<String>,
    pub thread_id: String,
    pub etag: String,
    pub text: String,
    pub like_count: i32,
    pub author_id: String,
    pub video_id: String,
    pub date_published: chrono::DateTime<chrono::Utc>,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub last_checked: chrono::DateTime<chrono::Utc>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_comment_threads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeCommentThread {
    pub id: String,
    pub etag: String,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeChannel {
    pub id: String,
    pub display_name: String,
    pub profile_image_url: String,
    pub channel_title: Option<String>,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_videos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeVideo {
    pub id: String,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub etag: Option<String>,
    pub view_count: Option<i64>,
    pub like_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub thumbnail: serde_json::Value,
    pub date_published: Option<chrono::DateTime<chrono::Utc>>,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub last_checked: Option<chrono::DateTime<chrono::Utc>>,
}
