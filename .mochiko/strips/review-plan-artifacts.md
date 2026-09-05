# Strip notes — `skills/review-plan-artifacts`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (v0.15.0). Plan-cluster-only
skill (the completeness mirror-checklist, mounted on `devils-advocate`) — strips ruled in-wave. Zero
strips proposed: the skill sits at altitude (matching the specify-wave `review-specifications` finding
after its Output-Format relocation, and the slice-wave `review-slices` zero-strip). Verdict-ownership
is not over-stated (the Verdict Criteria are mechanical; there is no redundant verdict-ownership Related
bullet like the one stripped from `review-specifications`). One contested keep (below). The library-wide
"letter/spirit" aphorism (L16) was **raised, not ruled** here — it recurs across the skill library and
is a library-wide consistency ruling, not a cluster call (see the wave return); note L16 already carries
the `loop-discipline` reference the strip disposition would add.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the review family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/review-plan-artifacts/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action at invoke, before any grading step: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
  action.** The schema is the source of truth for this skill's binding rules; this body carries
  identity and procedure only. Its rules are nested in six sections, each addressable by its
  section ID: `review-plan-artifacts.sec.independence` (author/grader separation) ·
  `review-plan-artifacts.sec.scope` (jurisdiction, routing, and what never shrinks the
  review) · `review-plan-artifacts.sec.inputs` (pre-asserts, checklist bindings, read duties) ·
  `review-plan-artifacts.sec.verdict` (the blocking lenses, the count mapping, the grading
  floors) · `review-plan-artifacts.sec.output` (report contracts) ·
  `review-plan-artifacts.sec.reserved` (decisions this seat never takes).

  Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
  `kind:` reads `constraint`; a rule carrying `when:` binds only where its terms hold against
  the schema's declared `conditions:`, except that a `class: floor` rule is always read and
  always delivered — `when:` gates when its obligation applies, never whether it reaches you.
  Where a rule carries `extends: review-common.<slug>`, the stub inherits `text` / `labels` /
  `pointer` only from `skill-review-common.yaml` — `class` and `kind` are always this schema's
  own, and the stub's `review-plan-artifacts.*` ID stays the citable ID; `${verdict}` in
  inherited text substitutes from this schema's `vars:`. Labels come from
  `../../schemas/skill-labels.yaml`. A `pointer:` rule binds you to that file's or skill's
  content, referenced never restated.

  The schema carries **the 11 rules of `class: floor`**. State the floor count back before the
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
  The schema carries **the 11 rules of `class: floor`**. State the floor count back before the
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
(RPA rows). Every entry below is one move of that conversion. Census-row → minted-ID map: 1 `gap-list-scope` ·
2 `feasibility-handoff` · 3 `not-for` · 4 `conformance-blocking` · 5 `material-divergence-autofail` ·
6 `adopt-first-lens` · 7 `rung-honesty-advisory` · 8 `completeness-supplied-sets` · 9 `tier1-preassert` ·
10 `issue-templates` · 11 `mechanical-verdict` · 12 `report-template` · 13a `cycle-card-checks` ·
13b `cycle-card-check-mirror` · 14 `caller-names-sets` · 15 `incremental-consistency-scope` ·
16 `incremental-read-bound` · 17 `no-prior-waiver` · 18 `escalate-full-reread` · 19 `contradiction-routing` ·
20 `unsure-targeted-review` · 21 `incremental-report` · 22 `default-fail` (stub) · 23a `letter-is-spirit` ·
23b `na-justified` · 24 `critical-blocks` · 25 `never-shrink` · 26 `evidence-floor` (stub) ·
27 `severity-classification` · 28 `verdict-criteria` · 29 `analysis-checklists` · 30 `store-delta-checklists` ·
31 `design-cross-checklists` · 32 `boundary-table` · 33 `tier1-forms-envelope` · (no census row)
`author-grader` (stub — census-deviation addition, C3 binds all 8 members; lead-ruled 2026-09-01).
All IDs carry the `review-plan-artifacts.` prefix.
Structural-vs-content accounting (D8/C1): delivered pair vs pre-conversion body = **1.20×** — the
overage is structural (IDs, keys, grammar) plus the census-ruled additions that had no body text to
relocate: the seven reference-borne stub rules (§B 27–33) and the fresh `author-grader` stub; no
content growth.

### Supersession-transfer — [v0.26.0] KEPT lineage: Red Flags · Common Rationalizations · Incremental Review Mode obligations
- **Disposition:** superseded — protection transfers to schema rules `never-shrink`,
  `letter-is-spirit`, `na-justified`, `critical-blocks`, and the incremental set
  (`incremental-consistency-scope` · `no-prior-waiver` · `escalate-full-reread` ·
  `incremental-read-bound` · `caller-names-sets` · `contradiction-routing` ·
  `unsure-targeted-review`), citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** the Floors clauses "feature size, producer seniority, time pressure, found-enough never
  shrink the review; a vague spec is a gap to flag, not permission to propagate, and 'obvious' never
  exempts a constraint from documentation" · "the letter IS the spirit: never skip a check, never
  downgrade a severity to dodge a finding (rationalizing DOWN means it is probably higher); an
  inapplicable check is flagged N/A with justification, never silently dropped" · "a Critical/Important
  issue blocks — never 'noted but not blocking'"; the Incremental-mode rule clauses of the body's
  **Incremental mode** paragraph, verbatim per the v0.87.0 compressed form.
