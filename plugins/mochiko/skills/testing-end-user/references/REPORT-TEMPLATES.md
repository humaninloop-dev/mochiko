# Verification Report Formats

Envelope + shared rules: `templates/report-format.md` — this file carries only the
verification payloads. Two distinct surfaces:

1. **The verification report file** — persisted per verification run (per-cycle and
   final-validation in the implement workflow, at the path the caller names). Machine-first:
   the frontmatter is the report; prose only on failure.
2. **The checkpoint presentation** — in-memory, shown to the human at the gate, discarded
   after the decision. Never persisted.

Reports are adaptive: minimal for success, rich for failures — a passing verification is
frontmatter-only; debug value concentrates in failures, which keep full detail.

## The Verification Report File

### Frontmatter Schema

```yaml
---
report: verification        # or final-validation for the whole-implementation run
feature: user-auth
cycle: 3                    # omitted on final-validation
attempt: 1                  # pairs with the cycle-report attempt this run verifies
status: pass | fail | partial | timeout | error
test_tasks:                 # one row per **TEST:** task executed
  - {id: T3.5, classification: CLI, status: pass, asserts: "4/4", duration: 3.2s}
  - {id: T3.6, classification: GUI, status: pass, asserts: "2/2", duration: 8.1s,
     evidence: "/tmp/claude/verify-T3.6-shot.png"}
quality_gates:
  lint:  {status: pass, command: "pnpm lint"}
  build: {status: pass, command: "pnpm build"}
  tests: {status: pass, command: "pnpm test", passed: 47, failed: 0, skipped: 2}
recommendation: approve | reject | retry | needs-human
---
```

### Field Definitions

| Field | Required | Description |
|-------|----------|-------------|
| `report` / `feature` / `slice` | yes | Per the envelope (`slice:` only when slice-scoped); `final-validation` for the whole-implementation run |
| `cycle` / `attempt` | per-cycle only | The cycle and attempt this verification pairs with |
| `status` | yes | Aggregate result: `pass` only when every assert passed and every gate is green; `partial` for mixed; `timeout`/`error` per the result classification |
| `test_tasks` | yes | One row per `**TEST:**` task: `id`, `classification` (CLI / GUI / SUBJECTIVE — drives auto-approve vs human checkpoint), `status`, `asserts` (passed/total), `duration`, `evidence` (path) where captured |
| `quality_gates` | yes | One entry per gate run: `status` from the exit code (`0` = pass — deterministic, never a judgment), `command`, and pass/fail/skip counts for test suites |
| `recommendation` | yes | The verifier's recommendation to the gate — input to the lead's verdict, never the verdict |

### Conditional Prose *(mandatory when `status` is not `pass`)*

Per failed/partial/timed-out/errored task, full detail under one section:

```markdown
## Failures

### T{N}.{X} — {FAIL | PARTIAL n/m | TIMEOUT | ERROR}

| # | Assert | Expected | Actual | Status |
|---|--------|----------|--------|--------|
| 1 | Console contains "FileWatchEvent: created" | Match | No match | FAIL |

**Output** (bounded excerpt — see Truncation):
```
Error: Permission denied for inotify
```

**Actions run:** `dart run bin/watcher.dart /tmp/watcher-test` (bg) · `touch /tmp/watcher-test/test.jsonl` (0.1s, exit 0)
**Analysis:** {what went wrong, one short paragraph}
**Phase** (ERROR only): setup | action | assert · **Limit** (TIMEOUT only): {elapsed}s of {limit}s
```

Failed quality gates likewise: the gate's failing output excerpt under `## Failures`.
A passing report carries **no prose** — no evidence tables, no narration; the captured
evidence stays in logs/scratch, pointed to by `evidence:` fields.

### Examples

Passing per-cycle report (complete file):

