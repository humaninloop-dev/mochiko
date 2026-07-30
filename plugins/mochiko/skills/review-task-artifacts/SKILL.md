---
name: review-task-artifacts
description: This skill MUST be invoked to grade a producer's task artifacts against the task-quality checklist — the task-mapping (`task-mapping.md` — story→cycle mapping plus slice rationale) and the task list (`tasks.md` — cycle→TDD tasks) — checking vertical-slice quality, TDD test-first ordering, cycle definition and sizing, `**TEST:**` verification-task presence, story→cycle→task traceability, and task-ID / file-path / marker presence. Emits a severity-classified gap report (Critical/Important/Minor) and a 3-state verdict (ready / needs-revision / critical-gaps). SHOULD also invoke whenever a tasks loop's mapping-review or tasks-review step needs an independent grade of the task artifacts, or when re-reviewing after a FAIL-loop revision. The task-artifact half of the tasks producer↔validator pair; does NOT cover plan-artifact completeness (requirements, constraints-and-decisions, NFRs, data-model, contracts — that is mochiko:review-plan-artifacts); defaults to FAIL; run by an independent validator, never the author.
---

# Reviewing Task Artifacts

## Overview

Find gaps in task artifacts and emit issues that must be resolved before the tasks proceed to
implementation. This is a **mirror checklist**: a fixed set of named checks, each with a fixed
question and a severity, producing a verdict derived mechanically from the issue counts. Focus on
vertical-slice quality, TDD test-first structure, and story→cycle→task traceability — not
implementation code, and not plan-artifact completeness (that is a separate reviewer; see *Scope*
below).

The output is a **gap-finding report plus a RECOMMENDED 3-state verdict** (ready / needs-revision /
critical-gaps) — an *input* to the loop, **not** a clearing PASS/FAIL. The `/mochiko:plan` **lead**
owns the clearing decision and routes the revision loop; this skill recommends, the lead routes.

**Violating the letter of the rules is violating the spirit of the rules.** Running the checklist
"in spirit" while skipping checks, or downgrading a severity to avoid a hard finding, is the exact
failure this skill exists to prevent. (The generic anti-rationalization doctrine lives in
`loop-discipline`; the review-specific red flags are at the foot of this file.)

## Scope — task artifacts, and the boundary vs plan review

| Lens | Artifacts | Owner |
|------|-----------|-------|
| **Task quality** | `task-mapping.md`, `tasks.md` | **this skill** |
| **Plan completeness** | requirements, constraints-and-decisions, NFRs, data-model, contracts, quickstart | `mochiko:review-plan-artifacts` |

The two reviewers are **disjoint**: different artifacts, different checks. This skill grades
vertical-slice quality, TDD ordering, `**TEST:**`-task presence, and story→cycle→task traceability;
the plan reviewer grades FR→TR coverage, measurability, schema-model consistency, and the like. The
only shared surface is **scaffolding** — the severity taxonomy, the 3-state verdict, the issue-doc
format, and the assembled report shape — all single-sourced (the mochiko validation-skill convention
plus `mochiko:advocate-report-template`). **Confirmed complementary — no structural merge.**

## When to Use

- Grading `task-mapping.md` (story→cycle mapping + slice rationale) for slice quality and coverage
- Grading `tasks.md` (cycle→TDD task list) for TDD structure, format, and completeness
- Cross-checking traceability between stories, cycles, and tasks (`tasks.md` ↔ `task-mapping.md`)
- Re-reviewing task artifacts after a FAIL-loop revision

## When NOT to Use

- **Plan artifact review** — use `mochiko:review-plan-artifacts` (requirements, constraints-and-decisions, NFRs, data-model, contracts — disjoint artifacts, disjoint checks)
- **Specification review** — use `mochiko:review-specifications`
- **Constitution review** — use `mochiko:validation-constitution`
- **Implementation code review** — use code-review tooling instead
- **Creating task artifacts** — this skill validates only; authoring is the producer's job (`mochiko:patterns-vertical-tdd`)
- **During active drafting** — wait for artifact completion before review

## Review Focus by Artifact Type

The caller supplies which task artifacts are in scope; this skill grades whichever it is handed.
When both are supplied, run the **Cross-Artifact Review** to cross-check `tasks.md` back against
`task-mapping.md` (the cumulative pass). Detailed checklists are in
[PHASE-CHECKLISTS.md](references/PHASE-CHECKLISTS.md).

Both artifacts follow the deliverable envelope (`templates/artifact-format.md`):
`task-mapping.md` is a compact mapping (tables + ≤ 2-line rationale cells), `tasks.md`
carries one-line task entries and no doctrine sections. **Density is never itself a
finding** — grade coverage, ordering, and traceability against the tables, not prose
volume.

| Artifact type | Focus | Key checks |
|---------------|-------|------------|
| **`task-mapping.md`** | story→cycle mapping, slice quality | story coverage, true vertical slices, foundation separation, cycle sizing, story→cycle traceability |
| **`tasks.md`** | cycle→TDD task list | cycle coverage, TDD test-first ordering, file paths, `**TEST:**` verification-task presence, task IDs, markers, checkpoints |
| **Cross-artifact** | the two together | mapping↔tasks alignment, story→cycle→task chain, cycle/dependency consistency |

