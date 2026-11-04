//! Contract between the SQL schema (`schema/*.sql`) and the Rust models
//! (`src/models`).
//!
//! For every model, against a fresh database loaded from the schema files:
//!
//! 1. the model's columns (ormlite `TableMeta::table_columns`, which already
//!    applies `#[ormlite(column = ...)]` renames) are exactly the table's columns;
//! 2. fully- and minimally-populated fixture rows round-trip through
//!    `Model::insert` and a `SELECT` decoded with the model's `FromRow`. sqlx checks
//!    type compatibility on every non-NULL value, so `i32` vs `bigint`, `f32` vs
//!    `double precision`, `String` vs `uuid`, enum labels etc. fail here; the
//!    test also asserts every column was non-NULL in at least one fixture so
//!    that check really ran for every column;
//! 3. nullability: each column is replaced by a typed `NULL` in turn and the
//!    row decoded again. Decoding succeeds iff the Rust field is an `Option`,
//!    which must match the column's `NOT NULL`-ness.
//!
//! The fixtures are struct literals, so adding/removing a struct field without
//! updating them is a compile error, and adding a column without a field fails (1).
//!
//! For every Rust enum the serde names, the sqlx-encoded labels and
//! `enum_range(NULL::type)` must be the same set, and every SQL enum and every
//! table must have a Rust counterpart.

mod common;

use std::collections::BTreeSet;
use std::fmt::Debug;

use chrono::{DateTime, Duration, TimeZone, Utc};
use ormlite::{Model, TableMeta};
use pgvector::Vector;
use serde::Serialize;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Postgres;
use uuid::Uuid;

use unidb::models::*;

// ---------------------------------------------------------------------------
// Prerequisite rows that fixtures reference through foreign keys.
// ---------------------------------------------------------------------------

const CHANNEL_CATEGORY: i64 = 1;
const CHANNEL: i64 = 2;
const USER: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0001);
const ACCOUNT: i64 = 10;
const MESSAGE: i64 = 100;
const MESSAGE_2: i64 = 101;
const ASSET: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_00a1);
const SCORE: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_00b1);
const EMOJI: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_00e1);
const PRESENCE: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_00f1);
const PLATFORM: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0099);
const CONNECTED: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_00c1);
const VIDEO: &str = "prelude-video";
const COMMENT: &str = "prelude-comment";
const EXTERNAL: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_00d1);
const TOPIC: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0071);
const FACT: Uuid = Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0081);

fn prelude_sql() -> String {
    format!(
        r#"
INSERT INTO discord_channels (channel_id, name, channel_type, parent_channel_id)
  VALUES ({CHANNEL_CATEGORY}, 'prelude-category', 'category', NULL),
         ({CHANNEL}, 'prelude-general', 'text', {CHANNEL_CATEGORY});
INSERT INTO vestibule_users (id) VALUES ('{USER}');
INSERT INTO discord_accounts (discord_user_id, vestibule_user_id, username, display_name)
  VALUES ({ACCOUNT}, '{USER}', 'prelude_user', 'Prelude User');
INSERT INTO messages (message_id, channel_id, sent_by, content, sent_at)
  VALUES ({MESSAGE}, {CHANNEL}, {ACCOUNT}, 'prelude message', '2024-01-01T00:00:00Z'),
         ({MESSAGE_2}, {CHANNEL}, {ACCOUNT}, 'prelude message 2', '2024-01-01T00:00:01Z');
INSERT INTO media_assets (id, content_type, object_key) VALUES ('{ASSET}', 'image/png', 'prelude/asset');
INSERT INTO scores (id, honesty, emotionality, extraversion, agreeableness, conscientiousness,
                    openness_to_experience, agency, achievement, influence, sarcasm, security,
                    self_reflection, technical_competence, busyness)
  VALUES ('{SCORE}', 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5);
INSERT INTO discord_emojis (id, discord_emoji_id) VALUES ('{EMOJI}', 'prelude-emoji');
INSERT INTO discord_user_presence (id, user_id, status) VALUES ('{PRESENCE}', {ACCOUNT}, 'online');
-- the seeded social_platforms rows have random ids; use our own platform
INSERT INTO social_platforms (id, platform_name, homepage, access_type)
  VALUES ('{PLATFORM}', 'ContractTest', 'https://example.test', 'public');
INSERT INTO connected_accounts (id, vestibule_user_id, platform_id, platform_display_name,
                                platform_username, reasoning)
  VALUES ('{CONNECTED}', '{USER}', '{PLATFORM}', 'Prelude', '@prelude', 'prelude');
INSERT INTO youtube_videos (video_id, channel_vestibule_id, title, published_at, duration_seconds, broadcast_status)
  VALUES ('{VIDEO}', '{CONNECTED}', 'Prelude video', '2024-01-01T00:00:00Z', 60, 'video');
INSERT INTO youtube_comments (comment_id, video_id, author_raw_channel_id, content, like_count, published_at)
  VALUES ('{COMMENT}', '{VIDEO}', 'UCprelude', 'prelude comment', 0, '2024-01-01T00:00:00Z');
INSERT INTO external_content (id, account_id, content_type, raw_data)
  VALUES ('{EXTERNAL}', '{CONNECTED}', 'github_repo', '{{}}');
INSERT INTO topic (id, name) VALUES ('{TOPIC}', 'prelude topic');
BEGIN;
INSERT INTO user_facts_and_activities (id, user_id, record_type, source, type, value)
  VALUES ('{FACT}', '{USER}', 'fact', 'manual', 'location', 'Vienna');
INSERT INTO fact_and_activity_evidence (fact_or_activity_id, message_id, reasoning)
  VALUES ('{FACT}', {MESSAGE}, 'prelude');
COMMIT;
"#
    )
}