```markdown
---
report: verification
feature: user-auth
cycle: 3
attempt: 1
status: pass
test_tasks:
  - {id: T3.5, classification: CLI, status: pass, asserts: "4/4", duration: 3.2s}
quality_gates:
  lint:  {status: pass, command: "pnpm lint"}
  build: {status: pass, command: "pnpm build"}
  tests: {status: pass, command: "pnpm test", passed: 47, failed: 0, skipped: 2}
recommendation: approve
---
```

Failing per-cycle report:

```markdown
---
report: verification
feature: user-auth
cycle: 4
attempt: 2
status: fail
test_tasks:
  - {id: T4.6, classification: CLI, status: fail, asserts: "1/2", duration: 5.1s}
quality_gates:
  lint:  {status: pass, command: "pnpm lint"}
  build: {status: pass, command: "pnpm build"}
  tests: {status: fail, command: "pnpm test", passed: 46, failed: 1, skipped: 2}
recommendation: reject
---

## Failures

### T4.6 — FAIL

| # | Assert | Expected | Actual | Status |
|---|--------|----------|--------|--------|
| 1 | Console contains "FileWatchEvent: created" | Match | No match | FAIL |
| 2 | File exists: /tmp/watcher-test/test.jsonl | Exists | Exists | PASS |

**Output:**
```
Starting file watcher...
Watching directory: /tmp/watcher-test
Error: Permission denied for inotify
```

**Actions run:** `dart run bin/watcher.dart /tmp/watcher-test` (bg) · `touch /tmp/watcher-test/test.jsonl` (0.1s, exit 0)
**Analysis:** The watcher failed to start (inotify permissions); the touch succeeded but no
event was detected. Same failure as the gate's `watcher_test.dart` red.
```

## Checkpoint Presentation

After generating the report, present the checkpoint to the human (in-memory — never
persisted; regenerate full evidence on "View Details").

### All Pass

```
AskUserQuestion(
  questions: [{
    question: "Verification T{N}.{X} passed.\n\nAll {count} assertions passed in {time}s.\n\nRecommendation: Approve",
    header: "Checkpoint",
    options: [
      {label: "Approve", description: "Proceed to next task"},
      {label: "View Details", description: "Show full evidence"},
      {label: "Retry", description: "Re-run verification"}
    ],
    multiSelect: false
  }]
)
```

### Any Failure

```
AskUserQuestion(
  questions: [{
    question: "Verification T{N}.{X} needs review.\n\n{pass}/{total} assertions passed.\nFailed: {failed_assert_summary}\n\nRecommendation: {recommendation}",
    header: "Checkpoint",
    options: [
      {label: "Approve", description: "Accept despite failures"},
      {label: "Reject", description: "Block completion"},
      {label: "Retry", description: "Re-run with adjustments"}
    ],
    multiSelect: false
  }]
)
```

### If Retry Selected

```
AskUserQuestion(
  questions: [{
    question: "What adjustments should be made?",
    header: "Retry",
    options: [
      {label: "Increase timeout", description: "Add more time for slow operations"},
      {label: "Retry as-is", description: "Run again without changes"},
      {label: "Skip assertion", description: "Remove problematic assertion"}
    ],
    multiSelect: false
  }]
)
```

## Truncation

Failure evidence in the report is bounded; full evidence always survives in a log file the
report points to:

- **Output excerpts:** if over 50 lines, include first 25 / last 25 with `[{N} lines
  truncated]` between, plus the full-log path: `` Full log: /tmp/claude/verify-T{N}.{X}-output.log ``
- **Assert tables:** if more than 10 asserts, show the failing rows plus the first passing
  rows to 10 total, with a count note.

## Storage

- **The verification report file** is persisted at the path the caller names (in implement:
  per-cycle and final-validation reports in the feature/slice directory) — it is what the
  lead Reads for the verdict and what a resumed run finds as workspace evidence.
- **The checkpoint presentation** is generated in memory, shown at the gate, and discarded
  after the human decision.
- **Captured evidence** (full console output, screenshots, logs) lives outside the report
  (`/tmp/claude/…` or the caller's scratch), referenced by path from `evidence:` fields and
  truncation pointers.
