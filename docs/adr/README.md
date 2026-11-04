# Architecture Decision Records

Decisions that shape Sentience / unidb. ADRs 0001–0002 were **reconstructed from the code on `main` (2026-11-04)**: they
document decisions that were already made implicitly, so future changes can be argued against an explicit baseline.

| # | Title | Status | Date |
|---|---|---|---|
| [0001](0001-shared-database-integration.md) | Integrate services through the shared unidb Postgres database | Accepted (reconstructed) | 2026-11-04 |
| [0002](0002-polling-attempts-table-work-queue.md) | LLM workers poll `messages` and record completion in per-worker attempts tables | Accepted (reconstructed) | 2026-11-04 |

## Process

1. Copy the structure of an existing ADR into `NNNN-short-title.md` (next free number, kebab-case).
2. Sections: Status, Context, Decision, Consequences (positive / negative), Alternatives considered.
3. Status is one of `Proposed`, `Accepted`, `Superseded by NNNN`, `Deprecated`. Never rewrite an accepted ADR's decision;
   supersede it with a new one and link both ways.
4. Add a row to the table above in the same PR. Update [`../SPEC.md`](../SPEC.md) if the decision changes behaviour.
