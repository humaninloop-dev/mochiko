# Strip notes — `templates/tasks-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.80.0] `schemas/tasks.yaml` — "one vertical slice" re-worded "one vertical increment" (both sites)

- **Disposition:** superseded → "Each card is one vertical increment", the replacement unit language the ruling fixes. Recorded here because this file carries the tasks artifact's whole lineage: `templates/tasks-template.md` was retired into `plugins/mochiko/schemas/tasks.yaml` at v0.76.0 (entry below), and the schema is a shipped primitive under the same edit ceremony (schema-based-template-guidance D8). `.mochiko/strips/tasks.md` is a different primitive — the retired `commands/tasks.md` — and correctly takes nothing from this wave.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`, "Slice vocabulary purged from shipped primitives", ruled 2026-08-19; `DECISIONS.md` row of the same date, which names `schemas/tasks.yaml` explicitly).
- **Content (superseded, verbatim — both sites, old → new):**
  - `:37`, the `Cycle Format` section `contract:` block — `Each card is one vertical slice: a coherent bundle of named test cases (expected behaviour,` → `Each card is one vertical increment: a coherent bundle of named test cases (expected behaviour,`
  - `:115`, the rendered `Cycle Format` prose — `Each card is one vertical slice: a coherent bundle of **named test cases** (expected behaviour,` → `Each card is one vertical increment: a coherent bundle of **named test cases** (expected behaviour,`
  - The two sites are the contract and its rendered echo and MUST agree; both moved together.
- **Kept deliberately:** the schema's three *gerund* uses — `Structure and slicing judgment come from mochiko:patterns-vertical-tdd` (`:12`), the same clause in the overview contract (`:24`), and the rendered `> Structure: \`mochiko:patterns-vertical-tdd\` (cycle-card shape, slicing judgment)` (`:104`). "Slicing judgment" names the design activity the skill owns, not a unit of work; the ruling purges the unit noun only, and a total word ban was considered and rejected. Everything else in the schema — skeleton, contracts, per-section `check` lines, register and density rules, sample cards — is untouched.
- **Measurements:** schema data files are not a budgeted class (the D7 ledger measures skill bodies, skill descriptions, and agent descriptions); no budget applies. The edit adds 4 chars per site and no lines; no `check` line, field name, or renderer-visible key changed, so `mochiko-cli template tasks` output shifts by the same two words and nothing else.
- **Consumers assessed:** `mochiko-cli template tasks` renders this schema (prose-only change, no key or structure touched — `cargo test` unaffected); `skills/patterns-vertical-tdd/SKILL.md` points at the schema as the canonical cycle-card skeleton and is re-worded to the same vocabulary in this wave (its own strip entry), so skill and schema stay in agreement; `skills/executing-tdd-cycle` and `skills/review-plan-artifacts` consume cycle-card structure and criteria, neither of which changed. The D8 raw-Read degraded path reads the same re-worded text — no divergence between the binary-rendered and raw-Read arms.