> The IP-XXX / platform-app-ordering / deployment-cycle / `[EXTEND]`·`[MODIFY]` brownfield /
> constraint-task-traceability checks are **present but dormant** — they belong to the
> roadmap/brownfield track, deferred for the core tasks loop, and activate in lock-step with the
> producer's parked side. Sequencing of which artifact is reviewed when is the lead's call, not this
> skill's.

## Issue Classification

Three severities — Critical / Important / Minor. The severity classification (categories and
examples per level), issue-documentation formats, the task-specific Checks-Executed tables, the
`TM-`/`TT-`/`TX-` ID conventions, and the verdict decision tree are single-sourced in
[ISSUE-TEMPLATES.md](references/ISSUE-TEMPLATES.md).

## Review Process

### Step 1: Gather context

Read and understand:
- The task artifact being reviewed (`task-mapping.md` and/or `tasks.md`)
- The spec / user stories the tasks should deliver (P1/P2 story coverage)
- The plan artifacts the mapping draws on (for consistency checks)
- `task-mapping.md` when reviewing `tasks.md` (for the cumulative cross-check)

### Step 2: Execute the checklist

For each check in the applicable artifact-type checklist: ask the question, look for evidence in
the artifact, classify any issue's severity, and document the issue with that evidence.

### Step 3: Cross-reference

Check traceability (the story → cycle → task chain is complete), consistency (`tasks.md` and
`task-mapping.md` agree), and completeness (nothing obviously missing).

### Step 4: Emit the report

In the `advocate-report-template.md` shape (machine-first — findings YAML): the verdict from the
issue counts (mechanical — see *Verdict Criteria*), one finding entry per issue with its evidence
anchor (`at:`) and an actionable one-line fix, and the one-line `strengths:` field filled.

> No deterministic pre-assert exists on the tasks side (unlike the plan reviewer's checker). The
> core of this review is model judgment — is this a *true* vertical slice? is the checkpoint
> observable vs test-only? — which no grep settles.

## Verdict Criteria

Derived mechanically from the issue counts — the mapping itself carries no judgment; it is
single-sourced in [ISSUE-TEMPLATES.md → Verdict Decision Tree](references/ISSUE-TEMPLATES.md#verdict-decision-tree).
This is the **recommended** verdict the lead reads; the lead owns the clearing decision and the
revision loop.

## Quality Checklist

Before finalizing the review, verify:

- [ ] All applicable artifact-type checks executed
- [ ] Issues properly classified by severity
- [ ] Evidence cited for each issue
- [ ] Suggested fixes are actionable
- [ ] Verdict matches the issue counts
- [ ] Cross-artifact traceability (`tasks.md` ↔ `task-mapping.md`) verified when both are in scope
- [ ] The one-line `strengths:` field filled

## Common Mistakes

| Mistake | Bad | Good |
|---------|-----|------|
| Skipping the cross-artifact check | Reviewing `tasks.md` in isolation | When both are in scope, verify every cycle in `task-mapping.md` appears in `tasks.md` and the story→cycle→task chain is unbroken |
| Over-classified severity | Marking formatting issues "Critical" | Reserve Critical for gaps that genuinely block implementation |
| Missing evidence | "The cycles are not vertical slices" | "Cycle 2 groups all database tasks (T2.1–T2.4) with no user-facing value — a horizontal layer, not a vertical slice" |
| Not verifying `**TEST:**` presence | Assuming every cycle has proper TDD structure | Explicitly check each cycle ends with a `**TEST:**` task carrying real-infrastructure Setup/Action/Assert steps, not a vague "Demo" |
| Ignoring traceability gaps | Accepting tasks with no story label | Every task traces back to a user story via its cycle mapping |
| Reviewing during active drafting | Reviewing an incomplete artifact mid-creation | Wait for the producer to finish the artifact before validation |

## Red Flags — STOP and Restart Properly

If you notice yourself thinking any of these, STOP immediately:

- "This case is different because…" — It is not. Run the checklist.
- "I'm following the spirit, not the letter" — The letter IS the spirit.
- "The tasks look good enough" — Good enough is not ready. Evidence or rejection.
- "This cycle is probably a vertical slice" — Probably is not verified. Check value, layers, and independent testability.
- "This severity is only Minor, not Critical" — If you are rationalizing severity DOWN, it is probably the higher level.
- "I'll note the missing `**TEST:**` task but not block on it" — A missing verification task is Critical. It blocks.

## Common Rationalizations

| Rationalization | Counter |
|-----------------|---------|
| "The mapping was vague, so the tasks can be vague" | Vagueness upstream is a gap to flag, not permission to propagate. |
| "This is a small feature, full review is overkill" | Scale does not change the process. Every artifact gets every applicable check. |
| "Time pressure means we can skip the cross-artifact check" | A broken story→cycle→task chain caught now saves days of rework later. |
| "The producer is senior, they know what they're doing" | Producer seniority is irrelevant. Evidence-based review only. |
| "I already found enough issues" | Finding issues is not a quota. Run every check, document every finding. |
| "This cycle is close enough to a vertical slice" | Close enough is horizontal. Value + layers + independent testability, or it is a Critical structural issue. |

## Related

- `mochiko:patterns-vertical-tdd` — the producer-side skill this review mirrors; the reviewer checks exactly what it teaches the producer to author
- `mochiko:review-plan-artifacts` — the plan-artifact reviewer; disjoint artifacts and disjoint checks, complementary, no overlap
- `mochiko:advocate-report-template` — the assembled deliverable report shape the lead reads
- `loop-discipline` — the source of the anti-rationalization and independent-validation doctrine this skill operationalizes
