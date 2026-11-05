# 0002 — LLM workers poll `messages` and record completion in per-worker attempts tables

- **Status:** Accepted (reconstructed from code, 2026-11-04)
- **Scope:** fact-extractor, topic-sorter, unidb tables `fact_extraction_attempts`, `message_classification_attempts`,
  `jobs`

## Context

Every ingested Discord message must be processed by each LLM worker (fact extraction, topic classification) exactly once
in the happy path, including the backlog produced by ingest's historical scan. Processing is slow and costs money (one
or more LLM calls per message or per batch). Workers can crash or be restarted at any time. The schema already contains a
generic `jobs` table (`schema/08_jobs.sql`) with `status`, `attempts`/`max_attempts`, `locked_by_worker_id`/`locked_at`,
`scheduled_for`, `last_error` and polymorphic targets (YouTube video, Discord channel, connected account, media asset —
but **not** message).

## Decision

Each LLM worker is a **single-process polling loop over `messages`**, with a **dedicated "attempts" table** that records
which messages it has finished:

- Work selection: `SELECT … FROM messages m WHERE NOT EXISTS (SELECT 1 FROM <attempts> a WHERE a.message_id =
  m.message_id) AND <worker filters> ORDER BY … LIMIT N` (fact-extractor: N = 25, `ORDER BY sent_at`; topic-sorter:
  N = 50, `ORDER BY channel_id, sent_at`).
- After a message (fact-extractor) or channel group (topic-sorter) is processed successfully — including "LLM returned
  nothing" — its ids are inserted into the attempts table (`INSERT … SELECT UNNEST($1::bigint[]) ON CONFLICT DO
  NOTHING`). **Row present = processed; no row = not yet processed or failed.**
- Failures are only logged; the message stays unrecorded and is selected again on the next iteration. When nothing
  succeeded (or nothing is pending) the loop sleeps 5 s.
- The generic `jobs` table is **not** used for message processing (it is unused overall).

Evidence: `sentience-workers/fact-extractor/src/extractor.rs:191-247`,
`sentience-workers/topic-sorter/src/sorter.rs:60-118`, `schema/09_activities.sql:41-44`, `schema/11_topics.sql:15-21`.

## Consequences

Positive
- No queue infrastructure and no enqueue step in ingest: any row that lands in `messages` (live, historical, or inserted
  by hand) is picked up automatically.
- Workers are independent: each has its own progress table, so adding a worker = adding a table, and one worker's
  backlog never blocks another.
- Operationally simple: re-process a message or everything by deleting attempts rows; progress is visible with a
  `COUNT(*)`.
- A failed message is retried automatically (e.g. after an LLM rate limit), which is what the worker READMEs promise.

Negative
- **No failure state, attempt counter or backoff.** A message that always fails is retried forever; because selection is
  `ORDER BY … LIMIT N`, ≥ N permanently failing messages at the head stall the worker completely while still spending
  LLM tokens (head-of-line blocking).
- **Not atomic:** results (facts/evidence, topics/relations) and the attempts row are written in separate
  transactions; a crash in between re-processes the message and duplicates results.
- **No claiming/locking:** two replicas of the same worker would select the same rows and do the work twice; the design
  only supports one instance per worker type.
- `NOT EXISTS` anti-join over all of `messages` on every iteration gets slower as history grows; the 5 s sleep is the only
  rate control.
- Processing state for one message is spread over several tables (and unidb `TODO.md` already proposes centralising it).
- The attempts FKs have no `ON DELETE`, so processed messages can no longer be hard-deleted (soft delete only).

## Alternatives considered (reconstructed)

- **Use the existing `jobs` table** (add a `message_id` target, claim rows with `FOR UPDATE SKIP LOCKED`, use
  `attempts`/`max_attempts`/`last_error`): gives retries with limits, locking and visibility, but needs an enqueue step for
  every message × worker and changes to the single-target CHECK.
- **Status columns on `messages`** (`triage_status`, `skill_status`, … as sketched in unidb `TODO.md:7-25`): simple, but
  couples every new worker to the `messages` table definition.
- **External queue** (Redis/NATS/SQS, or Postgres-based queues such as pgmq/pg-boss): robust delivery semantics, at the
  cost of extra infrastructure, contrary to ADR 0001's minimal-infrastructure stance.
- **Extending the attempts tables** with `status`, `attempt_count`, `last_error`, `next_attempt_at` and writing results +
  attempt in one transaction is the smallest change that removes the main negatives while keeping this decision; a future
  ADR should record it if adopted.
