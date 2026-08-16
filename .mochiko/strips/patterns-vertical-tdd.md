# Strip notes — `skills/patterns-vertical-tdd/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.22.0–v0.23.0] entries — workflow-token-reduction waves 1–2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6a + D4/wave-2 rulings).

## [v0.74.0] `tasks-template.md` read-pointer → `tasks` schema (two-arm) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template tasks`, or Read `plugins/mochiko/schemas/tasks.yaml` raw (D8-first-class). One site: the Overview cycle-card-shape pointer.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `in the cycle-card shape ([\`tasks-template.md\`](../../templates/tasks-template.md) is the canonical skeleton):`
- **Kept deliberately:** the `**TEST:**`-grammar ownership and all descriptive text; the in-skill reference pointers to `TEST-GRAMMAR.md` / `SLICE-IDENTIFICATION.md` (untouched — not in-scope templates).
- **Consumers assessed:** n/a (single-writer skill).

## [v0.64.0] Guardrails cut — When-to-Use removed, principles/TEST-gate/checklist kept; slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 5,492 → 5,189 chars, −303, −6%; description 814 → 496 chars):**
  - **Removed whole:** `## When to Use` — the four-bullet list ("Structuring an accepted plan's stories into cycle cards (`tasks.md`)" · "Deciding Simple / Split / Merge story→cycle cases and recording the rationale" · "Classifying foundation vs feature cycles and their dependencies" · "Authoring a cycle's `**TEST:**` gate (this skill owns the grammar)"). Restates the description; the Simple/Split/Merge firing survives in `### Case column`, the TEST-gate-ownership in `### 3. Verified against reality`, foundation/feature in `### 2. Foundation + Parallel`.
  - Old description verbatim: "This skill MUST be invoked when structuring a feature's implementation into vertical-slice cycle cards — mapping user stories to cycles (Simple / Split / Merge cases), classifying foundation versus feature cycles, and authoring `tasks.md` as cycle cards (stories + rationale, dependencies, acceptance criteria, the closing `**TEST:**` real-infrastructure gate, cycle-level brownfield exposure). SHOULD also invoke when the work involves \"structure implementation\", \"define cycles\", \"cycle cards\", \"vertical slice\", \"story→cycle mapping\", \"testable increment\", or \"implementation cycles\". Owns the `**TEST:**` grammar. Structures the cycles at design time — it does NOT write task lists: decomposing a card into concrete tasks with file paths happens at build time, owned by mochiko:executing-tdd-cycle, downstream."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** the letter/spirit epigraph (v0.27.0-KEPT under R4b), `## Overview`, `## When NOT to Use` (incl. the feature-map boundary), `## Core Principles` (vertical-over-horizontal + Foundation+Parallel + Verified-against-reality — all v0.49.0/v0.27.0-KEPT; TEST-gate ownership + TEST-GRAMMAR.md pointer), `## Identifying Cycles` (SLICE-IDENTIFICATION.md pointer + quick heuristics + the Simple/Split/Merge `### Case column`), `## Brownfield exposure`, `## Quality Checklist`. The description keeps the MUST trigger, the cycle-card gist, the `**TEST:**` gate + grammar ownership, the top trigger phrases, and the design-time vs build-time boundary with `executing-tdd-cycle`.
- **Protected-content reconciliation (MANDATORY):** the v0.27.0 KEPT set (Markers table, Common Rationalizations, Quality Checklist, task-mapping shape, letter/spirit epigraph) and the v0.49.0/v0.58.0 supersession-kept elements (vertical-over-horizontal, foundation/feature test, TEST-gate ownership, Simple/Split/Merge cases, every implementation-level "vertical slice" naming) all survive intact. The removed `## When to Use` is NOT among any prior KEPT enumeration (a description-restatement, densified 6→4 at v0.27.0 but never named a survivor). Nothing protected silently dropped.
- **Consumers assessed:** `commands/plan.md`, `skills/authoring-feature-map/SKILL.md`, `skills/brownfield-integration/SKILL.md`, `skills/executing-tdd-cycle/SKILL.md` (consumes cards + owns build-time decomposition — boundary intact), `skills/testing-end-user/SKILL.md` (+ `references/TASK-PARSING.md`; TEST-grammar pointer intact), router `skills/mochiko/SKILL.md`, `templates/artifact-format.md`, `templates/tasks-template.md`, `references/TEST-GRAMMAR.md` — all reference the skill by name; none links a removed section anchor. The `**TEST:**` grammar ownership and the design-time/build-time boundary are intact.

