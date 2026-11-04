//! The schema has no migration system: `schema/*.sql` is concatenated and run
//! once, when the Postgres volume is first initialised. These tests make sure
//! that step works on an empty database.

mod common;

use common::{file_name, schema_files, TestDb};

/// File names must be `NN_name.sql` with a unique two-digit prefix, so that
/// `cat schema/*.sql` (Makefile, sentience-workers docker/postgres/Dockerfile)
/// produces the same order under every locale's collation.
#[test]
fn schema_file_names_define_an_unambiguous_order() {
    let files = schema_files();
    assert!(!files.is_empty(), "schema/ has no .sql files");
    let mut prefixes = std::collections::BTreeSet::new();
    for path in &files {
        let name = file_name(path);
        let bytes = name.as_bytes();
        let well_formed = name.len() > 7
            && bytes[0].is_ascii_digit()
            && bytes[1].is_ascii_digit()
            && bytes[2] == b'_'
            && name.ends_with(".sql")
            && name[3..name.len() - 4]
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        assert!(
            well_formed,
            "{name}: schema files must be named NN_lowercase_name.sql"
        );
        assert!(
            prefixes.insert(name[..2].to_string()),
            "{name}: duplicate numeric prefix {}",
            &name[..2]
        );
    }
}

/// Every file must end with a newline: the files are concatenated, and a final
/// `-- comment` without one would swallow the first line of the next file.
#[test]
fn schema_files_end_with_newline() {
    for path in schema_files() {
        let sql = std::fs::read_to_string(&path).unwrap();
        assert!(
            sql.ends_with('\n'),
            "{} does not end with a newline",
            file_name(&path)
        );
    }
}

/// Applies the files one by one, in order, to an empty database. A file that
/// references a table/type created by a later file fails here, naming the file.
#[tokio::test]
async fn schema_applies_cleanly_in_file_order() {
    let Some(url) = common::admin_url() else {
        return;
    };
    let db = TestDb::create_empty(&url, "load").await;
    db.load_schema_files().await;

    // sanity: the objects consumers rely on exist
    let tables: i64 =
        sqlx::query_scalar("SELECT count(*) FROM pg_tables WHERE schemaname = 'public'")
            .fetch_one(&db.pool)
            .await
            .unwrap();
    assert!(tables > 0, "no tables were created");

    let vector: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'vector')")
            .fetch_one(&db.pool)
            .await
            .unwrap();
    assert!(vector, "pgvector extension not installed");

    // ingest-vestibule-retriever inserts placeholder messages authored by
    // discord_user_id 0; that account must be seeded by the schema.
    let deleted_user: Option<String> =
        sqlx::query_scalar("SELECT username FROM discord_accounts WHERE discord_user_id = 0")
            .fetch_optional(&db.pool)
            .await
            .unwrap();
    assert_eq!(deleted_user.as_deref(), Some("deleteduser"));

    db.drop_db().await;
}

/// The Postgres image does `cat schema/*.sql > unidb-schema.sql` and runs that
/// as ONE script. It must produce exactly the same catalog as applying the
/// files one at a time.
#[tokio::test]
async fn concatenated_schema_equals_file_by_file_schema() {
    let Some(url) = common::admin_url() else {
        return;
    };
    let per_file = TestDb::create(&url, "perfile").await;

    let concatenated = TestDb::create_empty(&url, "concat").await;
    let mut script = String::new();
    for path in schema_files() {
        script.push_str(&std::fs::read_to_string(&path).unwrap());
    }
    concatenated.exec(&script).await;

    let a = common::catalog_snapshot(&per_file.pool).await;
    let b = common::catalog_snapshot(&concatenated.pool).await;
    assert_eq!(a, b, "concatenated schema differs from file-by-file schema");

    per_file.drop_db().await;
    concatenated.drop_db().await;
}

/// Every table needs a primary key (ormlite models and drizzle both assume one).
#[tokio::test]
async fn every_table_has_a_primary_key() {
    let Some(db) = test_db!("pk").await else {
        return;
    };
    let missing: Vec<String> = sqlx::query_scalar(
        "SELECT c.relname::text FROM pg_class c
         JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE n.nspname = 'public' AND c.relkind = 'r'
           AND NOT EXISTS (SELECT 1 FROM pg_constraint p
                           WHERE p.conrelid = c.oid AND p.contype = 'p')
         ORDER BY 1",
    )
    .fetch_all(&db.pool)
    .await
    .unwrap();
    assert!(
        missing.is_empty(),
        "tables without primary key: {missing:?}"
    );
    db.drop_db().await;
}
