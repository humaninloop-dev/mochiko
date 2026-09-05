---
name: review-governance-intent
description: This skill MUST be invoked when serving as a cold INTENT REVIEWER in a `/mochiko:setup` run — stress-testing the frozen interrogation synthesis (`.mochiko/memory/governance-intent.md`) BEFORE the user ratifies it, spawned at the sizing gate, never a participant in the session. SHOULD also invoke for the verify pass over folded dispositions or the bounded delta-pass on a material post-review edit. Run by an independent reviewer, never the session lead; defaults to a FAIL posture.
allowed-tools: Bash(mochiko-cli *)
---

# Intent Review — Stress-Testing the Governance Synthesis

Cold reviewer of the frozen `governance-intent.md` — fact profile, floor-expression and
deck rulings, minted intents, waivers, modules, exclusions, each with a GI-ID and a
lead-assigned confidence mark (`Confident / Assumed / Contested / Unsure / Deferred`) — a
**traceable contract** on the surface-set producer. A lens brief may scope you to
*coverage* (agenda surface: missed dimensions, convergence-skip audits, card-acceptance +
waiver/module sweeps) or *coherence* (fact↔risk↔ruling alignment, mark/echo-rationale
audit, reality-conflict resolutions against the analysis, cross-element contradictions).

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-governance-intent · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-governance-intent · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-governance-intent --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-governance-intent --section review-governance-intent.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-governance-intent --section review-governance-intent.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-governance-intent --section review-governance-intent.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-governance-intent --section review-governance-intent.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-governance-intent --section review-governance-intent.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-governance-intent --section review-governance-intent.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Protocol

Form the attack sequestered, then read the frozen synthesis, the agenda, and — brownfield —
the codebase analysis. Work every hunt class the lens brief admits, the over-governance
hunt included; shape each finding to the contract; cross-examine in a pair; close with the
survivor report and the recommended status.
