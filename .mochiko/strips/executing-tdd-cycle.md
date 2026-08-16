# Strip notes — `skills/executing-tdd-cycle/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D3 + S8 + Q6; rulings ratified
2026-07-23) — reports strip to their verified consumers, machine-first. Skill-succinctness
wave-1 entries atop (batch-ratified 2026-07-25): body 164 → 140 lines, 24 cut = 15% — in the
10–40 previously-stripped band.

Verbosity/caveman wave-1 entries atop (design:
`.mochiko/brainstorms/verbosity-caveman-ops-separation/record.md`, D4 as folded at review
(S2/S13); ruling: `DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation
ruled" row) — the report repair: the format text that forced prose onto passing cycles is
corrected, and the envelope's register and prose-on-clean check are bound where the report is
actually authored.

## [v0.77.0] Reference `references/CYCLE-REPORT-FORMAT.md` DELETED at the phase-2 gate (companion to the entries below)
- **Disposition:** deleted — the reference's content was superseded into `schemas/cycle-report.yaml` (the reference-supersession entry below, P1 phase 1) and its two SKILL.md consumer pointers were re-pointed to the two-arm `cycle-report` form (the consumer-pointer entry below, authored by the P5 re-point seat, which explicitly anticipated this companion deletion entry). This entry records the file deletion landing after the V1 fidelity audit PASSED.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row).
- **Content:** the whole `references/CYCLE-REPORT-FORMAT.md` is embedded verbatim in the reference-supersession entry below — not duplicated here (GI-006 reconstruction is satisfied there).
- **Consumers assessed:** cold re-grep after the phase-2 landing confirms NO `CYCLE-REPORT-FORMAT` reference remains anywhere in `plugins/`. No router row added for `cycle-report` (plan-minimalism ruling — matches architect-report). Shared-write-surface note: the re-point seat (P5) and P1 both wrote this strip; the three v0.77.0 entries are disjoint (re-points · reference→schema supersession · this deletion).
## [v0.77.0] Reference `references/CYCLE-REPORT-FORMAT.md` superseded by schema — `schemas/cycle-report.yaml` + `mochiko-cli template cycle-report`
- **Disposition:** superseded → `schemas/cycle-report.yaml` + `mochiko-cli template cycle-report` (D8 raw-Read fallback when the binary is absent). Scope is the reference file ONLY — the `executing-tdd-cycle` SKILL.md body is untouched by this seat.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class B2 report-format skill references per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters (Identity, Decomposition, File & dependency changes, Deviations & self-assessment) + the two sanctioned conditional-prose blocks as optional sections (Notes of note, Failure narrative); the Field Definitions table folds into the per-section `contract:`; the two worked Examples ride as section `good:` values (the passing cycle on Decomposition, the failed attempt on Failure narrative); `skeleton:` = the annotated frontmatter schema block + the two prose-block stubs. The cycle-specific sanctioned-set doctrine (exactly two H2, a third is a defect; prose-on-clean is a defect) rides `overview:`; the SHARED report doctrine stays `form: report-format.md`, pointer only. `--check` lines authored NET-NEW (no checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
# Cycle Report Format

The implementer produces a `cycle-report.md` after each cycle execution (and after each
rework). Envelope + shared rules: `templates/report-format.md` (machine-first, conditional
prose, no restatement) — this file carries only the cycle payload. The report is a truthful
self-disclosure of what happened — not a verdict on whether the result passes; the lead
owns that verdict and the verifier grades independently. Consumers: the lead's checkpoint
verdict (the frontmatter) · the verification seat's code-minimalism lens
(`mochiko:review-code-minimalism` reads the disclosed decomposition and its rung claims
alongside the cycle's diff) · and, on failure, the debugging trail (the failure narrative).

## Frontmatter Schema

```yaml
---
report: cycle
feature: user-auth
cycle: 3                    # Cycle number (integer) or "fix" for fix passes
attempt: 1                  # Attempt number within this cycle (1 = first attempt)
status: pass | fail | blocked   # Execution outcome self-report (not the checkpoint verdict)
decomposition:              # The build-time task breakdown of the card (this run's, disclosed)
  - {id: T3.1, task: "failing test for POST /api/session", path: src/routes/session.test.ts, rung: 7}
  - {id: T3.2, task: "session route handler", path: src/routes/session.ts, rung: 7}
tasks_total: 4              # Total tasks in the decomposition above
tasks_completed: 4          # Decomposition tasks completed
failed_tasks: []            # Task IDs not completed / failing, with a one-line reason each
files_created:              # New files created during this cycle
  - src/routes/api.ts
  - src/routes/api.test.ts
files_modified:             # Existing files modified during this cycle
  - src/models/user.ts
brownfield_tasks: 1         # Count of decomposition tasks classified extend/modify
domain_deps_added: []       # Domain-layer registry additions this cycle (package names; [] if none)
deviations: []              # One-line, ID-cited departures from the task descriptions ([] if none)
checkpoint_criteria_met: true  # The implementer's self-assessment (the lead verifies independently)
---
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report` / `feature` / `slice` | envelope | yes | Per `templates/report-format.md` (`slice:` only when slice-scoped) |
| `cycle` | integer or `"fix"` | yes | Cycle number from tasks.md, or `"fix"` for fix passes |
| `attempt` | integer | yes | 1 for first attempt, increments on retry |
| `decomposition` | list | yes | The build-time task breakdown of the card — `{id, task, path, rung}` per task, IDs local to this report (`T{cycle}.{n}`). `rung` is the pre-code ladder choice per `mochiko:patterns-code-minimalism` (1–7; a rung-1 skip is a decomposition entry with no path, its why one line in `task:`). The disclosure surface for the decomposition (the card in `tasks.md` stays undecomposed); rework and failure reports cite these IDs; the verification seat grades the rung claims |
| `status` | enum | yes | `pass` (all tasks done, tests green) / `fail` (tasks failing) / `blocked` (could not proceed). A self-report of execution outcome, not the checkpoint verdict |
| `tasks_total` | integer | yes | Number of tasks in the decomposition |
| `tasks_completed` | integer | yes | Decomposition tasks completed |
| `failed_tasks` | list | yes | `[]` if none; else `- {id: T3.2, why: "<one line>"}` per failed/incomplete task |
| `files_created` | list of strings | yes | Paths of new files created (empty list if none) |
| `files_modified` | list of strings | yes | Paths of existing files modified (empty list if none) |
| `brownfield_tasks` | integer | yes | Count of decomposition tasks classified extend/modify (per the card's brownfield exposure) |
| `domain_deps_added` | list of strings | yes | Domain-layer dependency registry additions made this cycle (empty list if none). The visibility floor for registry growth: additions are disclosed here and surfaced at the checkpoint; a non-empty list always forces a human checkpoint — never auto-approved |
| `deviations` | list of strings | yes | Departures from the task descriptions, one line each, citing the task ID (e.g. `"T3.4: argon2 over bcrypt (C-012 allows)"`). `[]` if none |
| `checkpoint_criteria_met` | boolean | yes | The implementer's assessment of whether the cycle's checkpoint criteria are satisfied; a self-report, not the verdict — the lead verifies independently and decides |

## Conditional Prose

Prose sections per the conditional-prose rule (`report-format.md`) — a clean passing cycle
is frontmatter-only. This file's sanctioned set is **exactly the two sections below**; a
third H2 is a defect. Register per envelope rule 8: `## Notes of note` writes `ultra`, the
failure narrative `full`. And, restated here because this is where the report is authored —
**prose on a clean report is a defect** (envelope rule 9): `status: pass` with any body
section beyond those two is not a clean report; it fails the clearing conditions and returns
to the lead.

### Notes of note *(only when non-empty)*

```markdown
## Notes of note
```

The producer-authored uncertainty carrier: non-obvious decisions (pattern choices where
multiple valid approaches existed, technology choices within a task's scope — one line
each, ID-cited), genuine difficulties, and flagged blockers the lead should weigh at the
checkpoint. Not a narration of what the tasks already describe; cite IDs, never restate
task text.

### Failure narrative *(mandatory when `status` is `fail` or `blocked`, or a task failed in execution)*

```markdown
## Failure narrative
```

Debug value concentrates here — full detail: what failed (per failed task), why (the
failing test/output evidence), what was tried, and the state things were left in. A failed
cycle keeps the fuller narrative; the slim format above is the passing-cycle format.

**A task you were never meant to run is not an execution failure.** A verifier-owned
`**TEST:**` gate the producer must not execute belongs in `failed_tasks:` with that one-line
reason, and it triggers no narrative: a cycle whose only incomplete task is one of those,
everything else green, is a clean passing cycle and stays frontmatter-only.

## Examples

Passing cycle (complete report):

```markdown
---
report: cycle
feature: user-auth
cycle: 3
attempt: 1
status: pass
decomposition:
  - {id: T3.1, task: "failing E2E test for profile update", path: src/routes/api.test.ts, rung: 7}
  - {id: T3.2, task: "profile update route handler", path: src/routes/api.ts, rung: 7}
  - {id: T3.3, task: "wire route into router", path: src/routes/api.ts, rung: 6}
  - {id: T3.4, task: "[EXTEND] lastLogin field on User", path: src/models/user.ts, rung: 2}
tasks_total: 4
tasks_completed: 4
failed_tasks: []
files_created:
  - src/routes/api.ts
  - src/routes/api.test.ts
files_modified:
  - src/models/user.ts
brownfield_tasks: 1
domain_deps_added: []
deviations:
  - "T3.4: followed existing Sequelize patterns for the [EXTEND] on User"
checkpoint_criteria_met: true
---
```

Failed attempt:

```markdown
---
report: cycle
feature: user-auth
cycle: 4
attempt: 2
status: fail
decomposition:
  - {id: T4.1, task: "failing test for token refresh", path: src/middleware/auth.refresh.test.ts, rung: 7}
  - {id: T4.2, task: "refresh contract test", path: src/middleware/auth.contract.test.ts, rung: 7}
  - {id: T4.3, task: "refresh handling in auth middleware", path: src/middleware/auth.ts, rung: 2}
tasks_total: 3
tasks_completed: 2
failed_tasks:
  - {id: T4.3, why: "auth middleware test red — token refresh race"}
files_created: []
files_modified:
  - src/middleware/auth.ts
brownfield_tasks: 0
domain_deps_added: []
deviations: []
checkpoint_criteria_met: false
---

## Failure narrative

T4.3's refresh test (`auth.refresh.test.ts:41`) fails intermittently: the refresh handler
reads `user.lastLogin` before C3's write commits. Tried serializing on the session row
(still races under the test's parallel logins) and moving the read behind the commit hook
(breaks the T4.2 contract test). The middleware currently guards with a retry, which the
test's timing still beats about 1 run in 5. Left red; needs a decision between a
transaction boundary change (touches C3 code) and relaxing the timing assertion.
```
~~~
- **Kept deliberately:** nothing dropped — every frontmatter field, every Field-Definitions row, both conditional-prose sections, the "a task you were never meant to run is not an execution failure" rule, and both Examples have a home in the schema. The `executing-tdd-cycle` SKILL.md body and its execution discipline are untouched.
- **Consumers assessed:** consumers per the reference's own header — the lead's checkpoint verdict (the frontmatter), the verification seat's code-minimalism lens (`mochiko:review-code-minimalism`), and the failure-narrative debugging trail. The skill-body pointers `executing-tdd-cycle/SKILL.md:91` and `:158` reference `references/CYCLE-REPORT-FORMAT.md` by path. **FLAG:** the plan's re-point inventory (§4) does NOT list these two B2 skill-body pointers, nor a router row for cycle-report — their re-point ownership is unassigned. Flagged to the delivery manager; NOT actioned by this seat (schemas + strips only).

## [v0.77.0] `references/CYCLE-REPORT-FORMAT.md` consumer pointers → the `cycle-report` schema (two-arm) — D3 later-ratchet
- **Disposition:** superseded → the `cycle-report` schema (`mochiko-cli template cycle-report`, or Read `plugins/mochiko/schemas/cycle-report.yaml` raw when the binary is absent). Two SKILL.md pointers re-pointed: the Write Cycle Report step and the Reference Files entry.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, verbatim):**

