---
name: testing-end-user
description: This skill MUST be invoked when executing a `**TEST:**` verification task against real infrastructure — parsing its Setup/Action/Assert fields, running actions and capturing evidence, evaluating asserts, and classifying the task CLI/GUI/SUBJECTIVE to decide auto-approve vs human checkpoint. SHOULD also invoke when running quality gates (lint/build/test) as exit-code checks. Consumes the `**TEST:**` grammar owned by mochiko:patterns-vertical-tdd; verifies against real infrastructure, never mocks.
---

# End-User Verification Testing

## Overview

Execute verification tasks that validate real infrastructure behavior through structured Setup/Action/Assert sequences. Classify tasks at runtime (CLI/GUI/SUBJECTIVE) to determine whether to auto-approve or present a human checkpoint. This skill turns a task marked with `**TEST:**` into an executed verification sequence with captured evidence, a classified result, and a checkpoint.

**Violating the letter of the rules is violating the spirit of the rules.**

Verification testing exists to catch failures before they reach production. Every shortcut in this process is a potential production incident waiting to happen.

## Rules — load the schema first

Your first action, before any parsing or execution: **Read `schema.yaml` (this skill's own
directory) raw, in full** — the small families ship no common file and no stub binds, so
the pair's own schema is the whole first action. The schema is the source of truth for
this skill's binding rules, nested in six sections, each addressable by its section ID:
`testing-end-user.sec.independence` · `testing-end-user.sec.scope` ·
`testing-end-user.sec.inputs` · `testing-end-user.sec.verdict` ·
`testing-end-user.sec.output` · `testing-end-user.sec.reserved`. Interpret it live: a
rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule of
`class: floor` is always read and always delivered; a `pointer:` rule binds you to that
file's or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 7 rules of
`class: floor` are non-waivable. Before the first parsing step, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

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
**TEST:** {Description}   <!-- at the foot of a cycle card; legacy task-line form also parses -->
```

The full field skeleton (`**Setup**` / `**Action**` / `**Assert**` / `**Capture**`) and the legal marker set — unified `**TEST:**` plus the legacy variants (`TEST:VERIFY`, `TEST:CONTRACT`, `HUMAN VERIFICATION`) — are defined by the grammar owner in [`TEST-GRAMMAR.md`](../patterns-vertical-tdd/references/TEST-GRAMMAR.md) (§ *Unified TEST: Format*, § *Legacy Format Support*). How to find and read those tasks — the detection boundaries and field-extraction algorithm — lives in [references/TASK-PARSING.md](references/TASK-PARSING.md).

### Execution Sequence

**1. Parse Task**

Extract the structured task (ID, test type, setup, actions with modifiers, asserts, capture requirements, human-review criteria) per the parsing algorithm in [references/TASK-PARSING.md](references/TASK-PARSING.md). Legacy markers are normalized into the unified structure during parsing.

**2. Execute Setup**

Run setup commands sequentially, capturing each command's output.

**3. Execute Actions**

Run each action honoring its modifiers. The modifier *vocabulary* — `(background)`, `(timeout Ns)`, `(in path)` — is defined in [`TEST-GRAMMAR.md`](../patterns-vertical-tdd/references/TEST-GRAMMAR.md) (§ *Action Modifiers*); the execution semantics are the schema's `testing-end-user.modifier-execution-semantics`. Capture all console output, track background processes, and enforce timeouts — mechanics in [references/EVIDENCE-CAPTURE.md](references/EVIDENCE-CAPTURE.md).

**4. Evaluate Asserts**

Evaluate each assert against the captured evidence. The assert-pattern *vocabulary* — `Console contains "…"` (and its `(within Ns)` timed form), `File exists: …`, `Response status: …`, `Screen reached: …`, `Page contains "…"` — is defined in [`TEST-GRAMMAR.md`](../patterns-vertical-tdd/references/TEST-GRAMMAR.md) (§ *Assert Patterns*); the evaluation semantics are the schema's `testing-end-user.assert-evaluation-semantics`.

**5. Generate Report**

Generate the verification-report file per [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md).

**6. Present Checkpoint**

Ask the human to approve, reject, or retry, per the checkpoint presentation formats in [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md).

### Task Classification

Before execution, classify the task from its Action and Assert content — the classification criteria, the browser-flow exception, and the uncertain-default posture live in the schema's `testing-end-user.sec.verdict` section.

## Quality Gate Execution

When a verification run includes quality gates, execute them alongside `**TEST:**` task verification:

1. **Identify the quality-gate commands** (source per the schema's `testing-end-user.gate-source-binding`).
2. **Execute each command** sequentially (lint, build, tests).
3. **Record results** with exit code, stdout, and stderr.
4. **Include in the verification report** under the `quality_gates` frontmatter section, in the format defined in [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md).

Quality gate failures are surfaced through the verification report to the gate that consumes it, which evaluates them deterministically.

## Red Flags - STOP and Restart Properly

If any of these thoughts arise, STOP (`testing-end-user.rationalization-stop`):

- "The test obviously passed, no need for full evidence capture"
- "I already know this works from previous runs"
- "Just a quick verification, minimal report is fine"
- "The user seems impatient, skip to the result"
- "This is a simple test, full process is overkill"
- "Evidence capture is taking too long"
- "I can infer the result without running the test"
- "The setup is the same as last time"

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

| Mistake | What goes wrong | Fix |
|---------|-----------------|-----|
| Skipping setup validation | Actions fail mysteriously on assumed-complete setup | Run setup, capture its output, fail explicitly |
| Missing background cleanup | Stale processes interfere with the next test | Track all PIDs; kill after pass or fail; verify cleanup |
| Truncating evidence prematurely | Critical failure information cut from the report | Follow REPORT-TEMPLATES.md truncation rules; include log-file locations |
| PASS without assert verification | PASS claimed on unevaluated asserts | Every assert gets an explicit pass/fail; unevaluated = failure |
| Proceeding after rejection | Execution continues past an explicit human reject | Rejection gates completion; retry or abort |
| Skipping checkpoint presentation | Human never sees results — no audit trail, no gate | Every test ends with a checkpoint; no silent completion |

## Reference Files

- [references/TASK-PARSING.md](references/TASK-PARSING.md) — detection boundaries, field-extraction algorithm, and legacy-marker normalization (parse semantics; grammar vocabulary is referenced from the owner)
- [references/EVIDENCE-CAPTURE.md](references/EVIDENCE-CAPTURE.md) — console capture, background-process PID tracking, timeout handling, cleanup
- [references/REPORT-TEMPLATES.md](references/REPORT-TEMPLATES.md) — the verification-report file format (frontmatter + failure-only prose), checkpoint presentation, truncation
- **Grammar owner:** [`../patterns-vertical-tdd/references/TEST-GRAMMAR.md`](../patterns-vertical-tdd/references/TEST-GRAMMAR.md) — the canonical `**TEST:**` marker set, field skeleton, action-modifier vocabulary, and assert-pattern vocabulary this skill consumes (§ *Unified TEST: Format*, § *Field Definitions*, § *Action Modifiers*, § *Assert Patterns*, § *Legacy Format Support*)
