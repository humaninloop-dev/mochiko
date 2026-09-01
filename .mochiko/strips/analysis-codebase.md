# Strip notes — `skills/analysis-codebase/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave (R6; design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, doctrine D1–D5 + batch R1–R7; per-hunk
proposals user-ratified 2026-07-25). Baseline 313 file / 309 body lines at v0.23.0 → 175 file /
171 body lines at v0.24.0 (138 body lines cut, 45%). Landed in two passes: the initial land
dropped the Manual-Detection deletion (caught as a phantom strip entry by the independent audit,
fixed same day — the audit trail is the wave's ROADMAP row). Consumers assessed for every entry below:
`commands/setup.md`, `agents/principal-architect.md`, router `skills/mochiko/SKILL.md`,
`skills/authoring-constitution/` (+ its `references/ESSENTIAL-FLOOR.md`),
`templates/artifact-format.md` — grep 2026-07-25: none reference any stripped section;
`detect-stack.sh` references (setup.md, router) untouched.

## [v0.103.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2C, small families)

Ruling for every entry below: skill-content-schema D3 (obligations move, procedure stays
prose) / D8/C4 (protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema
ruled · Skill-schema wave-2 family doors ruled — the small-families door); census:
`.mochiko/brainstorms/skill-content-schema/census-small-families.md` §A (AC) + §B (AC rows
1–13). Schema home: `plugins/mochiko/skills/analysis-codebase/schema.yaml`. Minted IDs
carry the `analysis-codebase.` prefix (omitted below). Map — census §B row → minted ID:
1 `deliverable-two-arm-binding` · 2 `artifact-envelope-slimmed-legible` ·
3 `essential-floor-canonical-definition` · 4 `intent-blind-waiver-blind` ·
5 `never-soften-never-waive` · 6 `interpretation-reserved-to-session` ·
7 `collision-inventory-out-of-scope` · 8 `only-report-what-is-found` ·
9 `determinism-boundary` · 10 `brownfield-quality-checklist` (set-rule, all 12 items) ·
11 `capability-signals-seed-feature-map` · 12 `when-not-routing` ·
13 `context-gathering-scope-note` (reference stub, pointer
`references/CONTEXT-GATHERING.md`). 13 rules: 12 body moves + 1 stub, no splits.
**Section distribution (review six-set reused by the door ruling, 1 empty marker per the
census fit table):** scope {3, 7, 12, 13} · inputs {9} · verdict {4, 5, 8} · output {1, 2,
10, 11} · reserved {6} · independence — the explicit empty marker.
**Floor count 3 (rows 3 · 4 · 5)** — row grain and census tally agree. No `conditions:`
block — brownfield setup is the invoking context, unconditional within the skill (the
census's own "arguably unconditional" read, lead-confirmed); the load-first block legally
omits the `when:` grammar sentence (wave-1 RCM-4 wave-wide ruling). Row 1's two arms ride
the rule text VERBATIM (GI-020 — the binary-optional first-class raw-Read path); row 3's
`pointer:` carries the cross-directory climb
`../authoring-constitution/references/ESSENTIAL-FLOOR.md` (census J2-7, checker-resolved).
Accounting (seat-measured snapshot; the closer re-measures at the gate): body 6,613 → 3,814
(obligations out + the load-first Rules block in) + schema 8,284 = **payload 12,098**
(census §F estimate ~11,100, ×1.83 vs est ×1.7 — inside the ±30% band); the delta over the
pre-conversion body is structural overhead — no content growth claimed. The old 8,137 body
budget is superseded by the conversion re-seed (ledger's third seeding path, no headroom —
the wave closer executes the ledger row). Description byte-untouched at 349.

## [v0.103.0] J2-9 ruled repair — the dangling "indicators below" pointer re-homed (one line, riding the wave by ruling)

- **Disposition:** superseded → "read the canonical definition, then assess against the
  indicators it carries" — the sentence's referent re-homed onto
  `authoring-constitution/references/ESSENTIAL-FLOOR.md`, inside
  `analysis-codebase.essential-floor-canonical-definition`'s rule text, where the moved
  sentence now lives. Verified before wording: ESSENTIAL-FLOOR.md's four per-category
  "MUST address" blocks carry the indicator content the sentence points at.
- **Tier failed:** n/a — supersession by ruling (the wave-2 family-door ruling's repair
  clause — "analysis-codebase's dangling 'indicators below' pointer takes a one-line
  ruled repair riding its wave, never a silent fix", the sentence living in
  `.mochiko/brainstorms/skill-content-schema/record.md`, Wave-2 family-door rulings
  section; ruling pointer: the `DECISIONS.md` 2026-09-01 family-doors row; census
  J2-9). This closes the deviation flagged in this file's [v0.63.0] entry ("the surviving
  sentence 'assess against it using the indicators below' now has no indicators below
  it"), shipped byte-faithful to the ruled benchmark variant at the time because the
  dangling form was not the shipper's to rewrite.
- **Content (superseded, verbatim):**

  ```
  Read the canonical definition, then assess against it using the
  indicators below.
  ```

- **Kept deliberately:** the whole surrounding assess-status contract, moved intact into
  the schema (entry below); only the two-word referent changed.
- **Consumers assessed:** `commands/setup.md` relies on the deliverable and the
  present/partial/absent statuses, not on this sentence; `authoring-constitution` owns
  the pointed-at canonical file, unchanged.

## [v0.103.0] Two-arm output binding, envelope, feature-map seeding — protection transfers (census §A rows 4–5 + keep-set)

- **Disposition:** superseded — protection transfers per D8/C4 onto
  `analysis-codebase.deliverable-two-arm-binding` (must, binding — both arms verbatim:
  `mochiko-cli template codebase-analysis` when the binary is available, otherwise Read
  `plugins/mochiko/schemas/codebase-analysis.yaml` raw; GI-020),
  `analysis-codebase.artifact-envelope-slimmed-legible` (must, binding, pointer
  `../../templates/artifact-format.md`), and
  `analysis-codebase.capability-signals-seed-feature-map` (must, binding, pointer
  `mochiko:authoring-feature-map`); the Setup-Brownfield quality checklist →
  `analysis-codebase.brownfield-quality-checklist` (must set-rule, all 12 boxes in the
  rule text — none lost), its body section leaving.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows;
  protecting lineage `DECISIONS.md` 2026-08-16 schema-based-template-guidance D1/D8 for
  the two-arm binding — this file's [v0.76.0] entry).
- **Content:** the Mode section's `**Output**:` paragraph and the
  `### Setup-Brownfield Quality Checklist` section, verbatim in git history
  (pre-v0.103.0); the Output paragraph's body remnant now names the artifact and points
  at `analysis-codebase.sec.output`.
- **Consumers assessed:** `commands/setup.md` dispatches the skill and consumes
  `.mochiko/memory/codebase-analysis.md` (path unchanged, both arms preserved); the
  router row describes the produced output generically.

## [v0.103.0] Assess-status contract, parked-mode carve, When-NOT set — protection transfers + body slims

- **Disposition:** superseded — protection transfers per D8/C4 onto
  `analysis-codebase.essential-floor-canonical-definition` (floor — the
  canonical-definition boundary and "Do not redefine the categories here"),
  `analysis-codebase.intent-blind-waiver-blind` (floor),
  `analysis-codebase.never-soften-never-waive` (floor),
  `analysis-codebase.interpretation-reserved-to-session` (must, reservation),
  `analysis-codebase.collision-inventory-out-of-scope` (must, routing — the v0.91.0
  parked-mode carve, all its qualifiers preserved),
  `analysis-codebase.when-not-routing` (must, routing — the five When-NOT bullets),
  `analysis-codebase.determinism-boundary` (must — the detect-stack blockquote), and
  `analysis-codebase.only-report-what-is-found` +
  `analysis-codebase.essential-floor-canonical-definition` for the two obligation-shaped
  Common-Mistakes rows (the table itself stays body prose whole per its census
  disposition — the [v0.24.0] KEPT survives in the body AND as rules; the rows are
  teaching, not floors, so no dual-homing violation).
  Body sections leaving whole: `## When NOT to Use` ·
  `### Essential-Floor Status Assessment` · `## Other modes (moved to other clusters —
  not wired this run)` (its routing facts now rules 7 and 13) · the Detection Script
  blockquote. The Mode section's entity bullet and Essential-Floor bullet slim to
  non-restating forms (the collision parenthetical → rule 7; "see below" re-pointed at
  the schema sections — no dangling referent left behind).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows;
  protecting lineage `DECISIONS.md` 2026-08-10 validator-scope-and-verbosity (the
  v0.63.0 guardrails keep-set: assess-status contract, intent-blind/waiver-blind rule) ·
  2026-08-26 plan-stage-utility D1 (the [v0.91.0] parked-mode vocabulary re-key — meaning
  preserved again here: the mode stays parked, only its statement moved) · 2026-07-25
  skill-succinctness-strip (the [v0.24.0] KEPT Common Mistakes table).
- **Content:** the four sections named above, verbatim in git history (pre-v0.103.0);
  wording preserved verbatim in substance in the rule texts, including "waivers are
  governance rulings, not codebase facts" and "the same codebase gets the same status".
- **Kept deliberately:** the Common Mistakes table whole (CODEOWNERS row included), the
  Overview's deliverable-and-consumers narrative, the detect-stack invocation block, and
  the Related Skills section (not census-inventoried — stays prose at census grain).
- **Consumers assessed:** `commands/setup.md` (deliverable + statuses, unchanged) ·
  `agents/principal-architect.md` and the router (describe, never restate) ·
  `authoring-constitution` + its `references/ESSENTIAL-FLOOR.md` (canonical home,
  untouched, now the single referent of the repaired sentence) · the BACKLOG reclaim item
  owning the unwired mode (operating-doc content, meaning preserved).

## [v0.91.0] Parked Brownfield-mode carve-out re-keyed: "spec/plan-cluster" → "spec/design-cluster" — plan-stage retirement D1

- **Disposition:** superseded → "the spec/design-cluster Brownfield mode (not wired this run)",
  at both sites.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1; wording ruled by the wave lead
  2026-08-26 as a **vocabulary re-key only**).
- **Scope of the change, stated because this seat flagged it as not-obviously-mechanical:** the
  phrase names a mode that has never been wired, so the risk was re-keying a label whose referent
  had itself changed. The lead ruled it a vocabulary re-key: **the mode stays parked and the
  meaning of the BACKLOG reclaim item that owns it is preserved.** Nothing about the mode's
  status, scope, or wiring moved — only the cluster name it points at, which is the same cluster
  under its new name.
- **Content (superseded fragments, verbatim — two sites):**

  1. When-NOT-to-Use:

     ```
     - **Collision detection / JSON inventory**: that is the spec/plan-cluster Brownfield mode (not wired this run)
     ```
  2. Output-scope bullet:

     ```
       annotations; document what is found — the deeper collision-risk inventory is the
       spec/plan-cluster Brownfield mode, not produced here)
     ```

- **Kept deliberately:** both carve-outs entire — collision detection and the JSON inventory stay
  **out of this skill's scope**, the deeper collision-risk inventory is still explicitly not
  produced here, and the "(not wired this run)" / "not produced here" qualifiers both survive
  verbatim. The skill's own remit (deterministic stack detection, architecture and convention
  extraction, the intent-blind Essential-Floor status read) is untouched.
