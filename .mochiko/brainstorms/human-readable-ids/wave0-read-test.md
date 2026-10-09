# Wave 0 — the two-arm read test (`human-readable-ids` D8)

**Status:** run 2026-10-08 as a cold-seat test (the user waived their own marking) — **FAIL on
clause 1**; the build stopped and returned to the user.

D8 (as changed at review, S9): before anything is built, the user reads two arms of real citations
from this repo — arm 1, 20 random bare citations; arm 2, 20 other random citations with hand-coined
slugs — and marks each: knew it without a lookup · looked it up anyway · misled. Pass: arm 2 at least
15 "knew" and at least 5 more than arm 1, none "misled". A fail stops the build before anything ships
and returns to the user.

## Method

- **Sampler:** a read-only script in the lead's session scratchpad (`sample_citations.py`, seed
  `20261008`), not persisted. It collects citations from the live layer — `CLAUDE.md`,
  `DECISIONS.md`, `BACKLOG.md`, `ROADMAP.md`, every session record except this one — plus the eval
  fixture projects for the product family, and resolves each ID's true title: `GI` from the ledger's
  `### GI-NNN — <title>` headings and `governance-intent.md`'s `**GI-NNN — <name>:**` lines; session
  `D` from `### D<n> — <title>` card headings (records without that card layout are out of the pool);
  `FEAT` from the fixture entry files' `# FEAT-NNN — <name>` titles.
- **Pool:** 16 GI IDs (every GI ID with a resolvable title that is cited), 85 cross-session D cites,
  56 FEAT cites. Drawn: 8 GI + 9 D + 3 FEAT per arm, no ID in both arms, order shuffled.
- **What the reader sees:** the ID alone — bare in arm 1, joined in arm 2 — with the session qualifier
  for `D` (D11) and the fixture project for `FEAT`. No surrounding line: a citation's context varies
  at random in how much it gives away, and the test measures the slug.
- **Arm 2's slugs:** coined by the lead under D13 (topic, never rule words) and D19 (three lowercase
  words; existing file-name slugs kept — the three `FEAT` slugs are the fixture files' own). Disclosed:
  the lead, not a separate seat, coined them; the build's slug map is drafted by a producing seat and
  graded (D14).
- **Marking:** per arm, the user marks each ID `k` (knew what it refers to without a lookup) or `l`
  (would look it up). After both arms, the true titles of every `k` are shown and the user marks any
  that was wrong — `m` (misled).
