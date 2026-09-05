---
name: patterns-plan-minimalism
description: This skill MUST be invoked at a design decision — what the design phase authors inside `/mochiko:implement` (scoped to the sufficiency gap list), each producing seat's plan, the epic joint design-phase plan, and any design-artifact decision — running the simplest-execution ladder over every design element (stop at the first failing rung: required · simpler shape · already exists · minimum now · builder's room), disclosed rung-wise. SHOULD also invoke on 'plan minimalism' or 'is this artifact needed'. Single source of the design ladder; design-time sibling of `mochiko:patterns-code-minimalism`.
allowed-tools: Bash(mochiko-cli *)
---

# Plan Minimalism — The Simplest-Execution Ladder

**The cheapest artifact is the one the plan never has to carry.**

## Overview

Before an artifact or design element enters the design-phase package, run the ladder: a
ranked check over every design element. It grades the *solution the design commits the
build to*, not the weight of the documents — thin documents are a consequence, not the
test.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-plan-minimalism · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-plan-minimalism · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-plan-minimalism --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-plan-minimalism --section patterns-plan-minimalism.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-plan-minimalism --section patterns-plan-minimalism.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-plan-minimalism --section patterns-plan-minimalism.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-plan-minimalism --section patterns-plan-minimalism.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-plan-minimalism --section patterns-plan-minimalism.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-plan-minimalism --section patterns-plan-minimalism.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## The Ladder

Rung by rung — the stop rule, the rung scopes, and the read duty are delivered by `mochiko-cli`:

1. **Required?** — a ratified requirement or an asserted floor obligation names it, or it
   does not enter the package. Strict: no glue exception (glue is builder's room, rung 5),
   no speculative or YAGNI element.
2. **Simpler shape?** — a design with fewer parts meeting the same requirement wins; no
   new abstraction, the boring choice; no rich-domain modeling for operational or
   mechanical features.
3. **Already exists?** — a baseline, the current system, an installed dependency, or an
   adoptable proven component (per `mochiko:patterns-adopt-first`) carries it: extend,
   reference, or adopt — never re-design.
4. **Minimum now** — sized to the requirement as ratified; future-proof only where the
   retrofit is expensive.
5. **Builder's room** — the design states WHAT plus its binding constraints; HOW stays
   open, prescribed only where cost-of-getting-it-wrong is high (boundary contracts,
   persisted shapes, security). The rest is guidance the build may improve on.

## Sibling

`mochiko:patterns-code-minimalism` — the build-time continuation: the same philosophy over
code at card decomposition. Two ladders, one discipline, two altitudes.
