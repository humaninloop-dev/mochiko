# Slicer Report Template

The producer's self-disclosure report authored alongside a graduation-slice decomposition
(`slices.md`) on each round — read by the lead and the reviewer to follow what was produced
and what changed. Also where a **null-exit** outcome (recommend whole-spec, no `slices.md`
written) is disclosed. Envelope + shared rules (machine-first, no self-verdict, conditional
prose): `templates/report-format.md` — this file carries only the payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
round: {{round}}
produced: [slices.md]              # null exit: [] — no file written
null_exit: false                   # true = recommend whole-spec; reasoning in the prose block below
slices: {count: {{n}}, foundation: {{slice-id}}, stories_covered: "{{n}}/{{spec total}}"}
sc_coverage: "{{n}}/{{spec SC total}} mapped"   # every SC-# to a verifying slice
changed_this_round:                # round 1: [initial] (or [null-exit recommended]); later: reviewer gap IDs addressed
  - "{{G2: US7 rehomed to s2; extend obligation recorded on s3}}"
slicing_notes: []                  # judgment calls NOT already in slices.md (the rationale's home); [] on a clean round
governance_alignment: "{{aligned | the rules/GI IDs touched, one line}}"
open_questions: []                 # incl. any un-homeable cross-cutting story flagged for spec amendment
handoff: "{{what to grade + known soft spots, one line}}"
---

## Null-exit reasoning   <!-- mandatory when null_exit: true; omit otherwise -->

{{why fewer than two distinct value seams exist — the case for taking the spec whole}}
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** No done-state, no PASS/FAIL, no
   "Completion"/"Ready for Review" field (the envelope's no-self-verdict rule). The
   recommended verdict lives in the reviewer report; the lead owns the clearing decision.
2. **The null exit is a first-class outcome, not a failure** — `null_exit: true`,
   `produced: []`, and the mandatory `## Null-exit reasoning` block carrying the case
   (fewer than two distinct value seams).
3. **Machine-first; the rationale lives in the deliverable.** `slices.md` carries the
   slicing rationale, foundation designation, ordering, extend obligations, and the
   Feature-Done section — the reviewer grades it there. `slicing_notes` carries only
   judgment calls with no home in the artifact. Never restate overlay content; cite slice
   and SC IDs (`sc_coverage` discloses closure; the map itself is in `slices.md`).
4. **`changed_this_round`** — round 1: `[initial]` (or `[null-exit recommended]`). Later
   rounds: the specific reviewer gaps addressed by ID, so the lead can see progress and
   detect a stalled round.
5. **`handoff`** is a *pointer*, not a claim — **not** a "ready" assertion.
   **`open_questions`** feeds the clarification loop, including any un-homeable
   cross-cutting story flagged for spec amendment.
6. A routine round is frontmatter-only; add `## Notes of note` only for genuine
   difficulties the fields can't carry.
7. **Output location** — `.mochiko/specs/<feature>/slicer-report.md`, seeded and collected
   by the lead, alongside `slices.md` (when written).
8. **This is a reference template** — the producer fills in actual content following this
   structure.
