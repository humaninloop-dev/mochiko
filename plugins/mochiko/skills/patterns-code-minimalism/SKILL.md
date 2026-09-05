---
name: patterns-code-minimalism
description: This skill MUST be invoked at build-time card decomposition, BEFORE any red-phase test — running the pre-code ladder over each task (stop at the first applicable rung: exist at all · in codebase · stdlib · native platform · installed dep · one line · minimum), disclosed in the cycle report. SHOULD also invoke on 'should this code exist', 'reuse before build', 'stdlib first', 'over-engineering', 'YAGNI', or when slimming existing code that grew unneeded abstraction layers. Single source of the ladder; distinct from the green-phase 'minimum code to pass' rule.
allowed-tools: Bash(mochiko-cli *)
---

# Code Minimalism — The Pre-Code Ladder

**The cheapest code is the code never written.**

## Overview

Before any code is written for a task, run the ladder: a ranked pre-code check over each
task. Its design-time sibling `mochiko:patterns-plan-minimalism` runs the same discipline
over the design's elements upstream; this ladder is the build-time continuation.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-code-minimalism · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-code-minimalism · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-code-minimalism --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-code-minimalism --section patterns-code-minimalism.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-code-minimalism --section patterns-code-minimalism.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-code-minimalism --section patterns-code-minimalism.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-code-minimalism --section patterns-code-minimalism.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-code-minimalism --section patterns-code-minimalism.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-code-minimalism --section patterns-code-minimalism.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## The Ladder

Rung by rung — the stop rule and every bound on the walk live in the schema:

1. **Does it need to exist at all?** The requirement is already met, speculative, or
   YAGNI — skip the task entirely.
2. **Already in the codebase?** Reuse the existing helper, utility, or pattern — extending
   an existing surface beats inventing a parallel one.
3. **Standard library handles it?** Use it — no wrapper, no re-implementation.
4. **Native platform feature?** The runtime, framework, or platform already does this —
   use it.
5. **Installed dependency covers it?** A dependency already in the manifest does this —
   use it.
6. **Fits in one line?** Write the one line.
7. **Only then:** write the minimum that works.
