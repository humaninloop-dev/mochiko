---
name: patterns-vertical-tdd
description: This skill MUST be invoked when structuring a feature's implementation into vertical-slice cycle cards — mapping user stories to cycles (Simple/Split/Merge), classifying foundation vs feature cycles, and authoring `tasks.md` as cycle cards with the closing `**TEST:**` real-infrastructure gate. SHOULD also invoke on 'define cycles', 'cycle cards', 'vertical slice', or 'story→cycle mapping'. Owns the `**TEST:**` grammar. Design-time — NOT build-time decomposition (mochiko:executing-tdd-cycle).
---

# Vertical Slicing — Cycle Cards

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Transform a plan's stories into **cycle cards** — vertical slices that each deliver observable, testable value. The output is `tasks.md` in the cycle-card shape (the `tasks` schema is the canonical skeleton — invoke `mochiko-cli template tasks` when the binary is available; otherwise Read `plugins/mochiko/schemas/tasks.yaml` raw): per card — stories + feature rationale, foundation/feature type, dependencies, acceptance criteria (by ID), the closing `**TEST:**` gate, and cycle-level brownfield exposure.

This skill works at **design time**: it decides the slicing and states what each cycle must prove. It writes no task lists — the builder decomposes each card into concrete tasks, with file paths, at build time with the code in view (`mochiko:executing-tdd-cycle`, downstream). The card carries the *what and why*; the *how* is deliberately left to the build.

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

A card that cannot be demonstrated on its own is not a slice.

### 2. Foundation + Parallel

Foundation cycles run sequentially and establish what every feature depends on — platform infrastructure (IP-XXX items from constraints-and-decisions.md), data models, auth, API framework, error handling. **Identification:** ask "Could ANY user story work **in production** without this?" If no, it's foundation.

Feature cycles deliver user value incrementally — mapping directly to user stories, independently completable, parallel-eligible `[P]` once foundation is complete unless dependent on another feature cycle. **Identification:** ask "Does this deliver value a user could observe?" If yes, it's a feature.

### 3. Verified against reality

Every card closes with a **`**TEST:**` gate** — a real-infrastructure verification of the cycle's acceptance criteria, never a re-run of the automated tests. This gate is what makes a vertical slice actually vertical; a cycle that stops at the mock boundary has proven nothing. The grammar (fields, action modifiers, assert patterns, classification) lives in [TEST-GRAMMAR.md](references/TEST-GRAMMAR.md) — this skill owns it; downstream parsers consume it.

## Identifying Cycles

See [SLICE-IDENTIFICATION.md](references/SLICE-IDENTIFICATION.md) for detailed heuristics — the value-stream test, extraction from user stories, size calibration, dependency analysis, worked domain examples, and anti-patterns.

### Quick heuristics

A good cycle:
1. **Delivers user value** — something a user or operator could observe
2. **Touches all layers** — model, service, API, UI (as applicable)
3. **Is independently testable** — its gate can pass without later cycles
4. **Is sized appropriately** — completable in 1–3 implementation sessions

### Case column: Simple / Split / Merge

- **Simple** — story = cycle: a well-scoped story becomes one card.
- **Split** — story > cycle: a too-large story splits across cards (record the why, one line).
- **Merge** — stories < cycle: too-small stories share one card (record the why, one line).

The story→cycle decision and its rationale live **on the card** (Stories line) — there is no separate mapping artifact. Size calibration is in [SLICE-IDENTIFICATION.md](references/SLICE-IDENTIFICATION.md).

## Brownfield exposure

Each card carries a cycle-level exposure line: `none`, or the existing surfaces it extends/modifies. This is the design-time disclosure — the builder's per-task `[EXTEND]`/`[MODIFY]` handling happens at decomposition, downstream (`mochiko:brownfield-integration`). Plan-artifact brownfield markers (e.g. a data-model entity flagged as extending existing code) translate into the exposure line, so the classification survives design into the build.

## Quality Checklist

Before finalizing the cycle cards:

- [ ] Every P1/P2 story appears on at least one card
- [ ] Cards are vertical slices (not horizontal layers)
- [ ] Foundation cycles identified and sequenced; feature cycles marked `[P]` where independent
- [ ] Each card's acceptance criteria cite spec/plan IDs — never re-quoted content
- [ ] Each card ends with a `**TEST:**` gate (real-infrastructure, in the grammar)
- [ ] Story→cycle case (Simple/Split/Merge) and rationale recorded on each card
- [ ] Brownfield exposure stated per card (`none` counts)
- [ ] Dependencies minimal and explicit
- [ ] No task lists, no file paths — the card states what the cycle proves, not how it's built
