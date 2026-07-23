---
name: executing-tdd-cycle
description: This skill MUST be invoked when executing an already-structured cycle at runtime — turning one cycle's task list from `.mochiko/specs/<feature>/tasks.md` into working code by driving each task through the red→green→refactor execution sequence (write the failing test, run it, confirm it fails for the right reason, implement the minimum to pass, refactor only this cycle's code, mark the task `[x]`) and writing the `cycle-report.md`. SHOULD also invoke when "execute cycle", "execute the cycle tasks", "implement the cycle task list", or "write the cycle report" is the work at hand; when reworking the specific tasks reported as failing (targeted, test-first rework); when reproducing a reported failure with a failing test before fixing it; or when a task carries an `[EXTEND]`/`[MODIFY]` brownfield marker during implementation. This is the runtime EXECUTION of cycles — writing the code for a cycle that is already structured. STRUCTURING the cycles (identifying vertical slices, ordering tasks test-first, authoring the `tasks.md` skeleton) is design-time work owned by `patterns-vertical-tdd`, upstream and not this skill.
---

# Executing TDD Cycles

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Turn a cycle's task list into implemented code through strict red/green/refactor discipline. Parse the current cycle's tasks from `tasks.md`, write failing tests first, implement code to pass them, refactor, mark tasks complete, and produce a structured `cycle-report.md`. This skill governs the runtime *execution* of a cycle and of any targeted rework — it does not structure the cycles or decide when they run.

TDD discipline exists to catch failures before they compound. Every shortcut in this process is a regression waiting to happen.

## When to Use

- Executing a cycle's task list from `.mochiko/specs/<feature>/tasks.md` through red/green/refactor
- Reworking the specific tasks reported as failing (targeted, test-first rework)
- Fixing a reported failure against working code — reproduce it with a failing test, then green it
- Any task carrying an `[EXTEND]` or `[MODIFY]` brownfield marker (invoke `brownfield-integration` alongside)
- Writing the `cycle-report.md` after a cycle — or a rework — completes

## When NOT to Use

- **Structuring the cycles** — identifying the vertical slices, ordering a cycle's tasks test-first, or authoring the `tasks.md` skeleton — is design-time work owned by `patterns-vertical-tdd` (upstream). This skill executes an already-structured cycle; it does not create, split, or reorder tasks.
- **Running the quality gates** (lint, build, test suite) or the final real-infrastructure verification that gates a cycle — that is the verifier's work (`testing-end-user`), never this skill's. This skill executes the failing-test / implementation / refactor tasks and runs their tests; the final verification gate belongs to the verifier.
- **Evaluating checkpoint or validation reports, or deciding the clearing verdict** — the lead Reads the reports and owns that verdict. This skill produces its own report; it does not grade one.
- **Deciding which cycle runs next, whether to retry, or when to run a fix pass** — that routing is the lead's. This skill executes the cycle (or the rework) it is given.
- **Managing loop or orchestration state** — this skill executes one cycle or one rework and produces one report; it neither drives the loop nor tracks cross-cycle state.

## Core Process

### Cycle Execution Sequence

Execute in strict order. No skipping steps. No reordering.

**1. Parse Cycle Tasks**

Extract the task list for the current cycle from the feature's `tasks.md` (under `.mochiko/specs/<feature>/`). See [references/TASK-PARSING.md](references/TASK-PARSING.md) for parsing rules.

For each task, extract:
- Task ID (`T{N}.{X}`)
- Description
- File path(s) in backticks
- `[EXTEND]` or `[MODIFY]` markers
- Sub-bullet details

**2. Red Phase — Write Failing Tests**

For each task that specifies a test:
1. Write the test file at the specified path
2. Run the test to verify it **fails**
3. Verify the failure reason matches expectations (not a syntax error, import error, or wrong assertion)
4. If the test passes without implementation, the test is not testing what you think — rewrite it

**3. Green Phase — Implement Code**

For each implementation task:
1. Write the minimum code to make the failing test pass
2. Run the test to verify it **passes**
3. Do not add features, abstractions, or optimizations the task did not request
4. For `[EXTEND]` tasks: read the existing file first, follow existing patterns (invoke `brownfield-integration` skill)
5. For `[MODIFY]` tasks: read the existing file first, change only what the task specifies (invoke `brownfield-integration` skill)

**4. Refactor Phase**

After tests pass:
1. Remove duplication introduced in this cycle only
2. Improve names if unclear
3. Do NOT refactor code from previous cycles
4. Do NOT add abstractions "for the future"
5. Re-run tests after refactoring to confirm they still pass

