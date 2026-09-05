---
name: authoring-requirements
description: This skill MUST be invoked when authoring the functional-requirements layer of a feature specification — technology-agnostic functional requirements in FR-XXX format, edge cases, and measurable success criteria in SC-XXX format. SHOULD also invoke when the work involves 'functional requirements', 'FR-', 'success criteria', 'SC-', 'RFC 2119', 'MUST SHOULD MAY', or 'edge cases'.
allowed-tools: Bash(mochiko-cli *)
---

# Authoring Requirements

Write the functional-requirements layer of a feature specification — functional
requirements, edge cases, and measurable success criteria — landing in `spec.md`.

Worked example of the constraint-vs-posture floor
(`authoring-requirements.constraint-states-capability`): "The corpus is files on disk"
is a storage posture wearing constraint clothes when the real requirement is "retrieval
is locally computable."

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-requirements · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-requirements · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-requirements --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-requirements --section authoring-requirements.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-requirements --section authoring-requirements.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-requirements --section authoring-requirements.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-requirements --section authoring-requirements.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-requirements --section authoring-requirements.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-requirements --section authoring-requirements.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

A `pointer:` here may bind you to a script's content as well as a file's or skill's
procedure — referenced, never restated.

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
