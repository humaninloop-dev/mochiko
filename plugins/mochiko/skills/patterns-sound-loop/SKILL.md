---
name: patterns-sound-loop
description: This skill MUST be invoked before a judgment-authored write to a governing surface — capability map, product baselines, the architecture store, specs, governance, plugin primitives, product code — running the floor: a seat produces on a lead-approved plan (never the lead), a non-author seat reviews, the user rules. No size gate; desk delta cards take the review leg. SHOULD also invoke on 'sound loop', 'ritual floor', or 'seat wiring'. Single source of the floor; fourth sibling of the minimalism trio.
allowed-tools: Bash(mochiko-cli *)
---

# Sound Loop — The Ritual Floor

**The entry door never lowers the review.**

## Overview

Whenever a judgment-authored artifact is about to land on a governing surface, three
rituals become non-waivable — the floor this skill single-sources. Transport *choice*
stays neutral — a seat may be a teammate or a subagent, the lead's per-seat call; what
dies above the floor is the lead absorbing the seat — but transport *use* carries its own
floor (`mochiko:patterns-transport-floor`, teammate-message-races D3–D5). The three
minimalism siblings — `mochiko:patterns-plan-minimalism`, `mochiko:patterns-code-minimalism`,
`mochiko:patterns-map-minimalism` — size the artifact; this floor governs who produces and
who reviews.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-sound-loop · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-sound-loop · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-sound-loop --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-sound-loop --section patterns-sound-loop.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

