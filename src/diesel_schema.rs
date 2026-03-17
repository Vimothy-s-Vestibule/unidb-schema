// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    messages (message_id) {
        message_id -> Text,
        user_id -> Text,
        content -> Text,
        sent_at -> Timestamptz,
        added_at -> Timestamptz,
        score_id -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    scores (score_id) {
        score_id -> Text,
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
        activities -> Array<Nullable<Text>>,
        domains -> Array<Nullable<Text>>,
        embedding -> Nullable<Vector>,
        intro_diagram -> Nullable<Bytea>,
        current_diagram -> Nullable<Bytea>,
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
        // All major Acitivites over time, more general than per message
        // Embedding used to store a user's skills and abilities, changing over time (TODO cronjob?)
        score_id -> Nullable<Text>,
        status -> Text,
    }
}

diesel::joinable!(vestibule_users -> messages (intro_message_id));

diesel::joinable!(messages -> scores (score_id));
diesel::joinable!(vestibule_users -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(messages, vestibule_users, scores);
