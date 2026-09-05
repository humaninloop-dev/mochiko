---
name: review-code-minimalism
description: This skill MUST be invoked when independently grading a cycle's produced code against the pre-code ladder — the code-minimalism lens run by the verification seat: read the cycle's git diff AND `cycle-report.md`, grade each rung claim against `mochiko:patterns-code-minimalism` (the standard, never restated here), and emit a `minimalism:` findings block. Rungs 2, 3, and 5 carry a codebase-read obligation. Findings are ADVISORY, never a cycle-failing gate. Scope is the minimalism lens ONLY.
allowed-tools: Bash(mochiko-cli *)
---

# Review — Code Minimalism Lens

**Diff shows what was written; disclosure shows what the builder says they checked;
neither alone shows what should never have been written.**

## Overview

The per-cycle over-engineering lens: an independent, static read of a cycle's produced code
against the pre-code ladder. Cycle diffs are small, context is fresh, rework is cheapest at
cycle close.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-code-minimalism · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-code-minimalism · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-code-minimalism --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-code-minimalism --section review-code-minimalism.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-code-minimalism --section review-code-minimalism.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-code-minimalism --section review-code-minimalism.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-code-minimalism --section review-code-minimalism.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-code-minimalism --section review-code-minimalism.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-code-minimalism --section review-code-minimalism.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Locate the diff first: files created/modified per the cycle report locate it; `git diff`
over those paths.

**1. Read the disclosure.** Every decomposition task should carry a rung; the disclosure
surface exists so this lens can grade it — and it also shows whether rung-zero reading
happened at all.

**2. Grade each rung claim against the standard.** Open
`mochiko:patterns-code-minimalism`; per task, ask: does the code sit on the claimed rung,
and does a higher rung apply that the builder descended past?

**3. Verify rung-2/3/5 claims against the codebase.**
- **Rung 2:** targeted greps around the diff for existing helpers/utilities the new code
  duplicates — against the *current* codebase, which also catches cross-cycle accretion
  (cycle 5 duplicating cycle 2's helper reads as a rung-2 violation now).
- **Rung 3:** does the language's standard library already provide the written behavior?
- **Rung 5:** check the dependency manifest — does an installed dependency already cover
  it?

**4. Check the floor line.**

**5. Emit findings** as the schema's output contract shapes them, one line of evidence
each (the grep hit, the stdlib call, the manifest entry).
