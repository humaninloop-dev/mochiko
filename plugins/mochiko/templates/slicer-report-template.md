# Slicer Report Template

This template defines the structure for the producer's self-disclosure report authored alongside a
graduation-slice decomposition (`slices.md`) on each round — read directly by the lead and the
reviewer to follow what was produced and what changed. It is also where a **null-exit** outcome
(recommend whole-spec, no `slices.md` written) is disclosed.

---

```markdown
# Slicer Report

> Feature: {{feature_id}}
> Round: {{round}}
> Generated: {{timestamp}}

---

## What Was Produced

{{production_summary}}

---

## What Changed This Round

{{changes_this_round}}

---

## Slicing Rationale

{{slicing_rationale}}

---

## Feature-Done Coverage

{{feature_done_coverage}}

---

## Governance Alignment

{{governance_alignment}}

---

## Open Questions

{{open_questions}}

---

## Handoff to Review

{{handoff_to_review}}

---

## Artifacts Produced (optional)

| Artifact | What's in it |
|----------|--------------|
| {{artifact}} | {{artifact_summary}} |
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** It records what the producer did this
   round; it carries no done-state and no PASS/FAIL. The recommended verdict lives in the reviewer
   report, and the lead owns the clearing decision — never read this report as a gate. There is
   deliberately no "Completion" and no "Ready for Review" field.
2. **Foreground prose; the lead and reviewer read it directly.** There is no parser — write for a
   human reader.
3. **`{{round}}`** is the lead's bounded-loop round counter: round 1 is the first draft; later
   rounds are revisions following a reviewer critique.
4. **What Was Produced** discloses the outcome shape: a drafted `slices.md` (slice count,
   foundation choice, story coverage) — **or the null exit**: recommend whole-spec, with the
   reasoning (fewer than two distinct value seams), and no file written. The null exit is a
   first-class outcome, not a failure.
5. **What Changed This Round** — round 1: "Initial decomposition" (or "Null exit recommended").
   Later rounds: the specific reviewer gaps addressed, so the lead can see progress and detect a
   stalled round.
6. **Slicing Rationale** discloses why the slices were cut this way — the dependency structure
   found, the foundation designation (both halves: shared core + testable journey), ordering
   choices, sizing calls, and cross-cutting placements with their extend obligations. Descriptive
   self-disclosure that helps the reviewer judge the decomposition; not a claim that it is correct.
7. **Feature-Done Coverage** discloses how the SC coverage map was closed (every SC-# to a
   verifying slice) and which cross-slice seams were identified — including any SC whose coverage
   forced a re-slice.
8. **Governance Alignment** discloses how the decomposition aligns to the project's governance (the
   CLAUDE.md governance region + its `.claude/rules/mochiko/` files) when present; **Open
   Questions** records producer-surfaced unknowns (including any un-homeable cross-cutting story
   flagged for spec amendment) and feeds the clarification loop.
9. **Handoff to Review** is a *pointer*, not a claim: name what the reviewer should grade and any
   known soft spots. It is **not** a "ready" assertion.
10. **Output location** — `.mochiko/specs/<feature>/slicer-report.md`, seeded and collected by the
    lead, alongside `slices.md` (when written).
11. **This is a reference template** — the producer fills in actual content following this
    structure.
