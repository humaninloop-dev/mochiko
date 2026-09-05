---
name: patterns-model-tiering
description: This skill MUST be invoked when dispatching exploration or fact-finding work in any mochiko run — routing each read by the class key. locate/enumerate/targeted-read gaps go to a native `Explore` subagent spawned with an explicit `model: haiku` override; interpretive reads, decision-driving absences, and completeness-sensitive enumerations stay on the session tier. SHOULD also invoke on 'model tiering', 'cheap explorer', 'which model', 'explore the code', 'targeted read', or 'fact-find dispatch'. Governs dispatch tier only — rostered seats never change model (model-tiered-seats D5); third sibling of patterns-sound-loop and patterns-transport-floor.
allowed-tools: Bash(mochiko-cli *)
---

# Model Tiering — The Class-Keyed Dispatch Floor

**Every read rides the lowest tier where its answer can be trusted.**

## Overview

Rostered mochiko personas run on the strong tier and stay there; this floor governs the
*reads they and the lead dispatch along the way*. The economics are documented, not
assumed: Haiku is ~5× cheaper than Opus and ~10× cheaper than Fable per token both
directions, and on subscription seats cheap-model work preserves Opus-cap headroom
(model-tiered-seats D1).

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-model-tiering · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-model-tiering · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-model-tiering --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-model-tiering --section patterns-model-tiering.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-model-tiering --section patterns-model-tiering.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-model-tiering --section patterns-model-tiering.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-model-tiering --section patterns-model-tiering.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-model-tiering --section patterns-model-tiering.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-model-tiering --section patterns-model-tiering.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

