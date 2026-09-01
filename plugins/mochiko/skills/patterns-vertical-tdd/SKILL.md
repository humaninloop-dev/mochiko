---
name: patterns-vertical-tdd
description: This skill MUST be invoked when structuring a feature's implementation into cycle cards — mapping user stories to cycles as demonstrable test-case bundles (Simple/Split/Merge, walking skeleton first), and authoring `tasks.md` as cycle cards with the closing `**TEST:**` real-infrastructure gate. SHOULD also invoke on 'define cycles', 'cycle cards', 'vertical slice', or 'story→cycle mapping'. Owns the `**TEST:**` grammar. Design-time — NOT build-time decomposition (mochiko:executing-tdd-cycle).
---

# Vertical Slicing — Cycle Cards

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Transform a feature's stories into **cycle cards** — vertical increments that each deliver observable, testable value. A cycle is a coherent bundle of **named test cases** (expected behaviour, Given/When/Then grain, executable Setup/Action/Assert form) that demonstrate together to the user; the cycle is done when those cases show green against real infrastructure. The card carries the *what and why*; the *how* is deliberately left to the build.

## Rules — load the schema first

Your first action, before any slicing step: **Read `schema.yaml` (this skill's own directory) raw, in full** — the patterns family ships no common file, so the pair's own schema is the whole first action. The schema is the source of truth for this skill's binding rules, nested in six sections, each addressable by its section ID: `patterns-vertical-tdd.sec.trigger` · `patterns-vertical-tdd.sec.scope` · `patterns-vertical-tdd.sec.discipline` · `patterns-vertical-tdd.sec.inputs` · `patterns-vertical-tdd.sec.disclosure` · `patterns-vertical-tdd.sec.reserved`. Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule's `when:` resolves against the schema's declared `conditions:` (`new_end_to_end_path`) and gates when the obligation applies, never whether it is delivered; a rule of `class: floor` is always read and always delivered whatever its `when:`; a `pointer:` rule binds you to that file's or skill's procedure, referenced never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 5 rules of `class: floor` are non-waivable. Before the first slicing step, state the floor count back — a skipped or partial read leaves that count blank: halt and surface it, and halt likewise if the schema's `class: floor` count disagrees with the pin.

## Core Principles

### 1. Vertical Over Horizontal

**Wrong** (horizontal slicing):
```
Cycle 1: All models        Cycle 2: All services        Cycle 3: All endpoints
```

**Right** (vertical slicing):
```
Cycle 1: User creation (model + service + endpoint, end to end)
Cycle 2: User authentication (model + service + endpoint, end to end)
```

### 2. Walking Skeleton First, Infrastructure Homed by Need

A walking skeleton proves the path before the features ride it; infrastructure is homed by need at cycle grain rather than front-loaded. The binding forms — when the skeleton fires, how infrastructure homes, no infra-only cards — live in the schema (`patterns-vertical-tdd.sec.discipline`).

### 3. Verified against reality

Expected behaviour is the Assert fields; actual behaviour is the captured evidence. The grammar (fields, action modifiers, assert patterns, classification) lives in [TEST-GRAMMAR.md](references/TEST-GRAMMAR.md).

## Identifying Cycles

See [BUNDLE-IDENTIFICATION.md](references/BUNDLE-IDENTIFICATION.md) for detailed heuristics — the value-stream test, the walking skeleton, bundle identification, dependency analysis, and anti-patterns.

### Quick heuristics

A good cycle:
1. **Delivers user value** — something a user or operator could observe
2. **Touches all layers** — model, service, API, UI (as applicable)
3. **Is independently testable** — its cases can pass without later cycles
4. **Is worth demonstrating** — a bundle the user would want to watch pass; merge until it is

### Case column: Simple / Split / Merge

- **Simple** — story's cases = one bundle: a well-scoped story's test cases form one card.
- **Split** — story > bundle: a story whose cases span more than one demonstrable bundle splits across cards.
- **Merge** — stories < bundle: stories too thin to demonstrate alone share one bundle.

Bundle identification is in [BUNDLE-IDENTIFICATION.md](references/BUNDLE-IDENTIFICATION.md).

## Brownfield exposure

Each card's exposure line is the design-time disclosure; the builder's per-task `[EXTEND]`/`[MODIFY]` handling happens at decomposition, downstream (`mochiko:brownfield-integration`).

## Quality Checklist

Before finalizing the cycle cards:

- [ ] Every P1/P2 story appears on at least one card
- [ ] Cards are vertical increments (not horizontal layers)
- [ ] Where a new end-to-end path exists (greenfield / new path), the first cycle is a walking skeleton; growth on a standing path skips it
- [ ] No infra-only cards — infrastructure is homed inside the first bundle that needs it (skeleton-path infra in the skeleton)
- [ ] Each card carries its named test-case list (expected behaviour, in the `**TEST:**` grammar)
- [ ] Each named test case cites the spec/design ID(s) it covers — never re-quoted content
- [ ] Each card's `**TEST:**` gate is real-infrastructure (never a re-run of the automated tests)
- [ ] Story→cycle case (Simple/Split/Merge) and rationale recorded on each card
- [ ] Brownfield exposure stated per card (`none` counts)
- [ ] Dependencies minimal and explicit; `[P]` derives from dependencies, not a type column
- [ ] No task lists, no file paths — the card states what the cycle proves, not how it's built
