---
name: testing-end-user
description: This skill MUST be invoked when executing a `**TEST:**` verification task against real infrastructure — parsing its Setup/Action/Assert fields, running the actions (honoring `(background)` / `(timeout Ns)` / `(in path)` modifiers) with captured evidence, evaluating the asserts against that evidence, and classifying the task CLI/GUI/SUBJECTIVE to decide auto-approve versus human checkpoint. SHOULD also invoke when running quality gates (lint / build / test) as deterministic exit-code checks during verification, capturing execution evidence, classifying a verification result (PASS/FAIL/PARTIAL/TIMEOUT/ERROR), or presenting a verification checkpoint for human approval. Consumes the `**TEST:**` grammar owned by patterns-vertical-tdd; verifies against real infrastructure, never mocks.
---

# End-User Verification Testing

## Overview

Execute verification tasks that validate real infrastructure behavior through structured Setup/Action/Assert sequences. Classify tasks at runtime (CLI/GUI/SUBJECTIVE) to determine whether to auto-approve or present a human checkpoint. This skill turns a task marked with `**TEST:**` into an executed verification sequence with captured evidence, a classified result, and a checkpoint.

**Violating the letter of the rules is violating the spirit of the rules.**

Verification testing exists to catch failures before they reach production. Every shortcut in this process is a potential production incident waiting to happen.

**Grammar ownership (single source):** the `**TEST:**` construct — its legal marker set, field skeleton, action-modifier vocabulary, and assert-pattern vocabulary — is authored and owned by `patterns-vertical-tdd` in [`CYCLE-STRUCTURE.md`](../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md). This skill **consumes** that grammar; it does not redefine it. What this skill owns is the **runtime**: how to detect, parse, execute, evaluate, capture, classify, report, and gate. Where the grammar and the execution meet below, the vocabulary is referenced and the *how* is retained.

## When to Use

- Tasks marked with `**TEST:**` (or a legacy marker in the same family — see the grammar owner)
- CLI command verification with expected output
- File system state validation
- Real process behavior testing (not mocks)
- GUI/UI verification requiring human judgment
- End-to-end validation before release
- Quality gate execution (lint, build, test suite) as part of verification

## When NOT to Use

- Unit tests that run in isolation
- Mock-based testing without infrastructure
- Static code analysis tasks
- Documentation review tasks
- Tasks without clear pass/fail criteria
- When the verification environment is unavailable

## Core Process

### Task Detection

Detect a verification task by its marker line:

```markdown
- [ ] **TN.X**: **TEST:** - {Description}
```

The full field skeleton (`**Setup**` / `**Action**` / `**Assert**` / `**Capture**`) and the legal marker set — unified `**TEST:**` plus the legacy variants (`TEST:VERIFY`, `TEST:CONTRACT`, `HUMAN VERIFICATION`) — are defined by the grammar owner in [`CYCLE-STRUCTURE.md`](../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md) (§ *Unified TEST: Format*, § *Legacy Format Support*). Consume that vocabulary; do not re-enumerate it here. What this skill owns is *how to find and read those tasks* — the detection boundaries and field-extraction algorithm live in [references/TASK-PARSING.md](references/TASK-PARSING.md).

### Execution Sequence

Execute in strict order. No skipping steps. No reordering.

**1. Parse Task**

Extract the structured task (ID, test type, setup, actions with modifiers, asserts, capture requirements, human-review criteria) per the parsing algorithm in [references/TASK-PARSING.md](references/TASK-PARSING.md). Legacy markers are normalized into the unified structure during parsing.

**2. Execute Setup**

Run setup commands sequentially. Fail fast if any setup fails — a setup failure blocks action execution. Record all setup output for debugging.

**3. Execute Actions**

Run each action honoring its modifiers. The modifier *vocabulary* — `(background)`, `(timeout Ns)`, `(in path)` — is defined in [`CYCLE-STRUCTURE.md`](../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md) (§ *Action Modifiers*). The **execution semantics this skill owns**:

- **`(background)`** — start the action asynchronously and **track its PID** so its output can be read for later asserts and the process can be cleaned up (pass or fail).
- **`(timeout Ns)`** — enforce `N` seconds as the per-action limit, overriding the 60s default; on expiry, capture whatever output exists, kill the process, and mark the result `TIMEOUT`.
- **`(in path)`** — run the action in the given directory.

