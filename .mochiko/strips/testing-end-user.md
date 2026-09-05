# Strip notes — `skills/testing-end-user/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D3 + D6a/b; rulings ratified
2026-07-23). Skill-succinctness wave-1 entries atop (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
246 → 208 lines, 38 cut = 15% — in the 10–40 previously-stripped band.

Verbosity/caveman wave-1 entry atop (design:
`.mochiko/brainstorms/verbosity-caveman-ops-separation/record.md`, D4 as folded at review
(S2/S13); ruling: `DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation
ruled" row). The wave's edits to `references/REPORT-TEMPLATES.md` are **pure additions** —
the sanctioned-set closure, the register binding, and the prose-on-clean check — so they ride
the decision row; nothing was removed or rewritten, and the existing *"A passing report
carries **no prose**"* rule was extended, never replaced. The one entry below records a line
whose right to exist is contested in advance.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the dense-five family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/testing-end-user/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any parsing or execution: **Read `schema.yaml` (this skill's own
  directory) raw, in full** — the small families ship no common file and no stub binds, so
  the pair's own schema is the whole first action. The schema is the source of truth for
  this skill's binding rules, nested in six sections, each addressable by its section ID:
  `testing-end-user.sec.independence` · `testing-end-user.sec.scope` ·
  `testing-end-user.sec.inputs` · `testing-end-user.sec.verdict` ·
  `testing-end-user.sec.output` · `testing-end-user.sec.reserved`. Interpret it live: a
  rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule of
  `class: floor` is always read and always delivered; a `pointer:` rule binds you to that
  file's or skill's procedure, referenced never restated; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 7 rules of
  `class: floor` are non-waivable. Before the first parsing step, state the floor count
  back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire.
- **Consumers assessed:** none shared — the block was this skill's own text, and this family
  ships no common file.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the 7 rules of
  `class: floor` are non-waivable. Before the first parsing step, state the floor count
  back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.103.0] Converted to the `.md` + schema pair form — rule content relocated to `schema.yaml` (wave 2C, small families)

- **Disposition:** superseded → the pair form: obligation content relocated into
  `plugins/mochiko/skills/testing-end-user/schema.yaml` (23 rules — 7 floor · 16 must ·
  0 advisory — under the review six-section set reused by ruling,
  `testing-end-user.sec.independence` deliberately empty), the `SKILL.md` body keeping
  the Overview + letter/spirit epigraph, the new "Rules — load the schema first" block
  (floor pin 7 + read-back), When NOT to Use, the Core Process procedure (detection, the
  six-step execution sequence, the quality-gate sequence), the Red Flags list, the
  Common Rationalizations and Common Mistakes tables, and Reference Files. The
  frontmatter `description:` is byte-untouched (500).
- **Tier failed:** n/a — supersession by ruling (`skill-content-schema` D1–D9 as amended
  + the wave-2 small-families door ruling, `DECISIONS.md` 2026-09-01 — the dense five
  convert on the B/C drivers, the review six-set reused with explicit empty markers, no
  common file, zero `extends:`; D8/C4 supersession-transfer). Census referent:
  `.mochiko/brainstorms/skill-content-schema/census-small-families.md` §B TEU rows 1–21.
  Lead-ruled recorded deviations at plan approval: the a/b limb splits (5a/5b · 14a/14b —
  one `kind:` per rule, so 23 rules against the census header's 21) and the floor pin 7
  against the census tally's 6 (row grain beats tally — the §B stub-19 floor row was
  under-tallied). Label note, deliberate: the process rules (strict-order ·
  rationalization-stop · setup-fail-fast) carry `ladder` (its walk-order clause) — a
  ruled divergence from wave 2A's `boundary` mapping on ATR's STOP rule, chosen so
  detector runs read it as ruled, not drift. Protection transfers to the rule IDs via
  `.mochiko/provenance.yaml`.
- **Content (superseded body fragments, census-row → rule-ID relocation map; verbatim
  text survives in git history pre-v0.103.0 and verbatim-in-substance in the named
  rules):**
  - Overview grammar-ownership banner ("authored and owned by `patterns-vertical-tdd` …
    This skill **consumes** that grammar; it does not redefine it. What this skill owns
    is the **runtime**") → `testing-end-user.grammar-consumed-never-redefined` (row 1).
    The banner's tail clause ("Where the grammar and the execution meet below, the
    vocabulary is referenced and the *how* is retained.") was a reading instruction
    about the body's own layout that the conversion makes false — the moved semantics no
    longer sit "below"; its substance rides
    `testing-end-user.grammar-consumed-never-redefined` +
    `testing-end-user.grammar-owner-wins` ([v0.49.0] keep-set protection intact on those
    IDs).
  - "Execute in strict order. No skipping steps. No reordering." →
    `testing-end-user.strict-order` (row 2, floor).
  - "Fail fast if any setup fails — a setup failure blocks action execution. Record all
    setup output for debugging." → `testing-end-user.setup-fail-fast` (row 3).
  - Step-3 modifier execution-semantics bullets (`(background)` PID-tracked ·
    `(timeout Ns)` kill + `TIMEOUT` · `(in path)`) →
    `testing-end-user.modifier-execution-semantics` (row 4).
  - Step-4 assert evaluation-semantics bullets →
    `testing-end-user.assert-evaluation-semantics` (row 5a); "Any other assert text is a
    **custom assertion for human evaluation** at the checkpoint" →
    `testing-end-user.custom-assert-to-human` (row 5b).
  - "Each assert MUST receive an explicit pass/fail evaluation. **No default to PASS** —
    an unevaluated assert is a failure." → `testing-end-user.no-default-pass` (row 6,
    floor).
  - Step-5 machine-first report shape (frontmatter-only on all-PASS; `## Failures` on any
    FAIL/PARTIAL/TIMEOUT/ERROR) → `testing-end-user.report-machine-first` (row 7).
  - "The human decision gates completion — no proceeding without explicit human
    approval." → `testing-end-user.human-gate` (row 8, floor).
  - The Task Classification criteria table + "owned by this skill" →
    `testing-end-user.runtime-classification-owned` (row 9); the browser-flow exception
    paragraph → `testing-end-user.browser-flow-exception` (row 10, UX-D9 wording
    preserved in substance); "Default to SUBJECTIVE if uncertain … Any failure, on any
    classification, forces a checkpoint." → `testing-end-user.default-subjective`
    (row 11, floor).
  - The Result Classification table → `testing-end-user.result-vocabulary` (row 12).
  - The Quality Gates checklist + "No presenting partial results. No skipping evidence
    capture." → `testing-end-user.completion-set` (row 13).
  - Quality Gate Auto-Resolution ("always auto-resolve … the answer is an exit code, not
    a judgment"; "MUST NOT be softened into an LLM judgment call") →
    `testing-end-user.gates-auto-resolve` (row 14a, floor); the gate source (`tasks.md`
    `## Quality Gates` + the project's own build configuration) →
    `testing-end-user.gate-source-binding` (row 14b).
  - Red-Flags "All of these mean: Rationalization in progress. Return to the execution
    sequence. Follow every step." + the No-exceptions block →
    `testing-end-user.rationalization-stop` (row 15, floor; the Red Flags list and both
    teaching tables stay prose).
  - The Evidence Types section → `testing-end-user.evidence-capture-binding` (row 16).
  - Reference-borne obligations gain stubs, the files untouched (rows 17–21):
    `testing-end-user.grammar-owner-wins` · `testing-end-user.parse-error-halts`
    (TASK-PARSING.md) · `testing-end-user.sanctioned-set-closure` ·
    `testing-end-user.truncation-bounds` (REPORT-TEMPLATES.md) ·
    `testing-end-user.cleanup-protocol` (EVIDENCE-CAPTURE.md).
- **MANDATORY KEPT reconciliation:** the [v0.44.0] KEPT entry below protects the
  envelope's register + prose-on-clean check restated in
  `references/REPORT-TEMPLATES.md` — a RULED dual-homing beside
  `templates/report-format.md` ("Cut it only with a ruling that also re-homes the
  check"). **This conversion cuts nothing there:** REPORT-TEMPLATES.md is untouched, and
  the new stub `testing-end-user.sanctioned-set-closure` POINTS at it, never duplicates
  it — the protection transfers onto the stub ID via `.mochiko/provenance.yaml`,
  anchored on the 2026-08-01 verbosity-caveman-ops-separation ruling (D8/C4; census
  J2-8). An audit must read the reference-side restatement as the ruled exception, not a
  D6 anti-dual-homing violation. The [v0.49.0] keep-set: the parsing algorithm, field
  extraction, and legacy-marker normalization stay in TASK-PARSING.md untouched; the
  grammar-owner banner obligation re-homes to
  `testing-end-user.grammar-consumed-never-redefined` + the grammar-owner-wins stub. The
  [v0.64.0] guardrails keep-set: obligations re-home per the map above; the teaching
  surfaces it protects (epigraph, When NOT to Use, Red Flags, Rationalizations,
  Mistakes, Reference Files) stay in the body. The [v0.91.0] keep-set (always-auto-
  resolve, ruled gate-source wording, exit-code classification, `quality_gates` slot)
  re-homes to rows 14a/14b, `testing-end-user.completion-set`, and
  `testing-end-user.report-machine-first`. No protected line is deleted.
- **Kept deliberately:** the Overview + letter/spirit epigraph · When NOT to Use · Task
  Detection and the six-step Execution Sequence as procedure prose · the quality-gate
  execution sequence · the Red Flags list · the Common Rationalizations and Common
  Mistakes tables · Reference Files incl. the grammar-owner row · the `description:`
  byte-identical at 500.
- **Budget:** re-seed per D8/C1 — delivered-at-invoke payload body 8,696 + schema 12,095
  = **20,791** (was body 13,123 against the 16,407 budget); third seeding path, no
  headroom; the ledger row is the closer seat's write.
- **Consumers assessed:** qa-engineer (mounts it) · implement (binds the verification
  seat) · executing-tdd-cycle (cross-links; the `**TEST:**` gate stays the verifier's) ·
  patterns-vertical-tdd (grammar owner — the seam wording preserved in substance in
  rule 1 and the grammar-owner-wins stub) · review-code-minimalism · the mochiko router.
  No shipped surface links a removed body section anchor.

## [v0.91.0] Quality-gate command source: the `plan.md` build-configuration read dies — plan-stage retirement D4

- **Disposition:** superseded → quality-gate commands come from `tasks.md`'s `## Quality Gates`
  section **and the project's own build configuration**.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D4**: "`plan.md` (the summary
  artifact) dies — no restatement artifact"). Scope for this file was opened by the wave lead's
  second extension ruling of 2026-08-26.
- **Content (superseded text, verbatim):**

  ```
  1. **Identify quality gate commands** from the `## Quality Gates` section of `tasks.md` and/or the build configuration in `plan.md`.
  ```

- **Disposition judgment, raised by this seat and then ruled:** D4 rehomes the `quickstart.md`
  null path and the sufficiency verdict explicitly, but says nothing about where build
  configuration lives. The sufficiency report is a *verdict* artifact, so pointing a gate-command
  read at it would have been inventing a home. This seat flagged the gap rather than guessing;
  the **wave lead ruled the replacement wording on 2026-08-26** — "`tasks.md` and the project's
  own build configuration", dropping the dead `plan.md` half, the line already naming `tasks.md`.
  The reading behind it: build configuration was never really `plan.md`'s to hold — it belongs to
  the project.
- **Kept deliberately:** `tasks.md`'s `## Quality Gates` section as the first source, the
  always-auto-resolve rule for quality gates (deterministic ground truth, never judgment), the
  execute/record/classify sequence, exit-code classification, and the `quality_gates` frontmatter
  reporting slot — all untouched.
- **Budget:** body **13,123** against the 16,407 budget; description untouched at 500 against
  625. Both inside. The ruled wording is terser than the text it replaced, so the body ends 2
  chars *below* the ledger's recorded 13,125 rather than above it. (Figure taken after the
  ruled-wording alignment, not before it.)
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` owns the `**TEST:**` grammar this
  skill consumes (untouched); `implement.md` (P1's rewrite) dispatches the verification seat.
  No other site named `plan.md` as a build-configuration source.

## [v0.80.0] REPORT-TEMPLATES.md — envelope row drops the `slice` key; storage path drops `feature/slice` — slice-vocabulary purge

- **Disposition:** superseded → the same field-definition row carrying `report` / `feature` only,
  and the same storage bullet naming the feature directory (the envelope field itself was deleted
  in the same wave — `.mochiko/strips/report-format.md`).
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`).
- **Content (verbatim, the two superseded lines):**

  ```
  | `report` / `feature` / `slice` | yes | Per the envelope (`slice:` only when slice-scoped); `final-validation` for the whole-implementation run |
  ```

  ```
    per-cycle and final-validation reports in the feature/slice directory) — it is what the
  ```

  Replaced by:

  ```
  | `report` / `feature` | yes | Per the envelope; `final-validation` for the whole-implementation run |
  ```

  ```
    per-cycle and final-validation reports in the feature directory) — it is what the
  ```

- **Kept deliberately:** the `final-validation` wording in the envelope row — the whole-
  implementation run is a live report kind and the row is its only statement of that. The
  storage bullet keeps everything else: the caller-names-the-path rule, the lead-Reads-it-for-
  the-verdict clause, and the resumed-run workspace-evidence clause.
- **Consumers assessed:** reference file, not a loaded surface. Its sibling envelope row in
  `skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md` carries the same shape and was
  edited in the same wave; the envelope itself is `templates/report-format.md`, also edited.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 13,522 → 13,125 chars (−3%); description 790 → 500
  chars (−37%). Body cut: the **When to Use** section deleted whole (seven bullets restating the
  description's invocation conditions — `**TEST:**` tasks, CLI verification, filesystem-state
  validation, real-process testing, GUI verification, end-to-end validation, quality-gate
  execution; each obligation survives in Task Detection, the Execution Sequence, Task
  Classification, and Quality Gate Execution). Description cut: the action-modifier enumeration
  (`(background)`/`(timeout Ns)`/`(in path)`), the result-classification enumeration
  (PASS/FAIL/PARTIAL/TIMEOUT/ERROR), and the "presenting a verification checkpoint" trigger
  compressed; the MUST clause, core triggers (TEST: against real infra + quality gates), the
  CLI/GUI/SUBJECTIVE classification, and the `patterns-vertical-tdd` grammar-owner + never-mocks
  distinctions kept. Verbatim homes: git history of this file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when executing a `**TEST:**` verification task against real infrastructure — parsing its Setup/Action/Assert fields, running the actions (honoring `(background)` / `(timeout Ns)` / `(in path)` modifiers) with captured evidence, evaluating the asserts against that evidence, and classifying the task CLI/GUI/SUBJECTIVE to decide auto-approve versus human checkpoint. SHOULD also invoke when running quality gates (lint / build / test) as deterministic exit-code checks during verification, capturing execution evidence, classifying a verification result (PASS/FAIL/PARTIAL/TIMEOUT/ERROR), or presenting a verification checkpoint for human approval. Consumes the `**TEST:**` grammar owned by patterns-vertical-tdd; verifies against real infrastructure, never mocks.
- **Kept deliberately:** the guardrails keep-set — the Overview + letter/spirit epigraph + the
  grammar-ownership banner, When NOT to Use, the entire Core Process (Task Detection, the
  Execution Sequence with the owned execution/evaluation semantics, Task Classification incl. the
  browser-flow exception, Result Classification, Evidence Types), Quality Gates, Quality Gate
  Execution, Red Flags, the Common Rationalizations table, the Common Mistakes table, and the
  Reference Files pointers.
- **MANDATORY KEPT reconciliation:** the [v0.44.0] KEPT entry protects the envelope's register +
  prose-on-clean check, but that content lives in `references/REPORT-TEMPLATES.md`, NOT the SKILL
  body — untouched by this body cut. The [v0.49.0] supersession KEPT the whole parsing algorithm,
  field extraction, legacy-marker normalization, and the grammar-owner banner — all live in Task
  Detection / Core Process / Reference Files, none in the deleted When-to-Use. No prior KEPT or
  protected line is touched.
- **Consumers assessed:** qa-engineer (mounts it) · implement (binds it) · executing-tdd-cycle
  (cross-links; the `**TEST:**` gate is the verifier's) · patterns-vertical-tdd (grammar owner,
  cross-references) · review-code-minimalism · mochiko router. None links the removed When-to-Use
  bullets or a description clause. Contract intact.

## [v0.49.0] TEST-gate source re-keyed to cycle cards
- **Disposition:** superseded → gate blocks at the foot of cycle cards; legacy task-line form kept parseable
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2)
- **Content:** the `- [ ] **TN.X**: **TEST:**` detection sample in SKILL.md · TASK-PARSING.md's task-line boundary rules (`START: - [ ] **T{N}.{X}**: **TEST:`) and task-ID extraction as the primary form · `T{N}.{X}` / `verify-T2.4` keys in REPORT-TEMPLATES.md and EVIDENCE-CAPTURE.md (now `C{N} gate` / `verify-C{N}-gate`).
- **Kept deliberately:** the whole parsing algorithm, field extraction, legacy-marker normalization, grammar-owner banner (TEST-GRAMMAR.md still owns the vocabulary) — the legacy task-line form remains parseable by design.
- **Consumers assessed:** qa-engineer (mounts it) · implement · patterns-vertical-tdd (grammar owner, co-edited).

## [v0.44.0] KEPT: the envelope's register + prose-on-clean check, restated in this payload home
- **Tier-2 evidence:** a deliberate exception to the no-restatement rule, recorded so a later
  minimalism wave does not read it as Tier-1 duplication and relocate it. Ground: record F72 —
  the driver run's report prose was authored against this payload home, one hop *below*
  `templates/report-format.md`, where the stricter frontmatter-only rule already failed to reach
  (F58: 8/8 passing reports carried prose, 79.9% of report bytes; the verification reports fanned
  into 7–15 H2 sections, most outside the sanctioned set — F60). D4's S2 fold names the
  restatement in both payload homes as a host of the check, so the binding is ruled, not
  stylistic. Cut it only with a ruling that also re-homes the check.

## [v0.25.0] Evidence Types capture-method table (4 rows)
- **Disposition:** relocated → `references/EVIDENCE-CAPTURE.md` (already catalogues all four types with full capture mechanics — verified before landing; type names kept in the pointer line)
- **Tier failed:** 1 (index copy of the reference's own sections)
- **Content:** console/screenshot/logs/timing → capture-method rows
- **Consumers assessed:** TEST-GRAMMAR grammar seam untouched (vocabulary stays with `patterns-vertical-tdd`); 7 consumer files checked, none reference the table

## [v0.25.0] Quality-gate YAML report-format example (12 lines)
- **Disposition:** relocated → `references/REPORT-TEMPLATES.md` (the declared report-format home since v0.22.0; `quality_gates` documented there at lines 30/46/89/108 — verified before landing)
- **Tier failed:** 1 (format example restating the home's field table)
- **Content:** the three-gate `quality_gates:` YAML block + its two explanation lines
- **Consumers assessed:** none reference the example

## [v0.25.0] Common Mistakes densified: 6 subsections → 6-row table (net −27 lines)
- **Disposition:** compressed in place (densification, zero deletions — every mistake/failure/fix survives as a row; wave-2 artifact-densification precedent)
- **Tier failed:** n/a — no content left the skill; form only
- **Content:** the six What-goes-wrong/Fix subsections (setup validation, background cleanup, evidence truncation, PASS-without-asserts, proceeding-after-reject, skipped checkpoint)
- **Consumers assessed:** none reference the subsection headings

## [v0.22.0] Per-outcome report scaffolds → machine-first verification-report file
- **Disposition:** relocated/contracted → `references/REPORT-TEMPLATES.md` (rewritten): the persisted per-cycle/final-validation report is YAML frontmatter (per-task results, quality gates, classification, recommendation) with a `## Failures` section only on FAIL/PARTIAL/TIMEOUT/ERROR
- **Tier failed:** consumption evidence (epic F-c): sole live consumer is the lead's verdict; kinako's 16 verification reports (~9.9k B avg) carried the full Setup/Actions/Asserts scaffold per report, ×16
- **Content:** the five per-outcome markdown templates (Success minimal / Failure rich / Partial / Timeout / Error) with per-report `**Description**/**Result**/**Duration**/**Recommendation**` blocks, full assertion + actions tables and analysis on every non-success. Preserved: rich-on-failure (S8 — the failure detail is the `## Failures` section), checkpoint presentation formats, truncation rules + full-log pointers, evidence-capture discipline. The prior "Reports are not persisted to disk" storage contradiction with implement.md's per-cycle files resolved: the checkpoint presentation is in-memory; the per-cycle aggregate file persists.

## [v0.22.0] `references/TESTING-EVIDENCE.md` archived (deleted)
- **Disposition:** deleted (D6b) — full content in git history (`plugins/mochiko/skills/testing-end-user/references/TESTING-EVIDENCE.md`, removed at v0.22.0); index line removed from SKILL.md Reference Files
- **Tier failed:** 2 (provenance, not procedure — the RED/GREEN/REFACTOR hardening record for the skill's anti-rationalization content; 4,444 B shipped with no runtime consumer)
- **Content (compressed):** the TDD build record: RED-phase pressure scenarios (simple-CLI / time-pressure / repeated-test / background-process / partial-success) with 8 verbatim captured rationalizations; GREEN-phase verification that the skill's Red Flags + Common Rationalizations tables counter each; REFACTOR-phase loophole closure.
