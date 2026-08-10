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
