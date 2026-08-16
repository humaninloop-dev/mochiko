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

## [v0.77.0] Reference Files line: stale "checkpoint presentation" coverage claim stripped (audit-caught)
- **Disposition:** deleted — the Reference Files entry for the `verification-report` schema still claimed the schema covers "checkpoint presentation" after the relocation below moved that content in-body; the DM's post-commit author≠grader audit caught the stale clause (the phase-2 producer's claimed fix targeted a different line). Reworded to "truncation; checkpoint presentation lives in-body at step 6, not in the schema."
- **Tier failed:** n/a — fix of an in-wave false claim, completing the relocation entry below; same v0.77.0 landing.
- **Content (old):** `— the verification-report file format (frontmatter + failure-only prose), checkpoint presentation, truncation`

## [v0.77.0] Checkpoint Presentation RELOCATED into SKILL.md step 6 + reference `references/REPORT-TEMPLATES.md` DELETED at the phase-2 gate
- **Disposition:**
  - **(a) relocated** — the in-memory `## Checkpoint Presentation` (the All Pass / Any Failure / If Retry Selected `AskUserQuestion` blocks + the in-memory / never-persisted / "View Details" note) moved from the reference into SKILL.md step 6 "Present Checkpoint": code blocks and prose VERBATIM, the three source `### ` sub-headings adapted to bold `**…**` labels to fit the numbered-step structure. The `verification-report` schema deliberately EXCLUDES the checkpoint presentation (it is the in-memory surface, not a file report) — which is precisely why it homed in the skill body, not the schema.
  - **(b) deleted** — the reference file; its file-report content was superseded into `schemas/verification-report.yaml` (the reference-supersession entry below, P1 phase 1) and its four SKILL.md consumer pointers were re-pointed to the two-arm `verification-report` form (the consumer-pointer entry below, authored + V3-audited PASS via the P5 re-point seat). This entry records the relocation and the file deletion landing after V1 PASS.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; the checkpoint relocation ruled by the DM at the phase-2 gate, atomic with the B2 scope; `DECISIONS.md` "Template-schema ratchet" row).
- **Content (verbatim — the terse pre-relocation step 6, superseded by the expanded step 6):**
~~~
**6. Present Checkpoint**

Ask the human to approve, reject, or retry. The human decision gates completion — no proceeding without explicit human approval.
~~~
  The relocated Checkpoint Presentation blocks (the source `## Checkpoint Presentation` section) are embedded verbatim in the reference-supersession entry below (full REPORT-TEMPLATES.md) — GI-006 reconstruction is satisfied there.
