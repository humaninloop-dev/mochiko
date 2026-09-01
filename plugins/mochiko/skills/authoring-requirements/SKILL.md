---
name: authoring-requirements
description: This skill MUST be invoked when authoring the functional-requirements layer of a feature specification — technology-agnostic functional requirements in FR-XXX format, edge cases, and measurable success criteria in SC-XXX format. SHOULD also invoke when the work involves 'functional requirements', 'FR-', 'success criteria', 'SC-', 'RFC 2119', 'MUST SHOULD MAY', or 'edge cases'.
---

# Authoring Requirements

Write the functional-requirements layer of a feature specification — functional
requirements, edge cases, and measurable success criteria — landing in `spec.md`.

Worked example of the constraint-vs-posture floor
(`authoring-requirements.constraint-states-capability`): "The corpus is files on disk"
is a storage posture wearing constraint clothes when the real requirement is "retrieval
is locally computable."

## Rules — load the schema first

Your first action at invoke, before any requirement is drafted: **Read `schema.yaml`
(this skill's own directory) and `../../schemas/skill-authoring-common.yaml` raw, in
full, in the same first action.** The schema is the source of truth for this skill's
binding rules; this body carries identity and teaching only. Its rules are nested in six
sections, each addressable by its section ID: `authoring-requirements.sec.independence`
(empty by design) · `authoring-requirements.sec.scope` (the PM-frame boundary and what
routes away) · `authoring-requirements.sec.inputs` (empty by design) ·
`authoring-requirements.sec.artifact` (the FR/SC/edge-case/entity grammar and its
floors) · `authoring-requirements.sec.output` (the advisory format check) ·
`authoring-requirements.sec.reserved` (decisions reserved to the user).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an
absent `kind:` reads `constraint`. Where a rule carries `extends:
authoring-common.<slug>`, the stub inherits `text` / `labels` / `pointer` only from
`skill-authoring-common.yaml` — `class` and `kind` are always this schema's own, and the
stub's `authoring-requirements.*` ID stays the citable ID. Labels come from
`../../schemas/skill-labels.yaml`. A `pointer:` rule binds you to that file's or
script's content, referenced never restated.

The schema carries **the 4 rules of `class: floor`**. State the floor count back before
the first procedural step; a skipped or partial schema read is a halt-and-surface, never
a silent continue.

## Functional Requirements Format

```markdown
## Functional Requirements

- **FR-001**: System MUST [specific capability]
- **FR-002**: Users MUST be able to [specific action]
- **FR-003**: System SHOULD [recommended behavior]
- **FR-004**: System MAY [optional capability]
```

## Success Criteria Format

```markdown
## Success Criteria

- **SC-001**: Users complete the task creation flow in under 2 minutes
- **SC-002**: 95% of users successfully create their first recurring task
- **SC-003**: Support tickets related to task scheduling decrease by 50%
```

## Key Entities (Optional)

When the feature involves data, describe entities conceptually and compactly — one
purpose line plus concept-level attribute and relationship lists.
