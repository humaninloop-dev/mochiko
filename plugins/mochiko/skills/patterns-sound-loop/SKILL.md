---
name: patterns-sound-loop
description: This skill MUST be invoked before a judgment-authored write to a governing surface — capability map, product baselines, the architecture store, specs, governance, plugin primitives, product code — running the floor: a seat produces on a lead-approved plan (never the lead), a non-author seat reviews, the user rules. No size gate; desk delta cards take the review leg. SHOULD also invoke on 'sound loop', 'ritual floor', or 'seat wiring'. Single source of the floor; fourth sibling of the minimalism trio.
---

# Sound Loop — The Ritual Floor

**The entry door never lowers the review.**

## Overview

Whenever a judgment-authored artifact is about to land on a governing surface, three
rituals become non-waivable — the floor this skill single-sources. Transport *choice*
stays neutral — a seat may be a teammate or a subagent, the lead's per-seat call; what
dies above the floor is the lead absorbing the seat — but transport *use* carries its own
floor (`mochiko:patterns-transport-floor`, teammate-message-races D3–D5). The three
minimalism siblings — `mochiko:patterns-plan-minimalism`, `mochiko:patterns-code-minimalism`,
`mochiko:patterns-map-minimalism` — size the artifact; this floor governs who produces and
who reviews.

## Rules — load the schema first

Your first action, before the trigger is evaluated: **Read `schema.yaml` (this skill's own
directory) raw, in full, as one declared first action.** The schema is the source of truth
for this skill's binding rules, nested in six sections, each addressable by its section
ID: `patterns-sound-loop.sec.trigger` · `patterns-sound-loop.sec.scope` ·
`patterns-sound-loop.sec.discipline` · `patterns-sound-loop.sec.inputs` ·
`patterns-sound-loop.sec.disclosure` · `patterns-sound-loop.sec.reserved`. Interpret it
live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
carrying `when:` binds only where its terms hold against the schema's declared
`conditions:`, except that a `class: floor` rule is always read and always delivered —
`when:` gates when its obligation applies, never whether it reaches you; a `pointer:` rule
binds you to that file's or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 6 rules of `class: floor`
are non-waivable. Before the trigger is evaluated, state the floor count back — a skipped
or partial read leaves that count blank: halt and surface it, and halt likewise if the
schema's `class: floor` count disagrees with the pin.
