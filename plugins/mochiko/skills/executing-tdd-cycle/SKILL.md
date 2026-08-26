---
name: executing-tdd-cycle
description: This skill MUST be invoked when executing a cycle card at runtime — turning one card from `.mochiko/specs/<feature>/tasks.md` into working code: decomposing it into tasks, driving each through red→green→refactor, flipping the checkbox, and writing `cycle-report.md` with the decomposition disclosed. SHOULD also invoke on 'execute cycle' or when reworking failed tasks test-first. Deciding WHAT the cycles are (slicing, cards, TEST gates) is design-time work owned by mochiko:patterns-vertical-tdd.
---

# Executing TDD Cycles

**Violating the letter of the rules is violating the spirit of the rules.** TDD discipline
exists to catch failures before they compound — every shortcut in this process is a regression
waiting to happen.

## Overview

Turn a cycle card into implemented code through strict red/green/refactor discipline. Read the
current card from `tasks.md`, **decompose it into concrete tasks yourself — at build time, with
the code in view** — then write failing tests first, implement to pass, refactor, flip the
card's checkbox, and produce a structured `cycle-report.md` that disclosed the decomposition.
This skill governs the runtime *execution* of a cycle and of any targeted rework — it does not
decide what the cycles are or when they run.

## When NOT to Use

- **Deciding what the cycles are** — the slicing, the cards, their acceptance criteria and `**TEST:**` gates are design-time work owned by `mochiko:patterns-vertical-tdd` (upstream). This skill executes the card it is given; it does not add, remove, re-scope, or reorder *cycles*.
- **Running the quality gates** (lint, build, test suite) or the final real-infrastructure verification that gates a cycle — that is the verifier's work (`testing-end-user`), never this skill's. This skill executes its own tasks and runs their tests; the `**TEST:**` gate belongs to the verifier.
- **Evaluating checkpoint or validation reports, or deciding the clearing verdict** — the lead Reads the reports and owns that verdict. This skill produces its own report; it does not grade one.
- **Deciding which cycle runs next, whether to retry, or when to run a fix pass** — that routing is the lead's. This skill executes the cycle (or the rework) it is given.
- **Managing loop or orchestration state** — this skill executes one cycle or one rework and produces one report; it neither drives the loop nor tracks cross-cycle state.

## Core Process

### Cycle Execution Sequence

Execute in strict order. No skipping steps. No reordering.

**1. Read the Cycle Card**

Extract the current card from the feature's `tasks.md` (under `.mochiko/specs/<feature>/`):
its stories, acceptance criteria (resolve the cited IDs against the spec/design artifacts),
dependencies, brownfield exposure, and `**TEST:**` gate. See
[references/TASK-PARSING.md](references/TASK-PARSING.md) for the card fields. The current
cycle is the first card in order whose checkbox is unchecked.

**2. Decompose the Card**

Break the card into concrete implementation tasks — **yours to decide, here, with the code in
view**: read the relevant existing code first, then cut tasks sized to a single reviewable
change, each with a specific file path, ordered so tests precede the implementation they pin.
The card's brownfield exposure tells you which surfaces are extend/modify — classify each task
accordingly. Scope discipline: decompose exactly what the card's acceptance criteria require —
nothing the card didn't ask for. Before any red-phase test is written, run the pre-code ladder
over each prospective task per `mochiko:patterns-code-minimalism` — stop at the first rung
that applies, and disclose each task's rung in the cycle report's decomposition.
The decomposition is **disclosed in the cycle report**
(task list, paths, ordering), not written back into `tasks.md` — the card stays the artifact,
your decomposition is execution detail.

**3. Red Phase — Write Failing Tests**

For each behavior in your decomposition:
1. Write the test file at your chosen path
2. Run the test to verify it **fails**
3. Verify the failure reason matches expectations (not a syntax error, import error, or wrong assertion)
4. If the test passes without implementation, the test is not testing what you think — rewrite it

**4. Green Phase — Implement Code**

