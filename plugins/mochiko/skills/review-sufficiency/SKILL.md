---
name: review-sufficiency
description: This skill MUST be invoked when grading a capability-batch's guidance sufficiency at `/mochiko:implement` entry — the ten-clause check per selected work row, collapsing to a three-clause form per delta card under delta scope, over the spec, the architecture store, and the product baselines, emitting a binding per-row `sufficient` verdict or the gap list that scopes the in-run design phase. SHOULD also invoke on 'sufficiency check', 'enough guidance', 'sufficiency verdict', or 'gap list'. Never reads code, `tasks.md`, `**TEST:**` cases, or cycle reports. Defaults to FAIL — a row is insufficient until every clause is graded; run by a seat that authored none of the graded sources.
allowed-tools: Bash(mochiko-cli *)
---

# Grading Guidance Sufficiency

Binding pre-build gate over one unit of selected work: **does the guidance that already
exists carry enough for a builder to build it?** The check is size-adaptive by construction —
the unit is the map's own unit of scope — and its answer either licenses the build directly
or hands the design phase an exact, named scope. The design phase that closes a gap is a
different seat.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-sufficiency · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-sufficiency · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules review-sufficiency --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-sufficiency --section review-sufficiency.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-sufficiency --section review-sufficiency.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-sufficiency --section review-sufficiency.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-sufficiency --section review-sufficiency.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-sufficiency --section review-sufficiency.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-sufficiency --section review-sufficiency.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Resolve the scope first — selection or delta — because it fixes both the unit and the clause
set. Then, per unit: read the fenced source set whole, walk every applicable clause in order
(testable criteria → contract exposure → data exposure → structural trigger → NFR targets →
commodity exposure → dependency order → UX trace → delivered-feature exposure → in-flight
exposure), and record what each clause yields — a hold, a gap in that clause's own gap form,
or a justified n/a.

Close by assembling the report: per-unit verdicts, the clause-keyed gap list, the
store-consult result, and everything routed onward — trips, in-flight conflicts, `[MODIFY]`
amendment namings. The design phase, cards, and build all key off this report, never off the
conversation.
