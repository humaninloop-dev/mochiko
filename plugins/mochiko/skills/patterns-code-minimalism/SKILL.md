---
name: patterns-code-minimalism
description: This skill MUST be invoked at build-time card decomposition, BEFORE any red-phase test — running the pre-code ladder over each task (stop at the first applicable rung: exist at all · in codebase · stdlib · native platform · installed dep · one line · minimum), disclosed in the cycle report. SHOULD also invoke on 'should this code exist', 'reuse before build', 'stdlib first', 'over-engineering', 'YAGNI', or when slimming existing code that grew unneeded abstraction layers. Single source of the ladder; distinct from the green-phase 'minimum code to pass' rule.
---

# Code Minimalism — The Pre-Code Ladder

**The cheapest code is the code never written.**

## Overview

Before any code is written for a task, run the ladder: a ranked pre-code check that stops at
the **first rung that applies**. The ladder fires at card decomposition
(`mochiko:executing-tdd-cycle`, step 2), before the red phase — it governs whether and how
much code exists, not how the code that exists is written. The chosen rung per task is
disclosed in the cycle report's decomposition, where the verification seat grades it against
this file (`mochiko:review-code-minimalism`). Its design-time sibling
`mochiko:patterns-plan-minimalism` runs the same discipline over the design's elements
upstream; this ladder is the build-time continuation.

## When NOT to Use

- **Inside red/green/refactor** — the ladder ran at decomposition; the green-phase
  "minimum code to make the failing test pass" rule (`mochiko:executing-tdd-cycle`) is a
  different, later discipline
- **Architecture-level topology sizing** — component-count minimalism is
  `mochiko:patterns-system-design`'s judgment, upstream
- **Cutting scope from the card** — the card's acceptance criteria are the floor of what to
  build; the ladder decides how cheaply to meet them, never whether to meet them

## Rung zero — read before you rank

Trace the real flow of the code being touched before rung 1 — rung 2 is unanswerable
without having looked. Brownfield touches ride `mochiko:brownfield-integration` (the
read-before-write procedure lives there, not here).

## The Ladder

Stop at the **first rung that applies**. Descend only when the rung above genuinely does
not apply — and be able to say why, in one line, when it doesn't.

1. **Does it need to exist at all?** The requirement is already met, speculative, or
   YAGNI — skip the task entirely.
2. **Already in the codebase?** Reuse the existing helper, utility, or pattern — extending
   an existing surface beats inventing a parallel one.
3. **Standard library handles it?** Use it — no wrapper, no re-implementation.
4. **Native platform feature?** The runtime, framework, or platform already does this —
   use it.
5. **Installed dependency covers it?** A dependency already in the manifest does this —
   use it. (Adding a NEW dependency is not a rung — it rides the domain-registry ruling
   where `references/DOMAIN-DEPENDENCIES.md` applies, and is never auto-approved.) A
   design-committed adopt-first choice reaches these cards as a binding constraint, not a rung
   to re-open — the design-time discipline is `mochiko:patterns-adopt-first`.
6. **Fits in one line?** Write the one line.
7. **Only then:** write the minimum that works.

## The floor — lazy, not negligent

No rung ever sacrifices a floor obligation (security, testing, error/data-loss handling,
observability — the project's asserted Essential Floor) **or accessibility** (named
explicitly here because the floor carries no a11y card yet, pending the frontend shelf).
Code is small because it is necessary, never golfed: a guard, a validation, an a11y
attribute is rung-7 code that must exist — the ladder deletes speculation, not safety.

## One intensity

The ladder ships at one strength — there is no off/lite/full/ultra dial. Per-project
variance rides the recorded-waiver machinery, never a mode line.

## Quality Checklist

Before closing a decomposition:

- [ ] Every task carries a rung, disclosed in the cycle report's decomposition
- [ ] No task sits on a lower rung while a higher one applies (a one-line why when close)
- [ ] Rung-zero honored — the touched code's real flow was traced before ranking
- [ ] No floor obligation or accessibility need was cut to reach a cheaper rung
- [ ] Rung-1 skips are genuine YAGNI, never silent scope cuts against the card's criteria
