# Strip notes — `skills/review-specifications`

Entry formats: `strips/README.md`. Wave context: the specify cluster wave (v0.13.0).
Single-consumer primitive (the specify critique, mounted on `devils-advocate`) — strips
ruled in-wave.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the review family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/review-specifications/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action at invoke, before any hunting: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
  action.** The schema is the source of truth for this skill's binding rules; this body carries
  identity and procedure only. Its rules are nested in six sections, each addressable by its
  section ID: `review-specifications.sec.independence` (author/grader separation) ·
  `review-specifications.sec.scope` (jurisdiction, routing, and what never gets added) ·
  `review-specifications.sec.inputs` (coverage duties, baselines, external claims) ·
  `review-specifications.sec.verdict` (the hunt taxonomy, the check sets, the severity
  grammar) · `review-specifications.sec.output` (question craft and report contracts) ·
  `review-specifications.sec.reserved` (the clearing this seat never issues).

  Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
  `kind:` reads `constraint`; a rule carrying `when:` binds only where its terms hold against
  the schema's declared `conditions:`, except that a `class: floor` rule is always read and
  always delivered — `when:` gates when its obligation applies, never whether it reaches you.
  Where a rule carries `extends: review-common.<slug>`, the stub inherits `text` / `labels` /
  `pointer` only from `skill-review-common.yaml` — `class` and `kind` are always this schema's
  own, and the stub's `review-specifications.*` ID stays the citable ID. Labels come from
  `../../schemas/skill-labels.yaml`. A `pointer:` rule binds you to that file's or skill's
  content, referenced never restated.

  The schema carries **the 8 rules of `class: floor`**. State the floor count back before the
  first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
  continue.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The per-section glosses this
  block carried are covered by the render, whose `sections` line prints a title per section and
  whose empty sections carry a `note:` giving the reason. The "this body carries identity and
  procedure only" clause is dropped: it stated a scope rather than an obligation, and the new
  halt paragraph states the same split. The `extends:` stub resolution and the family
  common-file co-Read are discharged by the render, which resolves every stub before the model
  sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-review-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The schema carries **the 8 rules of `class: floor`**. State the floor count back before the
  first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
  continue.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.100.0] Skill-content-schema conversion — body obligations moved to `schema.yaml`; the pair is now the graded unit

Wave context: skill-content-schema D1–D9 as amended (`.mochiko/brainstorms/skill-content-schema/record.md`,
`DECISIONS.md` 2026-09-01); census referent `.mochiko/brainstorms/skill-content-schema/census.md` §A/§B
(RSPEC rows). Census-row → minted-ID map: 1 `input-not-verdict` · 2 `author-grader` (stub) ·
3 `what-not-how` · 4 `not-for` · 5 `complete-coverage` · 6 `question-format` ·
7 `no-presupposed-mechanism` · 8 `clarifications-shape` · 9 `gap-taxonomy` · 10a `excess-admissibility` ·
10b `never-excess` (stub — row 10 split: the admissibility limb stays local per census §C/R2, the carve
binds the common block) · 11 `smuggled-posture` · 12 `external-claims` · 13 `feature-layer-same-report` ·
14 `map-git-baseline` · 15 `map-mirror` · 16 `feature-critical-checks` · 17 `feature-important-checks` ·
18 `sf-legal-shapes` · 19 `serve-and-click` · 20 `authority-split` · 21 `sf-critical-checks` ·
22 `sf-important-checks` · 23 `severity-grammar` · 24 `report-structure` · 25 `density-never-gap` ·
26a `evidence-floor` (stub) · 26b `gap-bound` · 27a `no-scope-creep` · 27b `check-existing-first`.
All IDs carry the `review-specifications.` prefix. Per census §C/J-6 this skill binds NEITHER
`review-common.default-fail` NOR `review-common.verdict-is-input` — it issues no clearing verdict; its
local posture rule is `input-not-verdict` (allowlist keep-distinct edge).
Deviation note (recorded, code correct): census §B marks row 18 `when: manifest-present`, but the
rule's own text adjudicates BOTH legal shapes (manifest or waiver line), so `sf-legal-shapes` is
built unconditional — gating it would make the waiver arm unreachable; rows 19–22 gate on the
declared `manifest-present` condition.
Structural-vs-content accounting (D8/C1): delivered pair vs pre-conversion body = **0.88×** —
relocation net of common-stub convergence and the row-10 grain split; structural overhead only, no
content growth.

