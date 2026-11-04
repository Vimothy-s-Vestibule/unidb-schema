//! `migrations/*.sql` must turn a database initialised from the old schema
//! (tests/fixtures/baseline_schema_a7a7c55.sql, the rev sentience-workers pins)
//! and filled with data into exactly what the current `schema/*.sql` creates.

mod common;

use common::{catalog_snapshot, repo_root, TestDb};

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn migrations() -> Vec<(String, String)> {
    let mut files: Vec<_> = std::fs::read_dir(repo_root().join("migrations"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "sql"))
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|p| (common::file_name(&p), std::fs::read_to_string(&p).unwrap()))
        .collect()
}

async fn apply_migrations(db: &TestDb) {
    for (name, sql) in migrations() {
        if let Err(e) = sqlx::raw_sql(&sql).execute(&db.pool).await {
            panic!("migration {name} failed: {e}");
        }
    }
}

async fn baseline_db(url: &str, label: &str) -> TestDb {
    let db = TestDb::create_empty(url, label).await;
    db.exec(&read("tests/fixtures/baseline_schema_a7a7c55.sql"))
        .await;
    db
}

async fn count(db: &TestDb, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(&db.pool).await.unwrap()
}

const PLATFORM_IDS: &str = "SELECT string_agg(platform_name || '=' || id, ',' ORDER BY platform_name) FROM social_platforms";

#[tokio::test]
async fn migrating_a_seeded_baseline_database_yields_the_current_schema() {
    let Some(url) = common::admin_url() else {
        return;
    };
    let old = baseline_db(&url, "mig_old").await;
    old.exec(&read("tests/fixtures/dev_seed.sql")).await;
    let messages = count(&old, "SELECT count(*) FROM messages").await;
    let reactions = count(&old, "SELECT count(*) FROM message_reactions").await;
    let links = count(&old, "SELECT count(*) FROM topic_message_relation").await;
    assert!(
        messages > 0 && reactions > 0 && links > 0,
        "seed did not load"
    );

    apply_migrations(&old).await;

    let fresh = TestDb::create(&url, "mig_fresh").await;
    assert_eq!(
        catalog_snapshot(&old.pool).await,
        catalog_snapshot(&fresh.pool).await,
        "migrated database differs from a database created from schema/*.sql"
    );
    let ids_old: String = sqlx::query_scalar(PLATFORM_IDS)
        .fetch_one(&old.pool)
        .await
        .unwrap();
    let ids_fresh: String = sqlx::query_scalar(PLATFORM_IDS)
        .fetch_one(&fresh.pool)
        .await
        .unwrap();
    assert_eq!(ids_old, ids_fresh, "social_platforms ids not migrated");

    // clean seed data is untouched
    assert_eq!(count(&old, "SELECT count(*) FROM messages").await, messages);
    assert_eq!(
        count(&old, "SELECT count(*) FROM message_reactions").await,
        reactions
    );
    assert_eq!(
        count(&old, "SELECT count(*) FROM topic_message_relation").await,
        links
    );

    // and the seed still loads into a fresh current-schema database
    fresh.exec(&read("tests/fixtures/dev_seed.sql")).await;

    old.drop_db().await;
    fresh.drop_db().await;
}

#[tokio::test]
async fn migrations_are_idempotent() {
    let Some(url) = common::admin_url() else {
        return;
    };
    // twice on an old database
    let old = baseline_db(&url, "mig_twice").await;
    apply_migrations(&old).await;
    let after_first = catalog_snapshot(&old.pool).await;
    apply_migrations(&old).await;
    assert_eq!(after_first, catalog_snapshot(&old.pool).await);

    // no-op on a database created from the current schema
    let fresh = TestDb::create(&url, "mig_noop").await;
    let before = catalog_snapshot(&fresh.pool).await;
    let ids_before: String = sqlx::query_scalar(PLATFORM_IDS)
        .fetch_one(&fresh.pool)
        .await
        .unwrap();
    apply_migrations(&fresh).await;
    assert_eq!(before, catalog_snapshot(&fresh.pool).await);
    let ids_after: String = sqlx::query_scalar(PLATFORM_IDS)
        .fetch_one(&fresh.pool)
        .await
        .unwrap();
    assert_eq!(ids_before, ids_after);

    old.drop_db().await;
    fresh.drop_db().await;
}

/// Rows the old schema allowed but the new rules forbid: the migration must
/// merge/clean them (or keep them under a NOT VALID check), not fail.
const DIRTY: &str = r#"
-- duplicate emoji rows (the ingest race): e2 is a duplicate of seed emoji '👍'
INSERT INTO discord_emojis (id, discord_emoji_id) VALUES
  ('10000000-0000-4000-8000-0000000000d2', '👍');
-- a reaction on the duplicate that collides with one on the canonical emoji ...
INSERT INTO message_reactions (message_id, user_id, emoji_id, reacted_at)
SELECT message_id, user_id, '10000000-0000-4000-8000-0000000000d2', reacted_at + interval '1 second'
FROM message_reactions WHERE emoji_id = '10000000-0000-4000-8000-000000000001' LIMIT 1;
-- ... and one that does not collide (a message nobody reacted to)
INSERT INTO message_reactions (message_id, user_id, emoji_id)
VALUES (200000000000000001, 100000000000000001, '10000000-0000-4000-8000-0000000000d2');
-- the same unresolvable (NULL) reaction stored by three historical scans
INSERT INTO message_reactions (message_id, user_id, emoji_id)
SELECT 200000000000000001, 100000000000000002, NULL FROM generate_series(1, 3);

