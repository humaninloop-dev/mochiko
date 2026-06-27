# Analyst Report Template

This template defines the structure for the producer's self-disclosure report — authored alongside `spec.md` on each round and read directly by the lead to follow what was produced and what changed.

---

```markdown
# Analyst Report

> Feature: {{feature_id}}
> Round: {{round}}
> Generated: {{timestamp}}

---

## Summary

{{summary}}

---

## Assumptions Made

| ID | Assumption | Rationale |
|----|------------|-----------|
| A1 | {{assumption}} | {{rationale}} |

---

## What Changed This Round

{{changes_this_round}}

---

## Notes

{{notes}}

---

## What I Created (optional)

| Section | Count |
|---------|-------|
| User Stories | {{user_story_count}} |
| Edge Cases | {{edge_case_count}} |
| Functional Requirements | {{requirement_count}} |
| Key Entities | {{entity_count}} |
| Success Criteria | {{criteria_count}} |
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** It records what the producer authored and assumed; it carries no PASS/FAIL. The clearing verdict lives in the critic's review, and the lead owns the loop decision — never read this report as a gate.
2. **Foreground prose; the lead reads it directly.** Summary, Assumptions Made, and What Changed This Round are the decision-relevant content. There is no parser — write for a human-style reader.
3. **`{{round}}`** is the lead's bounded-loop round counter: round 1 is the first draft; later rounds are revisions following a critic review.
4. **What Changed This Round** — on round 1, state "Initial specification." On later rounds, list the specific gaps addressed since the previous round, so the lead can see progress (and detect a stalled, no-change round).
5. **`## What I Created` is optional.** The counts mirror sections of `spec.md`, which the critic and the lead read directly; include the table only if a quick snapshot helps. It is a convenience disclosure, not a tracked metric — omit it if it adds nothing.
6. **Output location** — the filled report lives at `.mochiko/specs/<feature>/analyst-report.md`, seeded and collected by the lead.
7. **This is a reference template** — the producer fills in actual content following this structure.