// ---------------------------------------------------------------------------
// Generic checker
// ---------------------------------------------------------------------------

#[derive(Debug, sqlx::FromRow)]
struct CatalogColumn {
    name: String,
    sql_type: String,
    nullable: bool,
}

async fn catalog_columns(pool: &PgPool, table: &str) -> Vec<CatalogColumn> {
    sqlx::query_as(
        "SELECT a.attname::text AS name, format_type(a.atttypid, a.atttypmod) AS sql_type,
                NOT a.attnotnull AS nullable
         FROM pg_attribute a
         WHERE a.attrelid = ('public.' || quote_ident($1))::regclass
           AND a.attnum > 0 AND NOT a.attisdropped
         ORDER BY a.attnum",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .unwrap()
}

fn quote(ident: &str) -> String {
    format!("\"{}\"", ident.replace('"', "\"\""))
}

/// Primary-key value of a fixture as Postgres text (`pk::text`).
fn pk_text<T: Serialize>(row: &T, pk: &str) -> String {
    match &serde_json::to_value(row).unwrap()[pk] {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        other => panic!("unsupported primary key value {other:?}"),
    }
}

struct Spec<T> {
    fixtures: Vec<T>,
    /// Extra SQL executed inside the insert transaction, after the fixtures
    /// (needed for the deferred evidence trigger on user_facts_and_activities).
    in_tx_sql: Option<&'static str>,
}

async fn check_model<T>(label: &str, spec: Spec<T>)
where
    T: Model<Postgres>
        + TableMeta
        + for<'r> sqlx::FromRow<'r, PgRow>
        + Serialize
        + Debug
        + Clone
        + Send
        + Sync
        + Unpin
        + 'static,
{
    let Some(db) = test_db!(label).await else {
        return;
    };
    db.exec(&prelude_sql()).await;

    let table = T::table_name();
    let pk = T::primary_key().unwrap_or_else(|| panic!("{table}: model has no primary key"));
    let catalog = catalog_columns(&db.pool, table).await;
    assert!(
        !catalog.is_empty(),
        "{table}: table does not exist in schema/*.sql"
    );

    // 1. column sets ---------------------------------------------------------
    let model_cols: BTreeSet<&str> = T::table_columns().iter().copied().collect();
    let sql_cols: BTreeSet<&str> = catalog.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(
        model_cols,
        sql_cols,
        "{table}: column drift.\n  only in Rust model: {:?}\n  only in SQL table: {:?}",
        model_cols.difference(&sql_cols).collect::<Vec<_>>(),
        sql_cols.difference(&model_cols).collect::<Vec<_>>()
    );
    assert_eq!(
        T::table_columns().len(),
        model_cols.len(),
        "{table}: duplicate column in model"
    );

    // 2. round trip ----------------------------------------------------------
    let mut tx = db.pool.begin().await.unwrap();
    for (i, fixture) in spec.fixtures.iter().enumerate() {
        let returned = fixture.clone().insert(&mut *tx).await.unwrap_or_else(|e| {
            panic!("{table}: inserting fixture #{i} via ormlite failed: {e}\n{fixture:?}")
        });
        assert_eq!(
            format!("{returned:?}"),
            format!("{fixture:?}"),
            "{table}: INSERT .. RETURNING of fixture #{i} did not round-trip"
        );
    }
    if let Some(sql) = spec.in_tx_sql {
        sqlx::raw_sql(sql).execute(&mut *tx).await.unwrap();
    }
    tx.commit()
        .await
        .unwrap_or_else(|e| panic!("{table}: commit failed: {e}"));

    let select_list = catalog
        .iter()
        .map(|c| quote(&c.name))
        .collect::<Vec<_>>()
        .join(", ");
    for (i, fixture) in spec.fixtures.iter().enumerate() {
        let sql = format!(
            "SELECT {select_list} FROM {} WHERE {}::text = $1",
            quote(table),
            quote(pk)
        );
        let fetched: T = sqlx::query_as(&sql)
            .bind(pk_text(fixture, pk))
            .fetch_one(&db.pool)
            .await
            .unwrap_or_else(|e| {
                panic!("{table}: SELECT/decode of fixture #{i} failed (type drift?): {e}")
            });
        assert_eq!(
            format!("{fetched:?}"),
            format!("{fixture:?}"),
            "{table}: fixture #{i} did not round-trip"
        );
    }

    // every column must have been non-NULL in some fixture, otherwise sqlx never
    // checked its type
    let fixture_pks: Vec<String> = spec.fixtures.iter().map(|f| pk_text(f, pk)).collect();
    for col in &catalog {
        let sql = format!(
            "SELECT count({}) FROM {} WHERE {}::text = ANY($1)",
            quote(&col.name),
            quote(table),
            quote(pk)
        );
        let non_null: i64 = sqlx::query_scalar(&sql)
            .bind(&fixture_pks)
            .fetch_one(&db.pool)
            .await
            .unwrap();
        assert!(
            non_null > 0,
            "{table}.{}: no fixture sets this column, so its type is never checked; \
             add a fully-populated fixture",
            col.name
        );
    }

    // 3. nullability ---------------------------------------------------------
    let mut mismatches = Vec::new();
    for col in &catalog {
        let list = catalog
            .iter()
            .map(|c| {
                if c.name == col.name {
                    format!("NULL::{} AS {}", c.sql_type, quote(&c.name))
                } else {
                    quote(&c.name)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT {list} FROM {} WHERE {}::text = $1",
            quote(table),
            quote(pk)
        );
        let rust_optional = match sqlx::query_as::<_, T>(&sql)
            .bind(&fixture_pks[0])
            .fetch_one(&db.pool)
            .await
        {
            Ok(_) => true,
            Err(sqlx::Error::ColumnDecode { source, .. })
                if source.is::<sqlx::error::UnexpectedNullError>() =>
            {
                false
            }
            Err(e) => panic!(
                "{table}.{}: unexpected error decoding NULL probe: {e}",
                col.name
            ),
        };
        if rust_optional != col.nullable {
            mismatches.push(format!(
                "{table}.{} is {} in SQL ({}) but {} in Rust",
                col.name,
                if col.nullable { "NULLable" } else { "NOT NULL" },
                col.sql_type,
                if rust_optional {
                    "Option<_>"
                } else {
                    "not Option<_>"
                },
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "nullability drift:\n  {}",
        mismatches.join("\n  ")
    );

    db.drop_db().await;
}

// ---------------------------------------------------------------------------
// Fixtures. Timestamps have microsecond precision (what timestamptz stores).
// ---------------------------------------------------------------------------

fn ts(sec: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2024, 5, 6, 7, 8, 9).unwrap()
        + Duration::seconds(sec)
        + Duration::microseconds(123_456)
}

fn uid(n: u128) -> Uuid {
    Uuid::from_u128(0x1111_0000_0000_4000_8000_0000_0000_0000 | n)
}

fn vector() -> Vector {
    Vector::from(vec![0.5, -1.0, 2.25])
}

/// A snowflake above 2^53 (JS `number` precision) to make sure nothing narrows it.
const BIG_SNOWFLAKE: i64 = 1_234_567_890_123_456_789;

#[tokio::test]
async fn model_discord_channel() {
    check_model(
        "discord_channel",
        Spec {
            fixtures: vec![
                DiscordChannel {
                    channel_id: BIG_SNOWFLAKE,
                    name: "full".into(),
                    channel_type: DiscordChannelType::PublicThread,
                    parent_channel_id: Some(CHANNEL),
                },
                DiscordChannel {
                    channel_id: 3,
                    name: "minimal".into(),
                    channel_type: DiscordChannelType::Text,
                    parent_channel_id: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_platform() {
    check_model(
        "platform",
        Spec {
            fixtures: vec![
                Platform {
                    id: uid(1),
                    platform_name: "TestFull".into(),
                    homepage: "https://full.example".into(),
                    access_type: PlatformAccess::OauthRequired,
                    logo: Some(ASSET),
                },
                Platform {
                    id: uid(2),
                    platform_name: "TestMinimal".into(),
                    homepage: "https://minimal.example".into(),
                    access_type: PlatformAccess::Public,
                    logo: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_connected_account() {
    check_model(
        "connected_account",
        Spec {
            fixtures: vec![
                ConnectedAccount {
                    id: uid(1),
                    vestibule_user_id: USER,
                    platform_id: PLATFORM,
                    platform_user_id: Some("UCfull".into()),
                    platform_display_name: "Full".into(),
                    platform_username: "@full".into(),
                    bio: Some("bio".into()),
                    bio_additional_info: Some("extra".into()),
                    profile_picture: Some(ASSET),
                    name_embedding: Some(vector()),
                    mention_message_id: Some(MESSAGE),
                    reasoning: "mentioned".into(),
                },
                ConnectedAccount {
                    id: uid(2),
                    vestibule_user_id: USER,
                    platform_id: PLATFORM,
                    platform_user_id: None,
                    platform_display_name: "Minimal".into(),
                    platform_username: "@minimal".into(),
                    bio: None,
                    bio_additional_info: None,
                    profile_picture: None,
                    name_embedding: None,
                    mention_message_id: None,
                    reasoning: "manual".into(),
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_external_content() {
    check_model(
        "external_content",
        Spec {
            fixtures: vec![
                ExternalContent {
                    id: uid(1),
                    account_id: CONNECTED,
                    content_type: "github_repo".into(),
                    raw_data: serde_json::json!({"name": "unidb", "stars": 3, "nested": [1, null]}),
                    content_hash: Some("abc".into()),
                    fetched_at: ts(1),
                    updated_at: Some(ts(2)),
                },
                ExternalContent {
                    id: uid(2),
                    account_id: CONNECTED,
                    content_type: "strava_activity".into(),
                    raw_data: serde_json::json!({}),
                    content_hash: None,
                    fetched_at: ts(3),
                    updated_at: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_fact_and_activity_evidence() {
    let base = FactAndActivityEvidence {
        id: uid(0),
        fact_or_activity_id: FACT,
        message_id: None,
        youtube_comment_id: None,
        external_content_id: None,
        discord_presence_id: None,
        weight: 0.75,
        reasoning: "because".into(),
    };
    // has_single_evidence_source: exactly one source per row, so one row per source
    check_model(
        "evidence",
        Spec {
            fixtures: vec![
                FactAndActivityEvidence {
                    id: uid(1),
                    message_id: Some(MESSAGE_2),
                    ..base.clone()
                },
                FactAndActivityEvidence {
                    id: uid(2),
                    youtube_comment_id: Some(COMMENT.into()),
                    ..base.clone()
                },
                FactAndActivityEvidence {
                    id: uid(3),
                    external_content_id: Some(EXTERNAL),
                    ..base.clone()
                },
                FactAndActivityEvidence {
                    id: uid(4),
                    discord_presence_id: Some(PRESENCE),
                    weight: 1.0,
                    ..base.clone()
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_job() {
    let base = Job {
        id: uid(0),
        job_type: JobType::YoutubeChannelRetrieval,
        status: JobStatus::Pending,
        locked_by_worker_id: None,
        locked_at: None,
        scheduled_for: ts(0),
        attempts: 0,
        max_attempts: 3,
        last_error: None,
        youtube_video_id: None,
        discord_channel_id: None,
        connected_account_id: None,
        media_asset_id: None,
    };
    // has_single_target: exactly one target per row, so one row per target
    check_model(
        "job",
        Spec {
            fixtures: vec![
                Job {
                    id: uid(1),
                    status: JobStatus::InProgress,
                    locked_by_worker_id: Some("worker-1".into()),
                    locked_at: Some(ts(1)),
                    attempts: 2,
                    max_attempts: 5,
                    last_error: Some("boom".into()),
                    youtube_video_id: Some(VIDEO.into()),
                    ..base.clone()
                },
                Job {
                    id: uid(2),
                    job_type: JobType::DiscordChannelSync,
                    discord_channel_id: Some(CHANNEL),
                    ..base.clone()
                },
                Job {
                    id: uid(3),
                    job_type: JobType::StravaRetrieval,
                    connected_account_id: Some(CONNECTED),
                    ..base.clone()
                },
                Job {
                    id: uid(4),
                    job_type: JobType::LinkedinScrape,
                    status: JobStatus::Failed,
                    media_asset_id: Some(ASSET),
                    ..base.clone()
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_media_asset() {
    check_model(
        "media_asset",
        Spec {
            fixtures: vec![
                MediaAsset {
                    id: uid(1),
                    content_type: "image/webp".into(),
                    object_key: "test/full".into(),
                    size_bytes: Some(5_000_000_000),
                    content_hash: Some("e3b0c442".into()),
                    embedding: Some(vector()),
                },
                MediaAsset {
                    id: uid(2),
                    content_type: "image/png".into(),
                    object_key: "test/minimal".into(),
                    size_bytes: None,
                    content_hash: None,
                    embedding: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_message() {
    check_model(
        "message",
        Spec {
            fixtures: vec![
                Message {
                    message_id: BIG_SNOWFLAKE,
                    channel_id: CHANNEL,
                    sent_by: ACCOUNT,
                    content: "full ✓ 🦀".into(),
                    sent_at: ts(1),
                    last_edited: Some(ts(2)),
                    deleted_at: Some(ts(3)),
                    in_reply_to: Some(MESSAGE),
                    added_at: ts(4),
                },
                Message {
                    message_id: 102,
                    channel_id: CHANNEL,
                    sent_by: ACCOUNT,
                    content: String::new(),
                    sent_at: ts(5),
                    last_edited: None,
                    deleted_at: None,
                    in_reply_to: None,
                    added_at: ts(6),
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_message_attachment() {
    check_model(
        "message_attachment",
        Spec {
            fixtures: vec![
                MessageAttachment {
                    id: uid(1),
                    message_id: MESSAGE,
                    asset_id: ASSET,
                    added_at: ts(1),
                    deleted_at: Some(ts(2)),
                },
                MessageAttachment {
                    id: uid(2),
                    message_id: MESSAGE,
                    asset_id: ASSET,
                    added_at: ts(3),
                    deleted_at: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_message_edit() {
    check_model(
        "message_edit",
        Spec {
            fixtures: vec![MessageEdit {
                id: uid(1),
                message_id: MESSAGE,
                old_content: "before".into(),
                edited_at: ts(1),
            }],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_presence() {
    check_model(
        "presence",
        Spec {
            fixtures: vec![
                Presence {
                    id: uid(1),
                    user_id: ACCOUNT,
                    status: Some(PresenceStatus::ForcedOnline),
                    activity_type: Some("Playing".into()),
                    name: Some("Factorio".into()),
                    details: Some("In Menu".into()),
                    state: Some("Solo".into()),
                    url: Some("https://example.com".into()),
                    started_at: ts(1),
                    ended_at: Some(ts(2)),
                    evidence_message_id: Some(MESSAGE),
                },
                Presence {
                    id: uid(2),
                    user_id: ACCOUNT,
                    status: None,
                    activity_type: None,
                    name: None,
                    details: None,
                    state: None,
                    url: None,
                    started_at: ts(3),
                    ended_at: None,
                    evidence_message_id: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_message_reaction() {
    check_model(
        "message_reaction",
        Spec {
            fixtures: vec![
                MessageReaction {
                    id: uid(1),
                    message_id: MESSAGE,
                    user_id: ACCOUNT,
                    emoji_id: Some(EMOJI),
                    reacted_at: ts(1),
                },
                MessageReaction {
                    id: uid(2),
                    message_id: MESSAGE,
                    user_id: ACCOUNT,
                    emoji_id: None,
                    reacted_at: ts(2),
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_discord_emoji() {
    check_model(
        "discord_emoji",
        Spec {
            fixtures: vec![
                DiscordEmoji {
                    id: uid(1),
                    discord_emoji_id: "1234567890123456789".into(),
                    from_guild: Some("987654321".into()),
                    emoji_display_name: Some("ferris".into()),
                    is_animated: true,
                    emoji_url: Some("https://cdn.discordapp.com/emojis/1.webp".into()),
                    asset_id: Some(ASSET),
                },
                DiscordEmoji {
                    id: uid(2),
                    discord_emoji_id: "👍".into(),
                    from_guild: None,
                    emoji_display_name: None,
                    is_animated: false,
                    emoji_url: None,
                    asset_id: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_score() {
    let full = Score {
        id: uid(1),
        honesty: 0.1,
        emotionality: 0.2,
        extraversion: 0.3,
        agreeableness: 0.4,
        conscientiousness: 0.5,
        openness_to_experience: 0.6,
        agency: 0.7,
        achievement: 0.8,
        influence: 0.9,
        sarcasm: 1.0,
        security: 0.0,
        self_reflection: 0.123_456_789_012_345_6,
        technical_competence: 0.25,
        busyness: 0.75,
        llm_summary: Some("likes rust".into()),
        embedding: Some(vector()),
    };
    check_model(
        "score",
        Spec {
            fixtures: vec![
                full.clone(),
                Score {
                    id: uid(2),
                    llm_summary: None,
                    embedding: None,
                    ..full
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_topic() {
    check_model(
        "topic",
        Spec {
            fixtures: vec![
                Topic {
                    id: uid(1),
                    name: "rust borrow checker".into(),
                    embedding: Some(vector()),
                },
                Topic {
                    id: uid(2),
                    name: "lunch".into(),
                    embedding: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_topic_message_relation() {
    check_model(
        "topic_message_relation",
        Spec {
            fixtures: vec![TopicMessageRelation {
                id: uid(1),
                topic_id: TOPIC,
                message_id: MESSAGE,
            }],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_message_classification_attempt() {
    check_model(
        "classification_attempt",
        Spec {
            fixtures: vec![MessageClassificationAttempt {
                message_id: MESSAGE,
                attempted_at: ts(1),
            }],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_fact_extraction_attempt() {
    check_model(
        "fact_attempt",
        Spec {
            fixtures: vec![FactExtractionAttempt {
                message_id: MESSAGE,
                attempted_at: ts(1),
            }],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_vestibule_user() {
    check_model(
        "vestibule_user",
        Spec {
            fixtures: vec![
                VestibuleUser {
                    id: uid(1),
                    nickname: Some("Full".into()),
                    intro_message_id: Some(MESSAGE),
                    score_id: Some(SCORE),
                    score_last_updated: Some(ts(1)),
                    current_diagram: Some(vec![0x89, b'P', b'N', b'G', 0, 255]),
                    current_diagram_last_updated: Some(ts(2)),
                    intro_diagram: Some(vec![1, 2, 3]),
                },
                VestibuleUser {
                    id: uid(2),
                    nickname: None,
                    intro_message_id: None,
                    score_id: None,
                    score_last_updated: None,
                    current_diagram: None,
                    current_diagram_last_updated: None,
                    intro_diagram: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_discord_account() {
    check_model(
        "discord_account",
        Spec {
            fixtures: vec![DiscordAccount {
                discord_user_id: BIG_SNOWFLAKE,
                vestibule_user_id: USER,
                username: "full_user".into(),
                display_name: "Full Ünïcødé 🦀".into(),
            }],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_user_fact_and_activity() {
    check_model(
        "fact",
        Spec {
            fixtures: vec![
                UserFactAndActivity {
                    id: uid(1),
                    user_id: USER,
                    record_type: ActivityRecordType::Skill,
                    source: ActivitySource::LlmExtraction,
                    type_str: "programming_language".into(),
                    value: "Rust".into(),
                    confidence: 0.75,
                    level: Some(7),
                    score_id: Some(SCORE),
                    started_at: Some(ts(1)),
                    ended_at: Some(ts(2)),
                    is_current: Some(false),
                    created_at: ts(3),
                    type_value_embedding: Some(vector()),
                },
                UserFactAndActivity {
                    id: uid(2),
                    user_id: USER,
                    record_type: ActivityRecordType::Emotion,
                    source: ActivitySource::Manual,
                    type_str: "mood".into(),
                    value: "happy".into(),
                    confidence: 1.0,
                    level: None,
                    score_id: None,
                    started_at: None,
                    ended_at: None,
                    is_current: None,
                    created_at: ts(4),
                    type_value_embedding: None,
                },
            ],
            // ensure_fact_has_evidence (deferred) requires evidence before commit
            in_tx_sql: Some(
                "INSERT INTO fact_and_activity_evidence (fact_or_activity_id, message_id, reasoning)
                 SELECT f.id, 100, 'contract test' FROM user_facts_and_activities f
                 WHERE NOT EXISTS (SELECT 1 FROM fact_and_activity_evidence e
                                   WHERE e.fact_or_activity_id = f.id)",
            ),
        },
    )
    .await;
}

#[tokio::test]
async fn model_youtube_video() {
    check_model(
        "youtube_video",
        Spec {
            fixtures: vec![
                YoutubeVideo {
                    video_id: "dQw4w9WgXcQ".into(),
                    channel_vestibule_id: CONNECTED,
                    title: "Full".into(),
                    description: Some("desc".into()),
                    published_at: ts(1),
                    total_views: 9_000_000_000,
                    total_likes: 2,
                    total_comments: 3,
                    duration_seconds: 213,
                    broadcast_status: YoutubeVideoBroadcastStatus::PastLive,
                    thumbnail_asset_id: Some(ASSET),
                    transcript_asset_id: Some(ASSET),
                    audio_asset_id: Some(ASSET),
                    keyword_tags: Some(vec!["rust".into(), "db".into()]),
                    added_at: Some(ts(2)),
                },
                YoutubeVideo {
                    video_id: "minimal".into(),
                    channel_vestibule_id: CONNECTED,
                    title: "Minimal".into(),
                    description: None,
                    published_at: ts(3),
                    total_views: 0,
                    total_likes: 0,
                    total_comments: 0,
                    duration_seconds: 0,
                    broadcast_status: YoutubeVideoBroadcastStatus::None,
                    thumbnail_asset_id: None,
                    transcript_asset_id: None,
                    audio_asset_id: None,
                    keyword_tags: None,
                    added_at: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

#[tokio::test]
async fn model_youtube_comment() {
    check_model(
        "youtube_comment",
        Spec {
            fixtures: vec![
                YoutubeComment {
                    comment_id: "full".into(),
                    video_id: VIDEO.into(),
                    author_raw_channel_id: "UCfull".into(),
                    author_channel_id: Some(CONNECTED),
                    content: "nice".into(),
                    like_count: 42,
                    published_at: ts(1),
                    edited_at: Some(ts(2)),
                    in_reply_to: Some(COMMENT.into()),
                    added_at: Some(ts(3)),
                },
                YoutubeComment {
                    comment_id: "minimal".into(),
                    video_id: VIDEO.into(),
                    author_raw_channel_id: "UCmin".into(),
                    author_channel_id: None,
                    content: String::new(),
                    like_count: 0,
                    published_at: ts(4),
                    edited_at: None,
                    in_reply_to: None,
                    added_at: None,
                },
            ],
            in_tx_sql: None,
        },
    )
    .await;
}

// ---------------------------------------------------------------------------
// Coverage: every table / enum in the schema has a Rust counterpart.
// ---------------------------------------------------------------------------

fn model_tables() -> BTreeSet<&'static str> {
    [
        DiscordChannel::table_name(),
        Platform::table_name(),
        ConnectedAccount::table_name(),
        ExternalContent::table_name(),
        FactAndActivityEvidence::table_name(),
        Job::table_name(),
        MediaAsset::table_name(),
        Message::table_name(),
        MessageAttachment::table_name(),
        MessageEdit::table_name(),
        Presence::table_name(),
        MessageReaction::table_name(),
        DiscordEmoji::table_name(),
        Score::table_name(),
        Topic::table_name(),
        TopicMessageRelation::table_name(),
        MessageClassificationAttempt::table_name(),
        VestibuleUser::table_name(),
        DiscordAccount::table_name(),
        UserFactAndActivity::table_name(),
        FactExtractionAttempt::table_name(),
        YoutubeVideo::table_name(),
        YoutubeComment::table_name(),
    ]
    .into_iter()
    .collect()
}

#[tokio::test]
async fn every_table_has_a_tested_model() {
    let Some(db) = test_db!("tables").await else {
        return;
    };
    let sql_tables: BTreeSet<String> = sqlx::query_scalar::<_, String>(
        "SELECT tablename::text FROM pg_tables WHERE schemaname = 'public'",
    )
    .fetch_all(&db.pool)
    .await
    .unwrap()
    .into_iter()
    .collect();
    let rust: BTreeSet<String> = model_tables().into_iter().map(String::from).collect();
    assert_eq!(
        rust,
        sql_tables,
        "table drift.\n  model without table: {:?}\n  table without (tested) model: {:?}",
        rust.difference(&sql_tables).collect::<Vec<_>>(),
        sql_tables.difference(&rust).collect::<Vec<_>>()
    );
    db.drop_db().await;
}

/// Lists every variant. The `match` has no wildcard arm, so adding a variant
/// to a Rust enum without listing it here does not compile.
macro_rules! all_variants {
    ($ty:ident: $($v:ident),+ $(,)?) => {{
        let all = vec![$($ty::$v),+];
        for v in &all {
            match v { $($ty::$v)|+ => {} }
        }
        all
    }};
}

async fn check_enum<T>(pool: &PgPool, sql_type: &str, variants: Vec<T>)
where
    T: Serialize
        + Debug
        + Copy
        + PartialEq
        + Send
        + Unpin
        + sqlx::Type<Postgres>
        + for<'q> sqlx::Encode<'q, Postgres>
        + for<'r> sqlx::Decode<'r, Postgres>
        + 'static,
{
    let mut serde_names = BTreeSet::new();
    let mut sqlx_names = BTreeSet::new();
    for v in &variants {
        let serde_name = serde_json::to_value(v)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        // the type name is part of the SQL so sqlx's statement cache does not
        // reuse a statement prepared for another enum's parameter type
        let sqlx_name: String = sqlx::query_scalar(&format!("SELECT $1::{sql_type}::text"))
            .bind(*v)
            .fetch_one(pool)
            .await
            .unwrap_or_else(|e| {
                panic!("{sql_type}: Rust variant {v:?} is not a value of the SQL enum: {e}")
            });
        assert_eq!(
            serde_name, sqlx_name,
            "{sql_type}: serde and sqlx disagree on the name of {v:?}"
        );
        serde_names.insert(serde_name);
        sqlx_names.insert(sqlx_name);
    }
    let sql_labels: BTreeSet<String> = sqlx::query_scalar::<_, String>(&format!(
        "SELECT unnest(enum_range(NULL::{sql_type}))::text"
    ))
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .collect();
    assert_eq!(
        sqlx_names,
        sql_labels,
        "{sql_type}: enum drift.\n  only in Rust: {:?}\n  only in SQL: {:?}",
        sqlx_names.difference(&sql_labels).collect::<Vec<_>>(),
        sql_labels.difference(&sqlx_names).collect::<Vec<_>>()
    );
    // and every SQL label decodes into a Rust variant
    let decoded: Vec<T> =
        sqlx::query_scalar(&format!("SELECT unnest(enum_range(NULL::{sql_type}))"))
            .fetch_all(pool)
            .await
            .unwrap_or_else(|e| {
                panic!("{sql_type}: SQL label does not decode into Rust enum: {e}")
            });
    assert_eq!(decoded.len(), sql_labels.len());
}

const RUST_ENUMS: &[&str] = &[
    "discord_channel_type",
    "activity_record_type",
    "activity_source",
    "presence_status",
    "platform_access",
    "job_type",
    "job_status",
    "youtube_video_broadcast_status",
];

#[tokio::test]
async fn enums_match_sql_enums() {
    let Some(db) = test_db!("enums").await else {
        return;
    };
    let p = &db.pool;
    check_enum(
        p,
        "discord_channel_type",
        all_variants!(DiscordChannelType: Text, TextThread, PublicThread, ForumPost, Voice, Forum, Stage, Category),
    )
    .await;
    check_enum(
        p,
        "activity_record_type",
        all_variants!(ActivityRecordType: Activity, Fact, Skill, Emotion),
    )
    .await;
    check_enum(
        p,
        "activity_source",
        all_variants!(ActivitySource: LlmExtraction, ExternalContent, Manual),
    )
    .await;
    check_enum(
        p,
        "presence_status",
        all_variants!(PresenceStatus: Online, ForcedOnline, Absent, DoNotDisturb, Offline),
    )
    .await;
    check_enum(
        p,
        "platform_access",
        all_variants!(PlatformAccess: Public, OauthRequired, Unavailable),
    )
    .await;
    check_enum(
        p,
        "job_type",
        all_variants!(JobType: YoutubeChannelRetrieval, DiscordChannelSync, StravaRetrieval, LinkedinScrape),
    )
    .await;
    check_enum(
        p,
        "job_status",
        all_variants!(JobStatus: Pending, InProgress, Completed, Failed),
    )
    .await;
    check_enum(
        p,
        "youtube_video_broadcast_status",
        all_variants!(YoutubeVideoBroadcastStatus: Video, CurrentLive, PastLive, ScheduledLive, None),
    )
    .await;

    let sql_enums: BTreeSet<String> = sqlx::query_scalar::<_, String>(
        "SELECT t.typname::text FROM pg_type t JOIN pg_namespace n ON n.oid = t.typnamespace
         WHERE n.nspname = 'public' AND t.typtype = 'e'",
    )
    .fetch_all(p)
    .await
    .unwrap()
    .into_iter()
    .collect();
    let rust: BTreeSet<String> = RUST_ENUMS.iter().map(|s| s.to_string()).collect();
    assert_eq!(
        rust,
        sql_enums,
        "SQL enum without Rust enum (or vice versa): only Rust {:?}, only SQL {:?}",
        rust.difference(&sql_enums).collect::<Vec<_>>(),
        sql_enums.difference(&rust).collect::<Vec<_>>()
    );
    db.drop_db().await;
}

/// `DiscordChannelType` also has a hand-written `Display`; it must agree.
#[test]
fn discord_channel_type_display_matches_serde() {
    for v in all_variants!(DiscordChannelType: Text, TextThread, PublicThread, ForumPost, Voice, Forum, Stage, Category)
    {
        assert_eq!(
            v.to_string(),
            serde_json::to_value(v).unwrap().as_str().unwrap(),
            "Display of {v:?}"
        );
    }
}