```text
SKILL.md:91   Produce `cycle-report.md` following the format in [references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md) — the decomposition (task list with file paths and ordering) is part of the report's structured fields.
SKILL.md:158  - [references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md) — Structured YAML frontmatter schema (incl. the decomposition fields) and the conditional prose rules
```
- **Kept deliberately:** the cycle-report production step and the decomposition-fields note — only the reference-file token was superseded.
- **Consumers assessed:** the `references/CYCLE-REPORT-FORMAT.md` → `cycle-report.yaml` conversion + file deletion is P1 (B2) scope; if P1 records that deletion in this same strip file it is a companion entry — **this strip file is a shared write surface** (flagged in the P5 report). Cold re-grep confirms no `CYCLE-REPORT-FORMAT` references remain outside the FORBIDDEN `templates/report-format.md` "Consumed by" line (flagged as out-of-scope drift).

## [v0.75.0] TASK-PARSING.md — foundation/feature card-type field superseded; test-case-bundle + Covers extraction added

- **Disposition:** superseded → the re-keyed `references/TASK-PARSING.md` Card Pattern and Fields-to-Extract: no card-type annotation, `[P]` derived from dependencies, the card's `**TEST:**` blocks parsed as the named test-case bundle (each with a `Covers` citation line). Execution discipline unchanged.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Vertical-TDD cycle anchor + QA test-case authorship (D1–D4)"; record `.mochiko/brainstorms/vertical-tdd-complexity-and-qa-role/record.md`, D1/D3 + the D2 acceptance-ID-relocation amendment). Scope is the reference only — the `executing-tdd-cycle` SKILL.md body is untouched.
- **Content:**
  - Card Pattern heading "`### - [ ] Cycle {N}: {title} *({Foundation|Feature})* ` `[P]`?`" → "`### - [ ] Cycle {N}: {title} ` `[P]`?`" (the `*({Foundation|Feature})*` type annotation removed); the pattern gains a `- **Covers**: spec/plan IDs this case covers` line inside the `**TEST:**` block and a note that a card may carry more than one `**TEST:**` block (the bundle) with Cycle 1 of a new path a walking skeleton.
  - Fields-to-Extract row "`Type + [P] | Foundation cards run sequentially, first; [P] marks parallel-eligible feature cards`" → "`[P] | Marks a parallel-eligible card — derived from dependencies, not a card type`".
  - Fields-to-Extract row "`Acceptance criteria | Cited IDs — resolve against the spec/plan artifacts; these bound the decomposition`" removed → merged into a re-keyed "`**TEST:** blocks`" row: "The card's named test-case bundle — the expected behaviour the cycle must ultimately demonstrate green; these bound the decomposition. Each block's `Covers` line cites the spec/plan IDs it verifies. Parse only to know what the cycle must prove; running the cases is `testing-end-user`'s work".
  - Full prior text: git history at v0.74.x.