- **Budget:** body 6,607 → **6,611** against the 8,137 budget; description unchanged at 349
  against 437. Both inside.
- **Third site, caught on the fix round's re-sweep:** the mode-inventory paragraph (:99) named
  the same parked mode a third time — `extraction → JSON collision inventory against a proposed
  spec) lives in the **spec/plan cluster**;` — re-keyed to "**spec/design cluster**" on the same
  ruling. The earlier sweeps' term list matched "plan-cluster" but not "spec/plan", which is why
  two passes missed it. Body 6,611 → **6,613** against the 8,137 budget; description unchanged at
  349. The standalone Context-report mode and the constitution-context cluster pointer beside it
  are untouched.
- **Consumers assessed:** the router's `analysis-codebase` row describes the skill's produced
  output and never named the parked mode — no re-key owed there. The BACKLOG item that owns the
  unwired mode is operating-doc content, outside `plugins/`, and its meaning is preserved per the
  ruling above.

## [v0.76.0] `codebase-analysis-template.md` read-pointers → `codebase-analysis` schema (two-arm) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template codebase-analysis`, or Read `plugins/mochiko/schemas/codebase-analysis.yaml` raw (D8-first-class). Two sites: `SKILL.md` Output pointer + `references/CONTEXT-GATHERING.md` scope-note mention.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `following` / `[\`codebase-analysis-template.md\`](../../templates/codebase-analysis-template.md).` — `SKILL.md` Output
  - `its findings flow into \`.mochiko/memory/codebase-analysis.md\` (per` / `> \`codebase-analysis-template.md\`).` — `references/CONTEXT-GATHERING.md`
