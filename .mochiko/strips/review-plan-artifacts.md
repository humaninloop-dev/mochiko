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