- **Kept deliberately:** the **Current-Cycle Identification** rule (first unchecked card in file order, all `Depends on` checked; flip at step 6) and the **Quality Gates Pattern** section — execution discipline untouched (D4: the skill consumes the cards, does not re-decide slicing). The Checkbox / Stories / Depends on / Brownfield-exposure extraction rows kept, re-keyed only where they named the type.
- **Consumers assessed:** this reference is loaded by `executing-tdd-cycle/SKILL.md` only (build-time card reader). Upstream owner `patterns-vertical-tdd` re-keyed same wave (card shape); `tasks-template.md` re-keyed same wave (the card shape parsed here); `testing-end-user` runs the `**TEST:**` cases (grammar unchanged, ignores `Covers`). No other consumer reads this file.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 10,151 → 9,676 chars (−5%); description 1,082 → 498
  chars (−54%). Body cut: the **When to Use** section deleted whole (five bullets restating the
  description's invocation conditions — executing a cycle card through decompose→r/g/r, reworking
  failed tasks, fixing a reported failure, the brownfield-exposure co-fire, writing the
  cycle-report; each obligation survives in the Overview, the Core Process sequence, the
  Reworking / Fixing subsections, and the Green-Phase `brownfield-integration` co-fire steps).
  Description cut: the parenthetical red/green/refactor step spell-out and several SHOULD trigger
  phrases compressed; the MUST clause, core trigger, the decomposition-disclosed obligation, and
  the `patterns-vertical-tdd` design-time/runtime sibling distinction kept. Verbatim homes: git
  history of this file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when executing a cycle card at runtime — turning one card from `.mochiko/specs/<feature>/tasks.md` into working code by decomposing the card into concrete tasks (build-time, code in view), driving each task through the red→green→refactor execution sequence (write the failing test, run it, confirm it fails for the right reason, implement the minimum to pass, refactor only this cycle's code), flipping the card's checkbox, and writing the `cycle-report.md` with the decomposition disclosed. SHOULD also invoke when "execute cycle", "implement the cycle card", or "write the cycle report" is the work at hand; when reworking the specific tasks reported as failing (targeted, test-first rework); when reproducing a reported failure with a failing test before fixing it; or when the card's brownfield exposure names existing code. This is the runtime EXECUTION of cycles — decomposition included. Deciding WHAT the cycles are (the slicing, the cards, the TEST gates) is design-time work owned by `mochiko:patterns-vertical-tdd`, upstream and not this skill.
- **Kept deliberately:** the guardrails keep-set — the Overview + letter/spirit epigraph, When
  NOT to Use (the vertical-tdd / verifier / lead / loop-state boundaries), the entire Core
  Process (Read → Decompose → Red → Green → Refactor → Flip → Write, incl. the pre-code-ladder
  step), Progress Tracking, Reworking Specific Failed Tasks, Fixing a Reported Failure, Red
  Flags, the Common Mistakes table, and the Reference Files pointers.
- **MANDATORY KEPT reconciliation:** the [v0.44.0] KEPT entry protects the envelope's register +
  prose-on-clean check, but that content lives in `references/CYCLE-REPORT-FORMAT.md`, NOT the
  SKILL body — untouched by this body cut. The [v0.49.0] and [v0.53.0] supersessions KEPT the
  cycle-boundary restriction, red/green/refactor strict order, rework-only-failed-tasks, fix-pass
  scoping, the verifier boundary, and the report self-disclosure framing — all live in Core
  Process / When-NOT-to-Use / Progress Tracking, none in the deleted When-to-Use. Progress
  Tracking (the machine-first report obligation) was deliberately kept, not cut. No prior KEPT or
  protected line is touched.
- **Consumers assessed:** staff-engineer (mounts it) · implement (binds it) · qa-engineer /
  review-code-minimalism (read the disclosed decomposition) · brownfield-integration (co-fires) ·
  patterns-vertical-tdd, patterns-code-minimalism (cross-reference) · mochiko router. None links
  the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.53.0] Cycle-report consumer line: lead-only → lead + verification seat
