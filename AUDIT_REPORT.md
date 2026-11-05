# Sentience system audit — November 2026

Scope: `Vimothy-s-Vestibule/unidb-schema` (`c20a146`), `Creative-Difficulty/sentience-workers` (`c3432cb`),
`Creative-Difficulty/Sentience-dash` (`1fe79e2`). All three were audited as one product. Everything was run
locally in a sandbox: Docker, local Postgres + pgvector, a local rustfs, a mock LLM. No production system, real
Discord token or real LLM key was touched.

Companion documents in this PR: [`docs/SPEC.md`](docs/SPEC.md) (system specification),
[`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md), [`docs/adr/`](docs/adr/README.md).

---

## 1. Executive summary (plain language)

**What the system is.** A Discord bot copies everything said on one Discord server into a shared Postgres
database. Two background workers send those messages to an AI model: one pulls out "facts" about each person
(where they live, what they work on, their skills), the other groups messages into topics. A website shows
charts, a conversation browser and a page per person with the extracted facts. The database layout is the
contract between the three repositories.

**Overall verdict.** The core idea works and the code is readable, but the system is not yet safe to run
unattended or to expose to the internet. The problems fall into five buckets:

1. **It didn't build from a clean checkout.**
   - The workers could only compile while connected to a live database.
   - The website couldn't build at all without a database password set.
   - So the documented `docker compose up` never worked for a newcomer.
   - *Fixed in PRs.*
2. **It silently loses or duplicates data.**
   - Sticker-only messages lost their stickers.
   - Message edit history was recorded backwards.
   - A user whose Discord username had been used by someone else before could never be saved, and neither could
     any of their messages, permanently.
   - Reactions and emoji rows got duplicated on restarts.
   - Threads were never scanned.
   - *Fixed in PRs, each with a test that failed before.*
3. **The AI workers can get permanently stuck.**
   - If the AI returns something unusable for a batch of messages, the worker retries that same batch forever.
     Nothing behind it is ever processed, and every retry costs money.
   - The topic worker also pastes *every topic name ever created* into every request, so requests grow until
     they fail.
   - The end-to-end test reproduced the stall.
   - *Fixed in the LLM-workers PR.*
4. **Security.**
   - The dashboard has no login, yet shows every message and every AI-inferred personal fact (including health
     and relationship facts the prompt asks for).
   - The database and file store were published on all network interfaces with default passwords.
   - Database error messages, including SQL text, were shown to website visitors.
   - There were 32 known-vulnerable Rust dependencies and 57 npm advisories.
   - *Most of this is fixed in PRs. Real authentication is a decision for you (see roadmap).*
5. **Almost no tests.**
   - Before: 2 tests across all three repos.
   - After: about 330, plus fuzzing, mutation testing, an end-to-end test of the whole stack, and CI for every repo.

**The single most important structural risk** is not a bug. The database layout is written out by hand three
times: SQL, Rust and TypeScript. There is no migration system, so changes never reach an existing database
automatically. The new contract tests now catch drift. Adopting real migrations is the top roadmap item.

**Nothing secret leaked.** gitleaks found no leaks in any repo history. One weak hard-coded database role
password exists in old unidb history (§2, S-1).

---

## 2. Findings

Severity:
- **Critical** = exploitable now, or the system cannot run.
- **High** = data loss, corruption or a significant security gap.
- **Medium** = correctness or robustness issue with limited blast radius.
- **Low** = hygiene.

Line numbers refer to the audited `main` commits. "Fixed in" points to the draft PR (see §3 for links).

### Critical

| ID | Location | Finding | Fixed in |
|---|---|---|---|
| C-1 | Sentience-dash `src/routes/**` (no `hooks.server.ts`) | **No authentication** on a site that serves all messages (including private channels the bot can read) plus every LLM-inferred personal fact. Page data is also exposed as JSON via SvelteKit `__data.json`. | Optional Basic auth: dash#5. **Real SSO/proxy auth is still open** (roadmap) |
| C-2 | workers `docker-compose.yml:59,62`, `.env.example` | Postgres published on 0.0.0.0:5432 with default superuser `postgres/postgres`. Superuser allows `COPY … FROM PROGRAM`, i.e. code execution in the DB container. | Loopback-only by default: workers#3. Default password still allowed (roadmap) |
| C-3 | workers (no `.sqlx/`), `Dockerfile` | Workspace **does not compile without a live DB**, so `docker compose build` fails from a clean clone (29× `set DATABASE_URL…`; re-verified). | workers#3 |
| C-4 | Sentience-dash `src/lib/server/db/index.ts:6` | DB module throws at import time, so `bun run build` and `docker build` **always fail**. | dash#9 |
| C-5 | workers `fact-extractor/src/extractor.rs` `run()`, `topic-sorter/src/sorter.rs` `run()` | **Head-of-line stall and unbounded LLM spend.** Failed messages are never recorded, and the queue is `ORDER BY … LIMIT N`. One batch that always fails is re-fetched every 5 s forever and nothing else is processed. Reproduced by the e2e negative test. | workers LLM-workers PR (§3) |

### High

| ID | Location | Finding | Fixed in |
|---|---|---|---|
| H-1 | workers `ingest…/handlers/discord_user.rs:20-51`; unidb `schema/03_users_and_accounts.sql:28` | `ensure_user` is non-transactional and racy: 16 concurrent events → 15 orphan users. Errors are swallowed but the user is cached anyway. `username UNIQUE` + Discord recycled usernames → that user **and all their messages can never be stored**. | workers#5 (code), unidb#9 (drop UNIQUE + migration) |
| H-2 | workers `ingest…/discord_handler.rs:73`, `handlers/message_edit.rs:9` | Live edits never recorded (serenity cache holds 0 messages). History recorded **new content as old**. Embed-only updates created bogus edits. Stale events overwrote newer content. | workers#5 |
| H-3 | workers `ingest…/historical/scan.rs:23,38` | Historical scan **never scans threads** and buffers whole channels in memory (a crash loses all progress). It re-runs on every gateway re-identify. | workers#5 (threads, paging). Re-run on reconnect: roadmap |
| H-4 | workers `ingest…/handlers/channel.rs` insert loop | A channel whose parent is hidden fails its FK, is **cached as stored anyway**, then every message in it fails forever. | workers#5 |
| H-5 | unidb `schema/04_reactions.sql:6,34`; workers `historical/reactions.rs:63`, `handlers/reaction.rs` | `discord_emoji_id` not UNIQUE (racy duplicates). NULL `emoji_id` never deduplicates, so the historical scan duplicates those reactions on **every restart**. Live `reaction_add` fails on replays. | unidb#9, workers#5 |
| H-6 | Sentience-dash `src/lib/server/db/schema.ts` (23 int8 columns) | Discord snowflakes mapped as JS `number` (> 2^53) are **silently rounded** (…993 → …992). This is latent today only because queries cast to text. | dash#4 |
| H-7 | Sentience-dash all loaders (`+page.server.ts:156`, `explorer/…:19`, `userdata/…:31`, `userdata/[id]/…:90`) | `{error: e.toString()}` renders **full SQL text and parameters** to visitors (e.g. `/userdata/not-a-uuid`). | dash#5 |
| H-8 | workers `docker-compose.yml:28-39` | rustfs admin console on 0.0.0.0:9001 with `devadmin/devadmin`, CORS `*`, debug logging. Anyone on the network can download every stored attachment. | workers#3 (loopback, log level). CORS/default creds: roadmap |
| H-9 | workers `Cargo.lock` | `cargo audit`: **32 vulnerabilities + 6 warnings**, incl. rustls/webpki (distrusted CA roots, handshake), h2, quinn-proto. | workers#2 (→ 8, all non-actionable and justified) |
| H-10 | Sentience-dash `bun.lock` | `bun audit`: **57 advisories** (16 high), incl. SvelteKit unauthenticated Accept-header ReDoS, devalue, svelte SSR XSS. | dash#2 (`--prod`: 26 → **0**) |
| H-11 | Sentience-dash `src/routes/+layout.svelte:38,93,108-110` | **The known layout issue.** Root cause: `<SideNav isOpen fixed>` at every width (`fixed` disables Carbon's mobile overlay/hamburger) + hard-coded `margin-left: 256px`. On phones the nav covers 240 px of every page and can't be closed; the explorer is unusable; pages overflow by up to 501 px. | dash#7 (with before/after screenshots) |
| H-12 | workers `topic-sorter/src/sorter.rs:155-166` | **Every** topic name ever created is injected into every system prompt, so prompt size grows without bound (combined with C-5: permanent stall). It is also a persistent prompt-injection vector. | workers LLM-workers PR |
| H-13 | workers `fact-extractor/src/extractor.rs` | Facts committed in one transaction, attempt row inserted later in a separate statement. A crash in between **duplicates facts**. Same pattern in topic-sorter. | workers LLM-workers PR |

### Medium

| ID | Location | Finding | Fixed in |
|---|---|---|---|
| M-1 | workers `fact-extractor/src/extractor.rs:449-480` | **Cross-user prompt injection:** ~20 other users' messages are placed in the *system* prompt. A user can steer facts stored against the focal author. Facts are free-text and include special categories. | Partly (LLM-workers PR bounds/validates output). Full fix: roadmap |
| M-2 | workers `ingest…/handlers/attachments.rs:17`, `discord_user.rs:78` | Unbounded downloads into memory, no timeout, HTTP status ignored (a 404 page was stored as the asset). Memory DoS by any member posting large files. | workers#5 (25 MiB cap, 60 s timeout, status check) |
| M-3 | workers `ingest…/handlers/attachments.rs`, `discord_user.rs:108` | Media rows inserted after a failed S3 upload. Identical avatars collide on the content-hash UNIQUE. Concurrent identical attachments lost links (7/32 kept). | workers#5 |
| M-4 | workers `ingest…/handlers/message.rs:91,180` | Reply parent looked up in the wrong channel. Forwards treated as replies. Fake placeholders. Sticker-only messages lost stickers. | workers#5 |
| M-5 | unidb `schema/01,09,10` | No CHECK ranges on `scores.*`, `confidence`, `weight`. fact-extractor stores unclamped LLM values. | unidb#9 (CHECKs), workers LLM-workers PR (clamp) |
| M-6 | unidb `schema/11_topics.sql` | `topic.name` and `topic_message_relation.message_id` not UNIQUE. Racy duplicates, and joins multiply rows. No index on `message_id`. | unidb#9 |
| M-7 | unidb `src/scoring_schema.rs:103` | Prompt says "return null" but `scores.*` is `NOT NULL` / `f64`. Also a broken line continuation. | unidb#9 |
| M-8 | unidb `schema/06_connected_accounts.sql:13-20` | `social_platforms` seeded with random UUIDs, so ids differ per environment. | unidb#9 (UUIDv5 + remapping migration) |
| M-9 | unidb (whole repo); workers `docker/postgres/Dockerfile:3,11` | **No migration system.** Schema applies only at DB first-init. Postgres image pulls unidb `main` while workers compile against `a7a7c55`, so the DB and the code can disagree. | Pin: workers#3. Idempotent migration + proposal: unidb#9. Adoption: roadmap |
| M-10 | Sentience-dash `schema.ts:41,45,48,182…,577` | drizzle mirror drift: **40 differences** (types, vector dims, bytea-as-vector, enum order, opclasses, missing indexes, NULLS NOT DISTINCT). | dash#4 (contract test, 40 → 1 known) |
| M-11 | Sentience-dash `package.json` `db:push` | `drizzle-kit push` against the shared DB drops an FK and 4 indexes, converts bigint→text, then aborts half-way with no transaction (verified on a copy). | dash#3 (guarded) |
| M-12 | Sentience-dash `src/routes/api/messages/+server.ts:21` | `channelId`/`before` unvalidated, so `'abc'::bigint` gives a 500. | dash#5 |
| M-13 | Sentience-dash `src/routes/api/messages/+server.ts:51` | Reply preview shows the content of **deleted** parent messages. | dash#5 |
| M-14 | Sentience-dash (no CSP/headers) | No CSP, frame protection or nosniff. No `{@html}` exists, so no live XSS, but no defence in depth. | dash#5 (nonce CSP, headers) |
| M-15 | Sentience-dash `src/routes/+page.server.ts:21,80,97,123-129` | Overview charts depend on the **DB session timezone**. 24 h counts include deleted messages. Epoch placeholder rows leak into charts outside UTC (20,763 daily buckets instead of 46). | dash#6 |
| M-16 | Sentience-dash `api/messages/+server.ts:77,94,103`; `userdata/[id]` | `${col}::text IN (…)` defeats bigint indexes (seq scans). | dash#6 (reactions 18.4 → 0.19 ms; `/` 660 → 231 ms) |
| M-17 | Sentience-dash pages | 6 serious/critical axe violations (invalid lists, contrast 2.3:1). A 2 MB SVG used as favicon + LCP image. `optimizeCss` never ran. | dash#8 (a11y 93 → 100, SEO 92 → 100) |
| M-18 | all services | Healthcheck file `/tmp/ready` written before the service is ready. Default log filter `=trace` logs message content and raw LLM output (PII in logs). | LLM-workers PR (readiness for those two); logging: roadmap |
| M-19 | all DB clients | `sslmode=prefer` / no TLS, so there is no server authentication. READMEs recommend Supabase over the internet. | Roadmap |
| M-20 | workers `docker-compose.yml:2` | `env_file: .env` on every worker means every service receives every secret. Missing `.env` made compose fail. | workers#3 (`required: false`). Per-service secrets: roadmap |
| M-21 | workers `ingest…/discord_handler.rs` | No `message_delete` handler (`delete_message` is dead code). Deleted messages persist and are still sent to the LLM. | Roadmap (privacy) |
| M-22 | Discord-user privacy | Profiles built from messages incl. GDPR Art. 9 categories, no consent/retention/erasure, content sent to a third-party LLM. | Product/legal decision (roadmap) |

### Low

| ID | Location | Finding | Fixed in |
|---|---|---|---|
| L-1 | all three repos | No CI at all. `cargo fmt --check` fails in both Rust repos. `bun run lint` fails. | unidb#8, dash#3; workers CI: roadmap |
| L-2 | workers `Dockerfile`, compose | Containers ran as root. Unpinned `:latest` images. No `init` (every stop waited for SIGKILL). Fixed `container_name`s. | workers#3 |
| L-3 | workers | Two async-openai versions (0.37/0.38). aws-sdk default features pulled a second, legacy TLS stack. | workers#2 (TLS stack); LLM-workers PR (async-openai) |
| L-4 | workers `ingest…/handlers/attachments.rs` | Uploader-controlled `content_type` (CR/LF, empty) passed to S3/DB. Found by fuzzing. | workers#6 |
| L-5 | serenity 0.12.5 `model/misc.rs:146` (upstream) | `ImageHash::from_str` panics on non-ASCII input (found by fuzzing; not reachable from Discord). | Reported; pinned by a test in workers#6 |
| L-6 | unidb `schema/06_connected_accounts.sql:63` | `CHECK (mention_message_id IS NOT NULL OR reasoning IS NOT NULL)` is vacuous (`reasoning` is NOT NULL). | Roadmap |
| L-7 | unidb `schema/99_indexes.sql` | Missing indexes: `message_reactions.emoji_id`, `topic_message_relation.topic_id`, `connected_accounts.platform_id`, `fact_and_activity_evidence.message_id`. | Partly (unidb#9 indexes `message_id`); rest: roadmap |
| L-8 | unidb `schema/01,06,09,11` | Untyped `vector` columns (no dimension), so no HNSW index is possible. Nothing writes embeddings yet. | Roadmap (`TODO.md`) |
| L-9 | unidb `src/models/enums.rs:36-52` | `forum_post` never produced. News/media channels stored as `text`. | Roadmap |
| L-10 | workers `ingest…/main.rs:56-60` | Privileged `GUILD_MEMBERS` intent requested but unused. | Roadmap |
| L-11 | workers `ingest…/Cargo.lock`, `ingest…/.dockerignore` | Dead nested files (ignored inside the workspace). | Roadmap |
| L-12 | licensing | sentience-workers and Sentience-dash have **no license** (all rights reserved by default). unidb declares MIT but has no `LICENSE` file. No copyleft dependencies (only MPL-2.0 dev tooling). | Owner decision |
| L-13 | Sentience-dash CI (dash#3/#4) | GitHub Actions pinned by tag, not SHA. | Roadmap |

### Secrets (rule 5: found, not used)

| ID | Repo / file | Commit | Value | Verdict |
|---|---|---|---|---|
| S-1 | unidb-schema `migrations/2026-03-15-170646-0000_add_krauze_user/up.sql` | added `514341a`, removed `9b6b672` | `CREATE ROLE "krauzekrap" … PASSWORD 'temp…'` (redacted) | **True positive.** Weak hard-coded DB login password in history. If this role ever existed on a real database, drop it or rotate it. |
| S-2 | workers `docker-compose.yml`, `.env.example` | `3c51999` → HEAD | `post…`, `deva…` | Insecure defaults, not a leak. |
| — | everything else | all refs, full history (gitleaks 8.28 + manual regexes for Discord/AWS/OpenAI/GitHub tokens, PEM, Supabase URLs) | — | No findings. |

---

## 3. Pull requests opened (all **draft**)

Merge order matters where noted. "Stacked" PRs target the previous branch so each diff shows one layer; GitHub
retargets them to `main` as their bases merge.

### unidb-schema
| PR | Title | Notes |
|---|---|---|
| [#5](https://github.com/Vimothy-s-Vestibule/unidb-schema/pull/5) | [docs] Add CLAUDE.md onboarding guide | |
| [#6](https://github.com/Vimothy-s-Vestibule/unidb-schema/pull/6) | [security] Patch vulnerable transitive crates, cargo-deny/audit config | |
| [#7](https://github.com/Vimothy-s-Vestibule/unidb-schema/pull/7) | [contract] Schema↔model contract tests + `contract/schema.json` | merge before #9 |
| [#8](https://github.com/Vimothy-s-Vestibule/unidb-schema/pull/8) | [ci] GitHub Actions: fmt, clippy, tests against pgvector | |
| [#9](https://github.com/Vimothy-s-Vestibule/unidb-schema/pull/9) | [contract] Schema integrity + idempotent migration | stacked on #7; ship with workers LLM-workers PR + #5 |
| this PR | [docs] SPEC, threat model, ADRs, audit report | |

### sentience-workers
| PR | Title | Notes |
|---|---|---|
| [#1](https://github.com/Creative-Difficulty/sentience-workers/pull/1) | [docs] Add CLAUDE.md onboarding guide | |
| [#2](https://github.com/Creative-Difficulty/sentience-workers/pull/2) | [security] Patch vulnerable deps, drop legacy AWS TLS stack, cargo-deny/audit | |
| [#3](https://github.com/Creative-Difficulty/sentience-workers/pull/3) | [docker] `.sqlx` offline build, hardened Dockerfile, pinned schema/images | regenerate `.sqlx` after rebasing PRs that change queries |
| [#4](https://github.com/Creative-Difficulty/sentience-workers/pull/4) | [docker] One-command full local stack + mock LLM + e2e test | stacked on #3; needs dash#9 |
| [#5](https://github.com/Creative-Difficulty/sentience-workers/pull/5) | [arch] ingest: data-loss and race fixes | |
| [#6](https://github.com/Creative-Difficulty/sentience-workers/pull/6) | [fuzz] ingest: proptest + cargo-fuzz + mutants | stacked on #5 |
| LLM_WORKERS_PRS | | |

### Sentience-dash
| PR | Title | Notes |
|---|---|---|
| [#1](https://github.com/Creative-Difficulty/Sentience-dash/pull/1) | [docs] Add CLAUDE.md onboarding guide | |
| [#2](https://github.com/Creative-Difficulty/Sentience-dash/pull/2) | [security] Patch vulnerable npm packages (57 → 6) | `bun.lock` conflicts with #4; re-run `bun install` |
| [#3](https://github.com/Creative-Difficulty/Sentience-dash/pull/3) | [ci] Fix lint, CI workflow, guard drizzle scripts | stack 1/4 |
| [#4](https://github.com/Creative-Difficulty/Sentience-dash/pull/4) | [tests] vitest/fast-check/contract/e2e/Stryker; snowflake + drift fixes | stack 2/4 |
| [#5](https://github.com/Creative-Difficulty/Sentience-dash/pull/5) | [security] Param validation, no SQL leaks, CSP/headers, optional basic auth | stack 3/4 |
| [#6](https://github.com/Creative-Difficulty/Sentience-dash/pull/6) | [perf] UTC bucketing, index-friendly filters | stack 4/4 |
| [#7](https://github.com/Creative-Difficulty/Sentience-dash/pull/7) | [css] Responsive layout (before/after screenshots) | merge before #8 |
| [#8](https://github.com/Creative-Difficulty/Sentience-dash/pull/8) | [css] Accessibility fixes + 2 MB owl SVG | conflict resolution described in PR |
| [#9](https://github.com/Creative-Difficulty/Sentience-dash/pull/9) | [docker] Fix Docker build (lazy DB), pin bun, `/healthz` | |

---

## 4. Numbers: before vs after

All counts are from actual runs. Runs marked ✔ were re-executed independently by the orchestrator, not only by
the workstream that wrote the code.

### Tests

| Repo | Before | After (tip of each PR stack) |
|---|---|---|
| unidb-schema | 2 unit | **48** (lib 5, schema_load 5, model_contract 26, contract_snapshot 1, integrity 8, migrations 3) ✔ |
| sentience-workers: ingest | 0 | **74** (incl. 12 proptest properties × 512 cases) ✔ |
| sentience-workers: fact-extractor + topic-sorter | 0 | LLM_WORKERS_TESTS |
| sentience-workers: mock LLM | — | 5 (Python unit) |
| full stack | no e2e | **1 e2e** (`scripts/e2e.sh`: 13/13 checks PASS; the negative control correctly FAILs) |
| Sentience-dash | 0 | **128** (100 unit incl. fast-check ✔, 1 schema contract, 5 integration, 22 Playwright e2e) |
| **Total** | **2** | LLM_WORKERS_TOTAL |

### Fuzzing / property-based testing

| Target | Tool | Duration | Executions | Crashes |
|---|---|---|---|---|
| ingest `message_mapping` | cargo-fuzz | 721 s | 6,064,550 | 1 in first run (content-type) → fixed, 0 after |
| ingest `raw_payloads` | cargo-fuzz | 721 s | 28,283,067 | 1 in first run (upstream serenity panic) → pinned, 0 after |
| LLM_WORKERS_FUZZ | | | | |
| dash param parsing / grouping | fast-check | 50,000 runs/property | — | 0 |
| ingest mapping | proptest | 12 × 512 cases | — | 0 |

### Mutation testing

| Scope | Tool | Score |
|---|---|---|
| ingest mapping/media/scan | cargo-mutants | 62.4% → **94.5%** (103/109; 6 equivalent/untestable) |
| LLM_WORKERS_MUTANTS | | |
| dash `assemble.ts` | Stryker | **100%** (62/62) |
| dash `assemble.ts` + `server/http` | Stryker | **98.08%** (4 equivalent survivors) |

### Other before/after

| Metric | Before | After |
|---|---|---|
| Workers build without DB | fails (29 errors) ✔ | passes ✔ |
| Dash `docker build` | fails | passes (276 MB, non-root) |
| `docker compose up` full stack from clean clone | impossible | healthy in 57.6 s, no secrets needed |
| `cargo audit` workers / unidb | 32 + 6 / 24 + 6 | 8 + 1 / 8 + 0 (all justified ignores) |
| `bun audit --prod` | 26 | **0** |
| Layout check (overflow / nav overlap) | 11/20 shots pass | 46/46 |
| axe serious/critical | 6 | 0 |
| Lighthouse a11y / SEO (overview) | 93 / 92 | 100 / 100 |
| `/` median latency (200k msgs) | 660 ms | 231 ms |
| Worker image size (ingest) | 184 MB, root | 172 MB, uid 10001 |
| drizzle ↔ SQL drift | 40 | 1 (documented) |

---

## 5. Architecture review

Written for a non-specialist owner, ranked by impact.

1. **One database shared by everything, and its layout is written three times.**
   - **How it works today.** All three repos talk only through Postgres. That is a reasonable choice at this
     size (ADR 0001). The cost is that the table layout lives in SQL, again in Rust and again in TypeScript,
     and no migration system exists.
   - **What goes wrong.** A change can reach one copy but not the others, and it never reaches an existing
     database.
   - **What the PRs add.** Contract tests (unidb#7, dash#4) that fail on drift, and a first idempotent
     migration (unidb#9).
   - **Next steps.**
     - Adopt `sqlx migrate` in unidb-schema; the proposal is in `docs/MIGRATIONS.md`.
     - Tag unidb versions, pinning the workers' `Cargo.lock`, the Postgres image and the dash contract to the
       same tag.
     - Check a `schema_version` at worker startup.
2. **The AI workers' "to-do list" is fragile (ADR 0002).**
   - **How it works today.** "Done" = a row exists in an `*_attempts` table. There is no "failed" or
     "in progress" state and no lock.
   - **Single-replica problems.** One bad batch stalls the queue, and a crash can duplicate facts. The
     LLM-workers PR fixes both.
   - **Problem with two replicas.** Running two copies of a worker would process the same messages twice and
     pay twice. Still open.
   - **Recommended design.** A per-message status row (`pending / in_progress(locked_by, locked_at) / done /
     failed(attempts, last_error, next_attempt_at)`) claimed with
     `UPDATE … WHERE id IN (SELECT … FOR UPDATE SKIP LOCKED LIMIT n) RETURNING`. The schema already has an
     unused `jobs` table built for this.
3. **The ingester treated an in-memory list as the truth.**
   - **How it worked.** It loaded every message, channel and user id into memory at startup and decided "already
     stored?" from that list, even when the database write had failed.
   - **What the fix changes.** workers#5 makes the database the source of truth: transactions, upserts, and
     caching only after commit.
   - **Remaining.**
     - Each Discord event still spawns an unbounded task against a 5-connection pool; add a semaphore.
     - The full history rescan runs again on every reconnect; make it single-flight with a stored per-channel
       cursor.
4. **Errors are logged and forgotten.**
   - **How it works today.** There are no metrics and no visible list of failures, and the healthchecks report
     "ready" too early.
   - **Next steps.**
     - JSON logs at `info`, without message content.
     - Counters for processed / failed / retried, LLM latency and token use.
     - A tiny `/healthz` + `/metrics` endpoint per worker.
     - A dashboard page listing failed items.
5. **The AI boundary trusts too much.**
   - **How it works today.** Other users' text sits in the system prompt, and AI output is free text that is
     stored as "facts" about a person.
   - **Next steps.**
     - Put untrusted text in the user role as clearly delimited data.
     - Require the model to quote a verbatim evidence span and check it in code.
     - Cap the number of records per message.
     - Show facts as "unverified" in the UI.
6. **Least privilege is missing everywhere.**
   - **How it works today.** Every component is the Postgres superuser, every worker gets every secret, and the
     dashboard has no login.
   - **Next steps.**
     - One DB role per service; the dashboard gets read-only `SELECT`.
     - Per-service env files or Docker secrets.
     - Real auth: an authenticating reverse proxy, or Discord OAuth restricted to admins.
7. **Configuration is read ad hoc in each service.**
   - **How it works today.** Each service reads env vars in its own way, with a hard-coded S3 region `yo-mama`.
   - **Next step.** One typed config struct per service, validated at startup and printed (redacted).

**Changes that were too large or risky to do safely here** (roadmap, not PRs):
- the job-queue redesign (2)
- real authentication (6)
- adopting a migration tool (1)
- channel allowlist / consent / retention / erasure (privacy)
- single-flight resumable historical scan (3)
- per-service DB roles (6)

---

## 6. Prioritised roadmap

**Week 1: make it safe to run**
1. Merge the build fixes: workers#3, dash#9 (then workers#4 for the local stack).
2. Merge the security patches: workers#2, unidb#6, dash#2, dash#5.
3. Decide on dashboard access. Until real auth exists, set `DASH_BASIC_AUTH_*` and keep the dashboard on
   localhost/VPN.
4. Set a strong `POSTGRES_PASSWORD` and rustfs keys; remove the defaults (`${VAR:?}`).
5. If the role `krauzekrap` (S-1) exists anywhere, drop it.

**Weeks 2–3: stop data loss and stalls**

6. Merge workers#5/#6 and the LLM-workers PRs.
7. Merge unidb#7 → #9, and run `migrations/20261104_schema_integrity.sql` against a **copy of production first**,
   then production with a backup. **Merge these together**: unidb#9 adds CHECK constraints that require the
   workers' clamping.
8. Regenerate `.sqlx/` once all query-changing PRs are in (`cargo sqlx prepare --workspace`).
9. Merge the dash stack #3 → #4 → #5 → #6, then #7 → #8.

**Month 1: make it maintainable**

10. Adopt `sqlx migrate`, tag unidb releases, and pin every consumer to a tag.
11. Add CI to sentience-workers: fmt, clippy, tests with a pgvector service, `cargo sqlx prepare --check`,
    `cargo deny`, `cargo audit`, the e2e test. Pin Actions by SHA in all repos.
12. Least-privilege DB roles; TLS `verify-full` to Postgres.
13. Structured logging + metrics + honest healthchecks.

**Quarter: product and design decisions**

14. Job-queue redesign with claim/lease and backoff (enables more than one worker replica).
15. LLM prompt-injection hardening and verified evidence spans.
16. Privacy: handle `message_delete`, a channel allowlist, retention, per-user erasure, consent/notice. Review
    Discord's developer policy and GDPR obligations before running this on a real community.
17. Add licenses to sentience-workers and Sentience-dash, and a `LICENSE` file to unidb-schema.

---

## 7. Skipped or blocked by the hard rules, and assumptions

**Skipped because of the rules**
- **No merges, no pushes to existing branches** (rules 1–2). The session's pre-created branch
  `claude/brave-feynman-grnnu9` (identical to `main` in all three repos) was **not** used. All work is on new
  `claude/<topic>` branches.
- **No issues, comments, labels or releases** (rule 3). Findings that need a maintainer decision are in this
  report instead.
- **No production contact** (rule 4).
  - Nothing ran against the real Discord API, LLM provider or Supabase.
  - Prompt-injection impact (M-1) was assessed from code; no live LLM experiment was run.
  - The ingester's `discord` profile was only checked with `docker compose config`; it needs a real token.
  - The unidb migration was tested from the `a7a7c55` baseline only. The drizzle enum order hints that the live
    DB was hand-altered, so **run it on a production copy first**.
- **No secrets used** (rule 5). S-1 is reported only.
- **CI workflows** (rule 6). None of the repos had workflows before, so nothing blocked PRs. The workflows
  added in unidb#8 and dash#3/#4 only build, lint and test, with `contents: read` and no secrets. They will run
  on these PRs.

**Environment limits that affected the work**
- **Docker Hub returned HTTP 429.** Base images were pulled through `mirror.gcr.io` and retagged; digests were
  cross-checked against Docker Hub.
- **A TLS-intercepting proxy.** Its CA was passed to builds as an *optional* BuildKit secret, never baked into
  images.
- **Not needed upstream.** Neither of these workarounds is required outside this sandbox.
- **Disk.** The sandbox disk filled several times with parallel Rust builds. One stale scratch directory
  (`/tmp/cargo-install*`, a failed tool install) was deleted by a subagent before rules confining agents to their
  own directories were issued. The shared build cache was deleted with the owner's explicit approval.
- **Lighthouse performance scores are not reported.** The host load average was 15–19, which made them
  meaningless. Only byte sizes, a11y/SEO and query timings are reported.
- **Left behind.** A stopped test container `wsC-pg` remains. Removing it was denied by the permission system
  for the subagent, and was left for the owner.

**Assumptions**
- `main` of each repo is the deployed version.
- The dashboard is meant for server admins, not the public.
- One Discord guild per deployment (`GUILD_ID` is singular).
- The neutral score of 0.5 in the fixed scoring prompt (unidb#9) is a placeholder for a product decision.
- Body length limits and prompt bounds chosen in the LLM-workers PR are defaults, all configurable.
