# Bundle Identification Heuristics

This reference file provides detailed guidance on identifying good bundles — coherent
bundles of named test cases that demonstrate together — from requirements.

## The Value Stream Test

For each potential bundle, ask: "Can a user observe or use this independently?"

| Answer | Action |
|--------|--------|
| Yes, directly | Good bundle candidate |
| Yes, but needs other bundles first | Check if it's a dependency or can be deferred |
| No, it's infrastructure | Home it inside the first bundle that needs it (skeleton-path infra → the skeleton cycle) |
| No, it's internal refactoring | Not a bundle on its own; attach to one |

## Extraction from User Stories

### Step 1: List All User Stories

Extract from spec.md:
```
US-1-titled-task-creation (P1): As a user, I can create a task with a title
US-2-task-completion-marking (P1): As a user, I can mark a task as complete
US-3-task-priority-setting (P2): As a user, I can set task priority
US-4-task-status-filter (P2): As a user, I can filter tasks by status
US-5-task-csv-export (P3): As a user, I can export tasks to CSV
```

### Step 2: Identify the Walking Skeleton

Where the work opens a **new end-to-end path** — a greenfield feature or a genuinely new path
through the system — the first cycle is a **walking skeleton**: the thinnest end-to-end path
through all layers with one trivial case green. It is the foundation, by construction; the
infrastructure the skeleton path needs (including platform provisioning, IP-XXX from
constraints-and-decisions.md) lands **in the skeleton cycle**, because the skeleton deploys
production-shaped.

Growth or delta work on an **already-standing path skips the skeleton** — the path already
exists. All remaining infrastructure lands inside the first bundle that needs it (YAGNI at cycle
grain). **Infra-only cycles are never minted.**

### Step 3: Map Stories to Cycles

Each cycle is a bundle of named test cases. C1-walking-skeleton-path is the walking skeleton where one is
warranted.

| Story | Cycle | Rationale |
|-------|-------|-----------|
| `task-tracker` US-1-titled-task-creation | C1-walking-skeleton-path | Thinnest end-to-end path; one trivial case green establishes the stack |
| `task-tracker` US-2-task-completion-marking | C2-task-completion-transition | Completion behaviour — its own demonstrable bundle |
| `task-tracker` US-3-task-priority-setting | C3-task-priority-handling | Priority behaviour; independent of `task-tracker` US-2-task-completion-marking |
| `task-tracker` US-4-task-status-filter | C4-task-status-filtering | Query/filter behaviour; independent of `task-tracker` US-3-task-priority-setting |
| `task-tracker` US-5-task-csv-export | C5-csv-export-download | Export behaviour; can parallelize with others |

Record these decisions and their rationale on the cycle cards themselves — each card's Stories
line carries its story set and bundle rationale; the Case field records Simple/Split/Merge with
its one-line why.

### Step 4: Identify Parallelization

After the skeleton, parallel eligibility derives from dependencies (never from a card type):
- C2, C3, C4, C5 have no inter-dependencies → all proceed in parallel
- Mark each with [P]

## Bundle Grain

A cycle is a bundle of test cases **worth demonstrating together** — a demo the user would want
to watch pass. Grain is judged by demonstrability, not clock time.

### Too thin

Signs a bundle is too thin:
- No coherent behaviour a user or operator could watch demonstrated
- A single case that means nothing on its own

**Fix**: Merge with the related cases until the bundle is worth demonstrating.

### Too broad

Signs a bundle is too broad:
- Its cases span more than one distinct demonstration
- Internal phases ("first this whole demo, then that whole demo")

**Fix**: Split into separate bundles, each a demonstration of its own.

### Just right

A well-grained bundle:
- Its cases demonstrate one coherent behaviour together
- Passes or fails as a unit the user would want to watch
- Obvious when it's "done" — the named cases show green on real infrastructure

## Dependency Analysis

### Dependency Types

| Type | Description | Handling |
|------|-------------|----------|
| Data | Cycle B needs entity from Cycle A | B depends on A |
| API | Cycle B calls endpoint from Cycle A | B depends on A |
| UI | Cycle B shows component from Cycle A | B depends on A |
| Infrastructure | Cycle B needs a platform resource (IP-XXX) | Home it in the first bundle that needs it; skeleton-path infra in the skeleton |
| None | Cycles are independent | Both can be [P] |

### Minimizing Dependencies

1. **Home shared infrastructure where it is first needed**
   - Don't make bundles depend on each other for infra
   - The first bundle that needs a resource carries it; the skeleton carries the skeleton-path infra

2. **Accept some duplication**
   - If extracting creates complexity, duplicate
   - Refactor later in a dedicated cycle

3. **Order by priority when dependencies exist**
   - If C4-task-status-filtering depends on C3-task-priority-handling, and C3-task-priority-handling
     is P2 while C4-task-status-filtering is P3, natural order works

## Worked Example — Skeleton First, Then Bundles

```
C1-walking-skeleton-path — Walking skeleton: create-and-read a task round-trips through the
    full stack, one trivial case green (model + service + endpoint + storage, production-shaped)

C2-task-completion-transition — [P] Completion: mark a task complete; its cases demonstrate the
    status transition
C3-task-priority-handling — [P] Priority: set and read task priority
C4-task-status-filtering — [P] Filtering: filter tasks by status
```

The skeleton is the foundation by construction — no separate "all models" or "all services"
cycle. Each later cycle is a bundle of cases demonstrable on its own.

## Anti-Patterns

### Horizontal Slicing

**Wrong**:
```
C1-all-database-models
C2-all-service-classes
C3-all-api-endpoints
C4-all-test-suites
```

**Problem**: Nothing is demonstrable until C4-all-test-suites completes.

### Big Bang Integration

**Wrong**:
```
C1-entire-backend-build
C2-entire-frontend-build
C3-backend-frontend-integration
```

**Problem**: Integration issues discovered too late — the walking skeleton exists precisely to
prove the end-to-end path first.

### Premature Generalization

**Wrong**:
```
C1-generic-crud-framework
C2-framework-entity-rollout
```

**Problem**: Framework complexity without concrete use case.

## Decision Matrix

When unsure how to bundle, use this matrix:

| Question | If Yes | If No |
|----------|--------|-------|
| Do its cases demonstrate on their own? | Good bundle | Merge until they do |
| Does it need other cycles first? | Order by dependency | Can be [P] |
| Do its cases span more than one distinct demonstration? | Split it | Good bundle |
| Can its cases run against real infrastructure? | Good bundle | Reconsider boundaries |
