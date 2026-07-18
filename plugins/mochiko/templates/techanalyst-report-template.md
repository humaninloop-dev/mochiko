# Technical Analyst Report Template

This template defines the structure for the technical-analyst's self-disclosure report — authored alongside the analysis/design artifacts on each round and read directly by the lead and the reviewer(s) to follow what was produced and what changed.

---

```markdown
# Technical Analyst Report

> Feature: {{feature_id}}
> Phase: {{phase}}
> Round: {{round}}
> Generated: {{timestamp}}

---

## What Was Produced

{{production_summary}}

---

## What Changed This Round

{{changes_this_round}}

---

## Governance Alignment

{{governance_alignment}}

---

## Open Questions

{{open_questions}}

---

## Handoff to Review

{{ready_for_review}}

---

## Artifacts Produced (optional)

| Artifact | What's in it |
|----------|--------------|
| {{artifact}} | {{artifact_summary}} |
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** It records what the technical-analyst produced this round; it carries no done-state and no PASS/FAIL. The clearing verdict lives in the reviewer report(s), and the lead owns the loop decision — never read this report as a gate. There is deliberately no "Completion" / done field: the producer must not self-assert a state the lead owns.
2. **Foreground prose; the lead and reviewer(s) read it directly.** What Was Produced, What Changed This Round, Governance Alignment, and Open Questions are the decision-relevant content. There is no parser — write for a human reader.
3. **`{{round}}`** is the lead's bounded-loop round counter: round 1 is the first draft of this phase; later rounds are revisions following a reviewer critique.
4. **`{{phase}}`** discloses which phase this round produced — `analysis` or `design`. The lead owns the two-phase analysis→design sequence; this report only *discloses* which phase it produced, it does not drive the sequence.
5. **What Changed This Round** — on the first round of a phase, state "Initial {{phase}}." On later rounds, list the specific gaps addressed since the previous round, so the lead can see progress (and detect a stalled, no-change round).
6. **Handoff to Review** — `{{ready_for_review}}` is a *pointer*, not a claim: name what is handed to the reviewer(s) to grade (which artifacts, what to focus on, any known soft spots). It is **not** a "ready" / "done" assertion — the verdict is the reviewer's and the clearing decision is the lead's.
7. **Governance Alignment** discloses how the produced artifacts align to the project's governance (the CLAUDE.md governance region + its `.claude/rules/mochiko/` files); **Open Questions** records producer-surfaced unknowns and feeds the clarification loop / gap routing, where the reviewer stress-tests them.
8. **`## Artifacts Produced` is optional and phase-gated.** List the artifacts written this round — analysis phase produces artifacts such as `requirements.md`, `constraints-and-decisions.md`, and `nfrs.md`; design phase produces artifacts such as `data-model.md` and `contracts/api.yaml` — as a quick pointer to what the reviewer(s) should open. The reviewer(s) and lead read those artifacts directly, so this is a convenience disclosure, not a tracked metric (it replaces the brain-era per-section count tables) — omit it if it adds nothing.
9. **Output location** — the filled report lives at `.mochiko/specs/<feature>/techanalyst-report.md`, seeded and collected by the lead, alongside the analysis/design artifacts.
10. **This is a reference template** — the technical-analyst fills in actual content following this structure.