Capture all console output, track background processes, and enforce timeouts. See [references/EVIDENCE-CAPTURE.md](references/EVIDENCE-CAPTURE.md) for the capture, PID-tracking, timeout, and cleanup mechanics.

**4. Evaluate Asserts**

Evaluate each assert against the captured evidence. The assert-pattern *vocabulary* — `Console contains "…"` (and its `(within Ns)` timed form), `File exists: …`, `Response status: …` — is defined in [`CYCLE-STRUCTURE.md`](../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md) (§ *Assert Patterns*). The **evaluation semantics this skill owns**:

- **`Console contains "{text}"`** — substring match against the captured stdout/stderr. The `(within Ns)` form is a timed match against streaming (background) output — poll until the text appears or the window elapses.
- **`File exists: {path}`** — a filesystem check (`test -f {path}`).
- **`Response status: {code}`** — compare the captured HTTP status against the expected code.
- Any other assert text is a **custom assertion for human evaluation** at the checkpoint.

Each assert MUST receive an explicit pass/fail evaluation. **No default to PASS** — an unevaluated assert is a failure.

**5. Generate Report**

- **All PASS** → minimal report (status + duration).
- **Any FAIL** → rich report with the evidence table.

See [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md) for the report formats and truncation rules.

**6. Present Checkpoint**

Ask the human to approve, reject, or retry. The human decision gates completion — no proceeding without explicit human approval.

### Task Classification

Before execution, classify the task from its Action and Assert content. This runtime classification is **owned by this skill** — it decides, per task, whether a human must be involved:

| Classification | Criteria | Checkpoint Behavior |
|----------------|----------|---------------------|
| **CLI** | Backtick commands + measurable asserts | May auto-approve if 100% pass |
| **GUI** | UI actions (`click`, `tap`) or screenshot capture | Always human checkpoint |
| **SUBJECTIVE** | Qualitative terms (`looks`, `feels`, `appears`) | Always human checkpoint |

**Default to SUBJECTIVE if uncertain** — the safe fallback. Ambiguity is a reason to escalate to a human, never a reason to auto-approve. Any failure, on any classification, forces a checkpoint.

### Result Classification

| Status | Meaning |
|--------|---------|
| `PASS` | All asserts passed |
| `FAIL` | One or more asserts failed |
| `PARTIAL` | Mixed results, needs judgment |
| `TIMEOUT` | Action exceeded its time limit |
| `ERROR` | Execution error (not an assertion failure) |

### Evidence Types

| Type | Capture Method |
|------|----------------|
| `console` | stdout/stderr from commands |
| `screenshot` | Platform-specific screen capture |
| `logs` | Contents of specified log files |
| `timing` | Duration of each action |

## Quality Gates

Before presenting the checkpoint, verify completion of ALL items:

- [ ] All setup commands completed
- [ ] All actions executed (or timed out)
- [ ] All asserts evaluated
- [ ] Evidence captured per the Capture field
- [ ] Report generated with the proper detail level

No presenting partial results. No skipping evidence capture. No proceeding without human approval.

## Quality Gate Execution

When a verification run includes quality gates, execute them alongside `**TEST:**` task verification. Quality gates are command-based checks that **always auto-resolve** — they are deterministic ground truth, not a matter of judgment.

### Quality Gate Sequence

1. **Identify quality gate commands** from the `## Quality Gates` section of `tasks.md` and/or the build configuration in `plan.md`.
2. **Execute each command** sequentially (lint, build, tests).
3. **Record results** with exit code, stdout, and stderr.
4. **Classify**: exit `0` = pass, non-zero = fail.
5. **Include in the verification report** under the `quality_gates` frontmatter section.

### Quality Gate Report Format

Add a `quality_gates` section to the verification-report YAML frontmatter:

```yaml
verification:
  test_tasks:
    total: 2
    passed: 2
  quality_gates:
    lint:
      status: pass
      command: "pnpm lint"
    build:
      status: pass
      command: "pnpm build"
    tests:
      status: pass
      command: "pnpm test"
      passed: 47
      failed: 0
      skipped: 2
```

Each quality gate entry records the command run and its status. For test suites, include pass/fail/skip counts when available.

### Quality Gate Auto-Resolution

