<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by construction,
     human-legible. This file is CYCLE CARDS, not a task list: the builder decomposes each
     card into concrete tasks at build time, with the code in view (the decomposition is
     disclosed in the cycle report, never pre-written here). Cite spec/plan content by ID
     (US-#, FR-#, SC-#, C-#) — never re-quote it. Register: `full` per artifact-format.md
     rule 11; TEST-gate commands, file paths, and identifiers are never-compress items. -->

# Implementation Cycles: [FEAT-XXX — FEATURE NAME]

> Generated from the spec folder and the feature's produced design artifacts: spec.md, features/FEAT-XXX/plan.md, and whichever of requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/ the approved proposal included
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | [N] ([N] foundation + [N] feature) |
| Stories covered | [US-# list — every P1/P2 story on at least one card] |

## Cycle Format

Each card is one vertical slice: an observable, end-to-end behavior. The builder implements
the card test-first (red/green/refactor per `mochiko:executing-tdd-cycle`, decomposition at
build time) and the closing `**TEST:**` gate verifies it against real infrastructure —
see [`TEST-GRAMMAR.md`](../skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md) for the
canonical Setup/Action/Assert/Capture grammar. **The card's checkbox is the progress
surface**, flipped when the cycle's gate passes. Foundation cycles run sequentially, first;
feature cycles are parallel-eligible `[P]` unless dependent on another feature cycle.

---

<!--
  The two cards below are SAMPLES for illustration — replace them with actual cycles from
  the feature's spec + plan artifacts. DO NOT keep them in the generated tasks.md.
-->

## Foundation Cycles

> Sequential; establish what every feature cycle depends on. All complete before feature cycles begin.

### - [ ] Cycle 1: Core entity and basic CRUD

- **Stories:** US-1 — [why these stories share this cycle / what it establishes, ≤ 2 lines]
- **Depends on:** —
- **Case:** Simple <!-- Simple | Split — why, one line | Merge — why, one line -->
- **Acceptance criteria:** [spec/plan IDs this cycle must satisfy — cite, never quote]
- **Brownfield exposure:** none <!-- none | extends `path` | modifies `path` — cycle-level surfaces only -->

**TEST:** CRUD operations work via API
- **Action**: `curl -X POST localhost:3000/api/[entity] -d '{"name":"Test"}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "[entity]_id"
- **Capture**: console

---

## Feature Cycles

> Parallel-eligible once foundation is complete.

### - [ ] Cycle 2: [Feature title] `[P]`

- **Stories:** US-2 — [rationale ≤ 2 lines]
- **Depends on:** C1
- **Case:** [Simple | Split | Merge]
- **Acceptance criteria:** [IDs]
- **Brownfield exposure:** extends `src/models/[entity].py`

**TEST:** [behavior] works end to end via API
- **Setup**: Seed prerequisite [entity] data
- **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"value"}'`
- **Assert**: Response status: 200
- **Assert**: Console contains "[expected field]"
- **Capture**: console
