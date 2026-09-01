---
name: patterns-code-minimalism
description: This skill MUST be invoked at build-time card decomposition, BEFORE any red-phase test — running the pre-code ladder over each task (stop at the first applicable rung: exist at all · in codebase · stdlib · native platform · installed dep · one line · minimum), disclosed in the cycle report. SHOULD also invoke on 'should this code exist', 'reuse before build', 'stdlib first', 'over-engineering', 'YAGNI', or when slimming existing code that grew unneeded abstraction layers. Single source of the ladder; distinct from the green-phase 'minimum code to pass' rule.
---

# Code Minimalism — The Pre-Code Ladder

**The cheapest code is the code never written.**

## Overview

Before any code is written for a task, run the ladder: a ranked pre-code check over each
task. Its design-time sibling `mochiko:patterns-plan-minimalism` runs the same discipline
over the design's elements upstream; this ladder is the build-time continuation.

## Rules — load the schema first

Your first action, before any rung is ranked: **Read `schema.yaml` (this skill's own
directory) raw, in full, as one declared first action.** The schema is the source of truth
for this skill's binding rules, nested in six sections, each addressable by its section
ID: `patterns-code-minimalism.sec.trigger` · `patterns-code-minimalism.sec.scope` ·
`patterns-code-minimalism.sec.discipline` · `patterns-code-minimalism.sec.inputs` ·
`patterns-code-minimalism.sec.disclosure` · `patterns-code-minimalism.sec.reserved`.
Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
`constraint`; a rule of `class: floor` is always read and always delivered; a `pointer:`
rule binds you to that file's or skill's procedure, referenced never restated; labels come
from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 3 rules of
`class: floor` are non-waivable. Before the first rung is ranked, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

## The Ladder

Rung by rung — the stop rule and every bound on the walk live in the schema:

1. **Does it need to exist at all?** The requirement is already met, speculative, or
   YAGNI — skip the task entirely.
2. **Already in the codebase?** Reuse the existing helper, utility, or pattern — extending
   an existing surface beats inventing a parallel one.
3. **Standard library handles it?** Use it — no wrapper, no re-implementation.
4. **Native platform feature?** The runtime, framework, or platform already does this —
   use it.
5. **Installed dependency covers it?** A dependency already in the manifest does this —
   use it.
6. **Fits in one line?** Write the one line.
7. **Only then:** write the minimum that works.
