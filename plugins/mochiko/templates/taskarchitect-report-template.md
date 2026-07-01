# Task Architect Report Template

This template defines the structure for the task-architect's self-disclosure report — authored alongside the task artifacts (`task-mapping.md` and/or `tasks.md`) on each round and read directly by the lead and the reviewer to follow what was produced and what changed.

---

```markdown
# Task Architect Report

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

## Vertical-Slice Rationale

{{slice_rationale}}

---

## TDD Structure

{{tdd_structure}}

---

## Constitution Alignment

{{constitution_alignment}}

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

1. **This is a self-disclosure report, not a verdict.** It records what the task-architect produced this round; it carries no done-state and no PASS/FAIL. The clearing verdict lives in the reviewer report, and the lead owns the loop decision — never read this report as a gate. There is deliberately no "Completion" and no "Ready for Review" field: the producer must not self-assert a state the lead owns.
2. **Foreground prose; the lead and reviewer read it directly.** What Was Produced, What Changed This Round, the Vertical-Slice Rationale, the TDD Structure, Constitution Alignment, and Open Questions are the decision-relevant content. There is no parser — write for a human reader.
3. **`{{round}}`** is the lead's bounded-loop round counter: round 1 is the first draft of the artifact this round produced; later rounds are revisions following a reviewer critique.
4. **The report discloses which artifact-type it produced — it does not drive the sequence.** State in What Was Produced (and list in Artifacts Produced) whether this round produced the story→cycle mapping (`task-mapping.md`), the cycle-based task list (`tasks.md`), or a revision of one. The lead owns the mapping → tasks sequence and supplies the mapping as input when the task list is produced; this report only *discloses* what it made.
5. **What Changed This Round** — on the first round for an artifact, state "Initial mapping" or "Initial task list." On later rounds, list the specific gaps addressed since the previous round, so the lead can see progress (and detect a stalled, no-change round).
6. **Vertical-Slice Rationale** discloses why the slices were cut the way they were — why each cycle is a true vertical slice, how foundation was separated from features, and where inter-cycle dependencies come from. It is descriptive self-disclosure that helps the reviewer judge slicing quality; it is not a claim that the slicing is correct.
7. **TDD Structure** discloses how each cycle follows test-first discipline — the test-before-implementation ordering and the real-integration verification task that gates each cycle's completion. Descriptive, not a verdict on the structure's soundness.
8. **Handoff to Review** — `{{handoff_to_review}}` is a *pointer*, not a claim: name what is handed to the reviewer to grade (which artifacts, what to focus on, any known soft spots). It is **not** a "ready" / "done" assertion — the verdict is the reviewer's and the clearing decision is the lead's.
9. **Constitution Alignment** discloses how the produced artifacts align to the project constitution (`.mochiko/memory/constitution.md`); **Open Questions** records producer-surfaced unknowns and feeds the clarification loop / gap routing, where the reviewer stress-tests them.
10. **`## Artifacts Produced` is optional.** List the artifacts written this round — `task-mapping.md` and/or `tasks.md` — as a quick pointer to what the reviewer should open. The reviewer and lead read those artifacts directly, so this is a convenience disclosure, not a tracked metric (it replaces the brain-era per-phase count/metric tables) — omit it if it adds nothing.
11. **Output location** — the filled report lives at `.mochiko/specs/<feature>/taskarchitect-report.md`, seeded and collected by the lead, alongside the task artifacts.
12. **This is a reference template** — the task-architect fills in actual content following this structure.