**5. Mark Tasks Complete**

Update `tasks.md`: change `- [ ]` to `- [x]` for each completed task in this cycle.

**6. Write Cycle Report**

Produce `cycle-report.md` following the format in [references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md).

### Progress Tracking

- Mark each task `[x]` in `tasks.md` immediately after completing it
- Write `cycle-report.md` machine-first: the YAML frontmatter is the report — a clean passing cycle needs no prose
- The frontmatter's structured fields are your self-report — the lead reads them when deciding the cycle checkpoint, and verifies independently rather than trusting them
- Prose is conditional (per the format): `Notes of note` only when there are non-obvious decisions, difficulties, or blockers to flag; a `Failure narrative` (full detail) whenever the cycle failed or was blocked

### Reworking Specific Failed Tasks

When particular tasks in a cycle come back as failing, rework only those — never re-run the whole cycle:

1. Read the reported failures (from the checkpoint or verification report you were given)
2. Trace each failure to the responsible task(s)
3. Re-open only those tasks: change `- [x]` back to `- [ ]` in `tasks.md`
4. Execute only the re-opened tasks through red/green/refactor — write the failing test that pins the failure, then implement the minimum to make it pass
5. Leave passing code untouched — tasks that passed are done
6. Write a new `cycle-report.md` with the `attempt` number incremented

Whether to rework, how many attempts are permitted, and when to stop are the lead's routing decisions — not this skill's. You execute the rework you are given.

### Fixing a Reported Failure

When a failure is reported against already-working code, fix it test-first and keep the change scoped to that failure:

1. Read the reported failures
2. **Reproduce each one with a failing test before changing any code** — a fix that was never pinned by a failing test tends to come back
3. Make the narrowest change that turns the test green. You are not constrained by cycle boundaries here — you may touch files from any cycle as the failure requires — but scope strictly to the reported failure; this is not a refactoring opportunity
4. Write a `cycle-report.md` with `cycle: fix` in the frontmatter

When a fix like this is warranted, and how many are permitted, is the lead's routing decision — not this skill's.

## Red Flags — STOP and Restart Properly

If any of these thoughts arise, STOP immediately:

- "The test already passes so I'll skip writing it first"
- "I'll write all the code first and tests after"
- "This task is trivial, no test needed"
- "I'll refactor this existing code while I'm here"
- "The task says EXTEND but I need to rewrite this"
- "I'll add this helper/utility that will be useful later"
- "The checkpoint will catch it if something's wrong"
- "I know this works from the previous cycle"

**All of these mean:** Rationalization in progress. Return to the execution sequence. Follow every step.

**No exceptions:**
- Not for "trivial" tasks
- Not for "obvious" implementations
- Not for "tight deadlines"
- Not for "I already know this works"
- Not even if the user says "just write the code"

See [references/TDD-ANTI-RATIONALIZATION.md](references/TDD-ANTI-RATIONALIZATION.md) for the full rationalization table.

## Common Mistakes

### Mistake: Writing Tests After Implementation

**What goes wrong:** Tests become retroactive justification. They pass because they were written to match the code, not because the code satisfies the requirement.

**Fix:** Always write the test first. Run it. Verify it fails for the right reason. Then implement.

### Mistake: Full Cycle Re-Implementation on Retry

**What goes wrong:** Working code gets rewritten, introducing new bugs. Token budget wasted on already-complete tasks.

**Fix:** Trace failures to specific tasks. Re-open only those tasks. Leave passing code untouched.

### Mistake: Scope Creep During Refactor Phase

**What goes wrong:** "While I'm here" changes accumulate. New bugs appear in code that was working. Cycle report doesn't reflect actual changes.

**Fix:** Refactor phase is limited to code introduced in this cycle. Note improvement opportunities as a one-line entry in the cycle report's `Notes of note` block instead of acting on them.

### Mistake: Skipping Failure Reason Verification

**What goes wrong:** Test fails due to syntax error or wrong import, not because the assertion caught a missing implementation. Green phase "passes" the test by fixing the syntax, not by implementing the feature.

**Fix:** After writing a failing test, verify the failure message matches your expectation. A `ModuleNotFoundError` is not a meaningful test failure.

## Reference Files

- [references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md) — Structured YAML frontmatter schema and the conditional prose rules (notes of note, failure narrative)
- [references/TASK-PARSING.md](references/TASK-PARSING.md) — Task pattern extraction, file paths, markers
- [references/TDD-ANTI-RATIONALIZATION.md](references/TDD-ANTI-RATIONALIZATION.md) — Common shortcuts and why they fail