Quality gates **always auto-resolve**. No human checkpoint is needed for "did lint pass?" decisions — the answer is an exit code, not a judgment:

- **Exit `0`** = pass. Record and continue.
- **Non-zero exit** = fail. Record the failure output. Include it in the verification report.
- **No human checkpoint** for quality gate results — they are deterministic.

Quality gate failures are surfaced through the verification report to the gate that consumes it, which evaluates them deterministically. (This exit-code determinism is ground truth; it MUST NOT be softened into an LLM judgment call.)

**No exceptions:**
- Not for "simple tests that obviously pass"
- Not if the user seems impatient
- Not if evidence capture is slow
- Not even if setup was identical to a previous run
- Not even if "just checking one thing"

## Red Flags - STOP and Restart Properly

If any of these thoughts arise, STOP immediately:

- "The test obviously passed, no need for full evidence capture"
- "I already know this works from previous runs"
- "Just a quick verification, minimal report is fine"
- "The user seems impatient, skip to the result"
- "This is a simple test, full process is overkill"
- "Evidence capture is taking too long"
- "I can infer the result without running the test"
- "The setup is the same as last time"

**All of these mean:** Rationalization in progress. Return to the execution sequence. Follow every step.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Test obviously passed" | Obvious passes hide subtle failures. Capture evidence anyway. |
| "Already ran this before" | Previous runs are stale. Each execution is independent. Run again. |
| "User wants quick answer" | Quick answers without evidence are unreliable. Process protects the user. |
| "Simple test case" | Simple tests catch complex bugs. Full process regardless of simplicity. |
| "Evidence capture is slow" | Slow capture beats a fast wrong answer. Time investment protects quality. |
| "Can infer the result" | Inference is not verification. Execute and observe. |
| "Same setup as before" | Environments change. Run setup fresh. Validate assumptions. |
| "Just checking one thing" | One thing has dependencies. The full sequence catches hidden failures. |

## Common Mistakes

### Mistake: Skipping Setup Validation

**What goes wrong:** An action fails mysteriously because setup was assumed complete.

**Fix:** Always run setup commands. Always capture setup output. Fail explicitly if setup fails.

### Mistake: Missing Background Process Cleanup

**What goes wrong:** Background processes from previous tests interfere with the current test.

**Fix:** Track all PIDs. Kill processes after the test (pass or fail). Verify cleanup completed.

### Mistake: Truncating Evidence Prematurely

**What goes wrong:** Critical failure information is cut off from the report.

**Fix:** Follow the truncation rules in REPORT-TEMPLATES.md. Always include log file locations. Preserve full evidence for human review.

### Mistake: Reporting PASS Without Assert Verification

**What goes wrong:** Claiming PASS when asserts were not actually evaluated.

**Fix:** Each assert MUST have an explicit pass/fail evaluation. No default to PASS. Unevaluated asserts are failures.

### Mistake: Proceeding After a Rejected Checkpoint

**What goes wrong:** Continuing execution when the human explicitly rejected.

**Fix:** Rejection gates completion. Human approval is mandatory. Retry or abort on rejection.

### Mistake: Skipping Checkpoint Presentation

**What goes wrong:** The test runs but the human never sees results. No audit trail. No approval gate.

**Fix:** Every test MUST end with a checkpoint presentation. No silent completion. Human-in-loop is the point.

## Reference Files

- [references/TASK-PARSING.md](references/TASK-PARSING.md) — detection boundaries, field-extraction algorithm, and legacy-marker normalization (parse semantics; grammar vocabulary is referenced from the owner)
- [references/EVIDENCE-CAPTURE.md](references/EVIDENCE-CAPTURE.md) — console capture, background-process PID tracking, timeout handling, cleanup
- [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md) — minimal and rich report formats, checkpoint presentation
- [references/TESTING-EVIDENCE.md](references/TESTING-EVIDENCE.md) — RED/GREEN/REFACTOR hardening record for this skill's anti-rationalization content
- **Grammar owner:** [`../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md`](../patterns-vertical-tdd/references/CYCLE-STRUCTURE.md) — the canonical `**TEST:**` marker set, field skeleton, action-modifier vocabulary, and assert-pattern vocabulary this skill consumes (§ *Unified TEST: Format*, § *Field Definitions*, § *Action Modifiers*, § *Assert Patterns*, § *Legacy Format Support*)
