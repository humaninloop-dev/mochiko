---
name: validation-constitution
description: This skill MUST be invoked to grade a DRAFTED governance surface set against the quality checklist — there is NO constitution.md; the graded set is the CLAUDE.md governance region, the `.claude/rules/mochiko/` files, and the governance ledger. SHOULD also invoke for the setup loop's validate step, or when re-validating after a FAIL-loop revision. Validator-side skill of the governance producer↔validator pair; defaults to FAIL; run by an independent validator, never the author.
allowed-tools: Bash(mochiko-cli *)
---

# Validating Constitution

Independent binary grade of a drafted governance surface set — enforceable, testable,
trace-closed, anti-pattern-free before finalization. There is no constitution.md: the
deliverable under grade is the surface set itself, and the grade is earned by walking the
assembled checklist against the files, never by trusting the author's account of them.
Producer side: `mochiko:authoring-constitution` (never co-mounted; the validator is a
different agent).

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules validation-constitution · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · validation-constitution · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules validation-constitution --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-constitution --section validation-constitution.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-constitution --section validation-constitution.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-constitution --section validation-constitution.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-constitution --section validation-constitution.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-constitution --section validation-constitution.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-constitution --section validation-constitution.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Read [references/QUALITY-CHECKLIST.md](references/QUALITY-CHECKLIST.md) and assemble the
working checklist from the universal core plus each selected module's embedded fragment,
routed to where that module's content actually lives. Then walk it: every item against the
files, vague language against the patterns of
[references/ANTI-PATTERNS.md](references/ANTI-PATTERNS.md), excess governance hunted with the
same seriousness as missing governance, and a version-bump determination for every change.
Close by emitting the VALIDATION RESULT block in full — the verdict, the accounting, the
issues, and the advisory line.
