<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by construction,
     human-legible. This file is CYCLE CARDS, not a task list: each card is a coherent bundle
     of named test cases (the card's content), and the builder decomposes each card into
     concrete tasks at build time, with the code in view (the decomposition is disclosed in the
     cycle report, never pre-written here). Cite spec/plan content by ID (US-#, FR-#, SC-#, C-#)
     — never re-quote it; each test case cites the ID(s) it covers. Register: `full` per
     artifact-format.md rule 11; TEST-gate commands, file paths, and identifiers are
     never-compress items. -->

# Implementation Cycles: [FEAT-XXX — FEATURE NAME]

> Generated from the spec folder and the feature's produced design artifacts: spec.md, features/FEAT-XXX/plan.md, and whichever of requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/ the approved proposal included
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | [N] |
| Stories covered | [US-# list — every P1/P2 story on at least one card] |

## Cycle Format

Each card is one vertical slice: a coherent bundle of **named test cases** (expected behaviour,
in the `**TEST:**` grammar — see [`TEST-GRAMMAR.md`](../skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md)
for the canonical Setup/Action/Assert/Capture grammar) that demonstrate together. The builder
implements the card test-first (red/green/refactor per `mochiko:executing-tdd-cycle`,
decomposition at build time), and the cycle is done when its named cases show green against real
infrastructure. **The card's checkbox is the progress surface**, flipped when the bundle's cases
pass. Where the work opens a new end-to-end path (greenfield / new path), the **first cycle is a
walking skeleton**; growth on an already-standing path skips it. There is no foundation/feature
card type — `[P]` parallel eligibility derives from a card's dependencies, not from a type
column. Each named test case cites the spec/plan ID(s) it covers.

---

<!--
  The two cards below are SAMPLES for illustration — replace them with actual cycles from
  the feature's spec + plan artifacts. DO NOT keep them in the generated tasks.md.
-->

### - [ ] Cycle 1: Walking skeleton — [thinnest end-to-end path]

- **Stories:** US-1 — thinnest end-to-end path through all layers, one trivial case green; establishes the production-shaped stack [≤ 2 lines]
- **Depends on:** —
- **Case:** Simple <!-- Simple | Split — why, one line | Merge — why, one line -->
- **Brownfield exposure:** none <!-- none | extends `path` | modifies `path` — cycle-level surfaces only -->

**TEST:** [entity] round-trips through the full stack
- **Covers**: US-1 / SC-1
- **Action**: `curl -X POST localhost:3000/api/[entity] -d '{"name":"Test"}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "[entity]_id"
- **Capture**: console

---

### - [ ] Cycle 2: [Feature bundle title] `[P]`

- **Stories:** US-2 — [why these stories/cases form one demonstrable bundle, ≤ 2 lines]
- **Depends on:** C1
- **Case:** [Simple | Split — why | Merge — why]
- **Brownfield exposure:** extends `src/models/[entity].py`

**TEST:** [behavior] works end to end via API
- **Covers**: US-2 / SC-2 scenario 1
- **Setup**: Seed prerequisite [entity] data
- **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"value"}'`
- **Assert**: Response status: 200
- **Assert**: Console contains "[expected field]"
- **Capture**: console

**TEST:** [behavior] rejects [invalid case]
- **Covers**: US-2 / SC-2 scenario 2
- **Setup**: Seed prerequisite [entity] data
- **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"invalid"}'`
- **Assert**: Response status: 400
- **Capture**: console
