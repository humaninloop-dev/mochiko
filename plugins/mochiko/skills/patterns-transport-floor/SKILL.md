---
name: patterns-transport-floor
description: This skill MUST be invoked before composing or running any multi-seat work — cross-seat or lead-relayed messaging, or any shared write surface — the transport floor against message races and write collisions. Each lane is non-waivable once fired. SHOULD also invoke on 'transport floor', 'message race', 'single writer', 'fan-in confirmation', or 'mesh hold'. Governs transport use, never the neutral transport choice; sibling of patterns-sound-loop.
allowed-tools: Bash(mochiko-cli *)
---

# Transport Floor — Message Races and Write Collisions

**The message arrives; the work does not start until the lead opens it.**

## Overview

The sound-loop floor governs *who produces and who reviews*; this floor governs *how seats
talk and write while they do it*. It is kind-keyed on the transport axis: whenever a
multi-seat run carries messaging or a shared write surface, a set of legs becomes
non-waivable for the hazard class it fires against. The hazards are concrete: message races
(an order lands stale, a supersession arrives out of sequence, an idle ping fires without
its deliverable) and write collisions (two seats overwrite one file, a grader reads a
surface still moving).

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-transport-floor · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-transport-floor · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-transport-floor --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-transport-floor --section patterns-transport-floor.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-transport-floor --section patterns-transport-floor.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-transport-floor --section patterns-transport-floor.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-transport-floor --section patterns-transport-floor.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-transport-floor --section patterns-transport-floor.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-transport-floor --section patterns-transport-floor.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Sibling

`mochiko:patterns-sound-loop` — the ritual floor (who produces, who reviews) on the same
kind-keyed pattern, a different axis. That floor's neutrality line points here: the
transport *choice* stays neutral, transport *use* carries this floor.
