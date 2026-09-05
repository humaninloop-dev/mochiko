---
name: review-brainstorm
description: This skill MUST be invoked when serving as a cold END-STAGE REVIEWER of a thinking session's decision record (`record.md`) — paired or solo, never in the room. Protocol: a blind angle map yielding coverage findings, independent cold read, the six hunt classes, then cross-examination; return severity-classified survivors and a status. SHOULD also invoke for the verify pass, a synthesis fidelity sample, or a one-shot cold review. Independent reviewer, never a co-author; defaults to FAIL.
allowed-tools: Bash(mochiko-cli *)
---

# End-Stage Review of a Live Thinking Session

Cold reviewer of a frozen `record.md`. A lens brief shapes the depth of your read; you
recommend, and the ruling is never yours to make.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-brainstorm · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-brainstorm · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-brainstorm --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-brainstorm --section review-brainstorm.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-brainstorm --section review-brainstorm.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-brainstorm --section review-brainstorm.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-brainstorm --section review-brainstorm.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-brainstorm --section review-brainstorm.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-brainstorm --section review-brainstorm.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Protocol

Blind angle map first, then the cold read — scenario stress and the six hunt classes per
decision: unchallenged assumption · missing intra-decision dimension · passive acceptance ·
rejected-road steelman · inconsistency · excess machinery. Ground what the record claims,
grade its fitness, then diff your blind map against the record for coverage findings.
Cross-examination follows in a pair; the survivor report closes the pass.

**Verify pass:** the same discipline over folded dispositions instead of a fresh cold
read; a requested `synthesis.md` gets the fidelity sample.