- **Kept deliberately:** `references/CONTEXT-GATHERING.md`'s later "in the codebase-analysis template" conceptual phrase (line 13 — no file path, not a dead pointer); the `artifact-format.md` pointer; `detect-stack.sh` and all descriptive text.
- **Consumers assessed:** n/a (single-writer skill + its reference).

## [v0.63.0] Guardrails cut — detection/procedure prose removed, assessment contract kept; slim description

- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`variants/body/analysis-codebase/`, `variants/descriptions/analysis-codebase/`), one merged edit.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict; `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`)
- **Content (faithfully compressed — section-level inventory, body 8,962 → 6,509 chars, −27%):**
  - **Removed whole:** `## When to Use` (trigger list — carried by the description) · the four Essential-Floor per-category indicator subsections `#### Security — status indicators`, `#### Testing — status indicators`, `#### Error Handling — status indicators`, `#### Observability — status indicators` (see reconciliation below).
  - **Shortened:** `## Detection Script` (855 → 458 chars — the `detect-stack.sh` invocation and the determinism-boundary note kept; extra prose trimmed).
  - **Kept intact:** `# Analyzing Codebase`, `## Overview`, `## When NOT to Use`, `## Common Mistakes`, `## Mode: Setup Brownfield (the wired path)`, `### Essential-Floor Status Assessment` (the assess-status contract and intent-blind/waiver-blind rule), `### Setup-Brownfield Quality Checklist`, `## Other modes (moved to other clusters — not wired this run)`, `## Related Skills`.
  - Old description (new slim form is 349 chars; **old verbatim, 462 chars**): "This skill MUST be invoked when analyzing an existing codebase during a brownfield /mochiko:setup run — detecting the technology stack, extracting architecture and conventions, and assessing Essential-Floor status — to produce `.mochiko/memory/codebase-analysis.md`. SHOULD also invoke when a setup/constitution producer needs a deterministic stack baseline (`detect-stack.sh`) or a present/partial/absent read of an existing project before authoring governance."
  - Verbatim removed text survives in: git history of the SKILL.md; the before/after pair in `variants/`; archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (guardrails keep-set):** the goal/output contract (Overview + the `.mochiko/memory/codebase-analysis.md` deliverable), the assess-status-vs-require-floor boundary and the intent-blind/waiver-blind rule (`### Essential-Floor Status Assessment`), the anti-patterns (`## Common Mistakes`), the quality checklist, the determinism boundary, and the `references/CONTEXT-GATHERING.md` and template pointers.
