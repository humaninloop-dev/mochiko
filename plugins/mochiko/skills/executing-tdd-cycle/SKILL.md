---
name: executing-tdd-cycle
description: This skill MUST be invoked when executing a cycle card at runtime — turning one card from `.mochiko/specs/<feature>/tasks.md` into working code: decomposing it into tasks, driving each through red→green→refactor, flipping the checkbox, and writing `cycle-report.md` with the decomposition disclosed. SHOULD also invoke on 'execute cycle' or when reworking failed tasks test-first. Deciding WHAT the cycles are (slicing, cards, TEST gates) is design-time work owned by mochiko:patterns-vertical-tdd.
allowed-tools: Bash(mochiko-cli *)
---

# Executing TDD Cycles

**Violating the letter of the rules is violating the spirit of the rules.** TDD discipline
exists to catch failures before they compound — every shortcut in this process is a regression
waiting to happen.

## Overview

Turn a cycle card into implemented code through strict red/green/refactor discipline. Read the
current card from `tasks.md`, decompose it into concrete tasks, then write failing tests
first, implement to pass, refactor, flip the card's checkbox, and produce a structured
`cycle-report.md` that disclosed the decomposition. This skill governs the runtime
*execution* of a cycle and of any targeted rework.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules executing-tdd-cycle · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · executing-tdd-cycle · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules executing-tdd-cycle --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules executing-tdd-cycle --section executing-tdd-cycle.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules executing-tdd-cycle --section executing-tdd-cycle.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules executing-tdd-cycle --section executing-tdd-cycle.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules executing-tdd-cycle --section executing-tdd-cycle.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules executing-tdd-cycle --section executing-tdd-cycle.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules executing-tdd-cycle --section executing-tdd-cycle.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Core Process

### Cycle Execution Sequence

**1. Read the Cycle Card**

Extract the current card from the feature's `tasks.md` per
[references/TASK-PARSING.md](references/TASK-PARSING.md) — the card fields, and how far
card reading legally goes, are delivered by `mochiko-cli` as the `executing-tdd-cycle.sec.inputs` rules.

**2. Decompose the Card**

Break the card into concrete implementation tasks — yours to decide, here, with the code in
view. The decomposition discipline (task sizing, ordering, extend/modify classification,
scope, the pre-code ladder walk per `mochiko:patterns-code-minimalism`) is delivered by
`mochiko-cli` as the `executing-tdd-cycle.sec.scope` rules; the decomposition itself lands in the
cycle report, per `executing-tdd-cycle.sec.output`.

**3. Red Phase — Write Failing Tests**

For each behavior in your decomposition, write the test file at your chosen path and run
it — the failure-verification discipline is `executing-tdd-cycle.red-phase-failure-verified`.

**4. Green Phase — Implement Code**

For each implementation task, implement and run its test — minimum-code and brownfield
co-fire rules: `executing-tdd-cycle.green-minimum` · `executing-tdd-cycle.brownfield-co-fire`.

**5. Refactor Phase**

Remove duplication and improve names where unclear, inside the bounds of
`executing-tdd-cycle.refactor-scope`.

**6. Flip the Card**

Update `tasks.md`: change the card's `- [ ]` to `- [x]`, under
`executing-tdd-cycle.flip-is-self-report`.

**7. Write Cycle Report**

Produce `cycle-report.md` following the format in
[references/CYCLE-REPORT-FORMAT.md](references/CYCLE-REPORT-FORMAT.md) — the frontmatter's
structured fields are your self-report; prose is conditional per the format.

### Reworking Specific Failed Tasks

When particular tasks from your decomposition come back as failing: read the reported
failures (from the checkpoint or verification report you were given), trace each to the
responsible task(s) in your reported decomposition, and execute the rework under
`executing-tdd-cycle.rework-only-failed`.

### Fixing a Reported Failure

When a failure is reported against already-working code: read the reported failures and
fix each under `executing-tdd-cycle.fix-pass-test-first` — reproduce, pin with a failing
test, narrowest change.

## Red Flags — STOP and Restart Properly

If any of these thoughts arise, STOP (`executing-tdd-cycle.rationalization-stop`):

- "The test already passes so I'll skip writing it first"
- "I'll write all the code first and tests after"
- "This task is trivial, no test needed"
- "I'll refactor this existing code while I'm here"
- "The exposure says extends but I need to rewrite this"
- "I'll add this helper/utility that will be useful later"
- "The checkpoint will catch it if something's wrong"
- "I know this works from the previous cycle"
- "The card is clear enough, I'll decompose as I go" — the decomposition is a deliberate step, recorded before the first test is written

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