- **Disposition:** superseded → `references/CYCLE-REPORT-FORMAT.md`'s widened consumer line: the verification seat's code-minimalism lens (`mochiko:review-code-minimalism`) now reads the disclosed decomposition and its rung claims alongside the cycle's diff.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-05 "Ponytail code-minimalism ruled (PT-D1–D10)", record `.mochiko/brainstorms/ponytail-concepts-integration/record.md`, D8).
- **Content (verbatim, the superseded consumer statement):**
  ```
  Consumers: the lead's checkpoint
  verdict (the frontmatter) and, on failure, the debugging trail (the failure narrative).
  ```
- **Why the ruling retired it:** D8 — the qa lens grades produced code against the builder's ladder claims; it needs both the diff and the disclosure, and lead-relay would make doctrine out of relay quality. The `decomposition` rows gained a `rung:` note in the same edit (pure addition).
- **Kept deliberately:** the self-disclosure framing ("not a verdict"), the lead's verdict ownership, the verifier-grades-independently line — all verbatim.
- **Consumers assessed:** implement (lens wiring landed same wave) · qa-engineer + `review-code-minimalism` (the new read edge's owner) · staff-engineer (discloses rungs, unaffected as author).

## [v0.49.0] Decomposition restriction removed — builder decomposes the card (step 2)
- **Disposition:** superseded → the same skill's new "Decompose the Card" step (build-time tasks + file paths, code in view, disclosed in `cycle-report.md`'s new `decomposition` field); `references/TASK-PARSING.md` rewritten from `TN.X` task-line parsing to cycle-card reading
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2.1)
- **Content:** the restriction, both homes — Overview "it does not structure the cycles or decide when they run" (structuring half) and When-NOT-to-Use bullet 1 "Structuring the cycles — identifying the vertical slices, ordering a cycle's tasks test-first, or authoring the `tasks.md` skeleton — is design-time work owned by `patterns-vertical-tdd` … it does not create, split, or reorder tasks." · TASK-PARSING.md's task grammar (`- [ ] **T{N}.{X}**:` pattern, ID-prefix cycle identification, backtick path extraction, `[EXTEND]`/`[MODIFY]` marker table, multi-line sub-bullets, Checkpoint pattern) · "Mark each task `[x]` in `tasks.md` immediately after completing it" (now: flip the card at cycle close). Full text: git history at v0.48.0.
- **Kept deliberately:** the cycle-boundary restriction (does not add/remove/re-scope *cycles*) — decomposition is unlocked, slicing is not · red/green/refactor strict order · rework-only-failed-tasks · fix-pass scoping · verifier boundary (TEST gates + quality gates never this skill's).
- **Consumers assessed:** staff-engineer (persona wording re-keyed) · implement · CYCLE-REPORT-FORMAT.md (decomposition field added same wave) · router.

## [v0.44.0] Failure-narrative trigger: "or any task failed" narrowed to execution failures
- **Disposition:** superseded → `references/CYCLE-REPORT-FORMAT.md`'s corrected trigger, *"or a
  task failed in execution"*, plus a carve-out paragraph under the same section naming the
  verifier-owned case explicitly.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D4 part 1,
  the F59 clause fix; `DECISIONS.md` 2026-08-01 row above).
