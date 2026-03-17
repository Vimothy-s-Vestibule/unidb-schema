use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeComment {
    pub youtube_comment_id: String,
    pub parent_id: Option<String>,
    pub thread_id: String,
    // The Etag of this resource. "an eTag is basically used to determine if a resource has changed" (https://stackoverflow.com/questions/21752421/youtube-api-v3-and-etag)
    pub etag: String,
    pub text: String,
    pub like_count: i32,
    pub author_id: String,
    pub video_id: String,
    pub date_published: chrono::DateTime<chrono::Utc>,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub last_checked: chrono::DateTime<chrono::Utc>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_comment_threads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeCommentThread {
    pub youtube_comment_thread_id: String,
     // The Etag of this resource. "an eTag is basically used to determine if a resource has changed" (https://stackoverflow.com/questions/21752421/youtube-api-v3-and-etag)
    pub etag: String,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeChannel {
    pub youtube_channel_id: String,
    pub username: String, // e.g: @sylvan-franklin
    pub profile_image_url: String,
    // By default the channel title isn't returned when we get the comments.
    // You get it from other calls.
    pub channel_title: Option<String>, // e.g: Sylvan Franklin
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

// thumbnail field is not present because I think there is no need for it, but if it will ever be required we can lookup it's scheme here: https://developers.google.com/youtube/v3/docs/videos
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::diesel_schema::youtube_videos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeVideo {
    pub youtube_video_id: String,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    // The Etag of this resource. "an eTag is basically used to determine if a resource has changed" (https://stackoverflow.com/questions/21752421/youtube-api-v3-and-etag)
    pub etag: Option<String>,
    pub view_count: Option<i64>,
    pub like_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub date_published: Option<chrono::DateTime<chrono::Utc>>,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub last_checked: Option<chrono::DateTime<chrono::Utc>>,
}