- **Known limit (D8's accepted risk):** the reader wrote most of these IDs; the bare arm is the
  baseline that narrows, but does not remove, that familiarity.

## Arm 1 — bare

| # | ID as cited | Where (one occurrence) |
|---|-------------|------------------------|
| 1 | `command-content-schema` D11 | `command-plan-only-eval` record |
| 2 | GI-017 | `setup-product-agnostic` record |
| 3 | GI-009 | `BACKLOG.md` |
| 4 | FEAT-009 (fixture project `s3-groom-at-cap`) | its `specs/index.md` |
| 5 | GI-006 | `cli-schema-delivery` record |
| 6 | `model-tiered-seats` D1 | `orchestrator-model-selection` record |
| 7 | `model-tiered-seats` D5 | `DECISIONS.md` |
| 8 | GI-016 | `setup-product-agnostic` record |
| 9 | `lead-owned-process-flexibility` D4 | `validator-worktree-isolation` record |
| 10 | GI-019 | `DECISIONS.md` |
| 11 | `delta-files-vs-direct-baseline-edits` D4 | `DECISIONS.md` |
| 12 | `teammate-message-races` D1 | `producer-plan-enforcement` record |
| 13 | FEAT-006 (fixture project `s3-groom-at-cap`) | its `specs/fuel-dock/spec.md` |
| 14 | GI-012 | `skill-content-schema` record |
| 15 | `cli-schema-delivery` D1 | `ROADMAP.md` |
| 16 | FEAT-009 (fixture project `p4-groom-map-and-remit`) | its `specs/labour-cost/spec.md` |
| 17 | `setup-product-agnostic` D3 | `DECISIONS.md` |
| 18 | `hook-enforcement-field-review` D1 | `ROADMAP.md` |
| 19 | GI-020 | `DECISIONS.md` |
| 20 | GI-022 | `CLAUDE.md` |

## Arm 2 — joined

| # | ID as it would be cited | Where (one occurrence) |
|---|-------------------------|------------------------|
| 1 | FEAT-004-arrivals-departures (fixture project `s2-growth-mint-lane`) | its `lane-tide-tables/sufficiency-report.md` |
| 2 | FEAT-001-family-accounts (fixture project `tally`, g3) | its `specs/lunch-orders/derivation.md` |
| 3 | `cli-schema-delivery` D11-widened-kernel-admission | `CLAUDE.md` |
| 4 | `standing-seat-lifecycle` D1-checkpoint-seat-recycling | `team-lead-strategic-compaction` record |
| 5 | `feature-map-layer` D1-feature-map-altitude | `feature-map-granularity-and-reparenting` record |
| 6 | `setup-product-agnostic` D1-setup-fact-profile | `DECISIONS.md` |
| 7 | GI-005-record-layer-integrity | `DECISIONS.md` |
| 8 | GI-004-primitive-audit-ratchet | `author-grader-consolidation` record |
| 9 | GI-002-project-type-shelves | `cli-schema-delivery` record |
| 10 | `command-content-schema` D2-rules-prose-split | `command-schema-ontology` record |
| 11 | FEAT-004-kitchen-counts (fixture project `tally`, g3) | its `specs/lunch-orders/derivation.md` |
| 12 | `pm-role-and-feature-derivation` D2-capabilities-work-rows | `multi-feature-plan-implement` record |
| 13 | GI-015-live-token-exposure | `DECISIONS.md` |
| 14 | `feature-map-layer` D21-deferred-success-criteria | `pm-requirements-stacking` record |
| 15 | GI-001-project-fact-profile | `ops-observability-hardening` record |
| 16 | GI-010-changelog-elective-module | `DECISIONS.md` |
| 17 | GI-003-repo-secret-hygiene | `DECISIONS.md` |
| 18 | GI-018-architecture-doc-lag | `product-architecture-schema` record |
| 19 | `pm-role-and-feature-derivation` D7-capability-batch-pipeline | `multi-feature-plan-implement` record |
| 20 | `hook-enforced-artifact-schema` D8-transport-coverage-probe | `BACKLOG.md` |

## Marks

**User marking waived** 2026-10-08 ("i am okay for you to implement without testing involving me");
the user chose the cold-seat replacement ("a"): a fresh reader seat guesses from the IDs alone, a
second fresh seat scores the guesses against the true titles.

- **Reader:** `cold-reader` (general-purpose, `opus`), told to use no tool; attested `tools_used: none`.
  **Contamination, disclosed by the seat:** the project `CLAUDE.md` loads into every seat at spawn and
  names 8 of the bare IDs (GI-006, GI-009, GI-012, GI-017, GI-019, GI-020, GI-022, `cli-schema-delivery`
  D11); the seat marked every bare ID from the ID text alone and called them unknown. All 40 IDs went
  in one shuffled list (seed 7), each arm's 20 interleaved.
- **Mark mapping, fixed in the reader's brief before any answer:** "sure" = would act without a lookup
  (knew, if right; misled, if wrong); "guess" or "unknown" = would look it up.
- **Scorer:** `read-test-scorer` (general-purpose, `opus`), fresh, rules fixed in its brief before it
  saw the answers; the lead coined the slugs and did not score.

**Result — FAIL on clause 1.**

| | knew | misled | looked up | of which: guess right / wrong / inverted |
|---|---|---|---|---|
| Arm 1 (bare) | 0 | 0 | 20 | 0 / 0 / 0 (9 named only the session; 11 unknown) |
| Arm 2 (joined) | 7 | 0 | 13 | 12 / 0 / 1 |

- Clause 1 (arm 2 knew ≥ 15): **7 — fails.** Clause 2 (gain ≥ 5): 7 — passes. Clause 3 (0 misled):
  passes (sensitivity: scoring item 20 as misled would fail clause 3 too).
- The slug carried the right topic on 19 of 20 joined IDs (7 sure + 12 right guesses) against 0 of 20
  bare; the reader withheld "sure" wherever the ruling's verdict was unclear.
- **Hazard — item 38:** `` `setup-product-agnostic` D1-setup-fact-profile `` read as "setup captures a
  project fact profile"; the ruling is that the fact profile *leaves* setup — a topic slug on a removal
  read inverted (marked guess, so not misled). Item 29 carries the same risk.
- Bare `FEAT-009` maps to two titles in two fixture projects (items 5 and 37).

The answer key and the scorer's 40-row table are in the session's scratchpad (`reader_items.json`, the
scorer's reply); the key rows above are the decisive ones. Per D8 the build stops here and returns to
the user.