- **Content (verbatim, the whole superseded line):**
  ```
  ### Failure narrative *(mandatory when `status` is `fail` or `blocked`, or any task failed)*
  ```
- **Why the ruling retired it:** every cycle report in the `author-navigate` driver run carried a
  non-empty `failed_tasks:` naming one verifier-owned `**TEST:**` gate the producer must not
  execute (record F59). Under *"or any task failed"* the format itself owed a narrative on a
  passing cycle — a producer following the shipped text arrived at prose correctly, and 8/8
  passing reports carried it (F58, 79.9% of report bytes). The text was the defect, not the
  author.
- **Kept deliberately:** the other two trigger conditions (`status` `fail` / `blocked`) verbatim;
  the section's full-detail body paragraph; and the `failed_tasks:` disclosure itself — the
  verifier-owned task is still listed with its one-line reason. Only the narrative obligation
  lifts.
- **Consumers assessed:** `skills/executing-tdd-cycle/SKILL.md` is the reference's only consumer
  (`:74`, `:81`, `:142`) and needs no edit — `:81` already reads *"a `Failure narrative` (full
  detail) whenever the cycle failed or was blocked"*, i.e. the corrected trigger, and `:79`
  already reads *"a clean passing cycle needs no prose"*. The defect was localized to the
  reference. `templates/report-format.md` rule 2 never carried a task-level clause, so it is
  unaffected.

