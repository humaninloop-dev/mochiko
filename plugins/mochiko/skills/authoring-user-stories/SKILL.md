---
name: authoring-user-stories
description: This skill MUST be invoked when transforming a feature description into prioritized user stories — assigning P1/P2/P3 priority levels, authoring Given/When/Then acceptance scenarios, and specifying an independent test for each story. SHOULD also invoke when the work involves a user story, story prioritization (P1/P2/P3), acceptance scenarios, or breaking a large feature into separate, independently testable user journeys.
allowed-tools: Bash(mochiko-cli *)
---

# Authoring User Stories

Transform feature descriptions into testable user stories with clear business value,
prioritized by impact. This is a discipline-enforcing skill: the structured format exists
to keep stories unambiguous, testable, and properly prioritized — shortcuts create
ambiguous requirements that cause implementation failures.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-user-stories · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-user-stories · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-user-stories --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-user-stories --section authoring-user-stories.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-user-stories --section authoring-user-stories.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-user-stories --section authoring-user-stories.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-user-stories --section authoring-user-stories.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-user-stories --section authoring-user-stories.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-user-stories --section authoring-user-stories.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## User Story Format

The story lands in `spec.md`, every field to the delivered density rules. The exact
structure:

```markdown
### User Story N - [Brief Title] (Priority: P#)

[The user journey in plain language — ≤ 2 lines]

**Why this priority**: [value and priority level — one line]

**Independent Test**: [how this is tested standalone — one line]

**Acceptance Scenarios**:
1. **Given** [state], **When** [action], **Then** [outcome] — one line
2. **Given** [state], **When** [action], **Then** [outcome] — one line
```

## Priorities and Scenarios

P1 (core functionality — MVP requirement, blocks other features, must ship) · P2
(important — complete experience, but can ship without initially) · P3 (nice to have —
future consideration). Detailed assignment guidance:
[PRIORITY-DEFINITIONS.md](references/PRIORITY-DEFINITIONS.md).

Each scenario follows the Given/When/Then pattern — Given (the initial state or
precondition), When (the user's action), Then (the expected, observable outcome). Worked
good/bad pairs (scenarios, journeys, justifications, independent tests):
[EXAMPLES.md](references/EXAMPLES.md).

## Common Rationalizations

The Reality column answers each excuse as it arises mid-authoring.

| Excuse | Reality |
|--------|---------|
| "Priority is obvious, don't need justification" | Obvious to you ≠ obvious to others. Stakeholders disagree on "obvious." Justify anyway. |
| "This story is too simple for Given/When/Then" | Simple stories still need testable criteria. Ambiguity causes implementation bugs. Format anyway. |
| "We can add acceptance scenarios later" | "Later" means never. Incomplete stories get misimplemented. Write them now. |
| "The user just wants quick stories" | Quick incomplete stories waste more time than complete ones. Do it right. |
| "Independent test is overkill for small features" | Small features still need verification. QA can't test what isn't specified. Include it. |
| "Everyone knows what P1 means" | P1 without justification is opinion, not prioritization. Explain the value. |
