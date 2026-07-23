# Task Architect Report Template

The task-architect's self-disclosure report — authored alongside the task artifacts
(`task-mapping.md` and/or `tasks.md`) on each round; read by the lead and the reviewer to
follow what was produced and what changed. Envelope + shared rules (machine-first, no
self-verdict, conditional prose): `templates/report-format.md` — this file carries only the
payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
round: {{round}}
produced: [task-mapping.md]            # or [tasks.md], or both — discloses, never drives the mapping → tasks sequence
changed_this_round:                    # round 1 for an artifact: [initial]; later: the reviewer gap IDs addressed
  - "{{G1: C4 split into C4/C5 — two value seams}}"
slicing_notes:                         # deltas/judgment calls NOT already in task-mapping.md (the rationale's home); [] on a clean round
  - "{{C3 kept unsplit despite size — single seam, one line why}}"
governance_alignment: "{{aligned | the rules/GI IDs touched and any exception, one line}}"
open_questions: []                     # producer-surfaced unknowns feeding the clarification loop
handoff: "{{which artifacts to grade + known soft spots, one line}}"
---
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** No done-state, no PASS/FAIL, and
   deliberately no "Completion"/"Ready for Review" field — the producer must not
   self-assert a state the lead owns (the envelope's no-self-verdict rule). The clearing
   verdict lives in the reviewer report; the lead owns the loop decision.
2. **Machine-first; the rationale lives in the deliverable.** `task-mapping.md` is the
   source of truth for slicing decisions — the vertical-slice rationale, foundation
   separation, and dependency reasoning are authored *there*, where the reviewer grades
   them. `slicing_notes` carries only judgment calls that have no home in the artifact
   (e.g. a considered-and-rejected split). Never restate mapping content here; cite cycle
   IDs.
3. **`produced`** discloses which artifact-type this round made; the lead owns the
   mapping → tasks sequence and supplies the mapping as input when the task list is
   produced.
4. **`changed_this_round`** — first round for an artifact: `[initial]`. Later rounds: the
   specific reviewer gaps addressed by ID, so the lead can see progress and detect a
   stalled, no-change round.
5. **`handoff`** is a *pointer*, not a claim — **not** a "ready" assertion.
6. A routine round is frontmatter-only; add `## Notes of note` only for genuine
   difficulties the fields can't carry.
7. **Output location** — `.mochiko/specs/<feature>/taskarchitect-report.md`, seeded and
   collected by the lead, alongside the task artifacts.
8. **This is a reference template** — the task-architect fills in actual content following
   this structure.
