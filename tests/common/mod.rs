//! Shared harness for the database-backed integration tests.
//!
//! Every test gets its own freshly created database on the server named by
//! `UNIDB_TEST_DATABASE_URL`, loads `schema/*.sql` into it (in filename order,
//! exactly like `docker/postgres/Dockerfile` in sentience-workers does) and drops
//! it again at the end.
//!
//! The variable is deliberately NOT `DATABASE_URL`: the services and the
//! dashboard read `DATABASE_URL` from `.env`, and these tests run
//! `CREATE DATABASE` / `DROP DATABASE` on whatever server they are given. They
//! must never pick up an application database by accident.
//!
//! Point it at a throwaway server, e.g.
//!
//! ```sh
//! docker run -d --name unidb-test -e POSTGRES_PASSWORD=postgres -p 55433:5432 pgvector/pgvector:pg17
//! export UNIDB_TEST_DATABASE_URL=postgres://postgres:postgres@localhost:55433/postgres
//! cargo test
//! ```
//!
//! Without the variable the tests FAIL (so CI cannot silently skip them). Set
//! `UNIDB_SKIP_DB_TESTS=1` to skip them explicitly.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::str::FromStr;

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::{ConnectOptions, Connection, Executor, PgConnection};

pub const URL_VAR: &str = "UNIDB_TEST_DATABASE_URL";
pub const SKIP_VAR: &str = "UNIDB_SKIP_DB_TESTS";

/// Returns the admin URL, `None` if the test should be skipped, panics otherwise.
pub fn admin_url() -> Option<String> {
    match std::env::var(URL_VAR) {
        Ok(url) if !url.trim().is_empty() => Some(url),
        _ if std::env::var(SKIP_VAR).is_ok_and(|v| v == "1") => {
            eprintln!("SKIPPED: {SKIP_VAR}=1 and {URL_VAR} is not set");
            None
        }
        _ => panic!(
            "{URL_VAR} is not set. These tests need a disposable Postgres server with \
             pgvector, e.g. `docker run -d -e POSTGRES_PASSWORD=postgres -p 55433:5432 \
             pgvector/pgvector:pg17` and \
             `{URL_VAR}=postgres://postgres:postgres@localhost:55433/postgres`. \
             Set {SKIP_VAR}=1 to skip them explicitly."
        ),
    }
}

/// `let Some(db) = test_db!("name").await else { return };`
#[macro_export]
macro_rules! test_db {
    ($name:expr) => {
        async {
            match $crate::common::admin_url() {
                Some(url) => Some($crate::common::TestDb::create(&url, $name).await),
                None => None,
            }
        }
    };
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `schema/*.sql`, sorted byte-wise by file name (what `cat schema/*.sql` does
/// in the C locale; the naming test makes sure every locale agrees).
pub fn schema_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(repo_root().join("schema"))
        .expect("read schema/")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "sql"))
        .collect();
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    files
}

pub fn file_name(p: &Path) -> String {
    p.file_name().unwrap().to_string_lossy().into_owned()
}

pub struct TestDb {
    pub pool: PgPool,
    pub name: String,
    admin: PgConnectOptions,
}