For each implementation task:
1. Write the minimum code to make the failing test pass
2. Run the test to verify it **passes**
3. Do not add features, abstractions, or optimizations the card did not require
4. For extend-classified tasks: read the existing file first, follow existing patterns (invoke `brownfield-integration` skill)
5. For modify-classified tasks: read the existing file first, change only what the task specifies (invoke `brownfield-integration` skill)

**5. Refactor Phase**

After tests pass:
1. Remove duplication introduced in this cycle only
2. Improve names if unclear
3. Do NOT refactor code from previous cycles
4. Do NOT add abstractions "for the future"
5. Re-run tests after refactoring to confirm they still pass

**6. Flip the Card**

Update `tasks.md`: change the card's `- [ ]` to `- [x]` — after your tasks are complete and
their tests pass. (The `**TEST:**` gate is the verifier's; the lead treats the flip as your
self-report, verified independently.)

**7. Write Cycle Report**

Produce `cycle-report.md` following the format in [references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md) — the decomposition (task list with file paths and ordering) is part of the report's structured fields.

### Progress Tracking

- Write `cycle-report.md` machine-first: the YAML frontmatter is the report — a clean passing cycle needs no prose
- The frontmatter's structured fields are your self-report — the lead reads them when deciding the cycle checkpoint, and verifies independently rather than trusting them
- Prose is conditional (per the format): `Notes of note` only when there are non-obvious decisions, difficulties, or blockers to flag; a `Failure narrative` (full detail) whenever the cycle failed or was blocked

### Reworking Specific Failed Tasks

When particular tasks from your decomposition come back as failing, rework only those — never re-run the whole cycle:

1. Read the reported failures (from the checkpoint or verification report you were given)
2. Trace each failure to the responsible task(s) in your reported decomposition
3. Execute only those tasks through red/green/refactor — write the failing test that pins the failure, then implement the minimum to make it pass
4. Leave passing code untouched — tasks that passed are done
5. Write a new `cycle-report.md` with the `attempt` number incremented and the reworked tasks marked

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
- "The exposure says extends but I need to rewrite this"
- "I'll add this helper/utility that will be useful later"
- "The checkpoint will catch it if something's wrong"
- "I know this works from the previous cycle"
- "The card is clear enough, I'll decompose as I go" — the decomposition is a deliberate step, recorded before the first test is written

**All of these mean:** Rationalization in progress. Return to the execution sequence. Follow every step.

**No exceptions:**
- Not for "trivial" cards
- Not for "obvious" implementations
- Not for "tight deadlines"
- Not even if the user says "just write the code"

See [references/TDD-ANTI-RATIONALIZATION.md](references/TDD-ANTI-RATIONALIZATION.md) for the full rationalization table.

## Common Mistakes

| Mistake | What goes wrong | Fix |
|---------|-----------------|-----|
| Tests after implementation | Retroactive justification — tests pass because they were written to match the code | Test first, run it, verify it fails for the right reason, then implement |
| Decomposing beyond the card | "While I'm decomposing" scope creep — tasks the acceptance criteria never asked for | Decompose exactly the card's criteria; note opportunities in `Notes of note` |
| Full cycle re-implementation on retry | Working code rewritten, new bugs, budget wasted on complete tasks | Trace failures to specific tasks; rework only those; leave passing code untouched |
| Scope creep during refactor | "While I'm here" changes break code that was working; the report drifts from reality | Refactor only this cycle's code; note opportunities in `Notes of note` instead of acting |
| Skipping failure-reason verification | Syntax/import errors mistaken for meaningful failures — green "passes" by fixing syntax, not implementing | Verify the failure message matches expectation; a `ModuleNotFoundError` is not a test failure |

## Reference Files

- [references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md) — Structured YAML frontmatter schema (incl. the decomposition fields) and the conditional prose rules
- [references/TASK-PARSING.md](references/TASK-PARSING.md) — Cycle-card fields and how to read them
- [references/TDD-ANTI-RATIONALIZATION.md](references/TDD-ANTI-RATIONALIZATION.md) — Common shortcuts and why they fail