- **Kept deliberately:** step 6's completion-gating sentence survives (extended, not replaced). P5's four re-pointed consumer pointers are NOT touched by this entry — they stand in their V3-audited state (the Reference Files line keeps its "checkpoint presentation" descriptor per the DM's phase-2 supersession; a descriptor-vs-location note is flagged to the DM). D7 char-budget pre-assert: body re-measured at **14,889 chars ≤ the 16,407 budget** after the relocation.
- **Consumers assessed:** cold re-grep after the phase-2 landing confirms NO `REPORT-TEMPLATES` reference remains anywhere in `plugins/`. No router row added for `verification-report` (plan-minimalism ruling). Shared-write-surface note: P5 and P1 both wrote this strip; the three v0.77.0 entries are disjoint (re-points [P5] · reference→schema supersession [P1] · this relocation+deletion [P1]).
## [v0.77.0] Reference `references/REPORT-TEMPLATES.md` (verification report FILE) superseded by schema — `schemas/verification-report.yaml` + `mochiko-cli template verification-report`
- **Disposition:** superseded → `schemas/verification-report.yaml` + `mochiko-cli template verification-report` (D8 raw-Read fallback when the binary is absent). Scope is the two file-persisted report surfaces ONLY; the `testing-end-user` SKILL.md body is untouched by this seat.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class B2 report-format skill references per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters of the verification report FILE (Identity, Verification results, Quality gates, Minimalism & recommendation) + the two conditional-prose blocks as optional sections (Failures, Notes of note); the Field Definitions table, Truncation rules, and Storage note fold into the per-section `contract:` + `overview:`; the two worked Examples ride as section `good:` values (passing on Verification results, failing on Failures); `skeleton:` = the annotated frontmatter schema block + the two prose-block stubs. The SHARED report doctrine stays `form: report-format.md`, pointer only. `--check` lines authored NET-NEW (no checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
# Verification Report Formats

Envelope + shared rules: `templates/report-format.md` — this file carries only the
verification payloads. Two distinct surfaces:

1. **The verification report file** — persisted per verification run (per-cycle and
   final-validation in the implement workflow, at the path the caller names). Machine-first:
   the frontmatter is the report; prose only on failure.
2. **The checkpoint presentation** — in-memory, shown to the human at the gate, discarded
   after the decision. Never persisted.

Reports are adaptive: minimal for success, rich for failures — a passing verification is
frontmatter-only; debug value concentrates in failures, which keep full detail.

## The Verification Report File

### Frontmatter Schema

```yaml
---
report: verification        # or final-validation for the whole-implementation run
feature: user-auth
cycle: 3                    # omitted on final-validation
attempt: 1                  # pairs with the cycle-report attempt this run verifies
status: pass | fail | partial | timeout | error
test_tasks:                 # one row per **TEST:** gate executed (id = the owning cycle's gate)
  - {id: C3-gate, classification: CLI, status: pass, asserts: "4/4", duration: 3.2s}
  - {id: C3-gate-2, classification: GUI, status: pass, asserts: "2/2", duration: 8.1s,
     evidence: "/tmp/claude/verify-C3-gate-2-shot.png"}
quality_gates:
  lint:  {status: pass, command: "pnpm lint"}
  build: {status: pass, command: "pnpm build"}
  tests: {status: pass, command: "pnpm test", passed: 47, failed: 0, skipped: 2}
minimalism: []              # Code-minimalism lens findings (per-cycle only; [] if none)
  # - {task: T3.2, claimed: 7, observed: 2, evidence: "duplicates src/lib/session.ts:helper"}
recommendation: approve | reject | retry | needs-human
---
```

### Field Definitions

| Field | Required | Description |
|-------|----------|-------------|
| `report` / `feature` / `slice` | yes | Per the envelope (`slice:` only when slice-scoped); `final-validation` for the whole-implementation run |
| `cycle` / `attempt` | per-cycle only | The cycle and attempt this verification pairs with |
| `status` | yes | Aggregate result: `pass` only when every assert passed and every gate is green; `partial` for mixed; `timeout`/`error` per the result classification |
| `test_tasks` | yes | One row per `**TEST:**` task: `id`, `classification` (CLI / GUI / SUBJECTIVE — drives auto-approve vs human checkpoint), `status`, `asserts` (passed/total), `duration`, `evidence` (path) where captured |
| `quality_gates` | yes | One entry per gate run: `status` from the exit code (`0` = pass — deterministic, never a judgment), `command`, and pass/fail/skip counts for test suites |
| `minimalism` | per-cycle only | Code-minimalism lens findings (`mochiko:review-code-minimalism`): `{task, claimed, observed, evidence}` per finding, evidence one line (grep hit / stdlib call / manifest entry). **Advisory** — findings ride to the lead's checkpoint verdict and never fail a cycle the way a `**TEST:**` gate does; a builder-vs-reviewer rung dispute escalates to the user only at the checkpoint. `[]` when clean |
| `recommendation` | yes | The verifier's recommendation to the gate — input to the lead's verdict, never the verdict |

### Conditional Prose *(mandatory when `status` is not `pass`)*

Per failed/partial/timed-out/errored task, full detail under one section:

```markdown
## Failures

### C{N} gate — {FAIL | PARTIAL n/m | TIMEOUT | ERROR}

| # | Assert | Expected | Actual | Status |
|---|--------|----------|--------|--------|
| 1 | Console contains "FileWatchEvent: created" | Match | No match | FAIL |

**Output** (bounded excerpt — see Truncation):
```
Error: Permission denied for inotify
```

**Actions run:** `dart run bin/watcher.dart /tmp/watcher-test` (bg) · `touch /tmp/watcher-test/test.jsonl` (0.1s, exit 0)
**Analysis:** {what went wrong, one short paragraph}
**Phase** (ERROR only): setup | action | assert · **Limit** (TIMEOUT only): {elapsed}s of {limit}s
```

Failed quality gates likewise: the gate's failing output excerpt under `## Failures`.
A passing report carries **no prose** — no evidence tables, no narration; the captured
evidence stays in logs/scratch, pointed to by `evidence:` fields. One sanctioned exception,
spelled out below: the single `## Notes of note` line a non-blocking finding takes.

`## Failures` **is** this surface's failure narrative, and with it the sanctioned set closes
(`report-format.md` rule 2): that section, notes of note, and a null-exit block where the
workflow defines one — nothing else. A `## Findings`, `## Judgment-call grades`, or
`## Checkpoint recommendation` section is outside it: a blocking finding is a `## Failures`
row, and the recommendation is the `recommendation:` field. A **non-blocking** finding on an
otherwise clean run has an in-set home — `## Notes of note`, one ID-cited line, which is the
sole prose a passing report may carry; the no-prose rule above still bans evidence tables and
narration. Register per envelope rule 8 — `## Failures` writes `full`, since the lead scopes
its retry from it; whatever non-diagnostic prose remains writes `ultra`. And, restated here
because this is where the report is authored — **prose on a clean report is a defect**
(envelope rule 9): `status: pass` with a body section **outside that set** is not a clean
report; it fails the clearing conditions and returns to the lead.

### Examples

Passing per-cycle report (complete file):

```markdown
---
report: verification
feature: user-auth
cycle: 3
attempt: 1
status: pass
test_tasks:
  - {id: C3-gate, classification: CLI, status: pass, asserts: "4/4", duration: 3.2s}
quality_gates:
  lint:  {status: pass, command: "pnpm lint"}
  build: {status: pass, command: "pnpm build"}
  tests: {status: pass, command: "pnpm test", passed: 47, failed: 0, skipped: 2}
recommendation: approve
---
```

Failing per-cycle report:

```markdown
---
report: verification
feature: user-auth
cycle: 4
attempt: 2
status: fail
test_tasks:
  - {id: C4-gate, classification: CLI, status: fail, asserts: "1/2", duration: 5.1s}
quality_gates:
  lint:  {status: pass, command: "pnpm lint"}
  build: {status: pass, command: "pnpm build"}
  tests: {status: fail, command: "pnpm test", passed: 46, failed: 1, skipped: 2}
recommendation: reject
---

## Failures

### C4 gate — FAIL

| # | Assert | Expected | Actual | Status |
|---|--------|----------|--------|--------|
| 1 | Console contains "FileWatchEvent: created" | Match | No match | FAIL |
| 2 | File exists: /tmp/watcher-test/test.jsonl | Exists | Exists | PASS |

**Output:**
```
Starting file watcher...
Watching directory: /tmp/watcher-test
Error: Permission denied for inotify
```

**Actions run:** `dart run bin/watcher.dart /tmp/watcher-test` (bg) · `touch /tmp/watcher-test/test.jsonl` (0.1s, exit 0)
**Analysis:** The watcher failed to start (inotify permissions); the touch succeeded but no
event was detected. Same failure as the gate's `watcher_test.dart` red.
```

## Checkpoint Presentation

After generating the report, present the checkpoint to the human (in-memory — never
persisted; regenerate full evidence on "View Details").

### All Pass

```
AskUserQuestion(
  questions: [{
    question: "Verification C{N} gate passed.\n\nAll {count} assertions passed in {time}s.\n\nRecommendation: Approve",
    header: "Checkpoint",
    options: [
      {label: "Approve", description: "Proceed to next task"},
      {label: "View Details", description: "Show full evidence"},
      {label: "Retry", description: "Re-run verification"}
    ],
    multiSelect: false
  }]
)
```

### Any Failure

```
AskUserQuestion(
  questions: [{
    question: "Verification C{N} gate needs review.\n\n{pass}/{total} assertions passed.\nFailed: {failed_assert_summary}\n\nRecommendation: {recommendation}",
    header: "Checkpoint",
    options: [
      {label: "Approve", description: "Accept despite failures"},
      {label: "Reject", description: "Block completion"},
      {label: "Retry", description: "Re-run with adjustments"}
    ],
    multiSelect: false
  }]
)
```

### If Retry Selected

```
AskUserQuestion(
  questions: [{
    question: "What adjustments should be made?",
    header: "Retry",
    options: [
      {label: "Increase timeout", description: "Add more time for slow operations"},
      {label: "Retry as-is", description: "Run again without changes"},
      {label: "Skip assertion", description: "Remove problematic assertion"}
    ],
    multiSelect: false
  }]
)
```

## Truncation

Failure evidence in the report is bounded; full evidence always survives in a log file the
report points to:

- **Output excerpts:** if over 50 lines, include first 25 / last 25 with `[{N} lines
  truncated]` between, plus the full-log path: `` Full log: /tmp/claude/verify-C{N}-gate-output.log ``
- **Assert tables:** if more than 10 asserts, show the failing rows plus the first passing
  rows to 10 total, with a count note.

## Storage

- **The verification report file** is persisted at the path the caller names (in implement:
  per-cycle and final-validation reports in the feature/slice directory) — it is what the
  lead Reads for the verdict and what a resumed run finds as workspace evidence.
- **The checkpoint presentation** is generated in memory, shown at the gate, and discarded
  after the human decision.
- **Captured evidence** (full console output, screenshots, logs) lives outside the report
  (`/tmp/claude/…` or the caller's scratch), referenced by path from `evidence:` fields and
  truncation pointers.
~~~
- **Kept deliberately:** the `## Checkpoint Presentation` section (the in-memory AskUserQuestion blocks: All Pass / Any Failure / If Retry Selected) is **NOT folded into the schema** — it is out of scope (the in-memory checkpoint surface, not a file report). The whole reference file is reproduced verbatim above so the OLD file is reconstructible (GI-006). **FLAG (blocking for phase 2):** the checkpoint-presentation prose currently lives ONLY in this reference file (skill body `testing-end-user/SKILL.md:206,212` merely points at it). Deleting the reference at phase 2 would LOSE the checkpoint presentation unless it is first relocated into `testing-end-user/SKILL.md`. That relocation is unassigned in the plan and MUST happen before the phase-2 deletion — flagged to the delivery manager; NOT actioned by this seat.
- **Consumers assessed:** consumers — the implement workflow's per-cycle and final-validation verification runs (the lead Reads the file for the verdict). The skill-body pointers `testing-end-user/SKILL.md:76`, `:150`, `:203`, `:212` reference `references/REPORT-TEMPLATES.md` by path. **FLAG:** the plan's re-point inventory (§4) does NOT list these B2 skill-body pointers, nor a router row for verification-report — re-point ownership unassigned. Flagged to the delivery manager; NOT actioned by this seat (schemas + strips only).

## [v0.77.0] `references/REPORT-TEMPLATES.md` consumer pointers → the `verification-report` schema (two-arm) — D3 later-ratchet
- **Disposition:** superseded → the `verification-report` schema (`mochiko-cli template verification-report`, or Read `plugins/mochiko/schemas/verification-report.yaml` raw when the binary is absent). Four SKILL.md pointers re-pointed: the Generate Report line, the Quality Gate Report Format line, the anti-pattern truncation-rules cell (named-only; delivery single-sourced at the Reference Files entry), and the Reference Files entry.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, verbatim):**

```text
SKILL.md:76       Machine-first, per [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md): the
SKILL.md:149-150  (status from exit code, command, pass/fail/skip counts for suites) is defined in
                  [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md).
SKILL.md:203      | Truncating evidence prematurely | Critical failure information cut from the report | Follow REPORT-TEMPLATES.md truncation rules; include log-file locations |
SKILL.md:212      - [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md) — the verification-report file format (frontmatter + failure-only prose), checkpoint presentation, truncation
```
- **Kept deliberately:** the machine-first report doctrine, the quality-gate frontmatter rules, and the truncation-rule obligation — only the reference-file token was superseded.
- **Consumers assessed:** the `references/REPORT-TEMPLATES.md` → `verification-report.yaml` conversion + file deletion is P1 (B2) scope; if P1 records that deletion in this same strip file it is a companion entry — **this strip file is a shared write surface** (flagged in the P5 report). Cold re-grep confirms the only surviving `REPORT-TEMPLATES` reference is the FORBIDDEN `templates/report-format.md` "Consumed by" line (flagged as out-of-scope drift for the template owner).

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
