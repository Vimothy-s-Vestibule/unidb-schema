// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    channels (channel_id) {
        channel_id -> Text,
        name -> Text,
        channel_type -> Text,
        parent_channel_id -> Nullable<Text>,
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
        deleted_at -> Nullable<Timestamptz>,
        channel_id -> Text,
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

    social_platforms (id) {
        platform_name -> Text,
        url -> Text,
        id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    user_platform_association (platform_id, association_id) {
        association_id -> Uuid,
        vestibule_user_id -> Text,
        platform_username -> Text,
        platform_display_name -> Text,
        profile_url -> Nullable<Text>,
        platform_association_mention_message_id -> Text,
        reasoning -> Text,
        platform_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    user_platform_association_evidence (association_id) {
        association_id -> Uuid,
        platform_association_mention_message_id -> Text,
        reasoning -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    user_skill_evidence (user_skill_id, message_id) {
        message_id -> Text,
        weight -> Float4,
        reasoning -> Text,
        user_skill_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Vector;

    user_skills (id) {
        user_id -> Text,
        skill_id -> Uuid,
        level -> Int2,
        llm_context -> Nullable<Text>,
        id -> Uuid,
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
        score_last_updated -> Nullable<Timestamptz>,
        discord_display_name -> Text,
        current_diagram -> Nullable<Bytea>,
        current_diagram_last_updated -> Nullable<Timestamptz>,
        intro_diagram -> Nullable<Bytea>,
    }
}

diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> scores (score_id));
diesel::joinable!(user_platform_association -> messages (platform_association_mention_message_id));
diesel::joinable!(user_platform_association -> vestibule_users (vestibule_user_id));
diesel::joinable!(user_platform_association -> social_platforms (platform_id));
diesel::joinable!(user_skill_evidence -> messages (message_id));
diesel::joinable!(user_skill_evidence -> user_skills (user_skill_id));
diesel::joinable!(user_skills -> skills (skill_id));
diesel::joinable!(user_skills -> vestibule_users (user_id));
diesel::joinable!(vestibule_users -> messages (intro_message_id));
diesel::joinable!(vestibule_users -> scores (score_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    scores,
    skills,
    social_platforms,
    user_platform_association,
    user_skill_evidence,
    user_skills,
    vestibule_users,
);
