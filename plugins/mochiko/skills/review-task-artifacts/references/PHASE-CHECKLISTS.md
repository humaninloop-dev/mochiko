# Task-Artifact Review Checklists

Detailed review checklists for task artifacts, organized **by artifact type**: the mapping review
(`task-mapping.md`), the tasks review (`tasks.md`), and the cross-artifact pass that reviews the two
together. The caller (the `/mochiko:tasks` lead) supplies which task artifacts are in scope for a
given review; this reference does not encode a driving sequence — sequencing is the lead's, not the
checklist's.

Each check carries a fixed question and a severity (Critical / Important / Minor). The verdict is
derived **mechanically** from the issue counts (see SKILL.md → Verdict Criteria) — this is a mirror
checklist, not open-ended hunting. Every check here mirrors an authoring rule the producer
(`mochiko:patterns-vertical-tdd`) is taught to apply: the reviewer checks exactly what the producer
is told to build.

> **Dormant / parked checks.** Rows tagged *dormant/parked* below (IP-XXX coverage, platform-app
> ordering, deployment cycle, `[EXTEND]`/`[MODIFY]` brownfield markers, constraint-task
> traceability) belong to the roadmap/brownfield track that is deferred for the core tasks loop.
> They stay present here, in lock-step with the producer's parked side, and activate when that track
> ports — they are not live-required in a core review, and not dropped.

## Mapping Review — `task-mapping.md`

### Checklist Table

| Check | Question | Severity |
|-------|----------|----------|
| Story coverage | Are all P1/P2 stories mapped to cycles? | Critical |
| Cycle identification | Are cycles true vertical slices (not horizontal)? | Critical |
| Foundation separation | Are foundation cycles clearly identified? | Critical |
| Feature parallelization | Are feature cycles marked `[P]` where appropriate? | Important |
| Dependency accuracy | Are cycle dependencies correct and minimal? | Important |
| Slice sizing | Are cycles appropriately sized (not too big/small)? | Important |
| Traceability | Can we trace from story to cycle? | Important |
| Infrastructure coverage — *dormant/parked* | Are all IP-XXX items from constraints-and-decisions.md mapped to foundation cycles? | Critical |
| Platform-app ordering — *dormant/parked* | Do platform foundation cycles precede application foundation cycles? | Critical |

### Key Questions

- Are any P1/P2 stories missing from the mapping?
- Are there cycles that are really horizontal slices (all models, then all services)?
- Is the foundation too large? Too small?
- Are there unnecessary dependencies between feature cycles?
- Are any cycles too large (should be split)? Too small (should be merged)?
- *(dormant/parked)* Are there IP-XXX items with no corresponding cycle? Is platform foundation distinct from application foundation?

### Vertical Slice Validation

For each cycle, ask:
1. Does it deliver observable user value?
2. Does it touch multiple layers (model, service, API)?
3. Can it be tested independently?
4. Is it sized for 1–3 implementation sessions?

If NO to any: the cycle may need restructuring.

---

## Tasks Review — `tasks.md`

### Checklist Table

| Check | Question | Severity |
|-------|----------|----------|
| Cycle coverage | Does every cycle from the mapping have tasks? | Critical |
| TDD structure | Does each cycle have test-first task ordering? | Critical |
| File paths | Does every task have a specific file path? | Critical |
| Verification task | Does each cycle end with a `**TEST:**` verification task? | Critical |
| Real infrastructure | Do verification tasks specify real infrastructure (not mocks)? | Critical |
| Task IDs | Are task IDs properly formatted (`TN.X`)? | Important |
| Story labels | Are tasks linked to stories (`[US#]`) where appropriate? | Important |
| Parallel markers | Are `[P]` markers correctly applied to feature cycles? | Important |
| Checkpoints | Does each cycle have a human-observable checkpoint? | Important |
| Dependencies | Are dependencies between cycles correctly documented? | Important |
| IP-XXX task coverage — *dormant/parked* | Does every IP-XXX item have at least one corresponding task? | Critical |
| Deployment cycle — *dormant/parked* | Is there a deployment/CI/CD cycle if IP-XXX items require it? | Important |
| Brownfield markers — *dormant/parked* | Are `[EXTEND]`/`[MODIFY]` markers correctly applied? | Important |

### Key Questions

- Are any cycles from the mapping missing from `tasks.md`?
- Does every cycle start with a test task?
- Are there tasks without file paths (using vague descriptions)?
- Do the task IDs follow the `TN.X` format?
- Are feature cycles properly marked parallel-eligible?
- Do checkpoints describe observable, human-testable outcomes?
- **Does every cycle end with a `**TEST:**` verification task?**
- **Do verification tasks specify concrete steps against real infrastructure?**
- **Do verification tasks have a clear Setup/Action/Assert structure?**

### TDD Structure Validation

For each cycle, verify task ordering:
1. **First task**: write the failing test (`TN.1`)
2. **Middle tasks**: implementation (`TN.2`, `TN.3`, …)
3. **Near-last task**: refactor and verify the automated tests pass
4. **Last task**: `**TEST:**` verification against real infrastructure

If this order is violated: Critical issue.

### Verification Task Validation

This is a **design-time presence and structure** check on the `**TEST:**` task the producer authored
— confirming the task is well-formed, not executing it (runtime execution is a separate, downstream
consumer). For each cycle's final task, verify:

1. **Is it in `**TEST:**` format?** If it just says "Demo" or "Verify", it may be vague.
2. **Does it name real infrastructure?** Look for concrete paths, commands, or UI actions.
3. **Does it have explicit steps?** Setup/Action/Assert with specific commands.
4. **Does it have observable outcomes?** Clear Assert conditions.
5. **Does it include Capture?** Evidence collection for review (console, screenshot, logs).

