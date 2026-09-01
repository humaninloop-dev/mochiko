---
name: patterns-plan-minimalism
description: This skill MUST be invoked at a design decision — what the design phase authors inside `/mochiko:implement` (scoped to the sufficiency gap list), each producing seat's plan, the epic joint design-phase plan, and any design-artifact decision — running the simplest-execution ladder over every design element (stop at the first failing rung: required · simpler shape · already exists · minimum now · builder's room), disclosed rung-wise. SHOULD also invoke on 'plan minimalism' or 'is this artifact needed'. Single source of the design ladder; design-time sibling of `mochiko:patterns-code-minimalism`.
---

# Plan Minimalism — The Simplest-Execution Ladder

**The cheapest artifact is the one the plan never has to carry.**

## Overview

Before an artifact or design element enters the design-phase package, run the ladder: a
ranked check over every design element. It grades the *solution the design commits the
build to*, not the weight of the documents — thin documents are a consequence, not the
test.

## Rules — load the schema first

Your first action, before any rung is claimed: **Read `schema.yaml` (this skill's own
directory) raw, in full, as one declared first action.** The schema is the source of truth
for this skill's binding rules, nested in six sections, each addressable by its section
ID: `patterns-plan-minimalism.sec.trigger` · `patterns-plan-minimalism.sec.scope` ·
`patterns-plan-minimalism.sec.discipline` · `patterns-plan-minimalism.sec.inputs` ·
`patterns-plan-minimalism.sec.disclosure` · `patterns-plan-minimalism.sec.reserved`.
Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
`constraint`; a rule of `class: floor` is always read and always delivered; a `pointer:`
rule binds you to that file's or skill's procedure, referenced never restated; labels come
from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 2 rules of
`class: floor` are non-waivable. Before the first rung is claimed, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

## The Ladder

Rung by rung — the stop rule, the rung scopes, and the read duty live in the schema:

1. **Required?** — a ratified requirement or an asserted floor obligation names it, or it
   does not enter the package. Strict: no glue exception (glue is builder's room, rung 5),
   no speculative or YAGNI element.
2. **Simpler shape?** — a design with fewer parts meeting the same requirement wins; no
   new abstraction, the boring choice; no rich-domain modeling for operational or
   mechanical features.
3. **Already exists?** — a baseline, the current system, an installed dependency, or an
   adoptable proven component (per `mochiko:patterns-adopt-first`) carries it: extend,
   reference, or adopt — never re-design.
4. **Minimum now** — sized to the requirement as ratified; future-proof only where the
   retrofit is expensive.
5. **Builder's room** — the design states WHAT plus its binding constraints; HOW stays
   open, prescribed only where cost-of-getting-it-wrong is high (boundary contracts,
   persisted shapes, security). The rest is guidance the build may improve on.

## Sibling

`mochiko:patterns-code-minimalism` — the build-time continuation: the same philosophy over
code at card decomposition. Two ladders, one discipline, two altitudes.