### Supersession-transfer — [v0.26.0] KEPT lineage: severity table + Core Principle table substance
- **Disposition:** superseded — protection transfers to schema rules `severity-grammar` and
  `what-not-how`, citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "Critical = cannot build without this answer, ask now · Important = will cause rework,
  should ask now · Minor = polish, log and defer" · "WHAT is missing, never HOW to implement" (+
  product questions, never implementation questions).
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.25.0] RETURNED five-class canonical-home taxonomy
- **Disposition:** superseded — protection transfers to schema rule `gap-taxonomy` (the canonical-home
  binding), citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "the canonical hunt taxonomy `devils-advocate` leans on".
- **Kept deliberately:** the six class descriptors (the per-class parenthetical sub-lists) stay in the
  body's Procedure prose — teaching content per census §B (RSPEC row 9 note).
- **Consumers assessed:** `devils-advocate` leans on the taxonomy by name; the canonical home is
  unchanged (this skill), only its carrier moved body → schema.

### Supersession-transfer — [v0.67.0] class-6 excess row + calibration
- **Disposition:** superseded — protection transfers to schema rules `excess-admissibility` (local
  admissibility limb) and `never-excess` (`extends: review-common.never-excess`, the C6 carve), citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "**excess / unpaid scope** (no user need or ratified driver pays for it — admissible
  only naming the driver it fails to trace to or the cheaper shape; a floor / compliance-module /
  NFR-derived obligation is never excess)".
- **Consumers assessed:** family common block C6; the admissibility limb's object noun stays local per
  near-dup R2.

### Supersession-transfer — [v0.63.0] guardrails keep-set + review-evidence floor line
- **Disposition:** superseded — protection transfers to schema rules `density-never-gap`,
  `no-scope-creep`, `check-existing-first`, `gap-bound`, and `evidence-floor`
  (`extends: review-common.evidence-floor`), citing skill-content-schema D8/C4 + `DECISIONS.md`
  2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** the Floors paragraph clauses verbatim: "density is never itself a gap
  (`templates/artifact-format.md` envelope) — grade substance, never prose style; undisclosed overage
  past the envelope's size defaults is advisory per its rule 8 · verdict and per-finding dispositions
  land in the reviewed artifacts themselves — review evidence only in conversation is a floor
  violation · 5–7 Critical/Important gaps per round, related gaps grouped — never a 20-gap dump ·
  scope creep is not a gap: clarify existing features, never add new ones as 'missing requirements' ·
  check existing patterns and decisions first — never ask what is already answered."
- **Consumers assessed:** family common block C1; cross-grammar near-dup edge with command
  `common.yaml` is allowlist territory (census J-5).

### Supersession-transfer — [v0.58.0] feature-layer 10 checks + R13 git-baseline rule
- **Disposition:** superseded — protection transfers to schema rules `map-git-baseline`,
  `feature-layer-same-report`, `feature-critical-checks`, `feature-important-checks`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** the **Feature layer** paragraph whole — the baseline rule ("grade staged writes against
  the git state of the map at run open, never a workspace copy"), same-reviewer-same-report, and the
  ten checks (Critical six + Important four), carried verbatim into the two set-rules.
- **Consumers assessed:** `mochiko:authoring-feature-map` stays the single source of map machinery
  (`map-mirror` binds it); the check list remains the reviewer's mirror.

### Supersession-transfer — [v0.50.0] S&F 8 checks + serve-and-click + authority split
- **Disposition:** superseded — protection transfers to schema rules `sf-legal-shapes`,
  `serve-and-click`, `authority-split`, `sf-critical-checks`, `sf-important-checks`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** the **Screens & Flows** paragraph whole — two legal shapes, "Serve the prototype and
  click it … adversarial, not ceremonial", the authority split ("flows, screens, data shown are
  binding; layout and styling advisory — a cosmetic finding against a low-fi prototype is
  wrong-altitude"), and the eight checks (Critical five + Important three), carried verbatim into the
  set-rules. The S&F check rules gate on the declared `manifest-present` condition.
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.53.0] carve-out
- **Disposition:** superseded — protection transfers to schema rule `not-for`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "code review (sole carve-out `mochiko:review-code-minimalism`, implement-side)".
- **Consumers assessed:** `review-plan-artifacts` carries its own copy (its own strip entry covers
  it); allowlist keep-distinct edge per census §C.

