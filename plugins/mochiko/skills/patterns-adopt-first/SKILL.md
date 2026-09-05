---
name: patterns-adopt-first
description: This skill MUST be invoked at a design-phase decision or a build-time decomposition decision in a commodity category (storage, locking, serialization, queueing, caching, auth, search) — the alternatives name a real off-the-shelf candidate or state none exists; custom wins only in writing against it, and a build-time ruling halts to the user, never builder-decided. SHOULD also invoke on 'build vs buy', 'off-the-shelf', 'should we build this ourselves', 'shelf candidate', 'hand-rolled'. In-process/self-hostable only; SaaS buy is an IP-XXX call. Governs CHANGING the stack; `analysis-codebase` describes it.
allowed-tools: Bash(mochiko-cli *)
---

# Adopt First — Build vs Off-the-Shelf at Design and Build Time

**A problem older than the product has probably already been solved.**

## Overview

The minimalism ladders ask whether a piece of the system should exist and how small it can be.
This discipline asks a different question of the pieces that survive: **must we build this one
ourselves?** It fires in the **design phase**, where whole mechanisms are still on the table,
and again at **build-time decomposition** when a commodity need surfaces that the design phase
never ruled. It binds the D-XXX decision: name a real off-the-shelf candidate, and beat it in
writing before choosing custom.

The discipline is **weigh and disclose**, never adopt-always: the named candidate may lose on
merits. What may not happen is that it was never named at all. The canonical miss was itself a
framing artifact — a storage engine framed as a serialization choice, and the shelf question
never got asked.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-adopt-first · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-adopt-first · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-adopt-first --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-adopt-first --section patterns-adopt-first.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-adopt-first --section patterns-adopt-first.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-adopt-first --section patterns-adopt-first.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-adopt-first --section patterns-adopt-first.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-adopt-first --section patterns-adopt-first.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-adopt-first --section patterns-adopt-first.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

