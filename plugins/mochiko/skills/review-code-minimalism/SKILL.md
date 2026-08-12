---
name: review-code-minimalism
description: This skill MUST be invoked when independently grading a cycle's produced code against the pre-code ladder — the code-minimalism lens run by the verification seat: read the cycle's git diff AND `cycle-report.md`, grade each rung claim against `mochiko:patterns-code-minimalism` (the standard, never restated here), and emit a `minimalism:` findings block. Rungs 2, 3, and 5 carry a codebase-read obligation. Findings are ADVISORY, never a cycle-failing gate. Scope is the minimalism lens ONLY.
---

# Review — Code Minimalism Lens

**Diff shows what was written; disclosure shows what the builder says they checked;
neither alone shows what should never have been written.**

## Overview

The per-cycle over-engineering lens: an independent, static read of a cycle's produced code
against the pre-code ladder. The grading standard is `mochiko:patterns-code-minimalism` —
this skill carries the *grading procedure*, never a copy of the ladder. It runs inside the
per-cycle verification the verification seat already performs; no separate stage, no
final-pass sweep — cycle diffs are small, context is fresh, rework is cheapest at cycle
close.

## When NOT to Use

- **General code review** — naming, patterns, framework choices, correctness beyond tests:
  out of scope; this lens grades ladder discipline only
- **Executing `**TEST:**` gates or quality gates** — `mochiko:testing-end-user`, the same
  seat's other craft
- **Design-time artifact review** — plan/spec reviewers own those surfaces; the plan-time sibling of this lens is the rung-honesty grade in `mochiko:review-plan-artifacts` against `mochiko:patterns-plan-minimalism` (same posture, design-time altitude)

## Inputs

1. **The cycle's git diff** — what was actually built (files created/modified per the
   cycle report locate it; `git diff` over those paths).
2. **`cycle-report.md`** — the disclosed decomposition and its per-task rung claims. Read
   the report file itself, never a relay of it.
3. **The codebase around the diff** — obligated for rungs 2/3/5 (below).

## Procedure

**1. Read the disclosure.** Every decomposition task should carry a rung. A missing rung
note is itself a finding (the disclosure surface exists so this lens can grade it — and it
also shows whether rung-zero reading happened at all).

**2. Grade each rung claim against the standard.** Open
`mochiko:patterns-code-minimalism`; per task, ask: does the code sit on the claimed rung,
and does a higher rung apply that the builder descended past?

**3. Never take reuse claims on trust — the codebase-read obligation.** For rungs 2
(reuse), 3 (stdlib), and 5 (installed dependency), diff + disclosure alone cannot verify
"should have reused":
- **Rung 2:** targeted greps around the diff for existing helpers/utilities the new code
  duplicates — against the *current* codebase, which also catches cross-cycle accretion
  (cycle 5 duplicating cycle 2's helper reads as a rung-2 violation now).
- **Rung 3:** does the language's standard library already provide the written behavior?
- **Rung 5:** check the dependency manifest — does an installed dependency already cover
  it?

**4. Check the floor line.** Code cut to reach a cheaper rung that a floor obligation or
accessibility required is a finding — the standard's floor line is part of the standard.

**5. Emit findings.** One `minimalism:` entry per finding in the verification report
(format: `mochiko:testing-end-user`'s report templates): task ID, claimed rung, observed
rung, one-line evidence (the grep hit, the stdlib call, the manifest entry). No findings →
an empty block; never narrate a clean grade.

## Verdict semantics — advisory

A `minimalism:` finding **never fails a cycle** the way a `**TEST:**` gate does. Findings
ride the verification report to the lead's checkpoint verdict; the lead decides rework-now
or carry. A builder-vs-reviewer rung dispute escalates to the user only at the checkpoint,
never as a mid-cycle stop.

## Quality Checklist

- [ ] Diff AND cycle report both read — never one without the other
- [ ] Rung-2/3/5 claims verified by codebase read (greps / stdlib / manifest), not trusted
- [ ] Every finding cites task ID + evidence, one line each
- [ ] No general-code-review findings smuggled in (naming, style, patterns)
- [ ] Findings emitted as advisory `minimalism:` entries — no cycle failed on this lens alone