- **Kept deliberately:** the incremental narrative voice (what the pass IS) stays in the body's
  Procedure prose; only the obligations moved.
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.15.0] KEPT: incremental report-shape block
- **Disposition:** superseded — protection transfers to schema rule `incremental-report`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "The report adds `incremental: true`, `full_review:` / `consistency_only:` scope lists,
  and pass/fail `consistency_checks:` frontmatter (entity_names · schemas · decisions_honored ·
  architecture_conformance · sensitivity_annotations · integration_boundaries); a fail also lands as a
  finding."
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.64.0] review-evidence floor line
- **Disposition:** superseded — protection transfers to schema rule `evidence-floor`
  (`extends: review-common.evidence-floor`, `class: floor` local), citing skill-content-schema D8/C4 +
  `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "verdict and per-finding dispositions land in the reviewed artifacts themselves — review
  evidence only in conversation is a floor violation."
- **Consumers assessed:** the common block `review-common.evidence-floor` is the family home; the
  cross-grammar near-dup edge with command `common.yaml` is allowlist territory (census J-5).

### Supersession-transfer — [v0.67.0] three-lens machinery + material-divergence precedence override + class-7 seam sentence
- **Disposition:** superseded — protection transfers to schema rules `conformance-blocking`,
  `material-divergence-autofail`, `rung-honesty-advisory`, `feasibility-handoff`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "*conformance* — BLOCKING: every named gap closed, within the gap list's depth;
  **material divergence** (an artifact no gap named, or an element class materially past the gap list)
  auto-FAILs the package (critical-gaps), overriding the count mapping" · "*rung-claim honesty* —
  advisory, never drives the verdict: each seat's disclosed ladder stops graded against
  `mochiko:patterns-plan-minimalism` (the standard, never restated; the independent excess hunt is
  `review-feasibility`'s class 7)".
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.53.0] code-review carve-out (`review-code-minimalism` sole exception)
- **Disposition:** superseded — protection transfers to schema rule `not-for`, citing
  skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "Not for: code review (sole carve-out `mochiko:review-code-minimalism`, implement-side) ·
  specs (`review-specifications`) · constitution (`validation-constitution`) · artifacts still being
  drafted."
- **Consumers assessed:** `review-specifications` carries its own copy of the carve-out (its own strip
  entry covers it); allowlist keep-distinct edge per census §C.

### Supersession-transfer — [v0.75.0] oracle-semantics check (ruled wording)
- **Disposition:** superseded — protection transfers to schema rule `cycle-card-checks`, the wording
  carried verbatim inside the rule text, citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "oracle semantics (Asserts graded semantically against the acceptance scenario /
  criteria they cite — the *right* expected behaviour, not merely present and in-grammar)".
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.76.0] two-arm `--check` citation
- **Disposition:** superseded — protection transfers to schema rule `cycle-card-check-mirror`, both
  arms preserved (GI-020), citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "Mirrors the `tasks` `--check` view (`mochiko-cli template tasks --check`, or Read
  `plugins/mochiko/schemas/tasks.yaml` when the binary is absent)."
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.81.0] store-delta re-key incl. the qualifying-flow guard and deployment-view row
- **Disposition:** superseded — protection transfers to schema rule `store-delta-checklists`
  (reference-stub: `pointer: references/ARTIFACT-CHECKLISTS.md`; the checklist tables stay in the
  reference file untouched), citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** the body lens clause "*completeness within scope* — … (analysis · store delta when the
  package carries one, else the no-delta claim · design · cross-artifact), over whichever sets the
  caller supplies"; the kept-verbatim guard "a P1 journey is the floor, never the cap" is named in the
  rule text and remains in ARTIFACT-CHECKLISTS.md.
- **Kept deliberately:** ARTIFACT-CHECKLISTS.md itself — files untouched, stub-default per D3/C2.
- **Consumers assessed:** none — skill-local content.

### Supersession-transfer — [v0.91.0] gap-list floor + BLOCKING strength (plan-stage retirement D4/D5)
- **Disposition:** superseded — protection transfers to schema rules `gap-list-scope` and
  `conformance-blocking`, citing skill-content-schema D8/C4 + `DECISIONS.md` 2026-09-01.
- **Tier failed:** n/a — supersession by ruling.
- **Content:** "The run's floor is **the sufficiency report's gap list** (the named gaps the design
  phase was scoped to close), never a fixed artifact set."
- **Consumers assessed:** none — skill-local content.

### Relocation — remaining body obligations → schema rules (non-protected clauses)
- **Disposition:** relocated → `schema.yaml` rules `feasibility-handoff` (sibling split sentence),
  `completeness-supplied-sets`, `tier1-preassert`, `issue-templates`, `mechanical-verdict`,
  `report-template`, `adopt-first-lens`, `cycle-card-checks` (the six non-oracle checks),
  `severity-classification`, `verdict-criteria`, `analysis-checklists`, `design-cross-checklists`,
  `boundary-table`, `tier1-forms-envelope`.
- **Tier failed:** n/a — ruled conversion (skill-content-schema D3, obligations-only boundary).
- **Content:** the Protocol paragraph's obligation clauses ("Tier-1 pre-assert first — `python
  scripts/check-artifacts.py .mochiko/specs/<feature>/<artifact>.md …` — a `failed` count is ground
  truth, folded straight into the issue list → run every applicable check → classify and shape issues
  per ISSUE-TEMPLATES.md → verdict mechanically from the counts (its Verdict Criteria), subject to the
  divergence override → report per `mochiko:advocate-report-template`: evidence and an actionable fix
  per issue, the one-line `strengths:` field filled"); the adopt-first lens clause; the cycle-card
  check-set clauses (vertical integrity · `**TEST:**` gate · story traceability · dependency
  minimality · brownfield exposure stated · no task lists or file paths); the reference-frame
  obligations of ARTIFACT-CHECKLISTS.md / ISSUE-TEMPLATES.md now stubbed by pointer (files untouched).
- **Consumers assessed:** none — skill-local content.

### Relocation — default-FAIL floor line → common stub under strongest-wording-wins
- **Disposition:** relocated → `default-fail` (`extends: review-common.default-fail`, `class: floor`
  local, `${verdict}` = `ready`); the member tail superseded by the common block's strongest wording
  per near-dup convergence R2 (`.mochiko/decisions/2026-08-28-near-dup-convergence.md`).
- **Tier failed:** n/a — ruled conversion (skill-content-schema D5).
- **Content:** "defaults to FAIL — good enough is never ready: evidence or rejection".
- **Kept deliberately:** nothing of the tail needs a local rule — "evidence or rejection" is the
  default-posture obligation the common text states as "earned only by a completed hunt; absence of
  looking is never evidence".
- **Consumers assessed:** family common block; allowlist edge vs command `common.yaml` (census J-5).

## [v0.91.0] Fix round 4 — the four sibling "Plan Artifact" sites re-titled with the H1; slug still retained — plan-stage retirement D1/D5

- **Disposition:** superseded in four places across the skill's reference and script files, closing
  the family the entry below left open:
  - `references/ARTIFACT-CHECKLISTS.md` H1 → `# Design-Phase Artifact Review Checklists`
  - `references/ISSUE-TEMPLATES.md` intro prose → "the working report shape for design-phase
    artifact reviews"
  - `references/ISSUE-TEMPLATES.md` emitted report-block header →
    `## Design-Phase Artifact Review: {artifact set reviewed}`
  - `scripts/check-artifacts.py` module docstring → `Design-Phase Artifact Validation Script
    (Tier-1 deterministic pre-assert)`
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1/D5). Raised by this seat as a
  five-site expansion of the H1 question rather than edited unilaterally; **ruled by the wave lead
  2026-08-26** — "take ALL FIVE, slug untouched", one strip entry covering the family, with the
  report-block header called out as the point of the ruling.
- **Content (superseded text, verbatim):**

  ```
  # Plan Artifact Review Checklists

  Issue-documentation formats, severity classification rules, and the working report shape for plan
  artifact reviews.

  ## Plan Artifact Review: {artifact set reviewed}

  Plan Artifact Validation Script (Tier-1 deterministic pre-assert)
  ```

- **Why the report-block header carried the ruling:** the other three name the artifact set in
  maintainer-facing prose, but this one is written into the reviewer's own emitted report. Left
  alone, the retired stage name would surface in user-visible output on every design-phase review
  the skill grades — a dead stage announcing itself to the reader who is furthest from this repo.
- **Sixth site, self-ruled under the wave brief and declared to the lead:**
  `references/ARTIFACT-CHECKLISTS.md`'s scope paragraph read "The caller (the plan lead) supplies
  which artifacts are in scope for a given review" → "The caller (the implement run's lead)". Not
  in the lead's five; caught by this round's whitespace-collapsing sweep of the whole skill
  directory and re-keyed as the same class of stage-vocabulary residue the wave exists to retire.
  The reviewer's scope now comes from the run that dispatches it, and no plan lead survives to
  supply anything.
- **Kept deliberately:** the slug `review-plan-artifacts` (the reasoning is in the entry below and
  is unchanged — the mounts and index rows that address this skill by path all still resolve); the
  checklists themselves, the severity table, the Tier-1 protocol, and the `advocate-report-template`
  hand-off, which remains the assembled deliverable this file must not restate.
- **Consumers assessed:** `plugins/mochiko/agents/devils-advocate.md` mounts the skill by slug —
  unaffected. No primitive, template, or script quotes the four superseded strings; the report-block
  header is produced by the reviewer at runtime and read by a human, never parsed. `check-artifacts.py`
  re-verified with `python3 -m py_compile` after the docstring edit (PASS; the generated
  `__pycache__/` was removed).
- **Budget:** `SKILL.md` untouched this round — body stays **4,938** against 6,127, description
  **598** against 625. Reference and script files are budget-exempt per D7.

## [v0.91.0] Fix round 3 — H1 re-titled "Reviewing Design-Phase Artifacts"; slug retained — plan-stage retirement D1/D5

- **Disposition:** superseded → `# Reviewing Design-Phase Artifacts`. The directory slug
  `review-plan-artifacts` is **deliberately retained**.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1/D5). Raised by this seat as an
  explicit judgment call rather than edited unilaterally; **ruled by the wave lead 2026-08-26**,
  who directed the re-title and named the `authoring-technical-requirements` precedent as
  governing — that skill likewise kept its slug this wave while its H1 was re-titled to state the
  new scope.
- **Content (superseded text, verbatim):**

  ```
  # Reviewing Plan Artifacts
  ```

- **Why the slug stays:** renaming the directory would break the `devils-advocate` persona's
  `skills:` mount, the router's index row, `mochiko:review-feasibility`'s sibling-boundary
  pointer, `mochiko:patterns-plan-minimalism`'s grading pointer, and
  `mochiko:patterns-adopt-first`'s Who-grades-what row. The H1 states the scope; the slug is an
  address, and addresses that resolve are worth more than addresses that read well.
