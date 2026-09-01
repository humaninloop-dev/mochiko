---
name: patterns-transport-floor
description: This skill MUST be invoked before composing or running any multi-seat work — cross-seat or lead-relayed messaging, or any shared write surface — the transport floor against message races and write collisions. Each lane is non-waivable once fired. SHOULD also invoke on 'transport floor', 'message race', 'single writer', 'fan-in confirmation', or 'mesh hold'. Governs transport use, never the neutral transport choice; sibling of patterns-sound-loop.
---

# Transport Floor — Message Races and Write Collisions

**The message arrives; the work does not start until the lead opens it.**

## Overview

The sound-loop floor governs *who produces and who reviews*; this floor governs *how seats
talk and write while they do it*. It is kind-keyed on the transport axis: whenever a
multi-seat run carries messaging or a shared write surface, a set of legs becomes
non-waivable for the hazard class it fires against. The hazards are concrete: message races
(an order lands stale, a supersession arrives out of sequence, an idle ping fires without
its deliverable) and write collisions (two seats overwrite one file, a grader reads a
surface still moving).

## Rules — load the schema first

Your first action, before composing or opening any multi-seat work: **Read `schema.yaml`
(this skill's own directory) raw, in full** — the patterns family ships no common file, so
the pair's own schema is the whole first action. The schema is the source of truth for this
floor's binding rules, nested in six sections, each addressable by its section ID:
`patterns-transport-floor.sec.trigger` · `patterns-transport-floor.sec.scope` ·
`patterns-transport-floor.sec.discipline` · `patterns-transport-floor.sec.inputs` ·
`patterns-transport-floor.sec.disclosure` · `patterns-transport-floor.sec.reserved`.
Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
`constraint`; a rule's `when:` resolves against the schema's declared `conditions:` — the
two lanes, `messaging` and `shared_write_surface` — and gates when the obligation applies,
never whether it is delivered; a rule of `class: floor` is always read and always delivered
whatever its `when:`; a `pointer:` rule binds you to that file's or skill's procedure,
referenced never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`.
The floor pin: the 11 rules of `class: floor` are non-waivable. Before the first
composition or messaging step, state the floor count back — a skipped or partial read
leaves that count blank: halt and surface it, and halt likewise if the schema's
`class: floor` count disagrees with the pin.

## Sibling

`mochiko:patterns-sound-loop` — the ritual floor (who produces, who reviews) on the same
kind-keyed pattern, a different axis. That floor's neutrality line points here: the
transport *choice* stays neutral, transport *use* carries this floor.
