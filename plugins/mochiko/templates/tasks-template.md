<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by
     construction. Task lines stay one line each (path + behavior); cite spec/plan
     content by ID, never re-quote it; no doctrine sections (TDD discipline, execution
     strategy) — the skills single-source those. -->

# Implementation Tasks: [FEATURE NAME]

> Generated from `.mochiko/specs/<feature>/`: spec.md, plan.md, requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/
> Structure: Vertical slices organized as TDD cycles

## Overview

| Metric | Value |
|--------|-------|
| Total Cycles | [N] |
| Foundation Cycles | [N] |
| Feature Cycles | [N] |
| Parallel Opportunities | [N] |

## Cycle Format

Each cycle follows TDD discipline:
- **TN.1**: Write failing test first
- **TN.2+**: Implement to pass test
- **TN.X-1**: Refactor and verify
- **TN.X**: **TEST:** — verify [behavior] with real infrastructure (Setup/Action/Assert/Capture)

The final task of every cycle is a real-infrastructure `**TEST:**` verification task — not a plain demo. It exercises the slice against real APIs, files, or databases and **gates cycle completion**. See [`TEST-GRAMMAR.md`](../skills/patterns-vertical-tdd/references/TEST-GRAMMAR.md) for the canonical `**TEST:**` grammar (Setup/Action/Assert/Capture) and worked examples.

### Markers

| Marker | Meaning |
|--------|---------|
| `[P]` | Parallel-eligible (independent of other feature cycles) |
| `[US#]` | Maps to user story (e.g., [US1], [US2]) |
| `[EXTEND]` | Extends existing file (brownfield) |
| `[MODIFY]` | Modifies existing code (brownfield) |

---

<!--
  ============================================================================
  IMPORTANT: The cycles below are SAMPLE CYCLES for illustration purposes.

  The tasks workflow MUST replace these with actual cycles based on:
  - User stories from .mochiko/specs/<feature>/spec.md (with their priorities P1, P2, P3...)
  - Design artifacts from .mochiko/specs/<feature>/: plan.md, requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/

  Cycles MUST follow vertical slicing + TDD principles:
  - Each cycle delivers observable, testable user value
  - Test task comes BEFORE implementation tasks
  - Foundation cycles are sequential; feature cycles can parallelize

  More worked cycle examples (auth, filtering, brownfield markers):
  skills/patterns-vertical-tdd/references/CYCLE-STRUCTURE.md.
  DO NOT keep these sample cycles in the generated tasks.md file.
  ============================================================================
-->

## Foundation Cycles

> Sequential cycles that establish shared infrastructure. All must complete before feature cycles begin.

---

### Cycle 1: Core entity and basic CRUD

> Stories: US-1
> Dependencies: None
> Type: Foundation

- [ ] **T1.1**: Write failing E2E test for entity creation in tests/e2e/test_[entity]_crud.py
- [ ] **T1.2**: Create [Entity] model with core fields in src/models/[entity].py
- [ ] **T1.3**: Implement [Entity]Service with CRUD operations in src/services/[entity]_service.py
- [ ] **T1.4**: Create [entity] API endpoints in src/api/[entity].py
- [ ] **T1.5**: Refactor and verify all tests pass
- [ ] **T1.6**: **TEST:** - CRUD operations work via API
  - **Action**: `curl -X POST localhost:3000/api/[entity] -d '{"name":"Test"}'`
  - **Assert**: Response status: 201
  - **Assert**: Console contains "[entity]_id"
  - **Capture**: console

**Checkpoint**: Can create, read, update, delete [entity] via API

---

## Feature Cycles

> Parallel-eligible cycles that deliver user value. Can proceed independently once foundation is complete.

---

### Cycle 2: [P] [Feature title]

> Stories: US-2
> Dependencies: C1
> Type: Feature [P]

- [ ] **T2.1**: Write failing test for [behavior] in tests/e2e/test_[feature].py
- [ ] **T2.2**: [EXTEND] Add [field/method] to [Entity] in src/models/[entity].py
- [ ] **T2.3**: Implement [feature] logic in src/services/[service].py
- [ ] **T2.4**: Add [feature] endpoint in src/api/[endpoint].py
- [ ] **T2.5**: Refactor and verify all tests pass
- [ ] **T2.6**: **TEST:** - [behavior] works end to end via API
  - **Setup**: Seed prerequisite [entity] data
  - **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"value"}'`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "[expected field]"
  - **Capture**: console

**Checkpoint**: [Observable outcome]

---

## Story → Cycle Mapping  *(the ID index)*

> Derived echo of `.mochiko/specs/<feature>/task-mapping.md` (the source of truth for slice rationale and story→cycle decisions). This table is a read-only summary view regenerated from that file — not a second authoritative source. Cycle dependencies live on each cycle's `> Dependencies:` header line — no separate dependency diagram.

| Story | Priority | Cycle(s) |
|-------|----------|----------|
| US-1 | P1 | C1 |
| US-2 | P1 | C2 |