### Supersession-transfer — [v0.82.0] envelope wording
- **Disposition:** superseded — protection transfers to schema rule `density-never-gap`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "density is never itself a gap … undisclosed overage past the envelope's size defaults
  is advisory per its rule 8."
- **Consumers assessed:** none — skill-local content.

### Relocation — remaining body obligations → schema rules (non-protected clauses)
- **Disposition:** relocated → `schema.yaml` rules `complete-coverage`, `question-format`,
  `no-presupposed-mechanism`, `clarifications-shape`, `smuggled-posture`, `external-claims`,
  `map-mirror`, `report-structure`, `input-not-verdict`, and the stub `author-grader`
  (`extends: review-common.author-grader`; the body line "Independent reviewer, never the author"
  superseded by the common block's strongest wording per near-dup convergence R2).
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3, obligations-only boundary).
- **Content:** "Coverage is complete, never sampled: every user story reviewed for completeness, every
  success criterion checked for measurability, edge cases hunted per main flow" · "Every question is a
  decision the stakeholder can make: 2–3 concrete options, what each means for users, why it matters —
  specific, never vague, never presupposing a mechanism ('should we cache?' assumes caching) — in the
  Clarifications shape of `templates/advocate-report-template.md`, never a variant" · "Implementation
  posture smuggled into a constraint … is an assumption-gap finding — it freezes a choice stakeholders
  never ratified" · "A regulatory/product-legal assertion is a floor-class external claim — verify per
  ../review-brainstorm/references/EXTERNAL-CLAIMS.md; undisclosed is a gap" · "map machinery
  single-sourced in `mochiko:authoring-feature-map`, this list is the reviewer's mirror" · "Report
  structure (machine-first findings YAML, clarifications with options + impact, recommended verdict,
  one-line `strengths:`) single-sourced at `templates/advocate-report-template.md`; no report path
  named → same structure inline" · "**input, never a clearing PASS/FAIL verdict of its own**" ·
  "Independent reviewer, never the author".
- **Consumers assessed:** none — skill-local content.

## [v0.88.0] User-ruled true-deletion body cut — body 12,184 → 6,187 chars (−49.2%)

