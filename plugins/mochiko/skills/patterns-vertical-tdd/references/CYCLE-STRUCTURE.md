# Cycle Structure Reference

This reference file provides detailed cycle formatting, task structure, and examples. All formats here conform to the canonical `tasks.md` skeleton in [`tasks-template.md`](../../../templates/tasks-template.md); when the two would differ, the template wins. The `**TEST:**` verification-task grammar the producer authors into each cycle is single-sourced in [TEST-GRAMMAR.md](TEST-GRAMMAR.md); this file keeps only a pointer to it (see *Verification Task Requirements*).

## Cycle Anatomy

```markdown
### Cycle N: [Descriptive title of the vertical slice]

> Stories: US-X, US-Y (comma-separated story IDs this cycle covers)
> Dependencies: C1, C2 (cycles that must complete first, or "None")
> Type: Foundation | Feature [P] (Foundation = sequential, Feature [P] = parallel-eligible)

- [ ] **TN.1**: Write failing test for [behavior] in [test file path]
- [ ] **TN.2**: Implement [component] to pass test in [source file path]
- [ ] **TN.3**: Refactor and verify tests pass
- [ ] **TN.4**: **TEST:** - [What to verify with real infrastructure]
  - **Setup**: [Prerequisites] (optional)
  - **Action**: [Command or instruction]
  - **Assert**: [Expected outcome]
  - **Capture**: [console, screenshot, logs] (optional)

**Checkpoint**: [Observable outcome when cycle is complete]
```

## Task ID Format

Tasks use hierarchical IDs: `T{cycle}.{sequence}`

| Cycle | Task IDs |
|-------|----------|
| Cycle 1 | T1.1, T1.2, T1.3, T1.4 |
| Cycle 2 | T2.1, T2.2, T2.3, T2.4 |
| Cycle 3 | T3.1, T3.2, T3.3, T3.4 |

If a cycle needs more than 4 tasks:
- T1.5, T1.6, etc. for additional implementation
- Keep test task first, TEST verification task last

## File Path Conventions

Every task MUST include a specific file path.

### Test Files
```
tests/e2e/test_[feature].py
tests/integration/test_[feature].py
tests/unit/test_[module].py
tests/contract/test_[endpoint].py
```

### Source Files
```
src/models/[entity].py
src/services/[service].py
src/api/[endpoint].py
src/[feature]/[component].py
```

### Adjust for Project Structure

| Project Type | Source | Tests |
|--------------|--------|-------|
| Single app | `src/` | `tests/` |
| Backend/Frontend | `backend/src/`, `frontend/src/` | `backend/tests/`, `frontend/tests/` |
| Monorepo | `packages/[pkg]/src/` | `packages/[pkg]/tests/` |

## Foundation Cycle Examples

### Example: Core Entity

```markdown
### Cycle 1: Task entity and basic creation

> Stories: US-1
> Dependencies: None
> Type: Foundation

- [ ] **T1.1**: Write failing E2E test for task creation in tests/e2e/test_task_creation.py
- [ ] **T1.2**: Create Task model with title, status fields in src/models/task.py
- [ ] **T1.3**: Implement TaskService.create() in src/services/task_service.py
- [ ] **T1.4**: Create POST /api/tasks endpoint in src/api/tasks.py
- [ ] **T1.5**: Refactor and verify tests pass
- [ ] **T1.6**: **TEST:** - Task created via API and retrievable
  - **Action**: `curl -X POST localhost:3000/api/tasks -d '{"title":"Test"}'`
  - **Assert**: Response status: 201
  - **Capture**: console

**Checkpoint**: Can create a task via API and retrieve it
```

### Example: Authentication

```markdown
### Cycle 2: User authentication framework

> Stories: (infrastructure)
> Dependencies: C1
> Type: Foundation

- [ ] **T2.1**: Write failing test for user login in tests/e2e/test_auth.py
- [ ] **T2.2**: Create User model with password hash in src/models/user.py
- [ ] **T2.3**: Implement AuthService with login/logout in src/services/auth_service.py
- [ ] **T2.4**: Create POST /api/auth/login endpoint in src/api/auth.py
- [ ] **T2.5**: Add JWT middleware in src/middleware/auth.py
- [ ] **T2.6**: Refactor and verify tests pass
- [ ] **T2.7**: **TEST:** - Login returns a valid auth token
  - **Action**: `curl -X POST localhost:3000/api/auth/login -d '{"email":"test@example.com","password":"test"}'`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "token"
  - **Capture**: console

**Checkpoint**: Can log in and receive valid auth token
```

## Feature Cycle Examples

### Example: Simple Feature

