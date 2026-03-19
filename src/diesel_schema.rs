// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    channels (channel_id) {
        channel_id -> Text,
        name -> Text,
        channel_type -> Text,
    }
}

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
        in_reply_to -> Nullable<Text>,
        last_edited -> Nullable<Timestamptz>,
        channel_id -> Text,
        deleted_at -> Nullable<Timestamptz>,
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

    skills (id) {
        id -> Uuid,
        name -> Text,
        embedding -> Nullable<Vector>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    user_skill_evidence (user_id, skill_id, message_id) {
        user_id -> Text,
        skill_id -> Uuid,
        message_id -> Text,
        weight -> Float4,
        reasoning -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    user_skills (user_id, skill_id) {
        user_id -> Text,
        skill_id -> Uuid,
        level -> Int2,
        llm_context -> Nullable<Text>,
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
        status -> Text,
        score_id -> Nullable<Text>,
    }
}

diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> scores (score_id));
diesel::joinable!(user_skill_evidence -> messages (message_id));
diesel::joinable!(user_skills -> skills (skill_id));
diesel::joinable!(user_skills -> vestibule_users (user_id));
diesel::joinable!(vestibule_users -> messages (intro_message_id));
diesel::joinable!(vestibule_users -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    scores,
    skills,
    user_skill_evidence,
    user_skills,
    vestibule_users,
);