- **Disposition:** superseded → a single-file six-paragraph body (identity+routing · Method ·
  Feature layer · Screens & Flows · Severity+output · Floors) — true deletion, no relocation,
  no new file; the `description:` (490 chars) untouched. This skill has no `references/`
  directory, so every surviving rule survives in the body (or as an explicit pointer to
  another primitive's single source). Every behavioral rule of the baseline body survives as
  a compressed clause; all worked-example tables (the Core Principle wrong/right rows, the
  gap-category example questions, the Common-Mistakes ❌/✅ rows), the section forms
  (Quality Checklist, Common Mistakes, Review Process), and rationale prose are deleted. The
  user ruled **ship the rule-complete cut** at the ratification gate — the shallowest of the
  four compression passes (−49.2% vs −90.0%/−78.8%/−63.8%), because the 18 ruled checks and
  the six-class taxonomy are body-only with zero reference offload and the body was already
  twice-compressed (v0.26.0, v0.63.0). Deeper cuts declined with the deaths named: ~−70%
  degrades the 18 check clauses to bare labels and deletes the coverage clause + 4 Floors
  rules; −90% deletes the feature-layer and S&F check sets wholesale, breaking the router
  row's "including the feature-layer grade" citation and `devils-advocate`'s canonical-home
  anchor.
- **Tier failed:** n/a — supersession by ruling (in-session user ruling 2026-08-26 at the
  `compressing-skills` ratification gate, on the v0.82.0/v0.83.0/v0.87.0 "cut now, eval
  validates later" precedent; ADR
  `.mochiko/decisions/2026-08-26-review-specifications-true-deletion-cut.md`; `DECISIONS.md`
  2026-08-26 row). Evidence carried per the ceremony: the 81-entry rule inventory
  `evals/review-specifications/rules.json` (non-compressor-authored; 5 inventory-driven
  clause restorations pre-gate: R-012 no-mechanism-presupposition, R-015 specific-not-vague,
  R-064 every-story, R-065 every-SC-measurable, R-066 edge-cases-per-flow) and the
  span-by-span disposition map in `evals/review-specifications/pass-report.md`.
- **Disposition map (baseline body section → new home; verbatim home for all removed text:
  git history of this SKILL.md, pre-v0.88.0):**
  - *Overview* — compressed into the opening line (gap-finding input, never a clearing
    PASS/FAIL of its own; WHAT-not-HOW; independent, never the author).
  - *When NOT to Use (6 bullets)* — the "Not for:" line; all six routes survive, incl. the
    v0.53.0 `review-code-minimalism` carve-out and the `analysis-iterative`
    disjoint-triggers boundary.
  - *Core Principle + wrong/right table* — the product-not-implementation rule and its
    altitude sentence survive in Method; the four worked-example rows die.
  - *Question Format* — Method clause: 2–3 concrete options · what each means for users ·
    why it matters · specific-never-vague · never presupposing a mechanism · the
    Clarifications shape of `templates/advocate-report-template.md`, never a variant.
  - *Gap Categories (2 tables + 2 paragraphs)* — Method: the five category names survive;
    the six defect classes survive with compact descriptors, class 6 keeping its full
    v0.67.0 calibration clause; the canonical-home clause ("the canonical hunt taxonomy
    `devils-advocate` leans on") survives; the posture-smuggled-constraint rule survives
    with its defining example; the EXTERNAL-CLAIMS.md floor-class verify pointer survives.
  - *The feature layer* — survives compressed, complete: same-reviewer-same-report, the R13
    git-baseline rule, the `authoring-feature-map` single-source pointer, and all 10 checks
    with severities (grouped Critical then Important).
  - *Screens & Flows* — survives compressed, complete: same-reviewer-same-report, both
    legal shapes incl. the waiver line, the serve-and-click obligation, the authority
    split, and all 8 checks with severities.
  - *Severity Classification table* — one line, spec-specific wording intact ("cannot
    build without this answer" / "will cause rework" / "polish, log and defer" + actions).
  - *Output Format* — the Severity+output paragraph (advocate-report-template single
    source + inline fallback).
  - *Review Process (2 paragraphs)* — Floors: the v0.82.0 envelope wording (density never
    a gap · substance never prose style · undisclosed overage advisory per rule 8) and the
    v0.63.0 review-evidence floor line, both substance-intact with wording compressed.
  - *Quality Checklist (11 items)* — each item's rule survives in Method (all-stories /
    SC-measurability / edge-cases-per-flow / product-focus / options / why-it-matters) or
    the check paragraphs (feature-layer all-10, S&F all-8 + served-and-walked) or Floors
    (severity classification, grouping); the checklist-as-section form dies.
  - *Common Mistakes (8 rows)* — distinct rules survive in Floors (5–7 Critical/Important
    gaps per round · scope creep is not a gap · check existing patterns and decisions
    first) and Method (specific-not-vague, options, why-it-matters, no implementation
    bias); the table form dies.
  - *Related Skills (3 bullets)* — inline pointers: `authoring-requirements` and
    `analysis-iterative` in the Not-for line, `authoring-feature-map` in the Feature-layer
    paragraph.
- **MANDATORY KEPT reconciliation:** [v0.26.0] KEPT severity table + Core Principle table —
  substance survives compressed; the KEPT status of both *table forms* ends by this ruling.
  [v0.25.0] RETURNED five-class canonical-home table — all five classes + the canonical-home
  relationship survive compressed; the home stays honest. [v0.67.0] class-6 row — survives
  with its calibration clause. [v0.63.0] guardrails keep-set — every member's obligation
  survives per the map above; the floor line's wording superseded, substance intact.
  [v0.58.0] feature-layer table — all 10 checks + baseline rule survive. [v0.50.0] S&F
  section — all 8 checks + walk rules survive. [v0.53.0] carve-out — survives. [v0.82.0]
  envelope wording — survives.
- **Consumers assessed:** `agents/devils-advocate.md:52` (canonical gap taxonomy · severity
  rubric · structured output format — all three anchors survive) and `:19` · router
  `skills/mochiko/SKILL.md:60` (severity buckets, feature-layer grade, S&F walked-prototype
  grade — all survive) · `authoring-prototype` (graded-with-the-spec pointer — survives) ·
  `authoring-feature-map` (derivation + map delta graded here — survives) ·
  `analysis-iterative` (disjoint-trigger boundary — survives) ·
  `review-brainstorm/references/EXTERNAL-CLAIMS.md:94` (consumer listing — the verify
  pointer survives) · `templates/advocate-report-template.md` (findings-type vocabulary fed
  by the class names — all six survive). This skill has no references directory, so no
  reference-to-body pointers exist to go stale (the v0.87.0 sibling-pass defect class is
  structurally absent). No dead pointers created.

## [v0.82.0] Envelope-citation aligned to artifact-format v3 (audit fix 3)

- **Disposition:** superseded → the Review Process density sentence now reads "never prose
  *style*; undisclosed overage past the envelope's size defaults is an advisory finding per
  its rule 8". The old "never prose volume" restated the pre-v3 rule 8 this skill cites, made
  false by the verbosity-envelope-enforcement ruling.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`; `DECISIONS.md`
  2026-08-22 row).
- **Content (superseded, verbatim):** `unstated assumption), never prose volume.`
- **Kept deliberately:** "Density is never itself a gap" — the brevity half survives verbatim.
- **Consumers assessed:** body 12,184 vs budget 14,089 after the edit — inside; the skill's
  gap-report structure and reviewer independence untouched.

## [v0.67.0] Excess / unpaid-scope class added — defect-class lead-in re-keyed
- **Disposition:** superseded → the excess posture from the architect-role ruling: the defect-class table gains a remove-shaped sixth class, so the "five ... those questions hunt" lead-in was rewritten.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md`, D3 as amended by its F3 calibration clause; DECISIONS.md combined-wave build row).
- **Content (superseded, verbatim):** OLD lead-in — "The five requirement-defect classes those questions hunt (the canonical hunt taxonomy — the `devils-advocate` persona names these classes and leans on this section for the detail):" → NEW — "The requirement-defect classes — the canonical hunt taxonomy the `devils-advocate` persona leans on this section for the detail. The first five are what the product questions above surface; the sixth is the remove-shaped excess class:".
- **Kept deliberately:** the five absence/conflict classes (Missing requirements, Ambiguities, Edge cases, Assumption gaps, Contradictions) untouched; the pointer relationship to the `devils-advocate` persona preserved — the dropped "names these classes and" is inert, since the persona points here without a class count (grep-verified), so no ripple. Pure addition riding the decision row (no strip): the "Excess / unpaid scope" table row carrying the calibration clause in one breath.
- **Consumers assessed:** `devils-advocate` persona (`plugins/mochiko/agents/devils-advocate.md`) points at this skill for "the canonical gap taxonomy" with no hard count — unchanged. No command references the class count.