```markdown
### Cycle 4: [P] Task completion

> Stories: US-2
> Dependencies: C1
> Type: Feature [P]

- [ ] **T4.1**: Write failing test for marking task complete in tests/e2e/test_task_completion.py
- [ ] **T4.2**: [EXTEND] Add completed_at field to Task model in src/models/task.py
- [ ] **T4.3**: Implement TaskService.complete() in src/services/task_service.py
- [ ] **T4.4**: Create PATCH /api/tasks/{id}/complete endpoint in src/api/tasks.py
- [ ] **T4.5**: Refactor and verify tests pass
- [ ] **T4.6**: **TEST:** - Task completion updates timestamp
  - **Action**: `curl -X PATCH localhost:3000/api/tasks/1/complete`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "completed_at"
  - **Capture**: console

**Checkpoint**: Can mark a task as complete and see completion timestamp
```

### Example: Feature with Multiple Stories

```markdown
### Cycle 5: [P] Task filtering

> Stories: US-4, US-6
> Dependencies: C1
> Type: Feature [P]

- [ ] **T5.1**: Write failing tests for status and priority filters in tests/e2e/test_task_filtering.py
- [ ] **T5.2**: Implement TaskService.list() with filter params in src/services/task_service.py
- [ ] **T5.3**: Update GET /api/tasks with query params in src/api/tasks.py
- [ ] **T5.4**: Refactor and verify tests pass
- [ ] **T5.5**: **TEST:** - Filtering returns correct results
  - **Setup**: Create tasks with different statuses and priorities
  - **Action**: `curl "localhost:3000/api/tasks?status=pending&priority=high"`
  - **Assert**: Response status: 200
  - **Assert**: Console contains only matching tasks
  - **Capture**: console

**Checkpoint**: Can filter task list by status and priority via API
```

## Brownfield Markers

When working with existing code, apply markers:

| Marker | When to Use | Example |
|--------|-------------|---------|
| `[EXTEND]` | Adding to existing file | Adding a field to existing model |
| `[MODIFY]` | Changing existing code | Updating existing service method |
| (none) | New file | Creating new endpoint file |

### Example with Brownfield

```markdown
### Cycle 6: [P] Task priority

> Stories: US-3
> Dependencies: C1
> Type: Feature [P]

- [ ] **T6.1**: Write failing test for priority assignment in tests/e2e/test_task_priority.py
- [ ] **T6.2**: [EXTEND] Add priority field to Task model in src/models/task.py
- [ ] **T6.3**: [MODIFY] Update TaskService.create() to accept priority in src/services/task_service.py
- [ ] **T6.4**: [MODIFY] Update POST /api/tasks to accept priority in src/api/tasks.py
- [ ] **T6.5**: Refactor and verify tests pass
- [ ] **T6.6**: **TEST:** - Priority set on create and update
  - **Action**: `curl -X POST localhost:3000/api/tasks -d '{"title":"Urgent","priority":"high"}'`
  - **Assert**: Response status: 201
  - **Assert**: Console contains "priority":"high"
  - **Capture**: console

**Checkpoint**: Can create tasks with priority and update existing task priority
```

## Checkpoint Guidelines

Checkpoints should be:

1. **Observable**: Something observable or demonstrable
2. **Testable**: Automated tests should verify this
3. **Concrete**: Specific behavior, not abstract quality
4. **Verifiable**: Can be verified with real infrastructure (not just mocks)

### Good Checkpoints

- "Task created via API, console shows 201 response"
- "File watcher detects new file, console outputs event"
- "CLI export command creates CSV with correct data"
- "Server starts and responds to health check"

### Bad Checkpoints

- "Task model is complete" (not observable)
- "Code is clean" (subjective)
- "Service layer works" (too vague)
- "Ready for integration" (not testable)
- "All unit tests pass" (automated only, needs real verification)
- "PathValidator correctly rejects symlinks" (tested via mocks, not real files)
- "State updates reactively" (vague, likely tested via mocks)

---

## Verification Task Requirements

The final task of each cycle is the **TEST:** verification task — the gate that ensures
the cycle delivers real, working functionality against real infrastructure (never mocks).
The full grammar — the unified `**TEST:**` format, field definitions, action modifiers,
assert patterns, worked good/bad examples, and legacy-marker support — is single-sourced
in [TEST-GRAMMAR.md](TEST-GRAMMAR.md), split out so the runtime verifier loads only the
grammar. Author every cycle's closing verification task in that grammar.

## Complete Example: Task Management Feature

This worked `tasks.md` illustrates the canonical structure defined by [`tasks-template.md`](../../../templates/tasks-template.md) — the fill-target. It is an illustration, not a second source: the `[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` markers, the `TN.X` task IDs, the `**Checkpoint**:` lines, and the `**TEST:**` blocks below match that template so the two cannot drift. The Story → Cycle table is a **derived echo** of `task-mapping.md` (the source of truth), not an independent second mapping.

