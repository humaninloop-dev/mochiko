---
name: patterns-craft-floor
description: This skill MUST be invoked immediately before any edit on a UX-bearing cycle card — a card carrying SCR-XXX or FLOW-XXX trace — including small refinements, to hold the design craft floor at the edit — the absolute bans, the refuse-by-default reflexes, the verify list, the typeset / layout / colorize / animate / delight folds, and the register dial (bolder / quieter / distill) executed from the surface's Direction block and the design baseline's tokens. SHOULD also invoke on 'craft floor', 'UI edit', 'design bans', 'AI slop', 'honor the tokens', or 'register dial'. Never for planning-only work. Boundary: executes a direction already made — NOT choosing it (mochiko:patterns-design-direction), NOT the prototype (mochiko:authoring-prototype), NOT the accessibility standard (kept by mochiko:patterns-code-minimalism); never grades its own output.
allowed-tools: Bash(mochiko-cli *)
---

# Craft Floor — Design Discipline at the Edit

**The floor holds the mechanics; it never picks the direction.** *(upstream, verbatim)*

> Ported from `pbakaus/impeccable` at `e0881d2de397d5e9761d7b35ff5017d8f5ebf69b` (Apache-2.0 —
> `skill/reference/{craft-floor,typeset,layout,colorize,animate,delight,bolder,quieter,distill,operate}.md`),
> rewritten in mochiko's form; attribution in the repository's root `NOTICE`. Passages carried
> verbatim are marked *(upstream, verbatim)*.

## Overview

Every taste choice — register, visual world, type and colour intent, motion stance — was made
once, upstream, in the surface's Direction block. This floor carries those choices to the edit
together with the bans and reflexes no checklist-free build keeps. It decides nothing: where the
edit needs a choice the contract did not make, it stops.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-craft-floor · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-craft-floor · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-craft-floor --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-craft-floor --section patterns-craft-floor.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-craft-floor --section patterns-craft-floor.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-craft-floor --section patterns-craft-floor.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-craft-floor --section patterns-craft-floor.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-craft-floor --section patterns-craft-floor.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-craft-floor --section patterns-craft-floor.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Bans — no brief earns these back

- A kicker or eyebrow label above a heading; the heading carries its own weight.
- Cards nested in cards.
- Grey text on a coloured surface — tint secondary text from that hue.
- Bounce or elastic easing.
- A zero-offset coloured glow standing in for depth.
- Gradient text; emphasis comes from weight or size.
- Emoji or Unicode glyphs standing in for an icon system.

## Refuse by default — the contract can earn these

Same-size icon-heading-text card grids as page structure · the hero-metric template · section
numbers carrying no information · a modal for a task needing neither interruption nor protected
focus · decorative glass and blur · a coloured side stripe over 1px on cards or alerts · hard
offset shadows outside a world that chose them · sparklines and rings standing in for content ·
monospace as a "technical" costume · light or dark picked by category, not the use scene.
Reaching for one when the contract is silent means rewriting the element, not softening it.

## Verify — on the built result, in one batched round

- **Spacing:** tight groups, generous separation, more space above a heading than below.
- **Type:** body measure 65–75ch, obvious scale and weight steps, the real copy at every
  declared viewport with nothing overflowing.
- **Motion:** one authored moment at most; state change and feedback, not decoration; content
  visible before any entrance runs.
- **States:** hover, focus, active, disabled, loading, error, empty — none left at half.
- **Browser surfaces:** selection, caret, scrollbars, focus rings, underline offset, and tabular
  numerals themed from the palette, not left at defaults.
- **Copy:** the product's own language; controls name their action, errors name the recovery.
- **Coverage:** every screen's data and every flow's action from the manifest present and
  findable.

Contrast and every other accessibility check belong to the standard of record, kept in view by
`mochiko:patterns-code-minimalism` — not restated here.

## The Folds

- **Typeset** — the fewest families and roles that make hierarchy unmistakable; Operate and Read
  take a fixed role scale (ratio ~1.125–1.2), Persuade and Experience may let display type
  carry the voice; repeated roles identical across screens.
- **Layout** — group by meaning before adding containers; rhythm from tight-versus-generous
  intervals on one spacing scale; responsive change is structural (reorder, collapse, reflow);
  visual, DOM, and focus order agree.
- **Colorize** — roles, not swatches: surface, text, action, focus, selection, status; the
  accent spent on action and state, never decoration; dark theme composed, not inverted.
- **Animate** — motion explains state, relationship, or one earned focal moment; 150–250 ms for
  routine change, exits faster than entrances, natural deceleration; no page-load choreography
  on Operate surfaces.
- **Delight** — only at moments that earn it (first use, completion, recovery); never delays
  the task, never fakes progress, never jokes about money, privacy, or loss.

## The Register Dial

| Register | At the edit |
|---|---|
| **bolder** | raise the target to the conviction its neighbours already carry, in the system's own vocabulary; nothing outside the target changes |
| **quieter** | lower intensity — saturation, weight, decoration, motion — without going generic |
| **distill** | remove what does not earn its place: one primary action, one spacing scale, no redundant copy |
| **held** | neither; execute the contract as written |

## Platform

On a native `Platform` value, the reference in
`../patterns-design-direction/references/` governs structure, navigation, and controls; the
rule delivered above holds the routing.

## Related

- `mochiko:patterns-design-direction` — upstream: the Direction block this floor executes
- `mochiko:patterns-code-minimalism` — the ladder and floor line that stay binding at the same edit
- `mochiko:executing-tdd-cycle` — the cycle this floor loads inside, and the report it discloses to
