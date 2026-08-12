# Architect Report Template

The architecture producer's (principal-architect) self-disclosure report — authored alongside `architecture.md` and the
structural-decision rows on each architecture round; read by the lead and the reviewer(s) to
follow what the delta proposes and what changed. Envelope + shared rules (machine-first, no
self-verdict, conditional prose): `templates/report-format.md` — this file carries only the
payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
round: {{round}}
produced: [architecture.md, constraints-and-decisions.md#structural-decisions]
baseline: "{{seeded-from-ARCHITECTURE.md | reconstructed (confidence: high|medium|low) | greenfield-empty}}"
delta:                         # the proposed structural change, one line each; ["no structural change"] for a no-delta feature
  - "{{+ worker: settlement-processor (new box), request/response arrow from api}}"
scope: "{{full-system | delta-neighborhood (N components) — full view linked, not inlined}}"
structural_decisions: []       # the D-XXX IDs written into the structural-decisions section this round
changed_this_round:            # round 1: [initial]; later: the reviewer gap IDs addressed
  - "{{coverage: added sequence diagram for the async settlement flow}}"
governance_alignment: "{{respects BE-HEX layering per GI-XXX | the conflict + its chosen exit (redesign | ledger amendment), one line}}"
assumptions: []                # assumptions made this round, one line each with the rationale compressed in
open_questions:                # producer-surfaced unknowns feeding the clarification loop / gap routing
  - "{{the unknown, one line}}"
handoff: "{{what to grade (topology feasibility, coverage) + known soft spots, one line}}"
---
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** No done-state, no PASS/FAIL, and
   deliberately no "signed-off"/"approved" field — the producer must not self-assert a state
   the lead and the user own (the envelope's no-self-verdict rule; the sign-off is the lead's
   gate, the approval the user's). The clearing verdict lives in the reviewer reports.
2. **Machine-first; the map lives in the deliverable.** `architecture.md` is where the diagram,
   the sequence diagrams, and the component register live — the reviewer grades them *there*.
   Never restate the topology here; cite component names and D-/NFR- IDs.
3. **`baseline`** discloses the current-state seed and, when reconstructed (no `ARCHITECTURE.md`),
   its confidence — the input to the baseline-confirmation gate; a reconstructed baseline is
   confirmed by the user before any delta is designed on it.
4. **`delta` + `scope`** carry the proposed change and the size-bound scoping (delta neighborhood
   vs full system); a no-delta feature discloses `["no structural change"]`, still shown for
   sign-off, never silently asserted.
5. **`changed_this_round`** — first round: `[initial]`. Later rounds: the specific reviewer gaps
   addressed by ID, so the lead can see progress and detect a stalled, no-change round.
6. **`assumptions` + `open_questions` are the producer-authored uncertainty carrier** (the
   dispatching command's producer-authored branch); **`governance_alignment`** discloses conformance to the
   CLAUDE.md governance region + its `.claude/rules/mochiko/` files, or the conflict and its
   chosen exit, in one line.
7. **`handoff`** is a *pointer*, not a claim — what to grade and where the soft spots are;
   **not** a "ready" assertion. The rendered-diagram presentation and any "presented un-rendered"
   record are the lead's, set at the sign-off gate, not disclosed here.
8. **Output location** — `.mochiko/specs/<feature>/architect-report.md`, seeded and collected
   by the lead, alongside `architecture.md`.
9. **This is a reference template** — the architecture producer (principal-architect) fills in actual content following this
   structure.