-- duplicate topic names (the topic-sorter race) and a message with two topics
INSERT INTO topic (id, name) VALUES ('30000000-0000-4000-8000-0000000000d1', 'rust borrow checker');
-- (seed links every message with message_id % 3 = 0: 200..001 has topic ..02, 200..002 has none)
INSERT INTO topic_message_relation (topic_id, message_id) VALUES
  ('30000000-0000-4000-8000-0000000000d1', 200000000000000002),
  ('30000000-0000-4000-8000-000000000003', 200000000000000001);

-- out-of-range values
INSERT INTO scores (honesty, emotionality, extraversion, agreeableness, conscientiousness,
  openness_to_experience, agency, achievement, influence, sarcasm, security,
  self_reflection, technical_competence, busyness)
VALUES (1.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5);
UPDATE user_facts_and_activities SET confidence = 1.2
WHERE id = '40000000-0000-4000-8000-000000000001';

-- a connected account on a seeded platform (random id in the old schema)
INSERT INTO connected_accounts (vestibule_user_id, platform_id, platform_display_name,
  platform_username, reasoning)
SELECT '00000000-0000-4000-8000-000000000001', id, 'User 1', '@user1', 'test'
FROM social_platforms WHERE platform_name = 'YouTube';
"#;

#[tokio::test]
async fn migration_cleans_up_rows_that_violate_the_new_rules() {
    let Some(url) = common::admin_url() else {
        return;
    };
    let db = baseline_db(&url, "mig_dirty").await;
    db.exec(&read("tests/fixtures/dev_seed.sql")).await;
    db.exec(DIRTY).await;
    let reactions_before = count(&db, "SELECT count(*) FROM message_reactions").await;
    let messages_before = count(&db, "SELECT count(*) FROM messages").await;

    apply_migrations(&db).await;

    // emojis merged, reactions re-pointed, colliding/NULL duplicates removed
    assert_eq!(
        count(
            &db,
            "SELECT count(*) FROM discord_emojis WHERE discord_emoji_id = '👍'"
        )
        .await,
        1
    );
    assert_eq!(
        count(
            &db,
            "SELECT count(*) FROM message_reactions WHERE emoji_id = '10000000-0000-4000-8000-0000000000d2'"
        )
        .await,
        0
    );
    assert_eq!(
        count(&db, "SELECT count(*) FROM message_reactions WHERE message_id = 200000000000000001 AND user_id = 100000000000000001 AND emoji_id = '10000000-0000-4000-8000-000000000001'").await,
        1,
        "non-colliding reaction on the duplicate emoji must be kept and re-pointed"
    );
    assert_eq!(
        count(
            &db,
            "SELECT count(*) FROM message_reactions WHERE emoji_id IS NULL"
        )
        .await,
        1
    );
    // 1 colliding duplicate + 2 extra NULL rows removed
    assert_eq!(
        count(&db, "SELECT count(*) FROM message_reactions").await,
        reactions_before - 3
    );

    // topics merged by name; one topic per message, first link kept
    assert_eq!(
        count(
            &db,
            "SELECT count(*) FROM topic WHERE name = 'rust borrow checker'"
        )
        .await,
        1
    );
    assert_eq!(
        count(&db, "SELECT count(*) FROM (SELECT message_id FROM topic_message_relation GROUP BY 1 HAVING count(*) > 1) s").await,
        0
    );
    assert_eq!(
        count(&db, "SELECT count(*) FROM topic_message_relation WHERE message_id = 200000000000000002 AND topic_id = '30000000-0000-4000-8000-000000000001'").await,
        1,
        "link to the duplicate topic must be re-pointed to the canonical one"
    );
    assert_eq!(
        count(&db, "SELECT count(*) FROM topic_message_relation WHERE message_id = 200000000000000001 AND topic_id = '30000000-0000-4000-8000-000000000002'").await,
        1,
        "the first topic link of a message must be the one kept"
    );

    // out-of-range rows are kept; their checks exist but are NOT VALID ...
    let not_valid: Vec<String> = sqlx::query_scalar(
        "SELECT conname::text FROM pg_constraint WHERE NOT convalidated ORDER BY 1",
    )
    .fetch_all(&db.pool)
    .await
    .unwrap();
    assert_eq!(
        not_valid,
        [
            "scores_values_in_unit_range",
            "user_facts_and_activities_confidence_range"
        ]
    );
    // ... and new violations are rejected
    let err = sqlx::raw_sql(
        "UPDATE user_facts_and_activities SET confidence = 7 WHERE id = '40000000-0000-4000-8000-000000000002'",
    )
    .execute(&db.pool)
    .await
    .unwrap_err();
    assert!(err.to_string().contains("confidence_range"), "{err}");

    // connected account follows its platform to the fixed id
    assert_eq!(
        count(&db, "SELECT count(*) FROM connected_accounts WHERE platform_id = '05f4b960-1dfb-50c5-b9b2-53813526bf14'").await,
        1
    );
    assert_eq!(
        count(&db, "SELECT count(*) FROM messages").await,
        messages_before
    );

    // running it again on the cleaned database changes nothing
    let snapshot = catalog_snapshot(&db.pool).await;
    apply_migrations(&db).await;
    assert_eq!(snapshot, catalog_snapshot(&db.pool).await);

    db.drop_db().await;
}
