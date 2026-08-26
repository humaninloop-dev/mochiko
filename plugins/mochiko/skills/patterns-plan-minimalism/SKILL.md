---
name: patterns-plan-minimalism
description: This skill MUST be invoked at a design decision — what the design phase authors inside `/mochiko:implement` (scoped to the sufficiency gap list), each producing seat's plan, the epic joint design-phase plan, and any design-artifact decision — running the simplest-execution ladder over every design element (stop at the first failing rung: required · simpler shape · already exists · minimum now · builder's room), disclosed rung-wise. SHOULD also invoke on 'plan minimalism' or 'is this artifact needed'. Single source of the design ladder; design-time sibling of `mochiko:patterns-code-minimalism`.
---

# Plan Minimalism — The Simplest-Execution Ladder

**The cheapest artifact is the one the plan never has to carry.**

## Overview

Before an artifact or design element enters the design-phase package, run the ladder: a ranked
check that stops at the **first rung that fails**. It grades the *solution the design commits
the build to*, not the weight of the documents — thin documents are a consequence, not the test.

It fires at three generation-time sites inside the implement run's **design phase**: **what the
design phase authors** — scoped to the sufficiency gap list, signed by the user at the design
checkpoint — **each producing seat's plan**, and the **epic joint design-phase plan** (one plan
over all members). Each discloses a rung stop per element, and `mochiko:review-plan-artifacts`
grades them at review — rung-honesty advisory, gap-list conformance blocking.
`mochiko:patterns-code-minimalism` continues the discipline over code at build time.

## When NOT to Use

- **Build-time code** — the code-shaping ladder is `mochiko:patterns-code-minimalism`,
  downstream at card decomposition.
- **Cutting a ratified requirement or floor obligation** — rung 1 never deletes those (see the
  floor); the ladder removes speculation, never scope or safety.
- **A delta-scope run** — no design phase fires by default; its deliverable is the
  desk-confirmed delta card, already minimal.

## The Ladder

Per design element (component, entity, contract, mechanism, flow, constraint), stop at the
**first rung that fails**, before it enters the package. Descend only when the rung above
genuinely holds, and say why in one line when it doesn't.

1. **Required?** — a ratified requirement or an asserted floor obligation names it, or it does
   not enter the package. Strict: no glue exception (glue is builder's room, rung 5), no
   speculative or YAGNI element.
2. **Simpler shape?** — a design with fewer parts meeting the same requirement wins; no new
   abstraction, the boring choice; no rich-domain modeling for operational or mechanical
   features.
3. **Already exists?** — a baseline, the current system, an installed dependency, or an
   adoptable proven component (per `mochiko:patterns-adopt-first`) carries it: extend,
   reference, or adopt — never re-design.
4. **Minimum now** — sized to the requirement as ratified; future-proof only where the
   retrofit is expensive.
5. **Builder's room** — the design states WHAT plus its binding constraints; HOW stays open,
   prescribed only where cost-of-getting-it-wrong is high (boundary contracts, persisted
   shapes, security). The rest is guidance the build may improve on.

**Scope of each rung:** rungs 1, 4, 5 apply to every element without exception; rungs 2, 3
apply to design elements — shape and reuse are design judgments.

**Read before you claim:** a rung-2 or rung-3 claim is made only after reading the baselines
and current state — never on trust. A reuse claim with no read behind it is not a rung stop.

## The floor — lazy, not negligent

Rung 1's "required" reads **ratified requirements AND asserted floor obligations**:
production-floor categories, compliance-module obligations, and NFR-derived floor elements —
security controls, error and data-loss handling, observability, accessibility. Strictness
against speculation survives; the floor never enters the package through imagination, and never
leaves it to reach a cheaper rung.

## Disclosure grammar

Every rung claim is disclosed where it fires, one line per element:
`<element> — rung N (one-line why)`. The lead discloses across what the design phase authors,
each seat in its plan. An undisclosed element reads as rung-skipped at review.

## Sibling

`mochiko:patterns-code-minimalism` — the build-time continuation: the same philosophy over
code at card decomposition. Two ladders, one discipline, two altitudes.

## Quality Checklist

- [ ] Every authored artifact and major element carries a disclosed rung stop
- [ ] No element sits past a rung it fails (a one-line why when close)
- [ ] Rung-1 "required" honored both ways — no speculative element in, no floor obligation out
- [ ] Rung-2/3 claims backed by a real read of baselines + current state
- [ ] Rungs 1/4/5 applied to all elements, rungs 2/3 to design elements
