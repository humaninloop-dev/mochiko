---
name: patterns-vertical-tdd
description: This skill MUST be invoked when structuring a feature's implementation into cycle cards — mapping user stories to cycles as demonstrable test-case bundles (Simple/Split/Merge, walking skeleton first), and authoring `tasks.md` as cycle cards with the closing `**TEST:**` real-infrastructure gate. SHOULD also invoke on 'define cycles', 'cycle cards', 'vertical slice', or 'story→cycle mapping'. Owns the `**TEST:**` grammar. Design-time — NOT build-time decomposition (mochiko:executing-tdd-cycle).
allowed-tools: Bash(mochiko-cli *)
---

# Vertical Slicing — Cycle Cards

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Transform a feature's stories into **cycle cards** — vertical increments that each deliver observable, testable value. A cycle is a coherent bundle of **named test cases** (expected behaviour, Given/When/Then grain, executable Setup/Action/Assert form) that demonstrate together to the user; the cycle is done when those cases show green against real infrastructure. The card carries the *what and why*; the *how* is deliberately left to the build.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-vertical-tdd · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-vertical-tdd · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-vertical-tdd --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-vertical-tdd --section patterns-vertical-tdd.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-vertical-tdd --section patterns-vertical-tdd.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-vertical-tdd --section patterns-vertical-tdd.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-vertical-tdd --section patterns-vertical-tdd.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-vertical-tdd --section patterns-vertical-tdd.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-vertical-tdd --section patterns-vertical-tdd.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

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
