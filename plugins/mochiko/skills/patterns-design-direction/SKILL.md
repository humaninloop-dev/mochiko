---
name: patterns-design-direction
description: This skill MUST be invoked when setting the design direction for a UX-bearing surface before its prototype is drawn — choosing the surface's mode (Persuade / Operate / Read / Experience), applying the design laws, walking the shape checklist with its onboarding, UX-copy, and device-class folds, and writing the Direction block (mode · register · contract · declared tokens · incumbent world) at the head of the spec's Screens & Flows. SHOULD also invoke on 'design direction', 'direction contract', 'which mode', 'shape the UX', 'preserve or replace the visual world', or 'native iOS/Android guidance'. Boundary: decides the direction — NOT the prototype that renders it (mochiko:authoring-prototype), NOT build-time craft (mochiko:patterns-craft-floor); never grades its own output.
allowed-tools: Bash(mochiko-cli *)
---

# Design Direction — Mode, Laws, and the Direction Contract

**Decide what the surface is for before drawing what it looks like.**

> Ported from `pbakaus/impeccable` at `e0881d2de397d5e9761d7b35ff5017d8f5ebf69b` (Apache-2.0 —
> `skill/SKILL.src.md`, `skill/reference/{shape,new-work,onboard,clarify,adapt,operate}.md`),
> rewritten in mochiko's form; attribution in the repository's root `NOTICE`. Passages carried
> verbatim are marked *(upstream, verbatim)*.

## Overview

A prototype drawn before its direction is settled defaults to the category's habits. This skill
settles the direction once per surface of a UX-bearing spec — the mode, the register, the contract, the
tokens it intends, and what happens to the incumbent look — and writes it where the prototype,
the build, and the reviewers all read it: the Direction block in the spec's Screens & Flows.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-design-direction · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-design-direction · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-design-direction --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-design-direction --section patterns-design-direction.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-design-direction --section patterns-design-direction.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-design-direction --section patterns-design-direction.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-design-direction --section patterns-design-direction.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-design-direction --section patterns-design-direction.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-design-direction --section patterns-design-direction.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Modes

The mode names what the visitor's success looks like on this surface.

- **Persuade** — the visitor decides and acts; design is the product (landing, pricing,
  campaign). Earn attention and action.
- **Operate** — the visitor completes a task (app UI, dashboards, editors, settings).
  Scanability, consistency, and native expectations outrank expression; brand lives in precise
  details, and earned familiarity is the bar.
- **Read** — the visitor understands something (docs, guides, changelogs). Structure for
  comprehension, then make the reading worth staying in.
- **Experience** — the visitor is inside the work (portfolio, gallery, showcase). The artifact
  leads; the interface recedes.

## The Design Laws

1. **The brief wins.** Pinned aesthetics, eras, materials, fonts, and palettes are honored even
   against a saturated-pattern warning.
2. **Refinement preserves; redesign replaces.** *(upstream, verbatim)* "Never split the
   difference into polish on the discarded look."
3. **Visual authority is evidence, not a filename.** An empty design baseline does not make a
   product greenfield; the shipped UI is read first.

## The Shape Checklist

Answer from the stories, the Intent rulings, and the baseline first; a material gap becomes a
question, never an invented answer. Mark an item that does not apply `n/a`.

**Round 1 — purpose, people, outcome.** What is this surface for? Who reaches it, in what
situation and state of mind? What is the one thing they must understand or do, and what does
success look like? What is true here that a neighbouring product could not claim?

**Round 2 — material, states, boundaries.** What real content and data must it carry, at
minimum, typical, and maximum range? Which states matter — first-run, empty, loading, error,
success, permissions, overflow, expert use? What must stay untouched, and what would feel wrong
even if polished?

**Fold — onboard.** Time to first value, not a tour: the moment that proves the product is
worth it, reached in the fewest steps. Every empty state says what will appear, why it matters,
and the next action; first use, cleared, no results, no permission, and failure are different
states. Onboarding is skippable and never shown twice.

**Fold — clarify.** Controls name their action (verb and object); a destructive action names
its object and consequence. An error says what failed, why when known, and how to recover. One
term per concept across the product; the heading and the intro never say the same thing.

**Fold — adapt.** For each device class the surface ships to, rethink rather than scale: what
won't fit, what won't work (hover on touch, small targets), what is wrong for the context.
Write the declared viewports into the Direction block's `Contract` field — they have no field
of their own, and the build's verification reads them there.

## The Direction Block

One block per surface, at the head of Screens & Flows — a heading naming the surface and its
screens, then five fields, one line each:

```
### Direction — triage queue (SCR-001–SCR-003)
- **Mode:** Operate
- **Register:** quieter — the list view is loud; restore scanability
- **Contract:** thesis — one queue, no dashboards · own world — existing neutrals, one action accent · story — triage in a minute · first viewport — queue left, detail right · viewports — 390, 1280
- **Declared tokens (intent):** none — honors the baseline
- **Incumbent world:** preserve — shipped app UI is coherent (evidence: `src/theme.ts`)
```

## Platform Routing

Read the value in the design baseline's `## Platform` section and load the native reference it
names — `references/ios.md`, `references/android.md` (both for `adaptive`). The rule
delivered above holds the full routing, including the no-value case.

## Related

- `mochiko:authoring-prototype` — downstream: renders this block at low fidelity
- `mochiko:patterns-craft-floor` — the build-time floor that executes the contract at the edit
- `.mochiko/product/design/` — the design baseline read first (`mochiko-cli home .mochiko/product/design/design.md`)
