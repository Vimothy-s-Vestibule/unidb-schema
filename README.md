# unidb

unidb is the central, unified postgres database for the Vestibule bot ecosystem.

Realtime events emitted by users, on Discord and on other platforms, are aggregated here, natural language extractions and evaluations of them are also stored in unidb for later retrieval through bots like Mnemos.

## Table architecture

The `connected_accounts` table acts as the universal identity table.
Identities are centralized to the `vestibule_users` table, all content (discord messages and discord-related metadata and external (`youtube_comments`, `youtube_videos`, `connected_accounts`) content) descends from the `id` (UUID) of an entry in `vestibule_users`.

NOTE: A `vestibule_users` entry does not strictly need to have a `discord_accounts` entry (but can have one or multiple linking to it via FK) so that users that have not joined the discord can be recorded in the database too.

Many tables include nullable columns to store an LLM-generated evaluation or summary of that row. When INSERTing a row, these columns should be left blank as they will be populated by the processing pipeline.

## Project Structure

```text
.
├── schema/                 # Source of truth: PostgreSQL schema definition files
│   ├── 00_extensions.sql   
│   ├── ...                 
│   └── 99_indexes.sql      
├── contract/schema.json    # Generated, machine-readable schema contract (see below)
├── tests/                  # Schema/model contract tests (need a Postgres server)
├── schema.sql              # Auto-generated unified SQL schema by make
├── Makefile                # Run 'make' to rebuild schema.sql from all files in the schema/ directory
└── src/
    ├── models/             # Rust data structs mapped to SQL tables
    ├── models.rs           # Module reexports
    └── lib.rs              
```

## Contributing

When creating new files in the `schema/` folder, please pay attention to their order to ensure all foreign key relationships are satisfied.

### Tests and the schema contract

`tests/` checks that `schema/*.sql` and the Rust models agree, against a real
Postgres (pgvector) server:

- `schema_load`: the files apply cleanly to an empty database in filename order,
  and `cat schema/*.sql` (what the Postgres image runs) yields the same catalog.
- `model_contract`: for every model, column names, types (int/float widths,
  enums, `bytea`, `vector`, ...) and nullability match the table; fixture rows
  round-trip through the model; every Rust enum has exactly the SQL enum's labels;
  every table and SQL enum has a Rust counterpart.
- `contract_snapshot`: `contract/schema.json` (machine-readable schema, generated
  from the catalog) is up to date.

```sh
make test-db      # disposable pgvector container on 127.0.0.1:55433
make test         # UNIDB_TEST_DATABASE_URL defaults to that container
make contract     # regenerate contract/schema.json after editing schema/*.sql
```

The tests read `UNIDB_TEST_DATABASE_URL` (not `DATABASE_URL`, so they never pick
up an application database) and create/drop their own databases on that server.
Without it they fail; `UNIDB_SKIP_DB_TESTS=1` skips them explicitly.

### Changing the schema of an existing database

The schema files only run when the database is first created. Changes that an
existing database needs also get an idempotent script in `migrations/`; see
[docs/MIGRATIONS.md](docs/MIGRATIONS.md).

`schema.sql` is auto-generated. Make changes in the `schema/*.sql` files and run `make rebuild` to generate it.

### Example

```rust
use unidb::models::{VestibuleUser, Message};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await?;

    let users = sqlx::query_as!(
        VestibuleUser,
        "SELECT * FROM vestibule_users LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for user in users {
        println!("User ID: {}", user.id);
    }

    Ok(())
}
```
