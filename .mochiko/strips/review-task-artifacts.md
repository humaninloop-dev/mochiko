# Strip notes — `skills/review-task-artifacts`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 2 (review-\* cluster,
batch-1 ratified 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
First strip assessment of this skill (never-stripped band 30–70): body 225 → 171 lines = **24%**,
under-band — survivor evidence below.

## [v0.49.0] Skill retired — folded into `review-plan-artifacts` as the cycle-card checks
- **Disposition:** superseded → `review-plan-artifacts` (new Cycle cards row in Review Focus: vertical integrity · TEST-gate presence/grammar · story traceability · sizing · dependency minimality · brownfield exposure · no pre-written task lists); directory deleted (SKILL.md + references/PHASE-CHECKLISTS.md + references/ISSUE-TEMPLATES.md)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D4: one grader per accepted artifact — the package is graded whole)
- **Content:** the 174-line SKILL.md (mapping/tasks/cross-artifact checks, TM-/TT-/TX- ID conventions, verdict decision tree, dormant brownfield checks note) + both references. Per-task checks (task-ID format, per-task file paths, TDD task ordering) died with the task lists themselves (D2); artifact-level checks were carried over compressed. Full text: git history at v0.48.0.
- **Kept deliberately:** the "Confirmed complementary — no structural merge" boundary note is superseded in the opposite direction by this ruling — recorded here so the reversal is explicit, not an oversight.
- **Consumers assessed:** devils-advocate (skills: roster edited) · router (row removed) · review-plan-artifacts (absorbing home edited in the same wave).

## [v0.46.0] loop-discipline pointers out
- **Disposition:** superseded → the anti-rationalization content stands in this file's own red flags; loop ownership is the command's
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** same two edits as `review-plan-artifacts` — the overview parenthetical reworded, the Related bullet deleted.
- **Consumers assessed:** plan command (structuring stage) briefs unchanged.

## [v0.26.0] "Key Principles to Validate" section deleted (−21 lines)
- **Disposition:** relocated → three verified homes, each Read before landing: the check
  enumeration is the in-file Review Focus table (story coverage / true vertical slices /
  foundation separation / TDD ordering / `**TEST:**` presence / traceability); the validation
  detail is `references/PHASE-CHECKLISTS.md` (Vertical Slice Validation, TDD Structure
  Validation, Traceability Matrix Validation — confirmed to hold every bullet, incl. foundation
  separation and `[P]` parallelization via the Mapping Checklist Table + ISSUE-TEMPLATES
  categories); the mirror principle is the Related bullet, which states it near-verbatim
- **Tier failed:** 1 (a third in-file statement of content already double-sourced)
- **Content:** the mirror-principle intro + Vertical slicing / TDD structure / Traceability
  bullet groups
- **Consumers assessed:** wave-open enumeration — 5 citing files; grep confirms nothing in the
  plugin references "Key Principles"

## [v0.26.0] Severity + verdict tables → ISSUE-TEMPLATES pointers; steps, pre-assert note, and Common Mistakes densified
- **Disposition:** relocated → `references/ISSUE-TEMPLATES.md` (severity held richer under
  Severity Classification — categories + examples per level; verdict mapping held under Verdict
  Decision Tree incl. the lead-owns-clearing line — both Read and confirmed before landing) ·
  deleted (the pre-assert note's "possible future parity item" clause — roadmap material; the
  behavioral core kept: no pre-assert exists, the review's core is model judgment no grep
  settles) · densified (form-only): Step-2/3/4 lists → sentences (Step 4's emit gloss dropped —
  the `advocate-report-template` holds the `at:`/`strengths:`/verdict shape, verified), six
  Common-Mistakes Bad/Good subsections → one 3-column table
- **Tier failed:** 1 (tables) · 2 (future-parity clause — names no current reviewer behavior) ·
  n/a for the densifications — form only
- **Content:** the two 3-row tables; the future-parity clause **and the greppable-slice
  enumeration** (task-ID `TN.X` format, file-path / `**TEST:**`-task / marker / traceability
  presence — every item held by the in-file Review Focus table and PHASE-CHECKLISTS' Tasks
  Checklist Table); step mechanics; Common-Mistakes subsection headers (all Bad/Good pairs
  preserved)
- **Consumers assessed:** 5 citing files, none reference the stripped tables or clause; the
  in-body *Verdict Criteria* heading survives as the pointer so Step 4's see-reference stays valid

## [v0.26.0] KEPT: the remaining body (under-band survivor evidence, 24% vs 30–70)
- **Tier-2 evidence:** contested at the pass and kept — the Overview verdict-ownership paragraph
  is the sanctioned placement (the v0.13.0 `review-specifications` precedent deduped *to* the
  description + Overview); the Scope table incl. "Confirmed complementary — no structural merge"
  guards a watched boundary between disjoint reviewers; the dormant-checks note is behavioral
  (tells the reviewer which checks not to run and when they activate); the envelope note names
  the density-is-never-a-finding failure; every Red-Flag bullet and rationalization row names a
  failure + counter. The aphorism copy (L21) is consequence-anchored — qualifies under the R4b
  rider, no edit needed. Session ruling: batch-1 ratification 2026-07-25.
