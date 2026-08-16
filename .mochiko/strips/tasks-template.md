# Strip notes — `templates/tasks-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.76.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/tasks.yaml + mochiko-cli template tasks
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by construction,
     human-legible. This file is CYCLE CARDS, not a task list: the builder decomposes each
     card into concrete tasks at build time, with the code in view (the decomposition is
     disclosed in the cycle report, never pre-written here). Cite spec/plan content by ID
     (US-#, FR-#, SC-#, C-#) — never re-quote it. Register: `full` per artifact-format.md
     rule 11; TEST-gate commands, file paths, and identifiers are never-compress items. -->

# Implementation Cycles: [FEAT-XXX — FEATURE NAME]

> Generated from the spec folder and the feature's produced design artifacts: spec.md, features/FEAT-XXX/plan.md, and whichever of requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/ the approved proposal included
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | [N] ([N] foundation + [N] feature) |
| Stories covered | [US-# list — every P1/P2 story on at least one card] |

## Cycle Format

Each card is one vertical slice: an observable, end-to-end behavior. The builder implements
the card test-first (red/green/refactor per `mochiko:executing-tdd-cycle`, decomposition at
build time) and the closing `**TEST:**` gate verifies it against real infrastructure —
see [`TEST-GRAMMAR.md`](../skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md) for the
canonical Setup/Action/Assert/Capture grammar. **The card's checkbox is the progress
surface**, flipped when the cycle's gate passes. Foundation cycles run sequentially, first;
feature cycles are parallel-eligible `[P]` unless dependent on another feature cycle.

---

<!--
  The two cards below are SAMPLES for illustration — replace them with actual cycles from
  the feature's spec + plan artifacts. DO NOT keep them in the generated tasks.md.
-->

## Foundation Cycles

> Sequential; establish what every feature cycle depends on. All complete before feature cycles begin.

### - [ ] Cycle 1: Core entity and basic CRUD

- **Stories:** US-1 — [why these stories share this cycle / what it establishes, ≤ 2 lines]
- **Depends on:** —
- **Case:** Simple <!-- Simple | Split — why, one line | Merge — why, one line -->
- **Acceptance criteria:** [spec/plan IDs this cycle must satisfy — cite, never quote]
- **Brownfield exposure:** none <!-- none | extends `path` | modifies `path` — cycle-level surfaces only -->

**TEST:** CRUD operations work via API
- **Action**: `curl -X POST localhost:3000/api/[entity] -d '{"name":"Test"}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "[entity]_id"
- **Capture**: console

---

## Feature Cycles

> Parallel-eligible once foundation is complete.

### - [ ] Cycle 2: [Feature title] `[P]`

- **Stories:** US-2 — [rationale ≤ 2 lines]
- **Depends on:** C1
- **Case:** [Simple | Split | Merge]
- **Acceptance criteria:** [IDs]
- **Brownfield exposure:** extends `src/models/[entity].py`

**TEST:** [behavior] works end to end via API
- **Setup**: Seed prerequisite [entity] data
- **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"value"}'`
- **Assert**: Response status: 200
- **Assert**: Console contains "[expected field]"
- **Capture**: console
````
- **Kept deliberately:** Every line of guidance preserved — lifted into `plugins/mochiko/schemas/tasks.yaml` (skeleton / contract / overview / register / density) and rendered by `mochiko-cli template tasks`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). Nothing dropped.
- **Consumers assessed:** `commands/plan.md` (re-pointed by P4) · `commands/feature.md` (re-pointed by P4) · `skills/mochiko/SKILL.md` router row (re-described by P5) · `skills/patterns-vertical-tdd/SKILL.md` (re-pointed by P5) · `skills/review-plan-artifacts/SKILL.md` (D7 re-key — tasks cycle-card criteria cite the `--check` view, re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.67.0] Fixed design-input enumeration re-keyed to the proposal-produced set
- **Disposition:** superseded → the re-keyed provenance line: "Generated from the spec folder and the feature's produced design artifacts: spec.md, features/FEAT-XXX/plan.md, and whichever of requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/ **the approved proposal included**"
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/plan-structure-yagni/record.md` D1/D2 — artifacts are the approved proposal's, not a fixed set; combined wave `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` D5)
- **Content:** "> Generated from the spec folder: spec.md, features/FEAT-XXX/plan.md, requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/"
- **Kept deliberately:** the cycle-card shape, the `**TEST:**` grammar pointer, the sample cards, the Overview/Cycle-Format structure — untouched.
- **Consumers assessed:** patterns-vertical-tdd (fills it) · executing-tdd-cycle (reads cards) · review-plan-artifacts (grades it) · plan/plan-template (fixed-set re-key, same wave).

## [v0.58.0] Slice references re-keyed to FEAT
- **Disposition:** superseded → title "[FEAT-XXX — FEATURE NAME]" · provenance line "Generated from the spec folder: spec.md, features/FEAT-XXX/plan.md, …" · sample Stories line "[why these stories share this cycle / what it establishes, ≤ 2 lines]"
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md`, D4/D17/D18 — feature is the pipeline unit; per-feature artifacts under `features/FEAT-XXX/`)
- **Content:** title "# Implementation Cycles: [FEATURE NAME]" · "> Generated from `.mochiko/specs/<feature>/`: spec.md, plan.md, requirements.md, …" · sample Stories bracket "[why these graduate together / what this cycle establishes, ≤ 2 lines]"
- **Kept deliberately:** "Each card is one vertical slice" in Cycle Format — implementation-level cycle vocabulary owned by `patterns-vertical-tdd`, not graduation-slice language; the whole card shape, TEST-grammar pointer, and foundation/feature structure untouched.
- **Consumers assessed:** patterns-vertical-tdd (fills it; re-keyed same wave) · executing-tdd-cycle (reads cards — unaffected) · review-plan-artifacts (grades it) · plan/implement commands (re-keyed same wave).