- **Protected-content reconciliation (MANDATORY):** the two prior `[v0.24.0] KEPT:` entries reconciled —
  - **`[v0.24.0] KEPT: the four Essential-Floor check tables (what-to-assess criteria per category)`** — **superseded by this ruling.** The guardrails cut removes the four `#### … status indicators` subsections that carried the per-category assessment criteria (auth-at-boundaries, secrets-from-env, test-framework-configured, correlation-IDs, …). The canonical category definitions remain in `authoring-constitution/references/ESSENTIAL-FLOOR.md` (unchanged), which `### Essential-Floor Status Assessment` still points to as the source to assess against; the mochiko-side assess-hint tables are the superseded content. Verbatim in the three homes above. **DEVIATION FLAGGED (see build report):** the surviving sentence "assess against it using the indicators below" now has no indicators below it — the benchmark variant left a dangling internal pointer. Shipped byte-identical to the ruled variant (not the shipper's to rewrite); routed to the T4 audit.
  - **`[v0.24.0] KEPT: Common Mistakes table`** — **survives intact** (`## Common Mistakes`, including the CODEOWNERS governance row). Not touched by this cut.
- **Consumers assessed (1 consuming command):** `commands/setup.md` — dispatches the skill for the brownfield analysis artifact; relies on the `.mochiko/memory/codebase-analysis.md` output and the present/partial/absent status, not on any removed section anchor. Contract intact.


## [v0.24.0] "Scope (this run)" blockquote in Overview
- **Disposition:** deleted — in-file triplication; single home is the "Other modes" section (compressed same wave, below)
- **Tier failed:** 1 (restated in-file: same not-wired scope note appeared in Overview, a When-NOT-to-Use bullet, and the Other-modes section)
- **Content:** only Setup-Brownfield wired; collision mode → spec/plan cluster; Context-report mode → constitution-context cluster, sub-procedure kept in `references/CONTEXT-GATHERING.md`
- **Consumers assessed:** see header — none

## [v0.24.0] Project-type / web-framework / ORM / architecture detection tables (~48 lines)
- **Disposition:** deleted; the Mode section's entity bullet reworded ("use the tables above" → "models/, schema files, ORM annotations"); the determinism-boundary note also reworded ("The detection tables and the Essential-Floor assessment" → "Framework/architecture inference and the Essential-Floor assessment") — collateral dangling reference caught by the independent audit
- **Tier failed:** 2 (generic model knowledge — `package.json`→Node, `@app.get()`→FastAPI; no failure named that an unaided model produces; the deterministic layer is `detect-stack.sh`, whose findings the skill already declares ground truth)
- **Content:** four indicator tables mapping package-manager files → project type, framework/ORM code signatures, and directory layouts → architecture patterns
- **Consumers assessed:** see header — none

## [v0.24.0] Four Essential-Floor example-grep bash blocks (~44 lines)
- **Disposition:** deleted (the four per-category check tables KEPT — see survivor entry below)
- **Tier failed:** 2 (each block re-encoded its check table's "How to Detect" column as example greps the model synthesizes on demand; illustrations, not contracts — grep spelling does not change a present/partial/absent judgment)
- **Content:** ready-to-run grep/ls/find snippets per floor category (auth middleware, env secrets, validation libs; test config/files/CI; custom errors, error logging, status codes; logger config, correlation IDs, PII-in-logs negative check)
- **Consumers assessed:** see header — none

## [v0.24.0] "The script detects:" value-vocabulary list
- **Disposition:** deleted
- **Tier failed:** 2 (redundant encoding — the JSON example directly above shows the same fields; the model consumes actual script output at runtime, not the possible-value enumeration)
- **Content:** six bullets enumerating possible values for project_type / package_manager / frameworks / orms / architecture / ci_cd
- **Consumers assessed:** see header — none

## [v0.24.0] Manual Detection Commands section (~21 lines)
- **Disposition:** deleted; sole non-obvious hint (CODEOWNERS) relocated → Common Mistakes "Ignoring governance" fix cell
- **Tier failed:** 2 (generic shell one-liners — `cat package.json | jq`, `ls` directory probes — the model produces unaided; conditional on "script insufficient" but names no failure)
- **Content:** example bash for tech-stack / architecture / CI-CD / governance / test-structure detection when `detect-stack.sh` is insufficient
- **Consumers assessed:** see header — none

## [v0.24.0] "Other modes" section compressed 10 → 6 lines
- **Disposition:** relocated → itself (compressed in place; now the single in-file home for the not-wired scope note, absorbing the deleted Overview blockquote)
- **Tier failed:** 1 (the per-mode parentheticals restated the same cluster-routing fact twice per bullet)
- **Content:** same routing facts — collision mode → spec/plan cluster (JSON inventory schema moves with it), Context-report mode → constitution-context cluster (extraction sub-procedure kept in `references/CONTEXT-GATHERING.md`)
- **Consumers assessed:** see header — none

## [v0.24.0] KEPT: the four Essential-Floor check tables (what-to-assess criteria per category)
- **Tier-2 evidence:** contested during the pilot's bash-block strip and kept — they carry the assessment criteria themselves (auth-at-boundaries, secrets-from-env, test-framework-configured, correlation-IDs, …) with detection hints; deleting them would leave present/partial/absent status with no defined checks, un-anchoring the tier-blind assessment that `ESSENTIAL-FLOOR.md` (require-floor side) expects this skill to own (assess-status side). Session ruling: pilot ratification 2026-07-25.

## [v0.24.0] KEPT: Common Mistakes table
- **Tier-2 evidence:** contested (suspected generic-table) and kept — every row names a concrete failure and its fix (assuming framework without evidence, inventing findings, redefining the Essential Floor instead of assessing it, ignoring existing governance); gained "CODEOWNERS" in the governance row as the relocation target of the Manual-Detection strip. Session ruling: pilot ratification 2026-07-25.