- **Kept deliberately:** everything else in the skill — the design-phase output package as the
  graded object, the sufficiency gap list as the run's floor, the four lenses with conformance
  BLOCKING, the Tier-1 protocol, the cycle-card check set, Incremental mode, and every Floors
  clause.
- **Budget:** body **4,938** against the 6,127 budget; description unchanged at 598 against 625.
  Both inside. (Canonical-snippet count taken after the edit.)
- **Four sibling sites were left standing at this entry and reported to the wave lead** — the same
  retired stage name in `references/ARTIFACT-CHECKLISTS.md`'s H1, twice in
  `references/ISSUE-TEMPLATES.md` (its intro line and the emitted report-block header), and in
  `scripts/check-artifacts.py`'s docstring. **Ruled and closed the same day:** the lead took all
  five, slug untouched — see the Fix round 4 entry above, which supersedes this bullet. Their
  survival across this entry was pending a ruling, never an oversight.

## [v0.91.0] Fix round — cycle-card check set: "cited spec/plan IDs real" → "spec/design IDs" (advisory)

- **Disposition:** superseded → "cited spec/design IDs real" in the story-traceability check.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1). Caught on the fix round's own
  re-sweep: earlier passes matched "plan artifacts" and "plan package" but not the compact
  "spec/plan" form, so this and three sibling sites survived two sweeps.
- **Content (superseded fragment, verbatim):**

  ```
  story traceability (every P1/P2 story on ≥1 card; Simple/Split/Merge case + rationale; cited spec/plan IDs real) ·
  ```

- **Kept deliberately:** the story-traceability check entire — every P1/P2 story on at least one
  card, the Simple/Split/Merge case with its rationale, and the requirement that cited IDs
  actually **resolve**. Only the artifact class the IDs are resolved against is renamed.
