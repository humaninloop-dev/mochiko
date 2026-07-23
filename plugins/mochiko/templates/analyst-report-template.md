# Analyst Report Template

The producer's self-disclosure report — authored alongside `spec.md` on each round; the
lead reads it to follow what was produced and what changed. Envelope + shared rules
(machine-first, no self-verdict, conditional prose): `templates/report-format.md` — this
file carries only the analyst payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
round: {{round}}
produced: [spec.md]
changed_this_round:            # round 1: [initial]; later rounds: the gap IDs / changes addressed
  - "{{G3: added expiry edge case to US2}}"
assumptions:                   # one line each, with the rationale compressed in
  - {id: A1, assumption: "{{assumption}}", why: "{{rationale, one line}}"}
open_questions: []             # producer-surfaced unknowns feeding the clarification loop
handoff: "{{what the critic should grade + any known soft spots, one line}}"
---
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** It records what the producer
   authored and assumed; it carries no PASS/FAIL and no done-state (per the envelope's
   no-self-verdict rule). The clearing verdict lives in the critic's review, and the lead
   owns the loop decision — never read this report as a gate.
2. **Machine-first; the lead is the reader.** The frontmatter is the report — a round with
   nothing unusual needs no prose. Add a `## Notes of note` block only for genuine
   difficulties or non-obvious calls the fields can't carry.
3. **`round`** is the lead's bounded-loop round counter: round 1 is the first draft; later
   rounds are revisions following a critic review.
4. **`changed_this_round`** — round 1: `[initial]`. Later rounds: the specific gaps
   addressed (by the critic's gap IDs), so the lead can see progress and detect a stalled,
   no-change round. Cite IDs; never restate spec text (the spec is the artifact the critic
   and lead read directly).
5. **Output location** — `.mochiko/specs/<feature>/analyst-report.md`, seeded and collected
   by the lead.
6. **This is a reference template** — the producer fills in actual content following this
   structure.