**Good verification task:**
```markdown
- [ ] **T2.12**: **TEST:** - File watcher detects real files
  - **Setup**: `mkdir /tmp/test-dir`
  - **Action**: `dart run bin/watcher.dart /tmp/test-dir` (background)
  - **Action**: `sleep 1 && touch /tmp/test-dir/test.jsonl`
  - **Assert**: Console contains "FileWatchEvent: created"
  - **Capture**: console
```

**Bad verification task (REJECT):**
```markdown
- [ ] **T2.12**: Demo: Verify file watching infrastructure is functional
  - Checkpoint: PathValidator correctly rejects symlinks outside scope
```

Why bad? No concrete steps, no real files created, "checkpoint" describes what tests verify, not
observable behavior.

---

## Cross-Artifact Review — `tasks.md` ↔ `task-mapping.md`

Run this when both artifacts are in scope (the cumulative pass): review `tasks.md` in full **and**
cross-check it back against `task-mapping.md`.

### Checklist Table

| Check | Question | Severity |
|-------|----------|----------|
| Mapping-tasks alignment | Does every cycle in the mapping appear in `tasks.md`? | Critical |
| Story traceability | Can we trace Story → Cycle → Tasks? | Critical |
| Cycle consistency | Do cycle descriptions match between artifacts? | Important |
| Dependency consistency | Do dependencies match between artifacts? | Important |
| Foundation-feature alignment | Is the foundation/feature classification consistent? | Important |
| Constraint-task traceability — *dormant/parked* | Can we trace C-XXX → IP-XXX → Cycle → Tasks for infrastructure constraints? | Important |

### Cross-Reference Steps

1. **List all cycles from the mapping.**
2. **Verify each appears in `tasks.md`.**
3. **Check descriptions match.**
4. **Verify dependencies match.**
5. **Confirm parallel markers match.**

### Traceability Matrix Validation

Build and verify this chain:

```
US-1 (P1) -> Cycle 1 -> T1.1, T1.2, T1.3, T1.4
US-2 (P1) -> Cycle 2 -> T2.1, T2.2, T2.3, T2.4
US-3 (P2) -> Cycle 3 -> T3.1, T3.2, T3.3, T3.4
```

If any link is broken: Critical issue.

---

## Common Issues

### Mapping

| Issue | Severity | Fix |
|-------|----------|-----|
| Missing P1 story | Critical | Add a cycle for the story |
| Horizontal slice | Critical | Restructure as vertical |
| Missing foundation | Critical | Identify shared infrastructure |
| Too many dependencies | Important | Review if truly required |
| Oversized cycle | Important | Split into smaller cycles |
| Undersized cycle | Minor | Consider merging |

### Tasks

| Issue | Severity | Fix |
|-------|----------|-----|
| Missing cycle | Critical | Add tasks for the cycle |
| No test task first | Critical | Reorder to test-first |
| Vague file paths | Critical | Specify exact paths |
| Missing verification task | Critical | Add a `**TEST:**` task as the final task |
| Mock-only verification | Critical | Rewrite with real-infrastructure steps |
| Vague demo task | Critical | Add concrete Setup/Action/Assert steps |
| Wrong task-ID format | Important | Fix to `TN.X` format |
| Missing checkpoint | Important | Add a verifiable outcome |
| Test-only checkpoint | Important | Rewrite as observable behavior |
| Missing `[P]` marker | Minor | Add if parallel-eligible |

---

## Illustrative Worked Example

The following illustrates **how findings are documented** — issues with evidence, strengths, and a
verdict rationale. The actual deliverable is assembled with `mochiko:advocate-report-template` (see
ISSUE-TEMPLATES.md → Assembled Report); this is a worked illustration, not a second report template.

```markdown
## Tasks Review: .mochiko/specs/042-task-priority/tasks.md

### Verdict: needs-revision

### Critical (2)

**Issue TT-001**: Cycle 3 missing test-first structure

- **Evidence**: T3.1 is "Create PriorityService", T3.2 is "Write tests"
- **Impact**: Violates TDD discipline; tests may be an afterthought
- **Suggested Fix**: Reorder T3.1 to be the test, T3.2+ to be implementation

**Issue TT-002**: Cycle 2 has a mock-only verification task

- **Evidence**: T2.12 says "Demo: Verify file watching infrastructure is functional" with checkpoint "PathValidator correctly rejects symlinks outside scope"
- **Impact**: No real infrastructure exercised. All tests could pass while the feature is broken in production.
- **Suggested Fix**: Rewrite with `**TEST:**` format and real infrastructure (Setup: `mkdir /tmp/watcher-test`; Action: run the watcher; Assert: console shows the create event; Capture: console)

### Important (2)

**Issue TT-003**: Task T4.3 has a vague file path

- **Evidence**: "Update relevant service files"
- **Impact**: Unclear which files will be modified
- **Suggested Fix**: Specify the exact path, e.g. `src/services/task_service.py`

**Issue TT-004**: Cycle 5 checkpoint is test-only

- **Evidence**: Checkpoint says "State updates reactively from file events"
- **Impact**: Describes what tests verify, not what a human observes
- **Suggested Fix**: Rewrite as "Human sees the session appear in the UI within 1 second of creating the file"

### Minor (0)

None

### Strengths

- All P1/P2 stories mapped to cycles
- Foundation cycles clearly separated
- Good use of `[P]` markers for parallel features
- Task IDs follow the correct format

### Verdict rationale

**needs-revision**: 2 Critical issues (TDD ordering, mock-only verification) and 2 Important issues.
The mock-only verification is the most severe — without real-infrastructure testing, the feature
could be broken despite all tests passing. Fixable in one round by the responsible producer using
the unified `**TEST:**` format.
```