- **Budget:** body 4,928 → **4,930** against the 6,127 budget; description unchanged at 598
  against 625. Both inside.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` authors the cards this check grades and
  carries the matching "spec/design ID(s)" checklist line (main pass);
  `mochiko:executing-tdd-cycle` and its `references/TASK-PARSING.md` parse the same `Covers`
  lines and were re-keyed in this round — author, parser, and grader now agree.

## [v0.91.0] `references/ARTIFACT-CHECKLISTS.md` + `scripts/check-artifacts.py`: the `requirements.md` checklist and its Tier-1 rule die — plan-stage retirement D3/D4

- **Disposition:** the Technical Requirements checklist is **deleted** (its artifact no longer
  exists); the Tier-1 checker's `requirements.md` required-sections rule is **deleted**; four
  further sites superseded by re-key.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D3** ("No per-feature
  `requirements.md`") and **D4** ("`plan.md` (the summary artifact) dies"; the `quickstart.md`
  null-path record moves to the sufficiency report)). Reference and script scope was opened by
  the wave lead's extension ruling of 2026-08-26.
- **Scope note, recorded because it exceeds the two lines the extension named:** the lead's
  extension named `:88`/`:128` (design-phase wording). The rest below is the same TR-death and
  `plan.md`-death cluster in the same primitive, and leaving it would have been worse than
  cosmetic — the checker would have **demanded a deleted artifact** at Tier-1 pre-assert, and
  the completeness lens would have sent a reviewer to grade a file that cannot exist. Reported
  to the lead with the finished work.
- **Content (superseded/deleted, verbatim):**

  1. Analysis Artifacts lead-in + the whole Technical Requirements checklist:

     ```
     Grade these when reviewing the analysis output set (e.g. `requirements.md`,
     `constraints-and-decisions.md`). NFR-XXX rows are **not** graded here — their home is the
     architecture store's concern rows, graded in the store-delta section below.

     ### Checklist — Technical Requirements (`requirements.md`)

     | Check | Question | Severity |
     |-------|----------|----------|
     | FR coverage | Is every functional requirement from the spec mapped to at least one TR? | Critical |
     | Orphan TRs | Are there technical requirements with no business source? | Critical |
     | Testable criteria | Does every TR have measurable acceptance criteria? | Critical |
     | Dependency references | Do TRs reference relevant constraints and NFRs? | Important |
     | Priority assignment | Are TR priorities consistent with source FR priorities? | Important |
     | RFC 2119 language | Do requirements use MUST/SHOULD/MAY consistently? | Minor |
     ```
  2. Store-delta section lead-in: `Grade this when the plan package carries a **store delta**`
  3. NFR-targets check: `The `TR-XXX → NFR-XXX` chain survives D12 — only the path moved, so an NFR with no traceable source is the same finding it always was.`
  4. No-delta runs: `A plan run that judges the feature non-structural authors **no delta**. Grade the claim:`
  5. Quickstart conditionality check: `If none exists, is its null path recorded in `plan.md` (no stub file)?`
  6. Tier-1 usage example: `python scripts/check-artifacts.py .mochiko/specs/<feature>/requirements.md .mochiko/specs/<feature>/data-model.md` and the `# All plan artifacts` comment
  7. `scripts/check-artifacts.py`, the `REQUIRED_SECTIONS` map:

     ```python
         'requirements.md': [
             '## Technical Requirements',
         ],
     ```
  8. Added on the exhaustive final sweep, same file and same ruling — the scope-boundary
     section's seam line: `This is the explicit seam between the two plan reviewers, so the
     feasibility side can mirror it` → "the two design-phase reviewers". The seam itself (this
     skill keeps the left column, `mochiko:review-feasibility` owns the right) is untouched;
     both halves were re-scoped to the design phase earlier in this wave, so the seam's naming
     now matches both sides.

- **Kept deliberately:** the **Constraints and Decisions checklist entire** — every one of its
  ten checks survives, including sourced-constraints, alternative-analysis, rationale-quality,
  the C↔D cross-refs, and both IP-coverage checks. The NFR-targets check survives with its
  Critical severity and its no-traceable-source finding intact; only the chain's upper id moved
  (per ruling R4). The no-delta claim check, the quickstart conditionality check, and the whole
  cross-artifact and cycle-card sets are untouched. Where the deleted TR checklist carried a
  real obligation with a surviving owner, the replacement text **names that owner** rather than
  dropping it silently: testable-criteria coverage is now graded upstream at the sufficiency
  check's clause 1, and a line in the file says so.
- **Verification:** `python3 -m py_compile` on the edited checker passes; the two residual
  "requirements" hits in the script are a generic section-name word list and the surviving
  FR-XXX id pattern, neither affected.
- **Budget:** `references/` and `scripts/` files are both budget-exempt
  (`.mochiko/memory/primitive-cost-budgets.md`).
- **Consumers assessed:** the skill body's completeness lens names the checklist set ("analysis ·
  store delta … · design · cross-artifact") — the *analysis* set now holds one checklist rather
  than two, which the lens's wording already accommodates without an edit. The Tier-1 protocol
  line in the skill body names the script generically, so it needed no change.

## [v0.91.0] `description:` re-scoped — the graded object is the design-phase package, the floor is the sufficiency gap list — plan-stage retirement D1/D5

- **Disposition:** superseded → the same description grading the design-phase output package
  against the sufficiency report's gap list.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D5: "`review-plan-artifacts` and
  `review-feasibility` re-scope at build time to grade the design-phase output"; D4 records the
  death of plan's proposal-approval gate, which was this skill's former floor).
- **Content (superseded text, verbatim):**

  ```
  This skill MUST be invoked to grade a plan package against the approved artifact proposal — conformance (every proposed artifact present, nothing materially past approved depth; material divergence auto-FAILs — BLOCKING) and honesty of disclosed rung claims
  ```

- **Budget:** description-class edit, canonical snippet: **589 → 598 chars** against the
  recorded budget of **625**. Inside budget; no justification owed.
- **Kept deliberately:** conformance stays **BLOCKING** with the material-divergence auto-FAIL
  intact, the rung-honesty lane stays advisory, the completeness scope (coverage, measurability,
  cycle-card quality, consistency) is untouched, the 3-state verdict is unchanged, and the
  feasibility carve-out, default-FAIL posture, and independent-validator rule all survive
  verbatim.
- **Consumers assessed:** the router row (re-keyed same wave), `mochiko:patterns-plan-minimalism`
  (names this skill as its review grader — re-keyed same wave to "gap-list conformance
  blocking"), `mochiko:patterns-adopt-first`'s Who-grades-what table (names this skill for the
  disclosure line at conformance strength — that strength is unchanged, so the pointer holds),
  `mochiko:review-feasibility`'s sibling-boundary line (unchanged).

## [v0.91.0] Body floor moved from the approved artifact proposal to the sufficiency gap list — plan-stage retirement D4/D5

- **Disposition:** superseded → the sufficiency report's gap list as the run's floor.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D4: "Dead gates: plan's
  plan-the-plan proposal approval and package acceptance"; D2: "Any gap → the design phase
  authors exactly those gaps, nothing else").
- **Content (superseded fragments, verbatim):**

  1. Opening paragraph:

     ```
     Independent completeness grader of a plan package — the mirror-checklist half of the plan-review pair, never the author. The run's floor is **the approved artifact proposal** (the lead's rung-justified, user-approved proposal), never a fixed artifact set.
     ```
  2. Lenses, the conformance lens:

     ```
     **Lenses:** *conformance* — BLOCKING: every proposed artifact present, within approved depth; **material divergence** (an unproposed artifact, or an element class materially past approved depth) auto-FAILs the package (critical-gaps), overriding the count mapping
     ```

- **Budget:** body 4,901 → 4,928 chars against the recorded budget of **6,127**. Inside budget.
- **Kept deliberately:** the never-a-fixed-artifact-set principle (the floor is still a
  run-specific list, only its source moved from the approved proposal to the gap list), the
  BLOCKING strength and the auto-FAIL override of the count mapping, the adopt-first disclosure
  lens, the rung-claim-honesty lens, the completeness-within-scope lens and its mirror
  checklists, the Tier-1 pre-assert protocol, the whole cycle-card check set, Incremental mode
  in full, and every Floors clause. This skill was true-deletion cut at v0.87.0; nothing from
  that keep-set was touched here.
- **Consumers assessed:** `references/ARTIFACT-CHECKLISTS.md` still carries plan-run wording at
  two sites ("the plan package carries a store delta", "A plan run that judges the feature
  non-structural") — **out of this seat's writable scope, reported to the wave lead as an open
  ripple**, not silently left. `implement.md` (P1's rewrite dispatches this grade after the
  design phase).

## [v0.87.0] User-ruled true-deletion body cut — body 13,521 → 4,901 chars (−63.8%)

- **Disposition:** superseded → a single-file six-paragraph body (identity+boundary · Lenses ·
  Protocol · Cycle cards · Incremental mode · Floors) — true deletion, no relocation, no new
  reference file; both `references/` files, `scripts/check-artifacts.py`, and the
  `description:` (589 chars) untouched. Every behavioral rule of the baseline body survives
  as a compressed clause, or was deleted only where its single source already lives in an
  untouched `references/` file (the mirror-checklist tables). The user targeted −90% (the
  `review-brainstorm` v0.83.0 precedent) and ruled **ship the rule-complete cut** at the
  ratification gate with the trade named: −82% required deleting Incremental Review Mode
  (doubly KEPT), the cycle-card qualifiers, and the Floors' severity-discipline rules
  (~25 rules); strict −90% (≤1,352) additionally deletes the cycle-card set (leaving a dead
  pointer in ARTIFACT-CHECKLISTS.md, which punts to that body row) and the adopt-first lens
  (consumer-cited BLOCKING by `patterns-adopt-first`) — declined.
- **Tier failed:** n/a — supersession by ruling (in-session user ruling 2026-08-26 at the
  `compressing-skills` ratification gate, on the v0.82.0/v0.83.0 "cut now, eval validates
  later" precedent; ADR
  `.mochiko/decisions/2026-08-26-review-plan-artifacts-true-deletion-cut.md`; `DECISIONS.md`
  2026-08-26 row). Evidence carried per the ceremony: the 113-entry rule inventory
  `evals/review-plan-artifacts/rules.json` (non-compressor-authored) and the span-by-span
  disposition map in `evals/review-plan-artifacts/pass-report.md`.
- **Disposition map (baseline body section → new home; verbatim home for all removed text:
  git history of this SKILL.md, pre-v0.87.0):**
  - *Overview (three-lens prose, letter/spirit paragraph, review-evidence floor line)* —
    compressed into the opening paragraph, Lenses, and Floors (the L16 aphorism and the
    v0.64.0 floor line survive compressed — wording superseded, substance intact).
  - *Scope table* — compressed to the sibling-boundary sentence + the ARTIFACT-CHECKLISTS
    boundary-table pointer (the check-by-check seam's single source, untouched).
  - *When NOT to Use (5 bullets)* — compressed to the "Not for:" line; all four route
    targets and the wait-for-completion condition survive, incl. the v0.53.0 carve-out.
  - *Review Focus by Artifact Type table* — the analysis / store-delta / design /
    cross-artifact rows supersede to their single-source checklists in
    `references/ARTIFACT-CHECKLISTS.md` (every Key-checks token verified present there —
    pass-report map); the **cycle-cards row survives in the body, complete** (the reference
    explicitly punts to it), with the v0.75.0 oracle-semantics wording and the v0.76.0
    two-arm `--check` citation intact; the brownfield-discovery out-of-scope note stays
    single-sourced in the reference; sequencing-is-the-lead's survives as "over whichever
    sets the caller supplies".
  - *Issue Classification + Verdict Criteria sections* — compressed into Protocol: the
    ISSUE-TEMPLATES pointers, mechanical count-derived verdict, the material-divergence
    precedence override, and rung-honesty-never-drives-the-verdict all survive as clauses.
  - *Review Process / Step 2 pre-assert* — survives as Protocol leg 1: verbatim command
    line, failed-count-is-ground-truth, fold-in-before-judging.
  - *Incremental Review Mode (3 subsections + report-shape yaml)* — compressed to one
    paragraph: caller-names-the-sets, full-vs-consistency-only split, spot-check targets,
    flag-only-between-artifacts, no-full-re-read, the 1–2-minute budget, all three
    escalation rules (2+ issues → full re-read; contradiction → lead routes with the
    design-vs-decided / requirements-constraints split; unsure → targeted review), and the
    report-shape fields (`incremental:`, `full_review:`/`consistency_only:`, the six
    `consistency_checks:` fields, fail-also-lands-as-finding).
  - *Quality Checklist (15 items)* — each item's rule survives in Lenses / Protocol /
    Floors (itemized in the pass-report map); the checklist-as-section form dies.
  - *Common Mistakes (6 rows) + Red Flags (6 bullets) + Common Rationalizations (7 rows)* —
    compressed into Floors (default-FAIL / good-enough-is-never-ready, letter-IS-the-spirit,
    never-skip / N/A-with-justification, severity-down rationalization,
    noted-but-not-blocking, size/seniority/time-pressure/found-enough,
    vague-spec propagation, obvious-never-exempts-documentation); rows whose single source
    pre-exists in ISSUE-TEMPLATES (severity inflation, implementation focus, missing
    evidence, vague suggestions) ride that untouched home.
  - *Related (3 bullets)* — inline pointers: `review-feasibility` (opening),
    `patterns-plan-minimalism` (Lenses), `advocate-report-template` (Protocol).
- **MANDATORY KEPT reconciliation:** [v0.15.0] KEPT report-shape block — survives
  compressed (fields as clauses; the yaml example's form superseded by this ruling).
  [v0.26.0] KEPT Red Flags / Common Rationalizations / Incremental Review Mode — survive
  compressed into Floors and the Incremental paragraph; the KEPT status of their *long
  form* ends by this ruling. [v0.64.0] guardrails keep-set — every member's obligation
  survives per the map above; the review-evidence floor line's verbatim wording superseded,
  substance intact. [v0.67.0] three-lens machinery + precedence override + hunt-class-7
  seam — survives compressed. [v0.53.0] carve-out — survives. [v0.75.0] oracle semantics —
  survives with ruled wording; no time anchor or foundation word reintroduced. [v0.76.0]
  `--check` citation — survives, both arms. [v0.81.0] store-delta re-key — survives via
  the completeness lens (delta-when-carried / no-delta claim); detailed checks stay
  single-sourced in the untouched reference; the kept-verbatim qualifying-flow guard and
  deployment-view rows live there, untouched.
- **Consumers assessed:** router `skills/mochiko/SKILL.md:89` (conformance / material
  divergence auto-FAIL / rung honesty advisory / completeness within scope / store delta /
  no-delta / cycle cards / 3-state verdict / mirror-checklist half / independent — every
  cited term survives) · `agents/devils-advocate.md` (mounts the skill; plan sets +
  cycle-cards role intact) · `patterns-adopt-first` (BLOCKING sibling check survives) ·
  `review-code-minimalism` (rung-honesty sibling pointer survives) · `review-feasibility`
  (boundary vocabulary survives; its side untouched) ·
  `authoring-technical-requirements/references/TRACEABILITY-PATTERNS.md` (cross-artifact
  consistency grade owned here — survives) · `review-brainstorm/references/EXTERNAL-CLAIMS.md`
  (disclosure-line check lives in the untouched ARTIFACT-CHECKLISTS consistency checklist) ·
  `templates/advocate-report-template.md` (incremental fields — survive). `commands/plan.md`
  names neither the skill nor its section anchors.
- **Reference-to-body pointer re-labels (audit round 1, blocking — ride this same ruling):**
  the cut initially left three stale reference→body pointers, and this entry falsely claimed
  "No dead pointers created" — corrected here. Repaired in the same landing, in the
  reference files' label text only (no check content touched):
  `ARTIFACT-CHECKLISTS.md:9` "(see SKILL.md → Verdict Criteria)" → "(see ISSUE-TEMPLATES.md
  → Verdict Criteria)" (the actual single source); `:18`/`:22` "the Review Focus row in
  SKILL.md" / "the SKILL.md row is the complete check set" → "the Cycle cards check set in
  SKILL.md" / "the SKILL.md paragraph is the complete check set" (the table died, the check
  set survives as a paragraph); `:219` and `ISSUE-TEMPLATES.md:135` "SKILL.md → Incremental
  Review Mode" → "SKILL.md → Incremental mode" (the section's new name). After these
  re-labels: no dead pointers.

## [v0.81.0] Architecture checklist re-written to store-delta grammar — product-architecture-schema D3/D10/D12/D14

- **Disposition:** superseded → the `## Architecture Store Delta` section of
  `references/ARTIFACT-CHECKLISTS.md`, which grades the plan package's **drafted store delta**
  (topology + `AX-XXX` concern-row changes) against the standing store at
  `.mochiko/product/architecture/`. The per-feature `architecture.md` artifact this section
  graded no longer exists (D3).
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3 (per-feature artifact dies) ·
  D10 (consult contract, delta lifecycle, S13 no-delta claim) · D12 (`nfrs.md` absorbed,
  structural `D-XXX` die into store deltas) · D14 (floor precedence); `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — the whole `## Architecture Artifact` section):**

  ```
  ## Architecture Artifact

  Grade this when reviewing the design-time architecture (`architecture.md`) — the container-level
  topology + current→target delta the detailed design conforms to (authored by
  `mochiko:patterns-system-design`, upstream of the design set).

  ### Checklist — Architecture (`architecture.md`)

  | Check | Question | Severity |
  |-------|----------|----------|
  | Component-diagram coverage | Does every component-table entry appear in the container diagram, and every diagram box in the table? | Critical |
  | Qualifying-flow coverage | Does every **qualifying flow** — any flow crossing ≥2 components with non-trivial ordering or failure semantics (user journey *or* system flow) — have a sequence diagram? | Critical |
  | Delta-summary links | Does the delta summary link each structural change to a `D-XXX` row (link, not restatement)? | Important |
  | Status annotation | Is every component marked new / modified / existing? | Important |
  | Baseline present | Is the current-state baseline present — seeded from `ARCHITECTURE.md`, or reconstructed-and-confirmed with a confidence note, or greenfield-empty? | Important |
  | No-delta form | If the feature is no-delta, does it still present the reseeded diagram + the one-line no-structural-change claim? | Important |
  | Deployment-view conditionality | If `IP-XXX` rows exist, is the deployment view present? If none, is its absence recorded (not a stub)? | Minor |

  ### Key Questions — Architecture

  - Is there a component in the table with no box in the diagram, or a box with no table row?
  - Is there a multi-component flow with real ordering or failure semantics and no sequence diagram?
  - Does every structural change in the delta summary point at a `D-XXX` row?
  - Was the baseline actually seeded or confirmed, or silently assumed?
  ```

  Per-check disposition: *Component-diagram coverage* → **Delta-diagram coverage** (same
  bidirectional test, keyed to the delta's element list). *Delta-summary links* → **Ruling
  carried** — D12 sends structural `D-XXX` into the store delta, so the store ruling **is** the
  decision record and no back-link is owed. *Status annotation* (new / modified / existing) →
  **Lifecycle status correctness**, keyed to the D10 vocabulary (`in-flight` / `modifying` /
  `removing (FEAT-XXX)`). *Baseline present* re-keyed from `ARCHITECTURE.md` seeding to the
  standing store, gaining the D16 bootstrap-empty arm. *No-delta form* → the **No-delta runs**
  sub-block, which keeps the claim and drops the reseeded-diagram limb (a no-delta run authors
  no diagram; the store carries the standing one).
- **Kept deliberately:** the **qualifying-flow coverage** check verbatim, together with its
  keyed-to-ordering-not-P1 guard note ("a P1 journey is the **floor, never the cap**") — the
  protected line the check exists for; **deployment-view conditionality** verbatim; the
  feasibility-handoff note verbatim (topology feasibility + governance conformance stay
  `review-feasibility`'s architecture pass).
- **Consumers assessed:** `mochiko:review-feasibility` — its architecture pass and the mirrored
  boundary table re-key in the same wave (P4, this seat); the two halves are re-stated in
  lockstep. `mochiko:patterns-system-design` (P3) authors the graded artifact — its transform
  lands the same wave. `plan.md` (P2) dispatches this review. No other consumer names the
  Architecture checklist (grep clean).

## [v0.81.0] NFR checklist retired from the analysis set — nfrs.md absorbed into the store (D12)

- **Disposition:** superseded → relocated into the **Architecture Store Delta** checklist as the
  *NFR targets on touched rows* check (Critical). `nfrs.md` dies as a file; `NFR-XXX` ids survive
  homed on the store's concern rows, so the target / method / **source** grade travels with the
  row that carries them and fires as part of the store-delta grade instead of a standalone
  artifact pass.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12, `Contested` — user ruled
  absorb against the lead's coexist recommendation; `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim):**

  ```
  ### Checklist — NFRs (`nfrs.md`)

  | Check | Question | Severity |
  |-------|----------|----------|
  | NFR measurability | Does every NFR have a specific, measurable target? | Critical |
  | NFR measurement method | Is the measurement approach defined? | Critical |
  | NFR source tracing | Do NFR sources trace to valid TRs or business requirements? | Important |
  | Category coverage | Are all relevant quality categories addressed? | Important |

  > Whether the design can **meet** an NFR target, and whether NFR targets **conflict** with
  > constraints or with each other, are feasibility concerns → `mochiko:review-feasibility`.
  > This checklist grades only that targets are present, measurable, and have a defined measurement
  > method.
  ```

  Also superseded in the same edit: the Analysis-Artifacts scope line's `nfrs.md` mention (now
  pointing at the store section); the Key-Questions-Analysis bullet "Can each NFR target actually
  be measured with available tooling?" (folded into the relocated check); and the
  incremental-mode `consistency_only` list in SKILL.md, whose third element was `nfrs.md`.
- **Kept deliberately:** the measurability + measurement-method + source grade itself (relocated,
  not dropped) and the feasibility boundary it carried — "can the design **meet** the target" stays
  `review-feasibility`'s. `IP-NFR coverage` stays in the constraints checklist: `IP-XXX` rows
  remain in `constraints-and-decisions.md` (D12 reduces that artifact, it does not kill it).
  **NFR source tracing is kept and relocated** into the store-delta check ("a numeric target, a
  measurement method, and a **source**"): D12 moves the path, not the `TR-XXX → NFR-XXX` chain,
  so an NFR with no traceable source is the same finding it always was. *Category coverage* is
  the one check genuinely not restated here — the breadth guarantee is the shelf walk's (D5
  breadth invariant: every dimension walked, never silently skipped), which is stronger than a
  reviewer's after-the-fact category sweep.
- **CORRECTION (v0.81.0 V4 audit, B5 — ruled).** This entry originally justified dropping *NFR
  source tracing* by claiming it "rides the store row's own required-core shape
  (`plugins/mochiko/schemas/architecture-store.yaml`)". **That claim was false and is retracted.**
  The schema's `Targets` field holds "NFR-XXX ids and their measurable targets" and requires no
  source; its `--check` view grades the skeleton only and never the free-form body. The claim was
  asserted without reading the schema — the exact failure the preserved-responsibilities check
  exists to catch, and it would have silently dropped a Critical traceability grade. The check is
  restored to the store-delta checklist and to its Key Question, and the false justification is
  deleted above rather than left standing beside the correction.
- **Consumers assessed:** `mochiko:authoring-technical-requirements` (NFR grammar home, re-keyed
  the same wave, this seat) · `mochiko:testing-gap-finding` (runtime-NFR probes re-pointed, this
  seat) · `mochiko:review-feasibility` (NFR↔topology lens reads both sides in the store, this
  seat) · router design-surface paragraph (this seat). `plan.md` / `implement.md` baseline lists
  are P2's in the same wave.

## [v0.81.0] Dead `nfrs.md` entry deleted from the Tier-1 checker's REQUIRED_SECTIONS — product-architecture-schema D12

- **Disposition:** deleted. `scripts/check-artifacts.py` keyed a required-sections rule to a file
  D12 abolished. The map is keyed by filename, so with no `nfrs.md` on disk the entry could never
  match and produced no false failure — it was dead config a reader would nonetheless trust.
  Nothing replaces it: the NFR grade moved to the store-delta checklist (see the NFR-checklist
  entry above), and the store is not a `.md` artifact this checker reads.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12; `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim):**

  ```python
      'nfrs.md': [
          '## Non-Functional Requirements',
      ],
  ```
- **Kept deliberately:** every other `REQUIRED_SECTIONS` key —
  `constraints-and-decisions.md` (D12 reduces that artifact, it does not kill it),
  `requirements.md`, and `data-model.md` — plus all four checks (`unresolved_markers` ·
  `required_sections` · `traceability` · `pii_markers`), the entity-consistency cross-check, and
  the exit-code contract. The checker's role as the Tier-1 pre-assert whose `failed` count is
  ground truth is untouched.
- **Verified after the edit:** the module parses (`ast.parse`) and a smoke run against a
  `requirements.md` fixture returns `3 passed / 0 failed`, exit 0 — behavior unchanged for every
  surviving key.
- **Consumers assessed:** the invocation is documented in this skill's SKILL.md Step 2 and three
  times in `references/ARTIFACT-CHECKLISTS.md` → Automated Validation; none of the four enumerates
  `nfrs.md` in a command line (all use `<artifact>.md` or a glob), so no call site changed. The
  Automated-check-coverage table lists checks, not filenames — unaffected. No other primitive
  invokes this script (grep clean).

## [v0.81.0] Review-Focus rows, conformance rows, and boundary row re-keyed to the store

- **Disposition:** superseded → the same rows, keyed on the signed store delta.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3/D10/D12; `DECISIONS.md`
  2026-08-19).
- **Content (superseded, verbatim — four fragments):**

  ```
  | **Analysis artifacts** | requirements, constraints/decisions, NFRs | FR→TR coverage, orphan TRs, testable criteria, sourced constraints, ≥2 alternatives + rationale, NFR measurability, IP coverage |
  | **Architecture** | architecture.md (design-time topology + delta) | component-table↔diagram coverage, qualifying-flow sequence coverage, delta-summary D-XXX links, component status annotations |
  ```

  ```
    architecture_conformance: pass   # data-model/contracts conform to the approved architecture
  ```

  ```
  | Architecture conformance | Do `data-model.md` and contracts conform to the approved architecture — no entity or endpoint implying a component the architecture does not declare? | Critical |
  ```

  ```
  **Architecture Conformance**
  - Every entity's owning component and every endpoint's serving component is one the approved
    `architecture.md` declares — a `data-model.md` entity or a contract endpoint that implies a
    component the architecture never drew is a Critical consistency failure here (the design
    introduced structure the approved shape did not carry).
  ```

  ```
  | **Architecture** (topology artifact) | component-table↔diagram coverage; qualifying-flow sequence coverage; delta-summary D-XXX links; component status; **data-model / contracts conform to the approved architecture** | **topology feasibility (can the proposed topology meet the NFRs / be built under the constraints?); governance conformance (layer rules honored, dependencies within the allowlist, GI-linked principles satisfiable by the topology)** — its **architecture pass** |
  ```
- **Kept deliberately:** the conformance test's substance — a design element implying a component
  the approved shape never carried is still a Critical consistency failure; only the approved
  shape's name and home moved (`architecture.md` → the signed store delta over the standing
  spine). The named-consistency-group's second and third bullets (no new cross-component
  interaction; topology feasibility is `review-feasibility`'s) are untouched. The boundary
  table's right column keeps every feasibility item and **gains** the floor-precedence limb
  (D14 leg 1) as a pure addition.
- **Consumers assessed:** the boundary table is mirrored verbatim-in-substance by
  `mochiko:review-feasibility`'s *The boundary* section — both sides re-keyed in this same edit
  set by this seat, so the mirror holds. No command restates these rows (grep clean).

## [v0.76.0] Cycle-card Review Focus row cites the `tasks --check` view (D7 re-key, thin) — schema-based-template-guidance D7/D8
- **Disposition:** superseded → the Cycle cards row's Key-checks cell now cites `mochiko-cli template tasks --check` (or Read `plugins/mochiko/schemas/tasks.yaml` raw) as the source its cycle-card criteria mirror. THIN scope (contest-accepted per plan §5): only the tasks in-scope-template checklist is re-keyed; `references/ARTIFACT-CHECKLISTS.md` and all out-of-scope artifact checklists (requirements / constraints / nfrs / data-model / contracts / quickstart / architecture — their templates are in-skill refs, D3 leaves them `.md`) are left untouched.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D7 (`--check` grading view, checklists re-key)/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):** the Cycle cards row previously ended without a checklist-source citation —
  - `… brownfield exposure stated (\`none\` counts), **no task lists or file paths** (pre-written decomposition is a finding — the builder decomposes at build time) |`
- **Kept deliberately:** every existing cycle-card criterion (vertical integrity · `**TEST:**` gate · story traceability · sizing · dependency minimality · brownfield exposure · no-task-lists) — the `--check` citation is additive; `references/ARTIFACT-CHECKLISTS.md` untouched (out-of-scope per D3).
- **Consumers assessed:** n/a (single-writer skill; the ARTIFACT-CHECKLISTS reference deliberately not re-keyed).

## [v0.75.0] Cycle-cards Review-Focus row re-keyed to the vertical-TDD cycle anchor (D1) + foundation-type kill (D3); oracle-semantics check added (D2, pure addition)
- **Disposition:** superseded → the Cycle cards row's time-based **sizing** check and its **foundation-sequenced** ordering token are retired; the row now grades test-case-bundle cycles (no time anchor) and dependency minimality without the foundation word. The `references/ARTIFACT-CHECKLISTS.md` cycle-card summary line was aligned in the same edit (dropped `sizing`, added the oracle-semantics token). The **oracle-semantics** check itself and the matching Quality-Checklist item are pure additions (D2), riding this ruling row — not strips.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Vertical-TDD cycle anchor + QA test-case authorship (D1–D4 as review-amended)" row; record `.mochiko/brainstorms/vertical-tdd-complexity-and-qa-role/record.md` D1 [the test-case-bundle anchor supersedes the time-based sizing tables] + D3 [foundation/feature card type dies; the skeleton absorbs sequencing]).
- **Content (verbatim, superseded):**
  - From the Cycle cards Review-Focus row: "sizing (1–3 sessions or justified)" — the whole check clause, deleted.
  - From the same row's dependency check "dependency minimality/explicitness (foundation sequenced, `[P]` only where truly independent)": the token "foundation sequenced" — deleted, leaving "dependency minimality/explicitness (`[P]` only where truly independent)".
  - From `references/ARTIFACT-CHECKLISTS.md`'s cycle-card summary paragraph: the "sizing" token in the enumerated check list — deleted.
- **What replaced it:** the Cycle cards row gains a new **oracle semantics** check right after the TEST-gate check (D2 — each card's Asserts graded semantically against the acceptance scenario / criteria they cite, capturing the *right* expected behaviour, not merely present/in-grammar); dependency minimality/explicitness survives verbatim minus the foundation word; a Quality-Checklist item mirrors the oracle-semantics check. Both additions ride the D2 clause of the same ruling (pure additions).
- **Kept deliberately:** every other cycle-card check is untouched — vertical integrity, `**TEST:**` gate presence/grammar, story traceability (Simple/Split/Merge case + rationale), `[P]`-only-where-independent, brownfield exposure (`none` counts), no-task-lists/file-paths. The re-key retires only the time anchor (D1) and the foundation word (D3); the demonstrability judgment the anchor moved to lives in the record, not restated here.
- **Consumers assessed:** `devils-advocate` mounts this skill as the plan completeness reviewer — the re-key changes the grading dimensions it applies (drops time-sizing + foundation-ordering, adds oracle semantics), never the composition. The paired `plan.md` QA-seat wiring + card-field ripple land the same wave (v0.75.0). `review-feasibility` (sibling) is unaffected — cycle-card completeness is this skill's, not its. The card-shape source of truth (`patterns-vertical-tdd` / `tasks-template.md`) is P1's same-wave work; this row grades against it, it does not define it.

## [v0.67.0] Re-keyed from fixed-checklist completeness to approved-proposal conformance (blocking) + rung-honesty (advisory)
- **Disposition:** superseded → the three-lens Overview (conformance BLOCKING · rung-honesty advisory · completeness-within-scope), the re-keyed Scope Completeness question, the Verdict-Criteria precedence override, and the re-keyed description — plan re-identified as delivery of a package per the *approved artifact proposal*, not a fixed mandated artifact set.
- **Tier failed:** n/a — supersession by ruling (`plan-structure-yagni` record D5 as amended HF-2 [with D1/D2 artifact-set demotion], `.mochiko/brainstorms/plan-structure-yagni/record.md`; combined-wave landing `architect-role-pushback-and-abstraction` D3/D5, `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md`; DECISIONS.md combined-wave row).
- **Content (verbatim, superseded):**
  - Overview framing: "Find gaps in planning artifacts and emit issues that must be resolved before the plan proceeds. This is a **mirror checklist**: a fixed set of named checks, each with a fixed question and a severity, producing a verdict derived mechanically from the issue counts. Focus on design completeness, coverage, measurability, and cross-artifact consistency — not implementation details, and not whether the design can be built (that is a separate review; see *Scope* below)."
  - Scope Completeness question: "Is everything present, traceable, measurable, and internally consistent with the decisions that were made?"
  - Verdict Criteria: "Derived mechanically from the issue counts — the mapping itself carries no judgment; it is single-sourced in [ISSUE-TEMPLATES.md → Verdict Criteria](references/ISSUE-TEMPLATES.md#verdict-criteria)."
  - Old description (verbatim): "This skill MUST be invoked to grade plan artifacts against the completeness checklist — analysis, design, and cycle-card (`tasks.md`) sets — checking coverage, measurability, architecture coverage, cycle-card quality, and consistency. Emits a 3-state verdict (ready / needs-revision / critical-gaps). The completeness (mirror-checklist) half of the plan pair; does NOT cover feasibility/buildability (that is `review-feasibility`); defaults to FAIL; run by an independent validator, never the author."
- **What replaced it:** the run's default-FAIL floor is now the approved artifact proposal. Three lenses — (1) **conformance** BLOCKING: every proposed artifact present + within approved depth; material divergence (unproposed artifact, or element class materially past approved depth) auto-FAILs the package, stated as a body-level precedence override on the count mapping (`references/ISSUE-TEMPLATES.md` untouched, per the team-lead Q3 ruling); (2) **rung-claim honesty** advisory, graded against `mochiko:patterns-plan-minimalism` (the ladder, never restated here); (3) **completeness within scope** — the mirror checklist survives here, applied to the proposed artifacts. Related + Quality Checklist gained the matching pointer/items (pure additions).
- **Kept deliberately:** the mirror-checklist mechanic itself (named checks, fixed question, severity, count-derived verdict) survives as lens 3, applied to the proposed set — never deleted; the completeness-vs-feasibility Scope split; the Review Focus by Artifact Type table (incl. the `plan-task-granularity` D4 architecture-coverage + cycle-card rows); Review-Process Step 2 deterministic pre-assert (non-waivable floor); Incremental Review Mode; Red Flags; Common Mistakes; Common Rationalizations. One architect-role-sourced seam sentence added to lens 2 (consistency-note-7: rung-honesty is a disclosure grade, distinct from `review-feasibility`'s independent hunt class 7) — a clarifying seam, not a new mechanism; review-plan-artifacts stays sibling-D5-only otherwise.
- **MANDATORY KEPT reconciliation:**
  - **[v0.64.0] guardrails keep-set** (Scope table · Review Focus table · Issue/Verdict pointers · Step 2 pre-assert · Incremental mode · Quality Checklist · Common Mistakes · Red Flags · Common Rationalizations · Related) — all intact; this edit re-keyed the Overview / Scope-question / Verdict framing and the description, and added to Quality Checklist + Related, deleting none of the kept set.
  - **[v0.26.0] KEPT: Red Flags, Common Rationalizations, Incremental Review Mode** — untouched. Intact.
  - **[v0.15.0] KEPT: the "Report shape (incremental mode)" block** — untouched. Intact.
- **Consumers assessed:** agents — `devils-advocate` mounts this skill (its plan seat per `plan-structure-yagni` D5 / `architect-role` D2); the re-key changes what it grades against (the approved proposal), not the composition. Commands — `plan.md` binds it; the combined-wave `plan.md` re-key (separate seat) supplies the approved-proposal floor this grader now reads — the two land together (D5 one-wave ruling). Sibling `review-feasibility` is referenced by name — unchanged; the seam line names its hunt class 7 without moving it. Contract intact.

## [v0.64.0] Guardrails Wave 2 — body deletions (When to Use, Review-Process Steps 1/3/4/5) + slim description + review-evidence floor line
- **Disposition:** superseded → the guardrails-vs-detail Wave 2 editorial cut (D4 cut line): the "When to Use" list and the generic Review-Process walkthrough steps whose obligations already live in the Review-Focus table / Verdict-Criteria pointer / Quality Checklist / Common-Mistakes are deleted; description slimmed; one sanctioned floor line added. The deterministic pre-assert (Step 2) is retained as a non-waivable floor.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row [its Wave 2 residual authorization] + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — guardrails held across all four skill natures).
- **Content (faithfully compressed).** Description 1,464 → 500 chars (−66%). Body 12,078 → 10,855 chars (−1,223, −10%; net of the +~183-char floor-line pure addition). Sections removed:
  - **## When to Use** (five bullets) — restated the description + the Review-Focus-by-Artifact-Type table (analysis-set · design-set · cycle-cards · cross-artifact consistency · FAIL-loop re-review). All survive in the description and that table.
  - **### Step 1: Gather context** (read artifact / spec / prior artifacts / constitution) — generic gather; the read-upstream/prior obligation survives in the Cross-artifact row of Review Focus and Quality Checklist ("Cross-artifact concerns noted").
  - **### Step 3: Execute the checklist** — "ask the question, look for evidence, classify, document" restates the checklist mechanic; obligation survives in the artifact-type checklists (referenced) + Quality Checklist ("All applicable artifact-type checks executed").
  - **### Step 4: Cross-reference** — traceability/consistency/completeness; survives in the Cross-artifact row + Quality Checklist ("Cross-artifact concerns noted").
  - **### Step 5: Emit the report** — the `advocate-report-template` binding survives in `## Related`; verdict-from-counts survives in `## Verdict Criteria`; the `at:` evidence anchor + actionable fix survive in Common Mistakes ("Missing evidence"/"Vague suggestions") and Quality Checklist; the `strengths:` field survives in Quality Checklist ("The one-line strengths: field filled").
  - **Old description (verbatim):** "This skill MUST be invoked to grade a producer's plan artifacts against the completeness checklist — the analysis set (requirements, constraints-and-decisions, NFRs), the design set (data-model, API contracts, quickstart), and the cycle cards (tasks.md) — checking FR→TR coverage, orphan technical requirements, testable/measurable criteria, NFR measurability, entity and endpoint coverage, data-sensitivity annotations present, schema-model consistency, integration-boundary presence, architecture coverage (component-table↔diagram coverage, qualifying-flow sequence coverage keyed to ordering/failure not story priority, delta-summary D-XXX links), cycle-card quality (vertical integrity, TEST-gate presence and grammar, story traceability, sizing, dependency minimality, brownfield exposure), and cross-artifact consistency (does the design honor the decisions and conform to the approved architecture). Emits a severity-classified gap report (Critical/Important/Minor) and a 3-state verdict (ready / needs-revision / critical-gaps). SHOULD also invoke whenever a plan loop's completeness-review step needs an independent grade of the planning artifacts, or when re-reviewing after a FAIL-loop revision. The completeness (mirror-checklist) half of the plan producer↔validator pair; does NOT cover cross-artifact feasibility / buildability / contradiction (that is mochiko:review-feasibility); defaults to FAIL; run by an independent validator, never the author."
  - Verbatim homes for the removed body + description text: git history of this SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Floor line added (pure addition, cross-cutting finding 1 / F-X1 mitigation):** "The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation." Placed in `## Overview`, immediately after the "Violating the letter of the rules is violating the spirit" floor paragraph (this skill has no Independence section; Overview is its floor home). Rides the same decision row.
- **Kept deliberately (the guardrails keep-set):** the Scope table (completeness vs feasibility); the Review Focus by Artifact Type table (analysis · architecture · design · cycle-card · cross-artifact checks — the architecture-coverage and cycle-card rows are D4 plan-task-granularity ruled content); the Issue Classification + Verdict Criteria pointers to `ISSUE-TEMPLATES.md`; **Review Process Step 2 (the deterministic `python scripts/check-artifacts.py` pre-assert — a non-waivable floor, "a failed count is ground truth")**; Incremental Review Mode (with its report-shape block); Quality Checklist; Common Mistakes; Red Flags; Common Rationalizations; Related.
- **MANDATORY KEPT reconciliation:**
  - **[v0.26.0] KEPT: Red Flags (incl. the two generic bullets), Common Rationalizations, Incremental Review Mode** — all three untouched by this cut. Intact.
  - **[v0.15.0] KEPT: the "Report shape (incremental mode)" block** — untouched (lives inside the retained Incremental Review Mode). Intact.
  - The removed When to Use + Steps 1/3/4/5 carry no prior `KEPT:` or `DECISIONS.md`-traceable marking; each removed obligation is enumerated above with its surviving home.
- **Consumers assessed:** commands — `plugins/mochiko/commands/implement.md` matched the grep, but on the sibling `review-code-minimalism` string, not this skill (this skill is plan-cluster, mounted on `devils-advocate`, dispatched by the plan command). Agents — `plugins/mochiko/agents/devils-advocate.md` mounts it; the kept Review-Focus table, Scope table, verdict pointer, and checklists leave that composition intact. `review-feasibility` references it by name as the sibling — unchanged. Contract intact.

## [v0.53.0] Code-review punt line narrowed — minimalism-lens carve-out
- **Disposition:** superseded → the same When-NOT-to-Use bullet with a parenthetical carve-out naming `mochiko:review-code-minimalism` (implement-side) as the one exception; general code review stays punted.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-05 "Ponytail code-minimalism ruled (PT-D1–D10)", record `.mochiko/brainstorms/ponytail-concepts-integration/record.md`, D2 — punt reversal narrow, lens-only).
- **Content (verbatim, the superseded bullet):**
  ```
  - **Implementation code review** — use code-review tooling instead
  ```
- **Kept deliberately:** the punt itself for everything but the minimalism lens — naming, patterns, correctness beyond tests remain out of this skill and out of mochiko's review surface generally; the `:185` anti-pattern row ("Reviewing implementation details") untouched, still correct for this skill.
- **Consumers assessed:** devils-advocate (mounts it; plan-side scope unchanged) · plan (binds it; no behavior change — the carve-out points elsewhere).

## [v0.49.0] Absorbed the cycle-card checks (from retired review-task-artifacts); boundary line removed
- **Disposition:** superseded → the new Cycle cards row in Review Focus (the absorption); the When-NOT-to-Use "Task artifact review — use `mochiko:review-task-artifacts`" line deleted with its target
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D4)
- **Content:** the one boundary bullet; the absorbed checks are additions.
- **Kept deliberately:** the feasibility hand-off boundary — unchanged; the plan-review pair (completeness vs feasibility) survives whole.
- **Consumers assessed:** devils-advocate · plan · router.

## [v0.46.0] loop-discipline pointers out
- **Disposition:** superseded → the anti-rationalization content stands in this file's own red flags; loop ownership is the command's
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "(The generic anti-rationalization doctrine lives in `loop-discipline`; ...)" → "(The review-specific red flags are at the foot of this file.)"; the Related bullet "`loop-discipline` — the source of the anti-rationalization and independent-validation doctrine this skill operationalizes" deleted.
- **Consumers assessed:** plan command briefs unchanged.

## [v0.26.0] Severity + verdict tables → ISSUE-TEMPLATES pointers; steps and Common Mistakes densified (body 240 → 209, −13%, in-band 10–40)
- **Disposition:** relocated → `references/ISSUE-TEMPLATES.md` (severity table held there **verbatim** under Severity Levels; verdict mapping held richer under Verdict Criteria — both Read and confirmed before landing, wave-2 batch-1 ratification 2026-07-25) · densified (form-only, zero content deletions): Step-3 execute list → one sentence, Step-4 cross-reference bullets → one sentence, Step-5 emit bullets → one sentence (the `advocate-report-template` holds the `at:`/`strengths:`/verdict shape — verified), six Common-Mistakes Bad/Good subsections → one 3-column table
- **Tier failed:** 1 (both tables restated their declared single source, referenced directly below each) · n/a for the densifications — form only
- **Content:** the two 3-row tables; the numbered/bulleted step mechanics; the Common-Mistakes subsection headers (all Bad/Good pairs preserved in the table)
- **Consumers assessed:** wave-open enumeration — 7 citing files, none reference the stripped tables or section anchors; the in-body *Verdict Criteria* heading survives as the pointer so Step 5's internal see-reference stays valid

## [v0.26.0] KEPT: Red Flags (incl. the two generic bullets), Common Rationalizations, Incremental Review Mode
- **Tier-2 evidence:** contested at the wave-2 pass and kept — the two generic red-flag bullets
  ("this case is different", "spirit not letter") sit at the reviewer's point of temptation, the
  same presence mechanism the R4b aphorism ruling protects, and `loop-discipline` holds no
  red-flag list to relocate to; every rationalization row names a failure + counter pair; the
  Incremental section stands on its v0.15.0 KEPT (plan-only unique, re-affirmed untouched). The
  aphorism copy (L16) is consequence-anchored — qualifies under the R4b rider, no edit needed.
  Session ruling: batch-1 ratification 2026-07-25.

## [v0.15.0] KEPT: the "Report shape (incremental mode)" block
- **Tier-2 evidence:** scrutinized directly against the `review-specifications` Output-Format strip
  precedent (specify wave, v0.13.0) — does the embedded report mock restate a template-owned shape?
  Ruled **KEEP** because the **incremental report shape is plan-only unique content** and does NOT
  belong in the shared `advocate-report-template` (2 consumers: specify has no incremental mode, so
  relocating there would inject specify-irrelevant bloat into a shared template). The general report
  shape is already referenced (`advocate-report-template`, in Related); the incremental Cross-Artifact
  Consistency table is point-of-use for the incremental review, and its checks are single-sourced in
  `references/ARTIFACT-CHECKLISTS.md`. A reader would contest it (it partly resembles the stripped
  `review-specifications` block), so it earns this entry. Provenance: the incremental-review economy is
  plan's own (the {new design}/{prior analysis} set selection the plan lead supplies to the standing
  completeness reviewer in Phase 2).