impl TestDb {
    /// Creates an empty database (no schema loaded).
    pub async fn create_empty(admin_url: &str, label: &str) -> TestDb {
        let admin = PgConnectOptions::from_str(admin_url)
            .unwrap_or_else(|e| panic!("{URL_VAR} is not a valid postgres URL: {e}"));
        let suffix = uuid::Uuid::new_v4().simple().to_string();
        let name = format!("unidb_t_{}_{}", label, &suffix[..10]).to_lowercase();
        let mut conn = admin
            .connect()
            .await
            .unwrap_or_else(|e| panic!("cannot connect to {URL_VAR}: {e}"));
        conn.execute(format!(r#"CREATE DATABASE "{name}""#).as_str())
            .await
            .expect("CREATE DATABASE");
        conn.close().await.ok();
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(admin.clone().database(&name))
            .await
            .expect("connect to test database");
        TestDb { pool, name, admin }
    }

    /// Creates a database and loads every schema file into it.
    pub async fn create(admin_url: &str, label: &str) -> TestDb {
        let db = Self::create_empty(admin_url, label).await;
        db.load_schema_files().await;
        db
    }

    /// Loads schema/*.sql one file at a time, in order. Panics naming the file
    /// that failed (e.g. because it references a table from a later file).
    pub async fn load_schema_files(&self) {
        for path in schema_files() {
            let sql = std::fs::read_to_string(&path).unwrap();
            if let Err(e) = sqlx::raw_sql(&sql).execute(&self.pool).await {
                panic!("schema file {} failed to apply: {e}", file_name(&path));
            }
        }
    }

    pub async fn exec(&self, sql: &str) {
        if let Err(e) = sqlx::raw_sql(sql).execute(&self.pool).await {
            panic!("SQL failed: {e}\n---\n{sql}");
        }
    }

    pub async fn conn(&self) -> PgConnection {
        self.admin
            .clone()
            .database(&self.name)
            .connect()
            .await
            .unwrap()
    }

    /// Drops the database. Not done on panic, so a failing test leaves its
    /// database behind for inspection (the CI container is ephemeral).
    pub async fn drop_db(self) {
        self.pool.close().await;
        let mut conn = self.admin.connect().await.expect("admin connect");
        conn.execute(format!(r#"DROP DATABASE IF EXISTS "{}" WITH (FORCE)"#, self.name).as_str())
            .await
            .expect("DROP DATABASE");
    }
}

/// Machine-readable description of the `public` schema, built from the live
/// catalog (not from parsing SQL). This is what `contract/schema.json` holds.
///
/// Everything is ordered deterministically; columns keep their physical order
/// because positional `INSERT ... VALUES` (schema/03 seed rows, seed scripts)
/// depends on it.
pub async fn catalog_snapshot(pool: &PgPool) -> serde_json::Value {
    const SQL: &str = r#"
WITH
ext_objs AS (
  SELECT objid FROM pg_depend WHERE deptype = 'e'
),
tbls AS (
  SELECT c.oid, c.relname
  FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
  WHERE n.nspname = 'public' AND c.relkind IN ('r', 'p')
    AND c.oid NOT IN (SELECT objid FROM ext_objs)
),
cols AS (
  SELECT a.attrelid,
         jsonb_agg(jsonb_build_object(
           'name', a.attname,
           'type', format_type(a.atttypid, a.atttypmod),
           'nullable', NOT a.attnotnull,
           'default', pg_get_expr(d.adbin, d.adrelid)
         ) ORDER BY a.attnum) AS j
  FROM pg_attribute a
  LEFT JOIN pg_attrdef d ON d.adrelid = a.attrelid AND d.adnum = a.attnum
  WHERE a.attrelid IN (SELECT oid FROM tbls) AND a.attnum > 0 AND NOT a.attisdropped
  GROUP BY a.attrelid
),
cons AS (
  SELECT conrelid,
         jsonb_agg(jsonb_build_object(
           'name', conname,
           'kind', CASE contype WHEN 'p' THEN 'primary_key' WHEN 'u' THEN 'unique'
                                WHEN 'f' THEN 'foreign_key' WHEN 'c' THEN 'check'
                                WHEN 'x' THEN 'exclusion' ELSE contype::text END,
           'definition', pg_get_constraintdef(oid, true)
         ) ORDER BY contype, conname) AS j
  FROM pg_constraint
  WHERE conrelid IN (SELECT oid FROM tbls)
  GROUP BY conrelid
),
idx AS (
  SELECT i.indrelid,
         jsonb_agg(jsonb_build_object(
           'name', ic.relname,
           'definition', pg_get_indexdef(i.indexrelid)
         ) ORDER BY ic.relname) AS j
  FROM pg_index i JOIN pg_class ic ON ic.oid = i.indexrelid
  WHERE i.indrelid IN (SELECT oid FROM tbls)
  GROUP BY i.indrelid
),
trg AS (
  SELECT tgrelid,
         jsonb_agg(jsonb_build_object(
           'name', tgname,
           'definition', pg_get_triggerdef(oid, true)
         ) ORDER BY tgname) AS j
  FROM pg_trigger
  WHERE tgrelid IN (SELECT oid FROM tbls) AND NOT tgisinternal
  GROUP BY tgrelid
)
SELECT jsonb_build_object(
  'format_version', 1,
  'extensions', (SELECT coalesce(jsonb_agg(extname ORDER BY extname), '[]')
                 FROM pg_extension WHERE extname <> 'plpgsql'),
  'enums', (SELECT coalesce(jsonb_object_agg(typname, labels), '{}') FROM (
              SELECT t.typname, jsonb_agg(e.enumlabel ORDER BY e.enumsortorder) AS labels
              FROM pg_type t
              JOIN pg_namespace n ON n.oid = t.typnamespace
              JOIN pg_enum e ON e.enumtypid = t.oid
              WHERE n.nspname = 'public'
              GROUP BY t.typname) s),
  'functions', (SELECT coalesce(jsonb_agg(jsonb_build_object(
                   'name', p.proname,
                   'definition', pg_get_functiondef(p.oid)) ORDER BY p.proname), '[]')
                FROM pg_proc p JOIN pg_namespace n ON n.oid = p.pronamespace
                WHERE n.nspname = 'public' AND p.oid NOT IN (SELECT objid FROM ext_objs)),
  'tables', (SELECT jsonb_object_agg(t.relname, jsonb_build_object(
                'columns', coalesce(cols.j, '[]'),
                'constraints', coalesce(cons.j, '[]'),
                'indexes', coalesce(idx.j, '[]'),
                'triggers', coalesce(trg.j, '[]')))
             FROM tbls t
             LEFT JOIN cols ON cols.attrelid = t.oid
             LEFT JOIN cons ON cons.conrelid = t.oid
             LEFT JOIN idx ON idx.indrelid = t.oid
             LEFT JOIN trg ON trg.tgrelid = t.oid)
)
"#;
    let snapshot: serde_json::Value = sqlx::query_scalar(SQL)
        .fetch_one(pool)
        .await
        .expect("catalog snapshot query");
    snapshot
}

/// Rows the schema itself seeds, as `{table: [row, ...]}`, ordered by the
/// given column. Random ids (if any) are left out so the snapshot stays
/// deterministic; see `seed_snapshot_columns`.
pub async fn seed_snapshot(pool: &PgPool, tables: &[(&str, &str, &[&str])]) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    for (table, order_by, columns) in tables {
        let cols = columns
            .iter()
            .map(|c| format!("'{c}', {c}"))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT coalesce(jsonb_agg(jsonb_build_object({cols}) ORDER BY {order_by}), '[]') FROM {table}"
        );
        let rows: serde_json::Value = sqlx::query_scalar(&sql).fetch_one(pool).await.unwrap();
        out.insert(table.to_string(), rows);
    }
    serde_json::Value::Object(out)
}