```markdown
# Implementation Tasks: task-management

## Foundation Cycles

### Cycle 1: Task entity and basic CRUD

> Stories: US-1
> Dependencies: None
> Type: Foundation

- [ ] **T1.1**: Write failing E2E tests for task CRUD in tests/e2e/test_task_crud.py
- [ ] **T1.2**: Create Task model in src/models/task.py
- [ ] **T1.3**: Implement TaskService in src/services/task_service.py
- [ ] **T1.4**: Create task API endpoints in src/api/tasks.py
- [ ] **T1.5**: Refactor and verify tests pass
- [ ] **T1.6**: **TEST:** - CRUD operations work via API
  - **Action**: `curl -X POST localhost:3000/api/tasks -d '{"title":"Test"}'`
  - **Assert**: Response status: 201
  - **Assert**: Console contains "task_id"
  - **Capture**: console

**Checkpoint**: Can create, read, update, delete tasks via API

---

### Cycle 2: User authentication

> Stories: (infrastructure)
> Dependencies: C1
> Type: Foundation

- [ ] **T2.1**: Write failing test for auth flow in tests/e2e/test_auth.py
- [ ] **T2.2**: Create User model in src/models/user.py
- [ ] **T2.3**: Implement AuthService in src/services/auth_service.py
- [ ] **T2.4**: Create auth endpoints in src/api/auth.py
- [ ] **T2.5**: Add auth middleware in src/middleware/auth.py
- [ ] **T2.6**: Refactor and verify tests pass
- [ ] **T2.7**: **TEST:** - Login returns valid token
  - **Action**: `curl -X POST localhost:3000/api/auth/login -d '{"email":"test@example.com","password":"test"}'`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "token"
  - **Capture**: console

**Checkpoint**: Can authenticate and access protected endpoints

---

## Feature Cycles

### Cycle 3: [P] Task completion

> Stories: US-2
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T3.1**: Write failing test for task completion in tests/e2e/test_completion.py
- [ ] **T3.2**: [EXTEND] Add completion fields to Task in src/models/task.py
- [ ] **T3.3**: [EXTEND] Add complete() to TaskService in src/services/task_service.py
- [ ] **T3.4**: Add completion endpoint in src/api/tasks.py
- [ ] **T3.5**: Refactor and verify tests pass
- [ ] **T3.6**: **TEST:** - Task completion updates timestamp
  - **Setup**: Create task via POST /api/tasks
  - **Action**: `curl -X PATCH localhost:3000/api/tasks/1/complete`
  - **Assert**: Response status: 200
  - **Assert**: Console contains "completed_at"
  - **Capture**: console

**Checkpoint**: Can mark tasks complete with timestamp

---

### Cycle 4: [P] Task priority

> Stories: US-3
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T4.1**: Write failing test for priority in tests/e2e/test_priority.py
- [ ] **T4.2**: [EXTEND] Add priority field to Task in src/models/task.py
- [ ] **T4.3**: [MODIFY] Update TaskService for priority in src/services/task_service.py
- [ ] **T4.4**: [MODIFY] Update task endpoints for priority in src/api/tasks.py
- [ ] **T4.5**: Refactor and verify tests pass
- [ ] **T4.6**: **TEST:** - Priority assignment works
  - **Action**: `curl -X POST localhost:3000/api/tasks -d '{"title":"Urgent","priority":"high"}'`
  - **Assert**: Response status: 201
  - **Assert**: Console contains "priority":"high"
  - **Capture**: console

**Checkpoint**: Can assign and update task priority

---

### Cycle 5: [P] Task filtering

> Stories: US-4
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T5.1**: Write failing tests for filters in tests/e2e/test_filtering.py
- [ ] **T5.2**: [EXTEND] Add filter methods to TaskService in src/services/task_service.py
- [ ] **T5.3**: [MODIFY] Update list endpoint with query params in src/api/tasks.py
- [ ] **T5.4**: Refactor and verify tests pass
- [ ] **T5.5**: **TEST:** - Filtering returns correct results
  - **Setup**: Create tasks with different statuses
  - **Action**: `curl "localhost:3000/api/tasks?status=pending"`
  - **Assert**: Response status: 200
  - **Assert**: Console contains only pending tasks
  - **Capture**: console

**Checkpoint**: Can filter tasks by status and priority

---

### Cycle 6: [P] CSV export

> Stories: US-5
> Dependencies: C1, C2
> Type: Feature [P]

- [ ] **T6.1**: Write failing test for export in tests/e2e/test_export.py
- [ ] **T6.2**: Create ExportService in src/services/export_service.py
- [ ] **T6.3**: Add export endpoint in src/api/export.py
- [ ] **T6.4**: Refactor and verify tests pass
- [ ] **T6.5**: **TEST:** - CSV export creates valid file
  - **Setup**: Create several tasks
  - **Action**: `curl localhost:3000/api/export/csv -o /tmp/tasks.csv`
  - **Assert**: File exists: /tmp/tasks.csv
  - **Assert**: Console contains "Content-Type: text/csv"
  - **Capture**: console

**Checkpoint**: Can export task list to valid CSV file

---

## Story → Cycle Mapping

> Derived echo of `task-mapping.md` (the source of truth for story→cycle decisions and slice rationale). Regenerate from it — do not hand-edit here.

| Story | Priority | Cycle(s) |
|-------|----------|----------|
| US-1 | P1 | C1 |
| US-2 | P1 | C3 |
| US-3 | P2 | C4 |
| US-4 | P2 | C5 |
| US-5 | P3 | C6 |
```