## [v0.49.0] Rewritten task list → cycle cards
- **Disposition:** superseded → the cycle-card form in the same file (per card: heading checkbox · Stories+rationale · type · Depends on · Case · acceptance criteria by ID · brownfield exposure line · `**TEST:**` gate block)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D3)
- **Content:** the `TN.X` task-line skeleton (per-task checkbox/ID/description/file path), the Cycle Format TDD ladder (TN.1 failing test … TN.X TEST), the 4-marker table (`[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` at task level), the Checkpoint line, and the Story→Cycle Mapping table with its derived-echo-of-`task-mapping.md` note. Full text: git history at v0.48.0.
- **Kept deliberately:** `[P]` at cycle level · the `**TEST:**` gate (grammar unchanged, TEST-GRAMMAR.md still owner) · the never-compress rule for TEST commands/paths · foundation-sequential/feature-parallel structure. The Stories+Case card lines absorb `task-mapping.md`'s content — that artifact had no template file; its mapping+rationale role ends here.
- **Consumers assessed:** patterns-vertical-tdd (fills it) · executing-tdd-cycle (reads cards, decomposes at build time) · testing-end-user (gate parsing re-keyed) · review-plan-artifacts (grades it) · plan/implement commands.

## [v0.23.0] Execution Strategy + Notes sections stripped from the tasks.md skeleton
- **Disposition:** deleted (doctrine restatement into the artifact — R2 rule 7)
- **Tier failed:** 1 (altitude): "Execution Strategy" (MVP delivery / incremental delivery / parallel team strategy) and "Notes" (TDD discipline, vertical slices, foundation-first, parallel features, commit strategy, checkpoint validation) restate what `patterns-vertical-tdd`, `executing-tdd-cycle`, and the implement command single-source — and were regenerated verbatim into every authored tasks.md (kinako s1/tasks.md 54k B)
- **Content:** the two sections verbatim as listed above; identical every feature, zero feature-specific substance.
- **Consumers assessed:** implement's producer/verifier read cycles + task lines, never these sections (epic F-c consumption evidence); review-task-artifacts' checks are cycle/task-level — no checklist referenced either section (grep-verified).

## [v0.23.0] Cycle-dependency ASCII diagram stripped; Story→Cycle table kept as the ID index
- **Disposition:** diagram deleted; the Story→Cycle derived-echo table **kept** and designated the artifact's **ID index** per `templates/artifact-format.md` (the derived-echo annotation stands — task-mapping.md remains the source of truth)
- **Tier failed:** 1 (restatement): every dependency in the diagram already lives on its cycle's `> Dependencies:` header line; the diagram was a hand-drawn second copy that could drift
- **Content:** the `### Cycle Dependencies` fenced ASCII graph; a pointer line ("dependencies live on each cycle's header — no separate diagram") lands in the kept section.
- **Consumers assessed:** implement's lead sequences cycles from the `> Dependencies:` headers; review-task-artifacts' dependency checks read per-cycle headers (PHASE-CHECKLISTS grep-verified — no diagram reference).

## [v0.23.0] Sample cycles trimmed 5 → 2 (one foundation, one feature)
- **Disposition:** deleted (template-load density); the fill-guidance comment now points at `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md` for more worked examples (auth, filtering, brownfield markers)
- **Tier failed:** 1 (duplication): CYCLE-STRUCTURE.md already carries six worked cycle examples — three extra samples in the template taught nothing the producer's own reference doesn't
- **Content:** sample cycles 2 (auth foundation), 4, 5 (feature variants) — all present in equivalent form in CYCLE-STRUCTURE.md.
- **Consumers assessed:** tasks producer (loads both template and CYCLE-STRUCTURE per its skill); samples are replaced on fill by instruction, so downstream readers never saw them.
