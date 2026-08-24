# ADR — Verbosity-envelope enforcement (plan-run artifact classes)

**Date:** 2026-08-22 · **Status:** ruled (user), built same day · **Driver:** kinako EPIC-001
dogfood run, 2026-08-22.

## Evidence

The kinako EPIC-001 epic plan run wrote ~906KB across 55 files in one day — design phase only,
zero product code. The epic spine alone: 362KB in 6 files, of which the review/contest layer is
190KB — the reviews outweigh the artifacts reviewed (the same pathology the
workflow-token-reduction epic ruled on for cycle reports). Worst files: epic `architecture.md`
113KB · `reviews/architecture-feasibility.md` 86KB · `reviews/baseline-reconstruction.md` 56KB ·
`proposal.md` 49KB · `contest-brief.md` 48KB · specify's `derivation.md` 50KB. Character of the
bloat: process self-narration (30+-line lineage preambles), literary register, freehand
proof-of-hunt essays, code quoted verbatim where `file:line` pointers suffice, table cells
carrying paragraph rationale.

## Root causes (verified against the primitives)

1. **Unenveloped artifact classes.** `templates/artifact-format.md` scoped only the named
   pipeline chain; `templates/report-format.md` governs reports but **no command cites either
   template** (grep over `plugins/mochiko/commands/`: zero hits). The epic-run artifacts that
   ballooned — proposal, contest brief, epic architecture delta, plan-run reviews, derivation —
   had no format home, no template, no cap. Exact repeat of the `task-mapping.md` precedent
   (freehand artifact → 45.9KB → canonical compact form).
2. **Pathless template binding.** `review-feasibility/SKILL.md` referenced "the
   feasibility-report template" twice with no file path; `plan.md` never named it. The seat
   never saw the strict frontmatter template that would have made the 86KB review ~2KB.
3. **Unbounded proof-of-hunt.** `review-feasibility`'s "absence of looking is not [evidence]"
   plus default-FAIL obliges the reviewer to evidence the hunt, with no bounded field to put it
   in — the proof lands as freehand narrative ("What the pass covered, and at what depth").
4. **Reviewers gagged on verbosity.** `artifact-format.md` rule 8 ("never prose volume") and
   rule 4 ("reported, never graded") made bloat un-gradeable in either direction; the sole
   lever (producer self-report of the overage delta) was unenforced.
5. **Register never delivered.** `output-style.md` sets reports = `ultra`; `plan.md`'s Register
   line bound only user-facing prose, so seat briefs never carried it.

## Ruling

The user ruled the fixes applied, priority-ordered, as one landing:

- **`templates/artifact-format.md`** — scope widened to command-minted deliverables (epic
  proposal · contest brief · architecture delta · specify's derivation); rule 4/8 amended:
  brevity stays never-a-finding, but prose volume past the rule-4 defaults **without a
  disclosed justification** becomes an advisory finding a reviewer names; new rule: **no
  process self-narration** — provenance is one line, creation/review lineage lives in the run
  record/manifest, never the artifact. Format version v2 → v3.
- **`templates/report-format.md`** — rule 9's mechanical prose-bounce widened from the
  implement cycle checkpoint to **every report a lead collects**. Format version v2 → v3.
- **`commands/plan.md`** — Tools gains the report-envelope binding **with paths**
  (`templates/report-format.md`, `templates/feasibility-report-template.md`; the
  baseline-reconstruction review is a report, not a freehand essay); the proposal and contest
  brief get a prescribed compact shape (the `patterns-plan-minimalism` disclosure grammar —
  element table + one-line rung stops / one-line contest verdicts, no narrative); epic-spine
  artifacts bound to the deliverable envelope; seat briefs carry the register.
- **`skills/review-feasibility/SKILL.md`** — widened mid-landing by a further user ruling:
  **break up and cut the body by at least 90%** ("cut now, eval validates later" — the
  skill-compression eval pilot for this skill becomes a post-cut regression check rather than
  the cut's instrument, superseding its pre-arm role from the same-day compression ruling).
  Landed: body 18,959 → 1,893 chars (−90.0%), now floors + a mandatory-load dispatch line;
  unique content relocated into `references/FEASIBILITY-LENS.md` (class 7 section, merged
  reviewer-guardrails table, architecture pass renumbered A1–A3 to clear the class-7 numbering
  collision, gate-fuel field names repaired to the template's `gap / at / impact / fix`);
  template references gain real paths; hunt coverage is disclosed as **one line per class**,
  never a narrative section. Full disposition map + KEPT reconciliation:
  `.mochiko/strips/review-feasibility.md` [v0.82.0]. Body budget re-seeded 19,058 → 2,367.

Kept deliberately: rule 8's substance-only grading direction (reviewers still never grade
style, and brevity is still never a finding); the proof-of-hunt *obligation* itself (only its
form is bounded); `patterns-system-design` untouched — its one-line register grammar and
Common Mistakes already forbid what kinako's delta did; enforcement now exists downstream.

Out of scope, open: a compact canonical form for `derivation.md` beyond the envelope binding
(specify-side; take it up if the next dogfood run still balloons it); the same envelope sweep
over `specify.md` / `implement.md` command briefs.

## Trail

Shipped at v0.82.0. Strips: `.mochiko/strips/artifact-format.md`, `report-format.md`,
`plan.md`, `review-feasibility.md`. Audit: independent `mochiko:validator`, charter-form
exception applied to `plan.md`. Watch: next kinako epic run's on-disk artifact sizes are the
falsifier — reviews should land at report-envelope scale (KBs, not tens of KBs).
