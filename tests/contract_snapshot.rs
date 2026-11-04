//! `contract/schema.json` is a machine-readable description of the schema
//! (tables, columns with types/nullability/defaults, constraints, indexes,
//! triggers, functions, enums and the rows the schema seeds), generated from the
//! catalog of a database loaded from `schema/*.sql`.
//!
//! Consumers (sentience-workers, Sentience-dash) can check their own models
//! against it without running Rust or Postgres.
//!
//! This test fails when the committed file is stale. Regenerate with
//!
//! ```sh
//! UNIDB_UPDATE_CONTRACT=1 cargo test --test contract_snapshot
//! ```

mod common;

const SNAPSHOT_PATH: &str = "contract/schema.json";
const UPDATE_VAR: &str = "UNIDB_UPDATE_CONTRACT";

/// Seed rows to include: (table, ORDER BY, columns). Only deterministic columns.
const SEEDED: &[(&str, &str, &[&str])] = &[
    (
        "vestibule_users",
        "id",
        &[
            "id",
            "nickname",
            "intro_message_id",
            "score_id",
            "score_last_updated",
            "current_diagram",
            "current_diagram_last_updated",
            "intro_diagram",
        ],
    ),
    (
        "discord_accounts",
        "discord_user_id",
        &[
            "discord_user_id",
            "vestibule_user_id",
            "username",
            "display_name",
        ],
    ),
    // ids are fixed (UUIDv5 of the homepage), so they are part of the contract
    (
        "social_platforms",
        "platform_name",
        &["id", "platform_name", "homepage", "access_type", "logo"],
    ),
];

async fn generate(db: &common::TestDb) -> String {
    let mut snapshot = common::catalog_snapshot(&db.pool).await;
    snapshot["seed"] = common::seed_snapshot(&db.pool, SEEDED).await;
    snapshot["_comment"] = serde_json::Value::String(format!(
        "GENERATED from schema/*.sql by tests/contract_snapshot.rs - do not edit. \
         Regenerate: {UPDATE_VAR}=1 cargo test --test contract_snapshot"
    ));
    serde_json::to_string_pretty(&snapshot).unwrap() + "\n"
}

#[tokio::test]
async fn contract_snapshot_is_up_to_date() {
    let Some(db) = test_db!("snapshot").await else {
        return;
    };
    let generated = generate(&db).await;
    db.drop_db().await;

    let path = common::repo_root().join(SNAPSHOT_PATH);
    if std::env::var(UPDATE_VAR).is_ok_and(|v| v == "1") {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &generated).unwrap();
        eprintln!("wrote {}", path.display());
        return;
    }

    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    if committed != generated {
        let first_diff = committed
            .lines()
            .zip(generated.lines())
            .position(|(a, b)| a != b)
            .unwrap_or_else(|| committed.lines().count().min(generated.lines().count()));
        let context = |s: &str| {
            s.lines()
                .skip(first_diff.saturating_sub(3))
                .take(8)
                .collect::<Vec<_>>()
                .join("\n")
        };
        panic!(
            "{SNAPSHOT_PATH} is stale (schema/*.sql changed without regenerating it).\n\
             Run `{UPDATE_VAR}=1 cargo test --test contract_snapshot` and commit the result.\n\
             First difference at line {}:\n--- committed\n{}\n+++ generated\n{}",
            first_diff + 1,
            context(&committed),
            context(&generated)
        );
    }
}
