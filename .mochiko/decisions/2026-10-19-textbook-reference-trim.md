# Textbook reference trim — six skill references cut to the project's own conventions

**Date:** 2026-10-19
**Status:** ruled (user) 2026-10-19, "lets cleanup 1 and 3"; built on branch
`trim-textbook-references`, ships at plugin v0.122.0. Item 3 (delete `analysis-iterative/SYNTHESIS.md`
as an orphan) withdrawn before any edit: the file is live, the target of the schema rule pointer at
`migrations/0058-thinking-engine.yaml:211`.
**Driver:** the user's question which skills to trim on the ground that the content need not be
spelled out; the lead's ranked answer put the textbook reference files first.

## Context

Several `references/` files under `plugins/mochiko/skills/` restate general knowledge the model
already holds: RFC 2119 keyword meanings, HTTP status semantics, REST pagination and OpenAPI
boilerplate, relationship cardinality, generic technology trade-off tables, and TDD motivation
prose. Each Read of such a file costs context and teaches nothing a run needs. Earlier compression
passes (v0.27.0, v0.64.0) relocated several `SKILL.md` tables *into* these files, so a wholesale
delete would have orphaned relocated rules.

## Decision

Cut each file to the conventions mochiko itself fixes, keep every relocation home and every pointer
target, and delete the one file whose rows were all homed elsewhere:

| File | Words | Result |
|------|-------|--------|
| `authoring-requirements/references/RFC-2119-KEYWORDS.md` | 821 → 185 | slimmed; the schema pointer `authoring-requirements.rfc2119-pointer` still resolves |
| `authoring-constitution/references/RFC-2119-KEYWORDS.md` | 936 → 277 | slimmed to the keyword-to-enforcement mapping and the choice tests |
| `patterns-api-contracts/references/ERROR-PATTERNS.md` | 1,287 → 413 | slimmed; status table and naming rules kept (v0.27.0 relocation homes) |
| `patterns-api-contracts/references/PAGINATION-PATTERNS.md` | 1,259 → 327 | slimmed; the v0.64.0 quick-reference rule kept at the head |
| `patterns-entity-modeling/references/RELATIONSHIP-PATTERNS.md` | 918 → 474 | slimmed to the documentation formats and diagram notation |
| `patterns-technical-decisions/references/EVALUATION-MATRIX.md` | 737 → 490 | the illustrative technology-category tables cut; both comparison shapes and alignment scoring kept |
| `executing-tdd-cycle/references/TDD-ANTI-RATIONALIZATION.md` | 493 → deleted | every row already homed in the `SKILL.md` STOP list, its Common Mistakes, and `brownfield-integration` |

Total 6,451 → 2,166 words. Two `SKILL.md` bodies change only in their pointers: `executing-tdd-cycle`
loses its two lines to the deleted file; `patterns-technical-decisions` § Phase 1 drops "technology
category comparisons" from its pointer. No migration: no schema rule loses its pointer target.

## Rationale

The cut content was Tier 1 — altitude: restated doctrine with no mochiko-specific behavior. What
stays is what a contract or record must do the same way every time (envelopes, naming, formats,
enforcement mapping), which the model cannot infer.

## Alternatives considered

- Delete every file outright. Rejected: five are pointer targets and several are relocation homes of
  earlier strips; deletion would have needed a migration for the `rfc2119-pointer` rule and would have
  dropped relocated tables.
- Leave them. Rejected by the user's ruling.

## Records

Strip entries `[v0.122.0]` in `.mochiko/strips/` — `authoring-requirements.md`,
`authoring-constitution.md`, `patterns-api-contracts.md`, `patterns-entity-modeling.md`,
`patterns-technical-decisions.md`, `executing-tdd-cycle.md`.

## Audit

Gate grader: one plain fresh seat (`gate-grader`, opus) took all six units of the wave against the
rendered `mochiko:validation-primitive-edit` contract (binary 0.4.0 · grammar 2 · plugin 0.121.0;
11 floors read back). Pre-pass, run by the grader: `mochiko-cli migrate validate --report
--plugin-root plugins/mochiko` returned `0 rejecting · 118 advisory` and `pointer resolution: 89
checked` (log state `sha256:300e26a8…` · 94 documents · 1,266 rules); no advisory names an edited
file. Char budget, canonical snippet: `patterns-technical-decisions` body 5,554 → 5,511 (budget
5,783), description 469 unchanged; `executing-tdd-cycle` payload body 7,140 + render 12,998 =
20,138 (was 20,380; budget 20,063), so the standing [v0.109.0] ruled-HOLDS overage shrinks from
+317 to +75 with no growth and no new argument owed; description 501 unchanged. Units 3–5 have no
render (prose skills, absent from the log) and were graded as prose primitives: coherence and
preserved responsibilities. Every unit PASS in one round.

- audit: authoring-requirements (skill pair) · gate-grader · opus · 4 files · 1 rounds · 0 blocking
- audit: authoring-constitution (skill pair) · gate-grader · opus · 4 files · 1 rounds · 0 blocking
- audit: patterns-api-contracts (prose skill) · gate-grader · opus · 4 files · 1 rounds · 0 blocking
- audit: patterns-entity-modeling (prose skill) · gate-grader · opus · 3 files · 1 rounds · 0 blocking
- audit: patterns-technical-decisions (prose skill) · gate-grader · opus · 3 files · 1 rounds · 0 blocking
- audit: executing-tdd-cycle (skill pair) · gate-grader · opus · 4 files · 1 rounds · 0 blocking

Advisory notes (non-blocking; disposition: left for a later edit):

- `patterns-api-contracts`: `ERROR-PATTERNS.md` gives `details.fields` (`field`, `error`) as the
  validation-failure shape, while `PAGINATION-PATTERNS.md` gives `INVALID_FILTER` a
  `details.errors` list (`field`, `value`, `error`). Both shapes predate this edit; the trim makes
  both read as conventions, so the mismatch is now more visible.
- `patterns-entity-modeling`: the diagram example keeps `Task ──N:1──▶ Project`, but the in-entity
  table example no longer has its `project` row. Both examples are illustrative, and they already
  differed before this edit (`comments`, `assignees`).
- `.mochiko/memory/primitive-cost-budgets.md`: the two re-measured rows (5,511; 20,138, standing
  overage +75) are not yet recorded there. Record them at the release-gate sweep.
  Disposition: recorded at the release-gate sweep (commit `a630116`).

## Release gates

- `cargo test --all` green, `cargo fmt --all --check` clean; derived views equal the replay
  (`mochiko-cli views emit` diffed against `.mochiko/schema-views/`, no difference).
- Plugin contract suite (`python3 evals/contract/run.py`, full run against the rebuilt release
  binary): first run 2026-10-19 red, 7 host cases ok and every sandbox case failed before the plugin
  loaded — the `claude-mochiko` sandbox's workspace mount was empty (environment, not this edit; the
  fixture-plugin cases failed identically). After the user restored the mount, the rerun on
  2026-10-25 went green: 99 / 99 cases ok, exit 0. Gate 6 met.
