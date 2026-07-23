# Cycle Report Format

The implementer produces a `cycle-report.md` after each cycle execution (and after each
rework). Envelope + shared rules: `templates/report-format.md` (machine-first, conditional
prose, no restatement) — this file carries only the cycle payload. The report is a truthful
self-disclosure of what happened — not a verdict on whether the result passes; the lead
owns that verdict and the verifier grades independently. Consumers: the lead's checkpoint
verdict (the frontmatter) and, on failure, the debugging trail (the failure narrative).

## Frontmatter Schema

```yaml
---
report: cycle
feature: user-auth
cycle: 3                    # Cycle number (integer) or "fix" for fix passes
attempt: 1                  # Attempt number within this cycle (1 = first attempt)
status: pass | fail | blocked   # Execution outcome self-report (not the checkpoint verdict)
tasks_total: 4              # Total tasks in this cycle's task list
tasks_completed: 4          # Tasks marked [x] after execution
failed_tasks: []            # Task IDs not completed / failing, with a one-line reason each
files_created:              # New files created during this cycle
  - src/routes/api.ts
  - src/routes/api.test.ts
files_modified:             # Existing files modified during this cycle
  - src/models/user.ts
brownfield_tasks: 1         # Count of tasks with [EXTEND] or [MODIFY] markers
domain_deps_added: []       # Domain-layer registry additions this cycle (package names; [] if none)
deviations: []              # One-line, ID-cited departures from the task descriptions ([] if none)
checkpoint_criteria_met: true  # The implementer's self-assessment (the lead verifies independently)
---
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report` / `feature` / `slice` | envelope | yes | Per `templates/report-format.md` (`slice:` only when slice-scoped) |
| `cycle` | integer or `"fix"` | yes | Cycle number from tasks.md, or `"fix"` for fix passes |
| `attempt` | integer | yes | 1 for first attempt, increments on retry |
| `status` | enum | yes | `pass` (all tasks done, tests green) / `fail` (tasks failing) / `blocked` (could not proceed). A self-report of execution outcome, not the checkpoint verdict |
| `tasks_total` | integer | yes | Number of tasks in the cycle task list |
| `tasks_completed` | integer | yes | Number of tasks marked `[x]` after execution |
| `failed_tasks` | list | yes | `[]` if none; else `- {id: T3.2, why: "<one line>"}` per failed/incomplete task |
| `files_created` | list of strings | yes | Paths of new files created (empty list if none) |
| `files_modified` | list of strings | yes | Paths of existing files modified (empty list if none) |
| `brownfield_tasks` | integer | yes | Count of tasks with `[EXTEND]` or `[MODIFY]` markers |
| `domain_deps_added` | list of strings | yes | Domain-layer dependency registry additions made this cycle (empty list if none). The visibility floor for registry growth: additions are disclosed here and surfaced at the checkpoint; at `production`/`regulated` tiers a non-empty list forces a human checkpoint — never auto-approved |
| `deviations` | list of strings | yes | Departures from the task descriptions, one line each, citing the task ID (e.g. `"T3.4: argon2 over bcrypt (C-012 allows)"`). `[]` if none |
| `checkpoint_criteria_met` | boolean | yes | The implementer's assessment of whether the cycle's checkpoint criteria are satisfied; a self-report, not the verdict — the lead verifies independently and decides |

## Conditional Prose

Prose sections per the conditional-prose rule (`report-format.md`) — a clean passing cycle
is frontmatter-only:

### Notes of note *(only when non-empty)*

```markdown
## Notes of note
```

The producer-authored uncertainty carrier: non-obvious decisions (pattern choices where
multiple valid approaches existed, technology choices within a task's scope — one line
each, ID-cited), genuine difficulties, and flagged blockers the lead should weigh at the
checkpoint. Not a narration of what the tasks already describe; cite IDs, never restate
task text.

### Failure narrative *(mandatory when `status` is `fail` or `blocked`, or any task failed)*

```markdown
## Failure narrative
```

Debug value concentrates here — full detail: what failed (per failed task), why (the
failing test/output evidence), what was tried, and the state things were left in. A failed
cycle keeps the fuller narrative; the slim format above is the passing-cycle format.

## Examples

Passing cycle (complete report):

```markdown
---
report: cycle
feature: user-auth
cycle: 3
attempt: 1
status: pass
tasks_total: 4
tasks_completed: 4
failed_tasks: []
files_created:
  - src/routes/api.ts
  - src/routes/api.test.ts
files_modified:
  - src/models/user.ts
brownfield_tasks: 1
domain_deps_added: []
deviations:
  - "T3.4: followed existing Sequelize patterns for the [EXTEND] on User"
checkpoint_criteria_met: true
---
```

Failed attempt:

```markdown
---
report: cycle
feature: user-auth
cycle: 4
attempt: 2
status: fail
tasks_total: 3
tasks_completed: 2
failed_tasks:
  - {id: T4.3, why: "auth middleware test red — token refresh race"}
files_created: []
files_modified:
  - src/middleware/auth.ts
brownfield_tasks: 0
domain_deps_added: []
deviations: []
checkpoint_criteria_met: false
---

## Failure narrative

T4.3's refresh test (`auth.refresh.test.ts:41`) fails intermittently: the refresh handler
reads `user.lastLogin` before C3's write commits. Tried serializing on the session row
(still races under the test's parallel logins) and moving the read behind the commit hook
(breaks the T4.2 contract test). The middleware currently guards with a retry, which the
test's timing still beats about 1 run in 5. Left red; needs a decision between a
transaction boundary change (touches C3 code) and relaxing the timing assertion.
```