## [v0.76.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/tasks.yaml + mochiko-cli template tasks
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by construction,
     human-legible. This file is CYCLE CARDS, not a task list: each card is a coherent bundle
     of named test cases (the card's content), and the builder decomposes each card into
     concrete tasks at build time, with the code in view (the decomposition is disclosed in the
     cycle report, never pre-written here). Cite spec/plan content by ID (US-#, FR-#, SC-#, C-#)
     — never re-quote it; each test case cites the ID(s) it covers. Register: `full` per
     artifact-format.md rule 11; TEST-gate commands, file paths, and identifiers are
     never-compress items. -->

# Implementation Cycles: [FEAT-XXX — FEATURE NAME]

> Generated from the spec folder and the feature's produced design artifacts: spec.md, features/FEAT-XXX/plan.md, and whichever of requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/ the approved proposal included
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | [N] |
| Stories covered | [US-# list — every P1/P2 story on at least one card] |

## Cycle Format

Each card is one vertical slice: a coherent bundle of **named test cases** (expected behaviour,
in the `**TEST:**` grammar — see [`TEST-GRAMMAR.md`](../skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md)
for the canonical Setup/Action/Assert/Capture grammar) that demonstrate together. The builder
implements the card test-first (red/green/refactor per `mochiko:executing-tdd-cycle`,
decomposition at build time), and the cycle is done when its named cases show green against real
infrastructure. **The card's checkbox is the progress surface**, flipped when the bundle's cases
pass. Where the work opens a new end-to-end path (greenfield / new path), the **first cycle is a
walking skeleton**; growth on an already-standing path skips it. There is no foundation/feature
card type — `[P]` parallel eligibility derives from a card's dependencies, not from a type
column. Each named test case cites the spec/plan ID(s) it covers.

---

<!--
  The two cards below are SAMPLES for illustration — replace them with actual cycles from
  the feature's spec + plan artifacts. DO NOT keep them in the generated tasks.md.
-->

### - [ ] Cycle 1: Walking skeleton — [thinnest end-to-end path]

- **Stories:** US-1 — thinnest end-to-end path through all layers, one trivial case green; establishes the production-shaped stack [≤ 2 lines]
- **Depends on:** —
- **Case:** Simple <!-- Simple | Split — why, one line | Merge — why, one line -->
- **Brownfield exposure:** none <!-- none | extends `path` | modifies `path` — cycle-level surfaces only -->

**TEST:** [entity] round-trips through the full stack
- **Covers**: US-1 / SC-1
- **Action**: `curl -X POST localhost:3000/api/[entity] -d '{"name":"Test"}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "[entity]_id"
- **Capture**: console

---

### - [ ] Cycle 2: [Feature bundle title] `[P]`

- **Stories:** US-2 — [why these stories/cases form one demonstrable bundle, ≤ 2 lines]
- **Depends on:** C1
- **Case:** [Simple | Split — why | Merge — why]
- **Brownfield exposure:** extends `src/models/[entity].py`

**TEST:** [behavior] works end to end via API
- **Covers**: US-2 / SC-2 scenario 1
- **Setup**: Seed prerequisite [entity] data
- **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"value"}'`
- **Assert**: Response status: 200
- **Assert**: Console contains "[expected field]"
- **Capture**: console

**TEST:** [behavior] rejects [invalid case]
- **Covers**: US-2 / SC-2 scenario 2
- **Setup**: Seed prerequisite [entity] data
- **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"invalid"}'`
- **Assert**: Response status: 400
- **Capture**: console
````
- **Merge lineage:** the verbatim block above originally captured the **pre-v0.75.0** template (the version live on the `mochiko-cli` branch when this strip was authored); at the `mochiko-cli`←`main` merge it was updated to the **v0.75.0** template — the actual content deleted post-merge — because `main`'s vertical-TDD wave (D1–D4) had already re-keyed the template to the test-case-bundle grammar before deletion. `plugins/mochiko/schemas/tasks.yaml` was folded to that same v0.75.0 grammar at the merge; the [v0.75.0] entry below records the intermediate re-key.
- **Kept deliberately:** Every line of guidance preserved — lifted into `plugins/mochiko/schemas/tasks.yaml` (skeleton / contract / overview / register / density) and rendered by `mochiko-cli template tasks`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). Nothing dropped.
- **Consumers assessed:** `commands/plan.md` (re-pointed by P4) · `commands/feature.md` (re-pointed by P4) · `skills/mochiko/SKILL.md` router row (re-described by P5) · `skills/patterns-vertical-tdd/SKILL.md` (re-pointed by P5) · `skills/review-plan-artifacts/SKILL.md` (D7 re-key — tasks cycle-card criteria cite the `--check` view, re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.75.0] Foundation/feature grouping + type + standalone acceptance-criteria field superseded; skeleton-first sample, per-case citation

- **Disposition:** superseded → the test-case-bundle card shape in the same file: Overview `Cycles | [N]` (no foundation/feature split); a Cycle Format re-keyed to skeleton-first + `[P]`-from-dependencies + the bundle-as-card-content rule; a walking-skeleton sample card (Cycle 1) and a feature-bundle sample (Cycle 2) carrying named `**TEST:**` cases each with a `**Covers**:` citation line.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Vertical-TDD cycle anchor + QA test-case authorship (D1–D4)"; record `.mochiko/brainstorms/vertical-tdd-complexity-and-qa-role/record.md`, D1/D3 + the D2 acceptance-ID-relocation amendment).
- **Content:**
  - Overview table row "`| Cycles | [N] ([N] foundation + [N] feature) |`" → "`| Cycles | [N] |`".
  - Cycle Format sentence "Foundation cycles run sequentially, first; feature cycles are parallel-eligible `[P]` unless dependent on another feature cycle." → skeleton-first-where-a-new-path-opens + "There is no foundation/feature card type — `[P]` parallel eligibility derives from a card's dependencies, not from a type column." + "the cycle is done when its named cases show green against real infrastructure" + "Each named test case cites the spec/plan ID(s) it covers."
  - Section headers "`## Foundation Cycles`" (blockquote "Sequential; establish what every feature cycle depends on. All complete before feature cycles begin.") and "`## Feature Cycles`" (blockquote "Parallel-eligible once foundation is complete.") deleted — cards are no longer grouped by type.
  - Sample card heading "### - [ ] Cycle 1: Core entity and basic CRUD" → "### - [ ] Cycle 1: Walking skeleton — [thinnest end-to-end path]".
  - Standalone card field "`- **Acceptance criteria:** [spec/plan IDs this cycle must satisfy — cite, never quote]`" removed from every card → the per-case `- **Covers**: <IDs>` line inside each `**TEST:**` block carries the citation (D2: relocated, not dropped — each Assert graded against the scenario it cites).
  - Full prior text: git history at v0.74.x.
- **Kept deliberately:** the cycle-card shape and the `**TEST:**` grammar pointer to `TEST-GRAMMAR.md` (D4 — grammar unchanged), the checkbox-as-progress-surface rule, the Stories / Depends on / Case / Brownfield-exposure fields, the "SAMPLES — replace them" guard comment, the `full` register + never-compress note, the artifact-format.md envelope. The `**TEST:**` Setup/Action/Assert/Capture fields untouched — `**Covers**` is a card-level trace line, not a new executable verification field.
- **Consumers assessed:** `skills/patterns-vertical-tdd/SKILL.md` (fills it — re-keyed same wave) · `skills/executing-tdd-cycle/SKILL.md` + `references/TASK-PARSING.md` (reads cards — re-keyed same wave: type field dropped, `**TEST:**`-blocks + `Covers` extraction added) · `skills/testing-end-user/SKILL.md` (runs the `**TEST:**` cases — grammar unchanged, ignores `Covers`) · `review-plan-artifacts` (grades cards — the acceptance-ID cross-artifact clause survives via the per-case citation, P2's re-key) · `plan.md` (invokes the fill).

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
