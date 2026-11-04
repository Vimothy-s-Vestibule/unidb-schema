//! Integrity rules the services rely on, enforced by the schema itself.
//!
//! Each test states the invariant the workers/dashboard assume and checks that
//! the database rejects (or accepts) the corresponding write.

mod common;

use common::TestDb;
use uuid::Uuid;

const SETUP: &str = r#"
INSERT INTO discord_channels (channel_id, name, channel_type) VALUES (1, 'general', 'text');
INSERT INTO vestibule_users (id) VALUES ('00000000-0000-4000-8000-000000000001'),
                                        ('00000000-0000-4000-8000-000000000002');
INSERT INTO discord_accounts (discord_user_id, vestibule_user_id, username, display_name)
  VALUES (10, '00000000-0000-4000-8000-000000000001', 'alice', 'Alice');
INSERT INTO messages (message_id, channel_id, sent_by, content, sent_at)
  VALUES (100, 1, 10, 'hello', now()), (101, 1, 10, 'again', now());
INSERT INTO topic (id, name) VALUES ('00000000-0000-4000-8000-0000000000a1', 'rust'),
                                    ('00000000-0000-4000-8000-0000000000a2', 'lunch');
"#;

async fn db(label: &str) -> Option<TestDb> {
    let db = test_db!(label).await?;
    db.exec(SETUP).await;
    Some(db)
}

/// Runs `sql`; returns the name of the violated constraint, `None` if it succeeded.
/// A failed multi-statement script can leave an explicit transaction aborted,
/// so it runs on its own connection which is rolled back afterwards.
async fn violated(db: &TestDb, sql: &str) -> Option<String> {
    let mut conn = db.pool.acquire().await.unwrap();
    let result = sqlx::raw_sql(sql).execute(&mut *conn).await;
    sqlx::raw_sql("ROLLBACK").execute(&mut *conn).await.ok();
    match result {
        Ok(_) => None,
        Err(sqlx::Error::Database(e)) => Some(
            e.constraint()
                .map(str::to_string)
                .unwrap_or_else(|| panic!("failed without a constraint: {e}\n{sql}")),
        ),
        Err(e) => panic!("{e}\n{sql}"),
    }
}

fn score_insert(value: &str) -> String {
    format!(
        "INSERT INTO scores (honesty, emotionality, extraversion, agreeableness, conscientiousness,
           openness_to_experience, agency, achievement, influence, sarcasm, security,
           self_reflection, technical_competence, busyness)
         VALUES (0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, {value})"
    )
}

/// scoring_prompt() asks for every trait on a 0.0..=1.0 scale.
#[tokio::test]
async fn scores_must_be_within_unit_range() {
    let Some(db) = db("scores").await else { return };
    for ok in ["0", "1", "0.5"] {
        assert_eq!(violated(&db, &score_insert(ok)).await, None, "score {ok}");
    }
    for bad in ["1.0001", "-0.1", "'NaN'", "'Infinity'"] {
        assert_eq!(
            violated(&db, &score_insert(bad)).await.as_deref(),
            Some("scores_values_in_unit_range"),
            "score {bad} must be rejected"
        );
    }
    db.drop_db().await;
}

fn fact_insert(confidence: &str, weight: &str) -> String {
    format!(
        "BEGIN;
         INSERT INTO user_facts_and_activities (id, user_id, record_type, source, type, value, confidence)
           VALUES ('{id}', '00000000-0000-4000-8000-000000000001', 'fact', 'llm_extraction',
                   'location', 'Vienna', {confidence});
         INSERT INTO fact_and_activity_evidence (fact_or_activity_id, message_id, weight, reasoning)
           VALUES ('{id}', 100, {weight}, 'says so');
         COMMIT;",
        id = Uuid::new_v4()
    )
}

/// fact-extractor's prompt and JSON schema define confidence in 0..=1, and it
/// stores confidence as the evidence weight.
#[tokio::test]
async fn fact_confidence_and_evidence_weight_must_be_within_unit_range() {
    let Some(db) = db("confidence").await else {
        return;
    };
    assert_eq!(violated(&db, &fact_insert("0", "1")).await, None);
    assert_eq!(violated(&db, &fact_insert("1", "0")).await, None);
    for bad in ["1.5", "-0.01", "'NaN'"] {
        assert_eq!(
            violated(&db, &fact_insert(bad, "0.5")).await.as_deref(),
            Some("user_facts_and_activities_confidence_range"),
            "confidence {bad}"
        );
        assert_eq!(
            violated(&db, &fact_insert("0.5", bad)).await.as_deref(),
            Some("fact_and_activity_evidence_weight_range"),
            "weight {bad}"
        );
    }
    db.drop_db().await;
}

