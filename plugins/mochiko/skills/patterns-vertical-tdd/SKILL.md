---
name: patterns-vertical-tdd
description: This skill MUST be invoked when structuring a feature's implementation into cycle cards — mapping user stories to cycles as demonstrable test-case bundles (Simple/Split/Merge, walking skeleton first), and authoring `tasks.md` as cycle cards with the closing `**TEST:**` real-infrastructure gate. SHOULD also invoke on 'define cycles', 'cycle cards', 'vertical slice', or 'story→cycle mapping'. Owns the `**TEST:**` grammar. Design-time — NOT build-time decomposition (mochiko:executing-tdd-cycle).
---

# Vertical Slicing — Cycle Cards

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Transform a feature's stories into **cycle cards** — vertical increments that each deliver observable, testable value. A cycle is a coherent bundle of **named test cases** (expected behaviour, Given/When/Then grain, executable Setup/Action/Assert form) that demonstrate together to the user; the cycle is done when those cases show green against real infrastructure. The output is `tasks.md` in the cycle-card shape (the `tasks` schema is the canonical skeleton — invoke `mochiko-cli template tasks` when the binary is available; otherwise Read `plugins/mochiko/schemas/tasks.yaml` raw): per card — stories + rationale, dependencies, the named test-case list (the card's content), cycle-level brownfield exposure, and the Simple/Split/Merge case.

This skill works at **design time inside the `/mochiko:implement` run** — after the design phase, or directly on a zero-gap sufficiency verdict; never a separate plan run. It decides the slicing and states what each cycle must prove. It writes no task lists — the builder decomposes each card into concrete tasks, with file paths, at build time with the code in view (`mochiko:executing-tdd-cycle`, downstream). The card carries the *what and why*; the *how* is deliberately left to the build.

**Two authors, one card:** the design seat running this skill owns the **slicing judgment** — which bundles exist, Simple/Split/Merge, dependencies, the walking-skeleton call; the `qa-engineer` authors the **test-case content** (expected behaviour) in the grammar it later executes. The slicing seat is a **design seat, never the builder who will execute the card.**

## When NOT to Use

- **Bug fixes** — single-change fixes don't need cycle structure
- **Documentation-only or spike/research work** — no cycle discipline applies
- **Decomposing a card into tasks** — build-time work, owned by `mochiko:executing-tdd-cycle`
- **Deriving or scoping features** — the feature is the pipeline unit, owned by `mochiko:authoring-feature-map`, upstream (its vocabulary table disambiguates feature vs cycle); a cycle is a within-one-feature increment

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

A card whose test cases cannot be demonstrated on their own is not a cycle.

### 2. Walking Skeleton First, Infrastructure Homed by Need

Where the work opens a **new end-to-end path** — a greenfield feature or a genuinely new path through the system — the **first cycle is a walking skeleton**: the thinnest end-to-end path through all layers with one trivial case green. It is the foundation, by construction. Growth or delta work on an already-standing path **skips the skeleton**.

There is **no foundation/feature card type**. All cycles are test-case bundles. Infrastructure a bundle needs emerges **inside the first bundle that needs it** (YAGNI at cycle grain); platform provisioning (IP-XXX) for the skeleton path lands **in the skeleton cycle**, the rest inside the first bundle that needs it. **Infra-only cards are never minted.** Inter-card dependencies stay explicit; `[P]` parallel eligibility derives from dependencies, not from a type column.

### 3. Verified against reality

Every card closes with a **`**TEST:**` gate** — the cycle's named test cases run against real infrastructure, never a re-run of the automated tests. This gate is the demonstration the cycle is anchored on; a cycle that stops at the mock boundary has proven nothing. Expected behaviour is the Assert fields; actual behaviour is the captured evidence. The grammar (fields, action modifiers, assert patterns, classification) lives in [TEST-GRAMMAR.md](references/TEST-GRAMMAR.md) — this skill owns it; downstream parsers consume it.

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
- **Split** — story > bundle: a story whose cases span more than one demonstrable bundle splits across cards (record the why, one line).
- **Merge** — stories < bundle: stories too thin to demonstrate alone share one bundle (record the why, one line).

The story→cycle decision and its rationale live **on the card** (Stories line) — there is no separate mapping artifact. Bundle identification is in [BUNDLE-IDENTIFICATION.md](references/BUNDLE-IDENTIFICATION.md).

## Brownfield exposure

Each card carries a cycle-level exposure line: `none`, or the existing surfaces it extends/modifies. This is the design-time disclosure — the builder's per-task `[EXTEND]`/`[MODIFY]` handling happens at decomposition, downstream (`mochiko:brownfield-integration`). Design-artifact brownfield markers (e.g. a data-model entity flagged as extending existing code) translate into the exposure line, so the classification survives design into the build.

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
