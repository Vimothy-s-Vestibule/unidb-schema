# Schema changes and migrations

## Today

`schema/*.sql` is the source of truth. sentience-workers' Postgres image
concatenates it into `/docker-entrypoint-initdb.d/`, so it only runs when the
data volume is created. A database created earlier keeps its old schema forever:
there is no record of which version it has and nothing that upgrades it.

Until a real migration tool is adopted, every schema change that matters for an
existing database ships with a hand-written script in `migrations/`:

- named `YYYYMMDD_short_name.sql`, applied in filename order;
- idempotent (safe to run twice, a no-op on a database created from the current
  `schema/*.sql`), wrapped in one transaction;
- handles existing rows that the change would reject (merge/clean, or add a
  `CHECK ... NOT VALID` and report) instead of failing halfway;
- tested by `tests/migrations.rs`: baseline schema
  (`tests/fixtures/baseline_schema_a7a7c55.sql`) + `tests/fixtures/dev_seed.sql`
  + all migrations must produce exactly the catalog of a fresh `schema/*.sql`
  database (`contract/schema.json` content), and must clean up dirty data.

Apply to an existing database (take a backup first):

```sh
for f in migrations/*.sql; do
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$f"
done
```

## Proposal: adopt `sqlx migrate`

The workers already use sqlx, so `sqlx::migrate!` / `sqlx-cli` adds no new
dependency and gives what the current setup lacks: a `_sqlx_migrations` table that
records which migrations ran (with checksums), and a single command for both fresh
and existing databases.

1. Baseline: `migrations/0001_baseline.sql` = current `schema/*.sql`. For existing
   databases, insert the matching `_sqlx_migrations` row once (or run the
   idempotent scripts above and then mark the baseline applied).
2. Every later change is a new numbered migration; `schema/*.sql` is either
   regenerated from the migrated catalog (`pg_dump --schema-only`) or dropped in
   favour of the migrations directory. The contract test keeps checking that
   "fresh DB from migrations" == `contract/schema.json`.
3. Run migrations from one place only, e.g. a one-shot `migrate` service in
   sentience-workers' compose file that the other services depend on
   (`service_completed_successfully`), instead of `docker-entrypoint-initdb.d`.
   The Postgres image then no longer needs to download the schema at build time
   (which today pulls `main`, while the workers are compiled against the rev
   pinned in their Cargo.lock).
4. Version the crate together with the schema: bump `unidb`'s version (and tag
   the commit) whenever `schema/` or `src/models` change, and have the workers pin
   that tag; the Postgres image / migrate service uses the same tag.
5. The dashboard must not use `drizzle-kit push`/`migrate` against this database;
   its drizzle schema should be checked against `contract/schema.json` instead.

Rules for writing migrations (either system): never edit an applied migration;
additive changes first (new nullable column / `NOT VALID` constraint), backfill,
then tighten; destructive changes only after every consumer has shipped the code
that no longer needs the old shape.
