// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    messages (message_id) {
        message_id -> Text,
        user_id -> Text,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    vestibule_users (discord_user_id) {
        discord_user_id -> Text,
        discord_username -> Text,
        yt_username -> Nullable<Text>,
        yt_display_name -> Nullable<Text>,
        intro_message_id -> Nullable<Text>,
        honesty_humility -> Float8,
        emotionality -> Float8,
        extraversion -> Float8,
        agreeableness -> Float8,
        conscientiousness -> Float8,
        openness_to_experience -> Float8,
        agency -> Float8,
        communion -> Float8,
        self_direction -> Float8,
        stimulation -> Float8,
        hedonism -> Float8,
        achievement -> Float8,
        power -> Float8,
        security -> Float8,
        conformity -> Float8,
        tradition -> Float8,
        benevolence -> Float8,
        universalism -> Float8,
        intro_embedding -> Nullable<Vector>,
        intro_diagram -> Nullable<Bytea>,
        status -> Text,
        activities -> Array<Nullable<Text>>,
        domains -> Array<Nullable<Text>>,
    }
}

// code below made by gemini but I think it should be generated automatically

// diesel::table! {
//     use diesel::sql_types::*;
//     use pgvector::sql_types::Vector;

//     youtube_channels (id) {
//         id -> Text,
//         display_name -> Text,
//         profile_image_url -> Text,
//         channel_title -> Nullable<Text>,
//         date_added -> Timestamptz,
//         date_updated -> Nullable<Timestamptz>,
//     }
// }

// diesel::table! {
//     use diesel::sql_types::*;
//     use pgvector::sql_types::Vector;

//     youtube_comment_threads (id) {
//         id -> Text,
//         etag -> Text,
//         date_added -> Timestamptz,
//         date_updated -> Timestamptz,
//     }
// }

// diesel::table! {
//     use diesel::sql_types::*;
//     use pgvector::sql_types::Vector;

//     youtube_comments (id) {
//         id -> Text,
//         parent_id -> Nullable<Text>,
//         thread_id -> Text,
//         etag -> Text,
//         text -> Text,
//         like_count -> Int4,
//         author_id -> Text,
//         video_id -> Text,
//         date_published -> Timestamptz,
//         date_added -> Timestamptz,
//         date_updated -> Nullable<Timestamptz>,
//         last_checked -> Timestamptz,
//         deleted -> Bool,
//     }
// }

// diesel::table! {
//     use diesel::sql_types::*;
//     use pgvector::sql_types::Vector;

//     youtube_videos (id) {
//         id -> Text,
//         channel_id -> Nullable<Text>,
//         title -> Nullable<Text>,
//         etag -> Nullable<Text>,
//         view_count -> Nullable<Int8>,
//         like_count -> Nullable<Int8>,
//         comment_count -> Nullable<Int8>,
//         thumbnail -> Jsonb,
//         date_published -> Nullable<Timestamptz>,
//         date_added -> Timestamptz,
//         date_updated -> Nullable<Timestamptz>,
//         last_checked -> Nullable<Timestamptz>,
//     }
// }

// diesel::joinable!(vestibule_users -> messages (intro_message_id));

// diesel::allow_tables_to_appear_in_same_query!(
//     messages,
//     vestibule_users,
//     youtube_channels,
//     youtube_comment_threads,
//     youtube_comments,
//     youtube_videos,
// );