## [v0.58.0] Graduation-slice vocabulary re-keyed to feature
- **Disposition:** superseded → "stories + feature rationale" in the Overview card-field list · the When-NOT-to-Use feature boundary line ("Deriving or scoping features — the feature is the pipeline unit, owned by `mochiko:authoring-feature-map`, upstream (its vocabulary table disambiguates feature vs cycle); a cycle is a within-one-feature increment") · "its rationale" in the Case-column paragraph
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md`, D4 — graduation slices retire, `authoring-slices` superseded by `authoring-feature-map`)
- **Content:** Overview "per card — stories + slice rationale, …" · When NOT to Use "**Grouping stories into graduation slices** — spec-level decomposition is `mochiko:authoring-slices`, upstream (that skill's vocabulary table disambiguates the two 'slices')" · Case paragraph "The story→cycle decision and its slice rationale live **on the card** (Stories line)"
- **Kept deliberately:** every use of "vertical slice"/"slice" naming the implementation-level cycle unit (title, Core Principles, SLICE-IDENTIFICATION.md pointer, quick heuristics) — that vocabulary is this skill's own and survives D4; only graduation-slice (spec-level) references left. TEST-grammar ownership and the build-time-decomposition boundary untouched.
- **Consumers assessed:** tasks-template (re-keyed same wave) · executing-tdd-cycle (consumes cards, wording unaffected — grep-verified no graduation-slice reference) · plan command (re-keyed same wave) · authoring-feature-map (wave 1, already points here as downstream cycle owner).

## [v0.49.0] Slimmed to cycle-card structuring — task-level content superseded
- **Disposition:** superseded → build-time decomposition in `executing-tdd-cycle` (step 2 of its execution sequence); `references/CYCLE-STRUCTURE.md` deleted
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D2.1)
- **Content:** the task-mapping.md canonical compact shape section (story→cycle + cycles tables + slicing notes) · the TDD Task Sequence section (red/green/refactor as pre-ordered task rows) · the task-level Markers table · Layered Testability · the Common Rationalizations table and Red Flags list (plan-time task-ordering framing) · checklist rows "each cycle has TDD structure (failing test first)" / "every task has a specific file path" · CYCLE-STRUCTURE.md whole (cycle anatomy, task-ID format, file-path conventions, worked task examples). Full text: git history at v0.48.0.
- **Kept deliberately:** vertical-over-horizontal · foundation/feature test · TEST-gate ownership (TEST-GRAMMAR.md untouched as grammar owner) · SLICE-IDENTIFICATION.md (one stale task-mapping paragraph re-keyed to on-card rationale) · Simple/Split/Merge cases (now card fields). Red/green/refactor discipline was double-encoded design+runtime; the runtime copy (`executing-tdd-cycle`) is now the sole carrier — deliberate, not a loss.
- **Consumers assessed:** router · tasks-template · executing-tdd-cycle (boundary flipped: builder now decomposes) · testing-end-user (grammar pointer intact) · brownfield-integration (marker-source wording re-keyed).

## [v0.27.0] Third-copy formats, TDD-sequence snippets, case subsections, and Common Mistakes stripped (body 368 → 204, −45%, in-band)
- **Disposition:** relocated/deduped → verified homes, each Read before landing: the Standard
  Cycle Format block (verbatim in `references/CYCLE-STRUCTURE.md` Cycle Anatomy; conforms-to
  `templates/tasks-template.md`, the declared single source — the in-file third copy was the
  drift risk its own intro warned about) · Task Numbering (CYCLE-STRUCTURE's Task ID Format,
  richer — adds the >4-tasks rule) · Common Mistakes, all six rows homed: tests-after +
  missing-TEST-task (cycle-format/template ordering), horizontal-disguised (in-file Core
  Principle 1 + SLICE-IDENTIFICATION Anti-Patterns), foundation-without-TDD (the identical
  in-file rationalization row), cycles-too-large (SLICE-IDENTIFICATION Size Calibration),
  missing-file-paths (CYCLE-STRUCTURE File Path Conventions + the in-file Quality Checklist row)
  · densified (form + in-file dedup): TDD Task Sequence 50 → 9 lines (four re-shown task-line
  snippets cut; behavioral glosses kept — red must FAIL and express acceptance criteria, green
  minimal across all layers, refactor no behavior change, TEST = real-infra gate checking
  **spec acceptance criteria**), Principle-2 tree + Principle-3 diagram → one-line principles
  pointing at the sections that carry them, Foundation-vs-Feature 24 → 6 lines (both
  identification questions + IP-XXX pointer kept), Simple/Split/Merge 30 → 8 (Case-column form,
  micro-examples one line each, split/merge calibration pointed at SLICE-IDENTIFICATION),
  When-to-Use 6 → 4, No-exceptions 5 bullets → 1 line (refusal clause kept), duplicate
  TEST-GRAMMAR pointer collapsed to one
- **Tier failed:** 1 (every stripped block had ≥1 verified home, most in the skill's own tree) ·
  n/a for the form-only densifications
- **Content:** the format block, numbering list, six mistake subsections, four snippet blocks,
  two ASCII diagrams, three case subsections; nothing was written to `templates/` this wave —
  dedups run against pre-existing template content, so D4's destination ban is not engaged
  (R4a credit)
- **Consumers assessed:** wave-open enumeration — 13 citing files; none links a section anchor
  of this SKILL; `task-architect`'s "cycle structure lives in patterns-vertical-tdd" claim still
  holds via CYCLE-STRUCTURE.md (skill tree). Shared-home audit: `references/TEST-GRAMMAR.md`
  (4 external consumers) is a clean single source — ownership header, CYCLE-STRUCTURE holds
  pointer-only, no dead pointers, untouched

## [v0.27.0] KEPT: Markers table (ownership ruling), rationalizations, checklist, mapping shape, epigraph
- **Tier-2 evidence:** the Markers table was contested (identical table in `tasks-template.md` —
  a legal R4a dedup) and **kept on an ownership ruling**: `task-architect` and
  `brownfield-integration` both name this skill as where the marker vocabulary is *defined*, and
  the skill tree's own CYCLE-STRUCTURE covers only `[EXTEND]`/`[MODIFY]` — stripping would leave
  the claimed owner defining 2 of 4 markers, inverting ownership toward a fill-target. Common
  Rationalizations (8 rows, excuse + reality each) kept as the discipline core — distinct from
  `executing-tdd-cycle`'s runtime red flags (checked: no cross-skill duplicate). Quality
  Checklist kept (aggregation function). The task-mapping.md shape block untouched — it IS the
  v0.23.0 canonical home (below). The letter/spirit epigraph kept as-is under R4b: its
  consequence anchor is the Overview's discipline paragraph eight lines down. Session ruling:
  wave-3 batch-1 ratified 2026-07-25.

## [v0.23.0] task-mapping.md gains its canonical compact shape (wave 2)
- **Disposition:** addition, not a strip — recorded here because it materially changes the authored artifact: `task-mapping.md` had **no prescribed structure anywhere** (the commands called it "the freehand story→cycle mapping"), and kinako's freehand form ran to 45.9k B for one slice
- **Content:** SKILL.md's Mapping-Stories-to-Cycles section now carries the canonical shape — the Story→Cycle table (Case column for Simple/Split/Merge with one-line why) + a per-cycle table (≤ 2-line rationale cells) + an optional Slicing-notes section (≤ 3 lines each, omit when empty), under the `artifact-format.md` envelope. `commands/tasks.md`'s two "freehand" mentions updated to "compact".
- **Consumers assessed:** tasks producer (task-architect — authors it) + review-task-artifacts (mapping checklist grades coverage/slice-quality, shape-agnostic; density note added this wave) + implement's producer (reads it as a design input — a table serves that read better than prose).

## [v0.22.0] TEST: grammar split out of CYCLE-STRUCTURE.md
- **Disposition:** relocated → `references/TEST-GRAMMAR.md` (new; ownership stays with this skill) — the runtime verifier (`testing-end-user`) now loads only the grammar (~5k B) instead of the full CYCLE-STRUCTURE.md (18.6k B)
- **Tier failed:** pure waste (D6a): the verifier's mandated cross-skill read pulled 13k B of cycle-structuring content it never uses
- **Content:** the `## Verification Task Requirements` block (what verification MUST include · Unified TEST: format + runtime-classification table · field definitions · action modifiers · assert patterns · four worked examples · bad-verification examples · why-this-matters · legacy format support) moved verbatim; CYCLE-STRUCTURE.md keeps a pointer section. Retargeted refs: testing-end-user SKILL.md (5) + TASK-PARSING.md (3), patterns-vertical-tdd SKILL.md (2), tasks-template.md (1).
