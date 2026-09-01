---
name: authoring-user-stories
description: This skill MUST be invoked when transforming a feature description into prioritized user stories — assigning P1/P2/P3 priority levels, authoring Given/When/Then acceptance scenarios, and specifying an independent test for each story. SHOULD also invoke when the work involves a user story, story prioritization (P1/P2/P3), acceptance scenarios, or breaking a large feature into separate, independently testable user journeys.
---

# Authoring User Stories

Transform feature descriptions into testable user stories with clear business value,
prioritized by impact. This is a discipline-enforcing skill: the structured format exists
to keep stories unambiguous, testable, and properly prioritized — shortcuts create
ambiguous requirements that cause implementation failures.

## Rules — load the schema first

Your first action, before any authoring: **Read `schema.yaml` (this skill's own directory)
and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same declared first
action** — schema, then common. The schema is the source of truth for this skill's binding
rules, nested in six sections, each addressable by its section ID:
`authoring-user-stories.sec.independence` · `authoring-user-stories.sec.scope` ·
`authoring-user-stories.sec.inputs` · `authoring-user-stories.sec.artifact` ·
`authoring-user-stories.sec.output` · `authoring-user-stories.sec.reserved`. Interpret it
live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a
`pointer:` rule binds you to that file's or skill's procedure, referenced never restated;
labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: authoring-common.<slug>` inherits text/labels/pointer from
`skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `authoring-user-stories.*` ID stays the citable ID. The floor pin: the 4
rules of `class: floor` are non-waivable. Before the first authoring step, state the floor
count back — a skipped or partial read leaves that count blank: halt and surface it, and
halt likewise if the schema's `class: floor` count disagrees with the pin.

## User Story Format

The story lands in `spec.md`, every field to the schema's density rules. The exact
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
