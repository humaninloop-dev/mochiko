# Technical Analyst Report Template

The technical-analyst's self-disclosure report — authored alongside the analysis/design
artifacts on each round; read by the lead and the reviewer(s) to follow what was produced
and what changed. Envelope + shared rules (machine-first, no self-verdict, conditional
prose): `templates/report-format.md` — this file carries only the payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
phase: analysis | design       # which phase this round produced — discloses, never drives the sequence
round: {{round}}
produced: [requirements.md, constraints-and-decisions.md, nfrs.md]   # design rounds: [data-model.md, contracts/api.yaml, quickstart.md]
changed_this_round:            # round 1 of a phase: [initial]; later: the reviewer gap IDs addressed
  - "{{G2: NFR-003 given a numeric latency target}}"
governance_alignment: "{{aligned | the rules/GI IDs touched and any exception, one line}}"
assumptions: []                # assumptions made this round, one line each with the rationale compressed in
open_questions:                # producer-surfaced unknowns feeding the clarification loop / gap routing
  - "{{the unknown, one line}}"
handoff: "{{which artifacts to grade + known soft spots, one line}}"
---
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** No done-state, no PASS/FAIL, and
   deliberately no "Completion"/"ready" field — the producer must not self-assert a state
   the lead owns (the envelope's no-self-verdict rule). The clearing verdict lives in the
   reviewer report(s); the lead owns the loop decision.
2. **Machine-first; the lead and reviewer(s) read the fields.** A routine round is
   frontmatter-only. Add `## Notes of note` only for genuine difficulties or non-obvious
   calls. The artifacts themselves are what the reviewer(s) grade — never restate their
   content here (cite TR-/C-/NFR-/D- IDs).
3. **`phase`** discloses which phase this round produced (`analysis` or `design`); the lead
   owns the two-phase sequence.
4. **`changed_this_round`** — first round of a phase: `[initial]`. Later rounds: the
   specific reviewer gaps addressed by ID, so the lead can see progress and detect a
   stalled, no-change round.
5. **`handoff`** is a *pointer*, not a claim — what to grade and where the soft spots are;
   **not** a "ready" assertion.
6. **`assumptions` + `open_questions` are the producer-authored uncertainty carrier** (the
   shape's producer-authored branch): assumptions record what was assumed and why;
   open questions feed the clarification loop, where the reviewer stress-tests them.
   **`governance_alignment`** discloses alignment to the CLAUDE.md governance region + its
   `.claude/rules/mochiko/` files in one line.
7. **Output location** — `.mochiko/specs/<feature>/techanalyst-report.md`, seeded and
   collected by the lead, alongside the analysis/design artifacts.
8. **This is a reference template** — the technical-analyst fills in actual content
   following this structure.
