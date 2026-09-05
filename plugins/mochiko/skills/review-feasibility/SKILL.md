---
name: review-feasibility
description: This skill MUST be invoked to grade design-phase analysis/design artifacts for cross-artifact FEASIBILITY — hunting contradictions, impossibilities, buildability conflicts, plus unjustified structure / wrong altitude; plus the architecture pass when the design-phase package carries an architecture-store delta. Emits a 3-state `feasible / needs-revision / infeasible` verdict. The adversarial half of the design-phase review pair; its sibling `review-plan-artifacts` grades coverage/measurability/presence, this grades contradiction/buildability. Never defaults to `feasible`; not the constitution.
allowed-tools: Bash(mochiko-cli *)
---

# Reviewing Feasibility

Adversarial cross-artifact review: **can these artifacts be built together?** Hunt the
impossible combination no single artifact reveals — judgment, never a checklist; looking
buildable is not being buildable.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-feasibility · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-feasibility · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-feasibility --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-feasibility --section review-feasibility.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-feasibility --section review-feasibility.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-feasibility --section review-feasibility.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-feasibility --section review-feasibility.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-feasibility --section review-feasibility.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-feasibility --section review-feasibility.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

Your first action after the read-back, before any hunting: **Read
`references/FEASIBILITY-LENS.md` (this skill's own directory) raw, in full.**

## The hunt

Classes 1–6, class 7 (excess / wrong altitude, remove-shaped), and the architecture pass
A1–A3 all live in the lens file, worked examples and reviewer guardrails included. Hunt
each class across the package, then fill the report and hand the verdict up.
