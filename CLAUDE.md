# CLAUDE.md — unidb-schema

Full system spec: https://github.com/Vimothy-s-Vestibule/unidb-schema/blob/main/docs/SPEC.md
(lands with the docs PR that adds `docs/SPEC.md` + `docs/adr/`; until merged, read it on that branch).

## What this repo is

The shared Postgres (17 + pgvector) schema **unidb** and the Rust crate `unidb` that models it. It is the contract
between the other two repos — there are no service APIs, everything integrates through these tables (ADR 0001):

| Repo | Uses unidb how |
|---|---|
| `Creative-Difficulty/sentience-workers` | Rust workspace (ingest bot, fact-extractor, topic-sorter). Depends on this crate as a **git dep pinned in its `Cargo.lock`**; its `docker/postgres` image downloads `schema/*.sql` of branch `UNIDB_REF` (default `main`) from GitHub at image build time. |
| `Creative-Difficulty/Sentience-dash` | SvelteKit dashboard; hand-written drizzle mirror of the schema in `src/lib/server/db/schema.ts` (read-only). |

Layout: `schema/NN_*.sql` (source of truth, applied in filename order) · `src/models/*.rs` (one `ormlite::Model` per
table) · `src/models/enums.rs` (Postgres enums) · `src/scoring_schema.rs` (`scoring_prompt()`, currently unused by any
service) · `Makefile` (builds the git-ignored `schema.sql`).

## Commands (verified on main)

```sh
cargo build                      # no database needed
cargo test                       # 2 unit tests (trait-name lists == HexacoTraits/BehavioralTraits fields)
cargo clippy --all-targets       # clean
cargo fmt -- --check             # currently FAILS on main (src/models/job.rs, src/models/youtube.rs) — run `cargo fmt`
make schema.sql                  # cat schema/*.sql > schema.sql ; `make rebuild` forces it, `make clean` removes it
```

Load the schema into a throwaway local database (never a shared/production one):

```sh
docker run -d --name unidb-local -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=sentience \
  -p 127.0.0.1:5433:5432 pgvector/pgvector:pg17-trixie
cat schema/*.sql | docker exec -i unidb-local psql -U postgres -d sentience -v ON_ERROR_STOP=1
```

`.env.example` only has `DATABASE_URL` (not needed by this crate itself; consumers using `sqlx::query!` need it).

## Schema change procedure (all four, in one coordinated change)

1. **SQL**: edit/add `schema/NN_*.sql`. Order matters (FK dependencies: `00` extensions+enums, `01` independent tables …
   `99` indexes); add cyclic FKs with a later `ALTER TABLE` as in `03_users_and_accounts.sql`.
   Verify: load all files into an empty DB (above).
2. **Rust**: update the model in `src/models/` (field names = column names, `Option<T>` for nullable columns, enums in
   `enums.rs` with `#[sqlx(type_name = "...", rename_all = "snake_case")]`), re-export it in `src/models.rs`, bump
   `version` in `Cargo.toml` for breaking changes.
3. **Dash**: update `Sentience-dash/src/lib/server/db/schema.ts` by hand (camelCase TS keys, explicit snake_case column
   names; bigint as `{ mode: "number" }`; timestamps `{ withTimezone: true, mode: 'string' }`).
4. **Workers**: fix affected `sqlx::query!` SQL, then `cargo update -p unidb` in sentience-workers so its lock picks up
   the new commit, and rebuild against a DB that has the new schema.
5. **Existing databases**: there is **no migration system**. `schema/*.sql` only runs on an empty Postgres data dir
   (Docker first init). Ship a hand-written `ALTER …` script/instructions in the PR description.

Never run `drizzle-kit push` (or `db:push`/`db:migrate`) from the dash against a shared unidb database: the drizzle
mirror is lossy and push would alter/drop real tables.

## Conventions

- `snake_case` for SQL tables/columns/enum labels and Rust fields; Rust enum variants `PascalCase` (mapped by
  `rename_all = "snake_case"`); drizzle uses camelCase identifiers.
- Discord snowflakes are `bigint` / `i64` (`u64 as i64`). Never store them as `text`; in JS send them as strings.
- Timestamps are `timestamptz` / `chrono::DateTime<Utc>`. Ids are `uuid DEFAULT gen_random_uuid()` / `uuid::Uuid`
  (ingest generates v4 client-side).
- Reserved word column `type` → `#[ormlite(column = "type")] pub type_str: String`.
- Embedding columns are untyped `vector` (dimension undecided, see `TODO.md`); most models `#[serde(skip)]` them.
- LLM-populated columns are left `NULL` on insert (README).
- Invariants you must keep: exactly-one-source CHECK on `fact_and_activity_evidence`, exactly-one-target CHECK on
  `jobs`, deferred trigger `ensure_fact_has_evidence` (insert a fact **and** its evidence in one transaction), seeded
  "Deleted User" (`3d299903-2abf-4930-b2b9-8b083c1d21b2`, Discord account `0`) used for placeholder messages
  (`sent_by = 0`, `content = '[deleted message]'`, `sent_at = deleted_at = 1970-01-01`), attempts tables mean
  "row present = processed".

## Gotchas

- Workers pin a specific git rev of this crate; merging here changes nothing for them until `cargo update -p unidb`.
- The postgres image fetches the schema from **GitHub `main`** (or `UNIDB_REF`) at build time — pushing to `main` changes
  what new databases get.
- `social_platforms` is seeded with `gen_random_uuid()` ids: ids differ per environment; look platforms up by name.
- No range CHECKs on `confidence`, `weight`, `scores.*`; `scoring_prompt()` says "Return null" but `scores.*` are NOT NULL
  and the Rust fields are `f64` (known drift).
- `topic.name`, `discord_emojis.discord_emoji_id`, `topic_message_relation.message_id` are not UNIQUE;
  `discord_accounts.username` IS unique although Discord usernames are reusable.
- FKs from attempts tables / `topic_message_relation` to `messages` have no `ON DELETE`: messages are soft-deleted only.
- The README example uses `sqlx::query_as!`, which needs `DATABASE_URL` at compile time in the consuming crate.
- Many tables (`scores`, `jobs`, `discord_user_presence`, `connected_accounts`, `youtube_*`, `external_content`) have no
  producer yet; check `docs/SPEC.md` §5 (CRUD matrix) before assuming data exists.