## [v0.44.0] KEPT: the envelope's register + prose-on-clean check, restated in this payload home
- **Tier-2 evidence:** a deliberate exception to the no-restatement rule, recorded here so a
  later minimalism wave does not read it as Tier-1 duplication and relocate it. Ground: record
  F72 — the driver run's report prose was authored against this payload home, one hop *below*
  `templates/report-format.md`, where the stricter frontmatter-only rule already failed to reach
  (F58). D4's S2 fold names the restatement in both payload homes as a host of the check, so the
  binding is ruled, not stylistic. Cut it only with a ruling that also re-homes the check.

## [v0.25.0] Task-extraction preview list (net −7 lines)
- **Disposition:** relocated → `references/TASK-PARSING.md` (the declared parsing home, pointed at two lines above; pointer now names "the per-task fields to extract")
- **Tier failed:** 1 (preview copy of the reference's field list)
- **Content:** ID / description / file-paths / markers / sub-bullets extraction list
- **Consumers assessed:** 6 consumer files checked at wave open; none reference the list

## [v0.25.0] Common Mistakes densified: 4 subsections → 4-row table (net −17 lines)
- **Disposition:** compressed in place (densification, zero deletions — every mistake/failure/fix survives as a row)
- **Tier failed:** n/a — form only
- **Content:** tests-after-implementation, full-cycle re-implementation, refactor scope creep, failure-reason verification
- **Consumers assessed:** none reference the subsection headings

## [v0.25.0] Aphorism consequence-anchored (R4b rider, net 0 lines)
- **Disposition:** reordered in place — the existing consequence sentence ("TDD discipline exists to catch failures before they compound…") moved from mid-Overview to directly under the aphorism
- **Tier failed:** n/a — rider execution, not a strip
- **Content:** unchanged text, relocated within the file
- **Consumers assessed:** n/a

## [v0.22.0] Cycle-report prose sections (What Was Done · Decisions Made · Notes for Next Cycle)
- **Disposition:** deleted from `references/CYCLE-REPORT-FORMAT.md` (What Was Done, Decisions Made) · deleted per the epic's Q6 ruling, **no optional-field resurrection** (Notes for Next Cycle)
- **Tier failed:** consumption evidence (epic F-c): the user never reads cycle reports (Q4); the next cycle never reads the file (15/15 kinako reports authored the section, 0 back-references — the standing seat carries the context); fix/retry consumes a lead-relayed failure list
- **Content:** `### What Was Done` — narrative of what was implemented "in enough detail for the lead and the next cycle" (restated tasks.md); `### Decisions Made` — technology/pattern choices + trade-offs narrative; `### Notes for Next Cycle` — files/interfaces affecting future cycles, patterns established, potential conflicts, improvement opportunities. Replacements: non-obvious decisions + difficulties/blockers → the conditional `## Notes of note` block; deviations → the `deviations:` frontmatter list; failed cycles keep a mandatory `## Failure narrative` (S8). Improvement-opportunity noting (refactor-discipline pressure valve) retargeted to Notes of note (SKILL.md Common Mistakes).
- **Re-add trigger:** a dogfood run where the lead's checkpoint verdict or a fix pass demonstrably starved for the dropped narrative on a *passing* cycle (evidence-gated, marked override).