## [v0.63.0] Guardrails body + slim description; review-evidence floor line added (guardrails-vs-detail benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description
  (`.mochiko/benchmarks/guardrails-vs-detail/variants/body/review-specifications/` and
  `variants/descriptions/review-specifications/`; the shipped file is the deterministic merge of
  the two, plus one sanctioned floor-line addition — below).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict,
  `DECISIONS.md` 2026-08-10 benchmark-verdict row; record
  `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`, Benchmark execution;
  `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — body arm formal D6 PASS,
  descriptions arm 0 fire misses).
- **Content (faithfully compressed):** body 12,262 → 11,271 chars before the floor line
  (11,452 after; net −7%); description 1,337 → 490 chars (−63%). Body cut: **When to Use**
  deleted whole (four bullets restating invocation conditions) and the nine-step **Review
  Process** walkthrough deleted (read full spec → check stories → verify criteria → edge cases →
  feature-layer grade at baseline → Screens & Flows walk → classify → generate questions →
  group gaps; the two grading steps' obligations survive as the Quality Checklist's
  feature-layer and Screens & Flows rows, which carry the same all-checks + run-open-baseline +
  served-prototype wording). Description cut: the long check-enumeration compressed; MUST
  trigger, gap-finding-not-verdict boundary, and the analysis-iterative sibling distinction
  kept. Verbatim homes: git history of this file (pre-v0.63.0), the before/after pair under
  `variants/`, and archive branch `worktree-brainstorm-validator-scope`.
- **Floor line added (cross-cutting finding 1, F-X1 mitigation — agents ruling (b)):** "The
  independent review leaves its verdict and per-finding dispositions in the reviewed artifacts
  themselves — review evidence that lives only in conversation is a floor violation." Placed in
  Review Process. Pure addition riding the same decision row.
- **Kept deliberately:** the guardrails keep-set — Overview, When NOT to Use, Core Principle
  table, Question Format, Gap Categories (incl. the floor-class external-claim verify), the
  feature-layer 10-check table, the Screens & Flows 8-check table, Severity Classification,
  Output Format (advocate-report-template binding), the density-is-not-a-gap paragraph, Quality
  Checklist, Common Mistakes, Related Skills. The [v0.26.0] KEPT survivors (severity table,
  Core Principle table) and the [v0.58.0] feature-layer replacement survive in full — no prior
  KEPT or protected line is touched.
- **Consumers assessed:** devils-advocate (mounts it; same report shape, same tables) · specify
  (binds it; the deleted process steps' graded obligations remain in the checklist rows the
  reviewer must satisfy). Contract intact.

## [v0.58.0] Delivery Slices grade superseded by the feature-layer grade (D16)
- **Disposition:** superseded → the new "The feature layer" section (10-check table: derivation honesty, disposition completeness, dedup at the run-open git baseline, granularity, entry well-formedness, delta legality, SC re-homing, in-flight handling, selection-card deferred-SC honesty, specs-index agreement). Map machinery stays single-sourced in `mochiko:authoring-feature-map`; the table is the reviewer's mirror.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)", record `.mochiko/brainstorms/feature-map-layer/record.md`, D4 slices retire · D16 extended spec review · R13 git-baseline rule · D21 selection-card deferred-SC list · D8 delta legality).
- **Content (verbatim, the superseded section):**
  ```
  ## The Delivery Slices section

  The spec's Delivery Slices section is graded with the spec — same reviewer, same report. It is
  either a decomposition or the single line "Single slice — whole spec."; both shapes get graded
  (the one-liner via the depth second-guess). Vocabulary guard: a **graduation slice** groups
  user stories at spec level; a **vertical slice (cycle)** is implementation-level, downstream —
  a decomposition whose groups are shaped like implementation cycles is a wrong-altitude finding.

  | # | Check | Question | Typical severity |
  |---|-------|----------|------------------|
  | 1 | Story coverage | Every spec story in some slice — no orphans, no invented stories? | Critical |
  | 2 | Exactly one home | No story in two slices? | Critical |
  | 3 | Dependency closure | Every slice designable/buildable from earlier slices only? | Critical |
  | 4 | Foundation legitimacy | First slice establishes the shared design core AND delivers a testable journey (pure plumbing forbidden)? | Critical/Important |
  | 5 | Ordering rationale | Dependency first, priority as tie-break — and the rationale recorded? | Important |
  | 6 | Sizing | Within the soft 2–4 target, or explicitly justified? | Important |
  | 7 | Journey coherence | Each slice a coherent user journey, not a grab-bag? | Critical |
  | 8 | Cross-cutting visibility | Cross-cutting stories homed earliest-meaningful with extend obligations recorded on every touched slice? | Important |
  | 9 | Feature-Done SC coverage | Every SC-# mapped to a verifying slice? | Critical |
  | 10 | Feature-Done seams | Cross-slice seams named where slices interact? | Important |
  | 11 | Depth second-guess | Both directions: a decomposition the intent/spec didn't warrant, or a "Single slice" line hiding real value seams? Grades the depth call itself against the Intent section's delivery ruling. | Important |
  ```
  Also superseded, same edit (verbatim):
  - description clause: `grading its Delivery Slices section (story coverage, dependency closure, foundation legitimacy, Feature-Done, and the depth second-guess),` → the feature-layer clause;
  - Screens & Flows check 7: `| 7 | Slice tags | Decomposed spec → every SCR/FLOW row slice-tagged; out-of-slice screens greyed but reachable? | Important |` → FEAT tags (R10);
  - process step 5: `**Grade the Delivery Slices section** against the 11-check table above, reading the Intent section's delivery ruling first (the depth second-guess is graded against it)` → feature-layer grade at the run-open baseline;
  - checklist row: `- [ ] Delivery Slices section graded (all 11 checks; the single-slice line via the depth second-guess)` → feature-layer row.
