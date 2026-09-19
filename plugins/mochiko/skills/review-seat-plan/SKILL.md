---
name: review-seat-plan
description: This skill MUST be invoked when grading a producing seat's plan — the binary PASS/FAIL a plan-only dispatch takes before the lead approves it and resumes that seat to work (sound-loop leg 1). SHOULD also invoke on 'grade the plan', 'seat plan', 'plan-only dispatch', 'plan QA', or 're-plan'. Seven items are walked — scope fidelity · write set declared · reads named · rung claims present · stops and hand-offs named · no self-clearing step · size bound — 1 through 6 blocking, 7 advisory; a FAIL cites the item and gives the fix. Carried by a fresh peer of the author's persona type, never the author's own context and never the lead; a persona-less producer's plan goes to a fresh generic seat. Defaults to FAIL. Additive to sound-loop leg 2, never a substitute — the artifact the seat then produces is still graded by a non-author seat.
allowed-tools: Bash(mochiko-cli *)
---

# Grading a Seat Plan

The plan gate of sound-loop leg 1: one binary grade over one producing seat's plan, standing
between a plan-only dispatch and the lead's approval to resume that seat. The grade is additive
to leg 2, never a substitute — the artifact the seat goes on to produce is still graded by a
non-author seat before the user's gate. You are a fresh peer of the author's persona type, or a
fresh generic seat where the producer carries no mochiko persona, and the rules below, exactly as
`mochiko-cli` renders them, are the whole contract: your dispatch brief carries the seat's plan
verbatim, its assigned scope, and the other seats' declared write sets, and nothing else of the
bar you hold. The floor this grade serves: `mochiko:patterns-sound-loop`.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-seat-plan · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-seat-plan · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-seat-plan --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Read the plan as the brief carries it — the seat's own verbatim text, never a summary and never
its account of what it means. Then check the tree for every claim the plan makes about what is
already there: what the plan says it read, you read. Walk the seven items in order, confirming
each once against the plan and what you read, blocking on items 1 through 6 and recording item 7
as a finding. Close by emitting the verdict in full and returning it to the lead, who carries the
outcome into the run's disclosure line; the plan itself is never persisted. On a FAIL you hand
back the fix list and stop — you are resumed to grade the revision rather than respawned, and the
approval is the lead's, never yours.
