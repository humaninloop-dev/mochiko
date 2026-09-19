---
name: validation-primitive-edit
description: This skill MUST be invoked when running the primitive-edit gate audit — the binary PASS/FAIL grade a shipped `plugins/mochiko/` primitive takes before the `plugin.json` bump that ships it (GI-004). SHOULD also invoke on 'gate audit', 'primitive-edit audit', 'grade the pair', or 'author≠grader audit'. The graded unit is keyed by kind — command pair · skill pair · prose primitive · schema content — and the criteria follow the unit; the deterministic pre-pass is run first-hand by the grader and quoted, never from the brief. Defaults to FAIL; run by a plain fresh seat that authored nothing in the unit, never the editor. Not the input-job `review-*` families, and not setup's governance surface set (`validation-constitution`).
allowed-tools: Bash(mochiko-cli *)
---

# Validating a Primitive Edit

The primitive-edit gate: one binary grade over one edited unit, standing between a
`plugins/mochiko/` edit and the `plugin.json` bump that ships it. Nobody rules after this grade,
which is what makes it a gate rather than input — the `review-*` family carries the input job.
You are a plain fresh seat that authored nothing in the unit, and the rules below, exactly as
`mochiko-cli` renders them, are the whole contract: your dispatch brief carries the unit, its
file paths, and the pre-pass command, and nothing else of the bar you hold. Maintainer-side
ceremony: `.claude/rules/mochiko/primitive-edits.md`.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules validation-primitive-edit · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · validation-primitive-edit · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules validation-primitive-edit --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Run the pre-pass yourself before anything else — the commands your brief names — and keep the
output to quote. Then read the unit's own files: the edited primitive, its rules as
`mochiko-cli` renders them, the migration and its regenerated view diff, the strip entry where
one is owed. Walk the judgment items for this unit's kind, confirming each once against what
you read. Close by emitting the verdict block in full, and write the outcome line into the
record of the landing this edit belongs to. On a FAIL you hand back the fix list and stop:
you are resumed for the one bounded re-audit rather than respawned.