- **Kept deliberately:** the slice invariants themselves survive re-keyed to features in `mochiko:authoring-feature-map` (D18/D22) — coverage/one-home/SC-coverage live on as feature-layer checks 1–3 and 7 (dependency closure survives only in `authoring-feature-map`, not in the reviewer's table — audit-corrected); foundation-legitimacy demoted to ordering guidance (D22) and so deliberately absent from the reviewer's table; the depth second-guess dies with the decomposition-vs-single-slice fork it graded (selection replaces it, and selection is the user's ruling, not a graded artifact). All other sections (story/FR/SC quality, Screens & Flows walk, severity, question format) untouched.
- **Consumers assessed:** devils-advocate (mounts it; grades the new table with the same report shape) · specify (binds it; wave-2 rebuild lands the matching stage).

## [v0.53.0] Code-review punt line narrowed — minimalism-lens carve-out
- **Disposition:** superseded → the same When-NOT-to-Use bullet with a parenthetical carve-out naming `mochiko:review-code-minimalism` (implement-side) as the one exception; general code review stays a different domain.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-05 "Ponytail code-minimalism ruled (PT-D1–D10)", record `.mochiko/brainstorms/ponytail-concepts-integration/record.md`, D2 — punt reversal narrow, lens-only).
- **Content (verbatim, the superseded bullet):**
  ```
  - **Code review** - Different skill domain entirely
  ```
- **Kept deliberately:** the punt for everything but the minimalism lens — this skill grades specs, never code; scope unchanged.
- **Consumers assessed:** devils-advocate (mounts it; specify-side scope unchanged) · specify (binds it; no behavior change).

## [v0.50.0] Gained the Screens & Flows prototype-walk grade
- **Disposition:** pure addition riding the decision row (new 8-check section + process step + checklist row + description clause; the process-step renumber 6→7/7→8/8→9 is the only touched existing text)
- **Tier failed:** n/a — addition by ruling (`DECISIONS.md` row 2026-08-02 "UX mocking in specify (UX-D1–D9)"; record `.mochiko/brainstorms/ux-mocking-in-specify/record.md`, D7)
- **Content:** nothing removed from this skill.
- **Consumers assessed:** devils-advocate (mounts it) · specify (binds it).

## [v0.49.0] Absorbed the Delivery Slices grade (from retired review-slices)
- **Disposition:** pure addition riding the decision row (new 11-check section + process step + checklist row + description clause) — recorded here because the absorption is half of a supersession pair (see `review-slices.md`)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D9)
- **Content:** nothing removed from this skill.
- **Consumers assessed:** devils-advocate (mounts it) · specify (binds it).

