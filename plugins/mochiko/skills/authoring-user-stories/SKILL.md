---
name: authoring-user-stories
description: This skill MUST be invoked when transforming a feature description into prioritized user stories — assigning P1/P2/P3 priority levels, authoring Given/When/Then acceptance scenarios, and specifying an independent test for each story. SHOULD also invoke when the work involves a user story, story prioritization (P1/P2/P3), acceptance scenarios, or breaking a large feature into separate, independently testable user journeys.
---

# Authoring User Stories

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

Transform feature descriptions into testable user stories with clear business value, prioritized by impact. Each story MUST be independently testable with measurable acceptance criteria.

This is a discipline-enforcing skill. The structured format exists to ensure stories are unambiguous, testable, and properly prioritized. Shortcuts create ambiguous requirements that cause implementation failures.

**Boundary — authored inside the product-manager's frame.** The PM owns *which* capabilities (features, the story filter, selection advice — map machinery: `mochiko:authoring-feature-map`); this craft owns *how well* the stories are written. Neither edits the other's verdicts; a disagreement escalates to the user.

## When NOT to Use

- **Technical implementation tasks** - Use task decomposition instead
- **Bug reports** - Use issue templates with reproduction steps
- **When requirements are already in user story format** - Don't duplicate work
- **Architecture decisions** - Capture technical-decision rationale in the design/plan track instead; this skill authors user stories, not technical choices
- **API contract design** - Define endpoints and schemas in the design/plan track instead; this skill authors user stories, not interface contracts

## User Story Format

Generate 2-5 user stories per feature using this exact structure. The story lands in
`spec.md`, which follows the deliverable envelope in
[`artifact-format.md`](../../templates/artifact-format.md) — every field below is dense:
the journey ≤ 2 lines, the priority justification and independent test one line each,
and **each acceptance scenario a single line**:

```markdown
### User Story N - [Brief Title] (Priority: P#)

[The user journey in plain language — ≤ 2 lines]

**Why this priority**: [value and priority level — one line]

**Independent Test**: [how this is tested standalone — one line]

**Acceptance Scenarios**:
1. **Given** [state], **When** [action], **Then** [outcome] — one line
2. **Given** [state], **When** [action], **Then** [outcome] — one line
```

## Priority Definitions

P1 (core functionality — MVP requirement, blocks other features, must ship) · P2 (important —
complete experience, but can ship without initially) · P3 (nice to have — future consideration).
Detailed assignment guidance: [PRIORITY-DEFINITIONS.md](references/PRIORITY-DEFINITIONS.md).

## Acceptance Scenario Guidelines

Each scenario follows the Given/When/Then pattern — Given (the initial state or precondition),
When (the user's action), Then (the expected, observable outcome).

**Rules:**
1. Each story needs 2-3 acceptance scenarios — the happy path plus the key edge case(s); more than 3 means the story is compound or the scenarios overlap
2. Each scenario is **one line** — a scenario that needs a paragraph is hiding several scenarios or restating context the Given already carries
3. Scenarios must be independently verifiable
4. Use concrete, observable outcomes (not implementation details)

Worked good/bad pairs (scenarios, journeys, justifications, independent tests):
[EXAMPLES.md](references/EXAMPLES.md).

## Independent Test Requirement

Each user story must include an **Independent Test** description that explains:
- How QA can verify this story in isolation
- What data or setup is required
- What constitutes passing/failing

This enables parallel testing and clear verification.

## Quality Checklist

Before finalizing, verify each user story:

- [ ] Has a clear, descriptive title
- [ ] Priority is assigned with justification (one line)
- [ ] User journey is described in plain language (≤ 2 lines)
- [ ] Independent test is specified (one line)
- [ ] 2-3 acceptance scenarios using Given/When/Then, one line each
- [ ] No implementation details or technology references
- [ ] Outcomes are observable and measurable

## Validation Script

Validate format (priority markers + justification, Given/When/Then completeness, independent
test presence, header format):

```bash
python scripts/validate-user-stories.py path/to/spec.md
```

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Priority is obvious, don't need justification" | Obvious to you ≠ obvious to others. Stakeholders disagree on "obvious." Justify anyway. |
| "This story is too simple for Given/When/Then" | Simple stories still need testable criteria. Ambiguity causes implementation bugs. Format anyway. |
| "We can add acceptance scenarios later" | "Later" means never. Incomplete stories get misimplemented. Write them now. |
| "The user just wants quick stories" | Quick incomplete stories waste more time than complete ones. Do it right. |
| "Independent test is overkill for small features" | Small features still need verification. QA can't test what isn't specified. Include it. |
| "Everyone knows what P1 means" | P1 without justification is opinion, not prioritization. Explain the value. |

## Red Flags - STOP and Restart Properly

If any excuse in the table above arises as a thought mid-authoring, STOP immediately — you are
rationalizing; write complete stories with all required sections. No exceptions: not for
"simple" features, "we'll refine later", "tight deadlines", nor even if the user says "just
give me quick stories".