/// ingest's emoji resolution looks emojis up by `discord_emoji_id` and expects
/// at most one row (it uses the first match, and reaction_remove deletes by it).
#[tokio::test]
async fn discord_emoji_id_is_unique() {
    let Some(db) = db("emoji").await else { return };
    let ins = "INSERT INTO discord_emojis (discord_emoji_id) VALUES ('👍')";
    assert_eq!(violated(&db, ins).await, None);
    assert_eq!(
        violated(&db, ins).await.as_deref(),
        Some("discord_emojis_discord_emoji_id_key")
    );
    db.drop_db().await;
}

/// topic-sorter's insert_topic reuses an existing topic with the same name.
#[tokio::test]
async fn topic_name_is_unique() {
    let Some(db) = db("topic").await else { return };
    assert_eq!(
        violated(&db, "INSERT INTO topic (name) VALUES ('rust')")
            .await
            .as_deref(),
        Some("topic_name_key")
    );
    db.drop_db().await;
}

/// topic-sorter assigns at most one topic per message
/// (insert_topic_message_relation: `WHERE NOT EXISTS (... message_id = $2)`),
/// and both its context query and the dashboard join on message_id.
#[tokio::test]
async fn a_message_has_at_most_one_topic() {
    let Some(db) = db("tmr").await else { return };
    let ins = |topic: &str| {
        format!(
            "INSERT INTO topic_message_relation (topic_id, message_id)
             VALUES ('00000000-0000-4000-8000-0000000000{topic}', 100)"
        )
    };
    assert_eq!(violated(&db, &ins("a1")).await, None);
    assert_eq!(
        violated(&db, &ins("a1")).await.as_deref(),
        Some("topic_message_relation_message_id_key")
    );
    assert_eq!(
        violated(&db, &ins("a2")).await.as_deref(),
        Some("topic_message_relation_message_id_key")
    );
    // and the race-free form the worker can switch to
    assert_eq!(
        violated(&db, &(ins("a2") + " ON CONFLICT (message_id) DO NOTHING")).await,
        None
    );
    db.drop_db().await;
}

/// Reactions whose emoji could not be resolved have emoji_id NULL. The
/// historical scan re-inserts all reactions on every start with
/// `ON CONFLICT (message_id, user_id, emoji_id) DO NOTHING`; with NULLs
/// distinct that never conflicts and duplicates pile up on every restart.
#[tokio::test]
async fn null_emoji_reactions_are_deduplicated() {
    let Some(db) = db("reactions").await else {
        return;
    };
    let scan_insert =
        "INSERT INTO message_reactions (id, message_id, user_id, emoji_id, reacted_at)
                       VALUES (gen_random_uuid(), 100, 10, NULL, now())
                       ON CONFLICT (message_id, user_id, emoji_id) DO NOTHING";
    for _ in 0..3 {
        assert_eq!(violated(&db, scan_insert).await, None);
    }
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM message_reactions")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(
        n, 1,
        "three scans of the same NULL-emoji reaction stored {n} rows"
    );
    db.drop_db().await;
}

/// Discord usernames are released and reused (renames, deleted accounts), and
/// ingest never updates a stored username. A UNIQUE username therefore makes a
/// new user with a recycled name impossible to insert, after which all their
/// messages fail the messages.sent_by foreign key.
#[tokio::test]
async fn usernames_may_repeat_across_discord_accounts() {
    let Some(db) = db("username").await else {
        return;
    };
    assert_eq!(
        violated(
            &db,
            "INSERT INTO discord_accounts (discord_user_id, vestibule_user_id, username, display_name)
             VALUES (11, '00000000-0000-4000-8000-000000000002', 'alice', 'Alice (new)')"
        )
        .await,
        None
    );
    db.drop_db().await;
}

/// social_platforms ids must be identical in every environment (they are
/// referenced by connected_accounts.platform_id and by anything that seeds or
/// exports data).
#[tokio::test]
async fn social_platform_ids_are_deterministic() {
    let Some(url) = common::admin_url() else {
        return;
    };
    let a = TestDb::create(&url, "platforms_a").await;
    let b = TestDb::create(&url, "platforms_b").await;
    let q = "SELECT platform_name || '=' || id FROM social_platforms ORDER BY platform_name";
    let ids_a: Vec<String> = sqlx::query_scalar(q).fetch_all(&a.pool).await.unwrap();
    let ids_b: Vec<String> = sqlx::query_scalar(q).fetch_all(&b.pool).await.unwrap();
    assert_eq!(
        ids_a, ids_b,
        "seeded social_platforms ids differ between two fresh databases"
    );
    assert!(ids_a.contains(&"YouTube=05f4b960-1dfb-50c5-b9b2-53813526bf14".to_string()));
    a.drop_db().await;
    b.drop_db().await;
}