## [v0.26.0] Question Format → template pointer; What to Avoid deleted; When-to-Use merged; Common Mistakes densified (body 166 → 128, −23%, in-band 10–40)
- **Disposition:** deduped → `templates/advocate-report-template.md` (the **pre-existing**
  Clarifications block holds the exact question shape — Read and confirmed before landing;
  nothing was written to templates/ this wave, so D4's destination ban is not engaged — this is
  R4a dedup credit, not a relocation; the in-body copy had
  drifted: "Why this matters" vs the template's "Why it matters" — the same second-home symptom
  as this skill's v0.13.0 Output-Format strip) · deleted (the What-to-Avoid section — within-file
  triplication with the Core Principle table and When-NOT-to-Use routing; its one non-duplicated
  sentence folded into Core Principle) · deleted (three near-synonym When-to-Use bullets merged
  into one) · densified (form-only): eight Common-Mistakes ❌/✅ subsections → one 3-column table
- **Tier failed:** 1 (Question Format, What to Avoid) · 2 (the merged When-to-Use bullets — no
  distinct trigger per bullet) · n/a for the densification — all ❌/✅ pairs preserved
- **Content:** the fenced question template; the four avoid-bullets + closing line; two trigger
  bullets; Common-Mistakes subsection headers
- **Consumers assessed:** wave-open enumeration — 7 citing files, none reference the stripped
  sections; `mochiko:authoring-requirements` (both remaining pointers) verified to exist
