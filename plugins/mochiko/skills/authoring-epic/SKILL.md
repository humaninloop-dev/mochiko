---
name: authoring-epic
description: This skill MUST be invoked when authoring or updating an epic — the transient unit for a multi-feature batch: the manifest + spine at `.mochiko/epics/EPIC-XXX/`, mint-once/overlap guard, close semantics. SHOULD also invoke on 'epic', 'EPIC-XXX', 'mint an epic', 'epic manifest', 'epic spine', or 'multi-feature batch'. Boundary: owns the epic OBJECT — NOT its map marker/seam grammar (mochiko:authoring-feature-map), NOT implement-run mechanics. Selection-scope only; never grades its own output.
allowed-tools: Bash(mochiko-cli *)
---

# Authoring the Epic — Manifest and Spine

**The epic coordinates delivery; it never becomes what the product is.**

## Overview

An **epic** is the transient first-class delivery unit that runs a **closely related
multi-feature batch** — members designed and built as one unit through `/mochiko:implement`.
While in flight it is the active unit and member features' pending work rows carry an inline
`[EPIC-XXX]` marker on the map; at delivery the rows fold into their capabilities' extents,
the markers vanish, and the epic closes.

(*Spine* in this skill is the epic's, never the architecture store's topology spine; where
both appear, name which.)

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules authoring-epic · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · authoring-epic · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules authoring-epic --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-epic --section authoring-epic.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-epic --section authoring-epic.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-epic --section authoring-epic.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-epic --section authoring-epic.sec.artifact --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-epic --section authoring-epic.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules authoring-epic --section authoring-epic.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## The two faces

The **manifest** is the product view — who the members are, where the batch stands, and why
these members belong together. The **spine files** beside it are the tech view the implement
run consumes: the joint design-phase plan, the joint architecture + seam design, the
ordering, and the shared-baseline deltas. Per-feature detail stays in the member dirs the
downstream machinery already reads; the spine carries only what is genuinely joint.
