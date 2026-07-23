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

### Cycle 2: Authentication framework

> Stories: (infrastructure)
> Dependencies: C1
> Type: Foundation

- [ ] **T2.1**: Write failing test for authentication flow in tests/e2e/test_auth.py
- [ ] **T2.2**: Create User model with auth fields in src/models/user.py
- [ ] **T2.3**: Implement AuthService with login/logout in src/services/auth_service.py
- [ ] **T2.4**: Create auth endpoints in src/api/auth.py
- [ ] **T2.5**: Add auth middleware in src/middleware/auth.py
- [ ] **T2.6**: Refactor and verify all tests pass
- [ ] **T2.7**: **TEST:** - Login returns a valid auth token
  - **Action**: `curl -X POST localhost:3000/api/auth/login -d '{"email":"test@example.com","password":"test"}'`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "token"
  - **Capture**: console

**Checkpoint**: Can authenticate and receive valid token

---

## Feature Cycles

> Parallel-eligible cycles that deliver user value. Can proceed independently once foundation is complete.

---

### Cycle 3: [P] [Feature title]

> Stories: US-2
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T3.1**: Write failing test for [behavior] in tests/e2e/test_[feature].py
- [ ] **T3.2**: [EXTEND] Add [field/method] to [Entity] in src/models/[entity].py
- [ ] **T3.3**: Implement [feature] logic in src/services/[service].py
- [ ] **T3.4**: Add [feature] endpoint in src/api/[endpoint].py
- [ ] **T3.5**: Refactor and verify all tests pass
- [ ] **T3.6**: **TEST:** - [behavior] works end to end via API
  - **Setup**: Seed prerequisite [entity] data
  - **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"value"}'`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "[expected field]"
  - **Capture**: console

**Checkpoint**: [Observable outcome]

---

### Cycle 4: [P] [Another feature title]

> Stories: US-3
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T4.1**: Write failing test for [behavior] in tests/e2e/test_[feature].py
- [ ] **T4.2**: Create [Component] in src/[path]/[component].py
- [ ] **T4.3**: Integrate with existing services in src/services/[service].py
- [ ] **T4.4**: Add API endpoint in src/api/[endpoint].py
- [ ] **T4.5**: Refactor and verify all tests pass
- [ ] **T4.6**: **TEST:** - [feature] works via API
  - **Action**: `curl -X POST localhost:3000/api/[endpoint] -d '{"[field]":"[value]"}'`
  - **Assert**: Response status: 201
  - **Assert**: Console contains "[field]":"[value]"
  - **Capture**: console

**Checkpoint**: [Observable outcome]

---

### Cycle 5: [P] [Third feature title]

> Stories: US-4, US-5
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T5.1**: Write failing tests for [behaviors] in tests/e2e/test_[feature].py
- [ ] **T5.2**: Implement [feature] in src/services/[service].py
- [ ] **T5.3**: [MODIFY] Update API endpoint in src/api/[endpoint].py
- [ ] **T5.4**: Refactor and verify all tests pass
- [ ] **T5.5**: **TEST:** - [features] return correct results via API
  - **Setup**: Seed [entity] records covering each [feature] case
  - **Action**: `curl "localhost:3000/api/[endpoint]?[param]=[value]"`
  - **Assert**: Response status: 200
  - **Assert**: Console contains only matching [entity] records
  - **Capture**: console

**Checkpoint**: [Observable outcome]

---

## Execution Strategy

### MVP Delivery (Foundation + First Feature)

1. Complete Foundation Cycles (C1, C2) sequentially
2. Complete first Feature Cycle (C3)
3. **STOP and VALIDATE**: Test independently, demo to stakeholders
4. Deploy if ready

### Incremental Delivery

After foundation:
1. Feature cycles can proceed in any priority order
2. Each feature cycle is independently testable
3. Deploy after any complete cycle

### Parallel Team Strategy

With multiple developers after foundation:
- Developer A: Cycle 3
- Developer B: Cycle 4
- Developer C: Cycle 5

All feature cycles merge independently.

---

## Dependencies Summary

### Cycle Dependencies

```
C1 (Foundation) → C2 (Foundation)
                      ↓
         ┌───────────┼───────────┐
         ↓           ↓           ↓
      C3 [P]      C4 [P]      C5 [P]
```

### Story → Cycle Mapping

> Derived echo of `.mochiko/specs/<feature>/task-mapping.md` (the source of truth for slice rationale and story→cycle decisions). This table is a read-only summary view regenerated from that file — not a second authoritative source.

| Story | Priority | Cycle(s) |
|-------|----------|----------|
| US-1 | P1 | C1 |
| US-2 | P1 | C3 |
| US-3 | P2 | C4 |
| US-4, US-5 | P2 | C5 |

---

## Notes

- **TDD Discipline**: Every cycle starts with a failing test and ends with a real-infrastructure `**TEST:**` verification task that gates cycle completion
- **Vertical Slices**: Each cycle delivers testable user value
- **Foundation First**: All foundation cycles must complete before features
- **Parallel Features**: Feature cycles marked [P] can run in parallel
- **Commit Strategy**: Commit after each task or logical group
- **Checkpoint Validation**: Verify checkpoint after each cycle completes
