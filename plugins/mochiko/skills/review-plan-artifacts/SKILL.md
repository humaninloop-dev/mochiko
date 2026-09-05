---
name: review-plan-artifacts
description: This skill MUST be invoked to grade the design-phase output package against the sufficiency report's gap list — conformance (every named gap closed, nothing materially past the gap list; material divergence auto-FAILs — BLOCKING) and honesty of disclosed rung claims against `mochiko:patterns-plan-minimalism` (advisory), plus completeness (coverage, measurability, cycle-card quality, consistency) within scope. Emits a 3-state verdict (ready / needs-revision / critical-gaps). Does NOT cover feasibility (`review-feasibility`); defaults to FAIL; run by an independent validator, never the author.
allowed-tools: Bash(mochiko-cli *)
---

# Reviewing Design-Phase Artifacts

Independent completeness grader of the design-phase output package — the mirror-checklist
half of the design-phase review pair. This seat walks fixed checklists over what the caller
supplies and grades what is present, measurable, and consistent; the adversarial
contradiction hunt belongs to its sibling.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-plan-artifacts · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-plan-artifacts · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-plan-artifacts --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-plan-artifacts --section review-plan-artifacts.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Walk the four lenses in order: **conformance** against the sufficiency report's gap list ·
**adopt-first disclosure** over commodity-category decisions · **rung-claim honesty** over
each seat's disclosed ladder stops · **completeness within scope** through the mirror
checklists of [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md) (analysis · store
delta or the no-delta claim · design · cross-artifact).

Run shape: Tier-1 pre-assert → every applicable checklist over the supplied sets → classify
and shape issues per [ISSUE-TEMPLATES.md](references/ISSUE-TEMPLATES.md) → verdict → report.

In an incremental pass the work narrows rather than repeats: the {new} artifacts get the full
walk, the {prior} artifacts a consistency spot-check — entity names, requirement IDs,
decision references — escalating where the spot-check turns up trouble instead of silently
absorbing it.
