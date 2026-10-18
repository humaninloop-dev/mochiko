---
name: analysis-iterative
description: This skill MUST be invoked when running the shared thinking stage — a question worked through with the user one fork per turn, facts before forks, every ruling the user's, the next writer held inside the ruled set when the caller asks — in brainstorm's deciding, specify's intent and story stages, the architecture desk's focus stage, the feature desk's card visits, implement's design stage and setup's interrogation, and when a command asks a ruling outside its stage, by its asking line's named subset. SHOULD also invoke on 'think this through', 'thinking stage', 'ruled set', or a multi-decision trade-off. Distinct from review-specifications, which grades a drafted spec; this skill rules what gets written.
allowed-tools: Bash(mochiko-cli *)
---

# Iterative Analysis — the Shared Thinking Stage

## Overview

The shared thinking stage: a question thought through with the user and turned into rulings —
the asking, and, where a caller switches it on, the ruled-set contract that keeps the next writer
inside what the user ruled. Each question builds on the previous answer; style, depth and output
adapt to the complexity that emerges, and each question's format to the user's state —
confident, uncertain, or somewhere in between.

Its callers extend it with their domain part in their own rules: `/mochiko:brainstorm`'s
deciding, `/mochiko:specify`'s story stage, the `/mochiko:architecture` desk's focus stage, the
`/mochiko:feature` desk's card visits and `/mochiko:implement`'s design stage; `/mochiko:setup`'s
interrogation and specify's intent stage reach it through their own pointers, the contract off.
Outside its thinking stage, a command opts the rulings it asks into a named subset of these rules
by the shared asking line its own rules extend (`common.asking-line`).

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules analysis-iterative · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · analysis-iterative · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules analysis-iterative --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-iterative --section analysis-iterative.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-iterative --section analysis-iterative.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-iterative --section analysis-iterative.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-iterative --section analysis-iterative.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-iterative --section analysis-iterative.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules analysis-iterative --section analysis-iterative.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## When NOT to Use

- **Quick clarifications** — simple questions do not need iterative questioning
- **Build-time implementation details** — decomposing and coding a card is the builder's (`mochiko:executing-tdd-cycle`); a design choice the user rules, as in `/mochiko:implement`'s design stage, runs here
- **Specification review** — reviewing an already-drafted spec for gaps is post-draft work; use `mochiko:review-specifications` instead (this skill rules what gets written, never grades what was)
- **When user has clear direction** — use confirmations to verify, then wrap up fast. Do not slow down decisive users with unnecessary exploration.
- **Time-sensitive decisions** — iterative questioning takes time

## Confidence marks

Assign based on conversation observation:

| Indicator | Meaning |
|-----------|---------|
| `Confident` | Clear, reasoned choice with no hesitation |
| `Assumed` | Inferred from context, never explicitly confirmed |
| `Contested` | User disagreed with recommendation; deliberate choice |
| `Unsure` | Expressed uncertainty; decided provisionally |
| `Deferred` | Explicitly postponed — not enough information now |

## Common Mistakes

| Mistake | Why it fails | Fix |
|---------|--------------|-----|
| Options before an unsure user can choose | Options force premature decisions on an unsure user | Open probes help them discover what they think; the fork, with its options, follows once they can choose |
| Ignoring "unsure" signals | Uncertainty read as indifference → recommendations passively accepted, never owned | Probe deeper from a different angle |
| Multiple forks per turn | Fractured attention, shallow answers across all of them | One fork per turn; the defaults batch, confirmed on one screen, is the one stated exception |
| Questions that don't connect | Topic-jumping breaks collaborative momentum | Show how the previous answer shapes the direction |
| Premature synthesis | Forced wrap-up leaves core trade-offs unaddressed | Nudge, never force; the user has final say |
| Rigid opening script | A recited introduction ignores the topic's tone and energy | Adapt the opening to both |
| Padding the synthesis | Filler sections dilute a lean conversation's record | Output depth matches conversation depth |

## Reference

See [ADAPTIVE-EXAMPLES.md](references/ADAPTIVE-EXAMPLES.md) for annotated conversations showing
these principles in action; they predate the delivered rules, which hold where an example differs.
