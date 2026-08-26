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
