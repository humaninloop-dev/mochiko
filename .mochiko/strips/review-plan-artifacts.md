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

## [v0.77.0] `advocate-report-template` consumer pointers → the `advocate-report` schema (two-arm) — D3 later-ratchet
- **Disposition:** superseded → the `advocate-report` schema (`mochiko-cli template advocate-report`, or Read `plugins/mochiko/schemas/advocate-report.yaml` raw when the binary is absent). Five consumer pointers re-pointed: `SKILL.md` Incremental-mode report-shape line + the Related bullet; `references/ISSUE-TEMPLATES.md` assembled-deliverable line, working-report-shape line, and the Assembled report paragraph.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, verbatim):**

```text
SKILL.md:128  The same `advocate-report-template.md` shape, with the incremental fields set and this
SKILL.md:211  - `mochiko:advocate-report-template` — the deliverable report shape the lead reads
references/ISSUE-TEMPLATES.md:5   reads is the `mochiko:advocate-report-template` (see [Assembled report](#assembled-report) below) —
references/ISSUE-TEMPLATES.md:84  the lead is assembled with the `advocate-report-template` — see below.
references/ISSUE-TEMPLATES.md:132 The deliverable report the lead reads is structured with `mochiko:advocate-report-template`
```
- **Kept deliberately:** all surrounding prose (the incremental `consistency_checks:` block, the working-report shape, the Assembled report body) — only the template-file token was superseded.
- **Consumers assessed:** the `advocate-report-template.md` template deletion + schema authoring is P1 scope; the router row for the schema is re-typed in `strips/mochiko.md` (P5). The plan-review ARTIFACT-CHECKLISTS `--check` re-key is a **separate P2 edit to this same primitive** — its companion strip entry (if present) is P2's; this strip file is a shared write surface (flagged in the P5 report). Cold re-grep confirms zero remaining `advocate-report-template` references in `plugins/`.

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
