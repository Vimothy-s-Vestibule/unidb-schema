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
├── schema.sql              # Auto-generated unified SQL schema by make
├── Makefile                # Run 'make' to rebuild schema.sql from all files in the schema/ directory
└── src/
    ├── models/             # Rust data structs mapped to SQL tables
    ├── models.rs           # Module reexports
    └── lib.rs              
```

## Contributing

When creating new files in the `schema/` folder, please pay attention to their order to ensure all foreign key relationships are satisfied.

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