- **Wave-1 reconciliation:** the +11 canonical hunt-taxonomy table (v0.25.0 RETURNED below) sits
  cleanly in Gap Categories — nothing else in the file duplicates it; both Gap-Categories tables
  KEPT as the canonical home `devils-advocate` points at

## [v0.26.0] KEPT: the severity table and Core Principle table
- **Tier-2 evidence:** contested at the wave-2 pass and kept — the severity table carries
  **spec-specific wording** ("Cannot build without this answer" / "Will cause rework") and this
  skill has no references/ tree to relocate to; the Core Principle wrong/right table is the
  skill's unique teaching content, now also carrying the folded altitude sentence. Session
  ruling: batch-2 ratification 2026-07-25.

## [v0.25.0] RETURNED: the five requirement-defect classes landed in Gap Categories (canonical-home landing, +11 lines)
- **Evidence:** wave-1 audit catch (skill-succinctness pass, 2026-07-25) — the `devils-advocate` agent's "What You Hunt For" catalog was stripped with disposition "relocated → this section" (R4b item 2), but the section held only the question-framing taxonomy; the pointer in the agent ("the canonical gap taxonomy … lives in `mochiko:review-specifications`") had been dishonest since before the wave. The five-class table (missing requirements / ambiguities / edge cases / assumption gaps / contradictions) landed here at audit-fix time, making this section the true canonical home. Provenance: `.mochiko/strips/devils-advocate.md`. This is a cross-primitive dedup landing (R4a Tier-1 credit), not a re-add of previously stripped content from this skill.

## [v0.13.0] Output Format block
- **Disposition:** relocated → `templates/advocate-report-template.md` (the report shape's single source; the skill now references it)
- **Tier failed:** 1 (altitude — a second home for the report structure, already drifted: the skill's block lacked the Verdict and What's-Strong sections the template carries)
- **Content:** the fenced `## Gaps Found` markdown block (Critical / Important / Minor buckets with Gap / Question / Options fields)
- **Consumers assessed:** specify only (user-ratified)

## [v0.13.0] Verdict-ownership Related-Skills bullet
- **Disposition:** deduped to the skill's own `description:` field + Overview (both already state input-not-verdict; the `review-*` family boundary is defined in REGISTRY's split note)
- **Tier failed:** 1
- **Content:** "**Verdict ownership** — the severity-bucketed gaps and clarifying questions this skill emits are INPUT to the reviewer/lead, who owns the clearing verdict and drives any revision round. This skill finds and frames gaps; it does not emit a clearing PASS/FAIL of its own."
