# 0001 — Integrate services through the shared unidb Postgres database

- **Status:** Accepted (reconstructed from code, 2026-11-04)
- **Scope:** unidb-schema, sentience-workers, Sentience-dash

## Context

Sentience consists of four runtime components in three repositories: the Discord ingest bot, two LLM workers
(fact-extractor, topic-sorter) and the SvelteKit dashboard. They have to exchange messages, users, channels, media
metadata, topics and extracted facts. The project is maintained by a very small team, deployed with one docker-compose
file on one host, and is expected to grow more producers (YouTube, GitHub, Strava, presence, scoring — tables for these
already exist).

## Decision

All components integrate **only through the shared Postgres database "unidb"**. There are no service APIs, RPCs, queues
or events between them.

- The schema is owned by the `unidb-schema` repo: hand-written `schema/*.sql` (applied in filename order) is the source
  of truth; the `unidb` Rust crate provides matching `ormlite`/`sqlx` models and is consumed by the workers as a git
  dependency pinned in `sentience-workers/Cargo.lock`.
- Each worker reads its input tables and writes its output tables directly (`sqlx::query!` macros checked against a live
  database at compile time, plus `ormlite` inserts).
- The dashboard reads the same database directly through a **hand-written drizzle mirror**
  (`Sentience-dash/src/lib/server/db/schema.ts`). It is read-only by convention.
- The Postgres image (`sentience-workers/docker/postgres/Dockerfile`) bakes the schema of a given unidb-schema branch
  (`UNIDB_REF`) into `/docker-entrypoint-initdb.d` at image build time.

Evidence: no HTTP server or client exists between components (only the dash serves HTTP, to browsers); every
cross-component data flow in `docs/SPEC.md` §3/§5 goes through a table.

## Consequences

Positive
- Minimal infrastructure: one Postgres (+ rustfs for blobs); nothing else to deploy or monitor.
- Strong consistency and referential integrity across components (FKs, CHECKs, the deferred evidence trigger) instead of
  eventual consistency between services.
- New consumers (e.g. another LLM worker, an analytics notebook) can be added without changing producers.
- Ad-hoc inspection, backfills and re-processing are plain SQL (e.g. deleting attempts rows re-queues work, ADR 0002).

Negative
- **The schema is the API, with three representations** (SQL, Rust models, drizzle) kept in sync by hand. Drift already
  exists (drizzle types `intro_message_id` as `text` and two `bytea` columns as `vector(1536)`). Any column change needs
  coordinated edits in three repos.
- **No migrations**: the schema is applied only to an empty data directory; existing deployments need manual `ALTER`s.
  Breaking changes cannot be rolled out per service.
- **Compile-time coupling**: workers do not compile without a reachable database containing the current schema (no
  `.sqlx` offline cache), which also breaks `docker compose build`. The dash build likewise needs `DATABASE_URL`.
- **No access boundary**: every component holds full read/write credentials (single `DATABASE_URL`); the dash can read
  every message and every inferred personal fact, and `drizzle-kit push` from the dash repo could rewrite shared tables.
- Contention and coordination happen in the database (no locks today; see ADR 0002), and one component's heavy queries
  affect all others.
- Pinned git dependency: workers silently keep using an old `unidb` until someone runs `cargo update -p unidb`.

## Alternatives considered (reconstructed)

- **Per-service HTTP/gRPC APIs** (ingest exposes messages, workers expose results): clearer contracts and access control,
  but more code, deployment and versioning work than the project's size justified.
- **Message broker / event stream** (ingest publishes `message.created`, workers subscribe): decouples timing and enables
  fan-out, but adds infrastructure and still needs a store for the dashboard; the LLM workers need historical context
  queries that a log alone does not serve.
- **Generated clients from one schema** (e.g. drizzle introspection or sqlx-generated types in CI) would keep the
  shared-database decision but remove hand-sync drift; not done yet and a natural follow-up.
