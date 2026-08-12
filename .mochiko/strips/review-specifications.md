# Strip notes — `skills/review-specifications`

Entry formats: `strips/README.md`. Wave context: the specify cluster wave (v0.13.0).
Single-consumer primitive (the specify critique, mounted on `devils-advocate`) — strips
ruled in-wave.

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
