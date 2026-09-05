---
name: review-specifications
description: This skill MUST be invoked when reviewing an already-drafted specification for gaps — missing requirements, ambiguities, unstated assumptions, and missing edge cases — including its feature layer and the Screens & Flows of a UX-bearing spec. Reach for it on 'review spec', 'find gaps', 'what's missing', or 'is the spec complete'. Produces gap-finding INPUT, not a clearing PASS/FAIL verdict. For enriching a sparse feature idea before a spec exists, use mochiko:analysis-iterative instead.
allowed-tools: Bash(mochiko-cli *)
---

# Reviewing Specifications

Gap-finder over a drafted spec — severity-bucketed gaps plus clarifying questions a
stakeholder can answer. The seat hunts what is missing, ambiguous, assumed, or contradictory
in a spec that already exists; enriching a sparse idea before a spec exists is a different
skill's work.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-specifications · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-specifications · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-specifications --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-specifications --section review-specifications.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-specifications --section review-specifications.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-specifications --section review-specifications.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-specifications --section review-specifications.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-specifications --section review-specifications.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-specifications --section review-specifications.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Hunt the user-facing categories for the six requirement-defect classes: **missing
requirements** (mentioned-not-specified · implicit expectations · dependencies on undefined
behavior) · **ambiguities** (unquantified terms · open interpretation · unclear limits) ·
**edge cases** (empty states · cancelled mid-flow · missing permissions · unstated limits) ·
**assumption gaps** (assumptions that should be requirements, and the reverse · hidden
dependencies) · **contradictions** (conflicting requirements · inconsistent terminology ·
mutually exclusive acceptance criteria) · **excess / unpaid scope** (no user need or ratified
driver pays for it).

Then widen to the spec's other layers where they exist: the feature layer (the map checks,
graded from the git baseline) and the Screens & Flows of a UX-bearing spec (walk the
prototype as a skeptic, then run the check sets). Bucket findings by severity, shape the
questions as decisions, and land everything in the report shapes the schema binds.
