---
name: patterns-adopt-first
description: This skill MUST be invoked at a design-phase decision or a build-time decomposition decision in a commodity category (storage, locking, serialization, queueing, caching, auth, search) — the alternatives name a real off-the-shelf candidate or state none exists; custom wins only in writing against it, and a build-time ruling halts to the user, never builder-decided. SHOULD also invoke on 'build vs buy', 'off-the-shelf', 'should we build this ourselves', 'shelf candidate', 'hand-rolled'. In-process/self-hostable only; SaaS buy is an IP-XXX call. Governs CHANGING the stack; `analysis-codebase` describes it.
---

# Adopt First — Build vs Off-the-Shelf at Design and Build Time

**A problem older than the product has probably already been solved.**

## Overview

The minimalism ladders ask whether a piece of the system should exist and how small it can be.
This discipline asks a different question of the pieces that survive: **must we build this one
ourselves?** It fires in the **design phase**, where whole mechanisms are still on the table,
and again at **build-time decomposition** when a commodity need surfaces that the design phase
never ruled. It binds the D-XXX decision: name a real off-the-shelf candidate, and beat it in
writing before choosing custom.

The discipline is **weigh and disclose**, never adopt-always: the named candidate may lose on
merits. What may not happen is that it was never named at all. The canonical miss was itself a
framing artifact — a storage engine framed as a serialization choice, and the shelf question
never got asked.

## Rules — load the schema first

Your first action, before any shelf check: **Read `schema.yaml` (this skill's own directory)
raw, in full, as a declared first action.** The schema is the source of truth for this skill's
binding rules, nested in six sections, each addressable by its section ID:
`patterns-adopt-first.sec.trigger` · `patterns-adopt-first.sec.scope` ·
`patterns-adopt-first.sec.discipline` · `patterns-adopt-first.sec.inputs` ·
`patterns-adopt-first.sec.disclosure` · `patterns-adopt-first.sec.reserved`. Interpret it
live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
carrying `when:` binds only where its terms hold against the schema's declared `conditions:`,
except that a `class: floor` rule is always read and always delivered — `when:` gates when its
obligation applies, never whether it reaches you; a `pointer:` rule binds you to that file's
or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 7 rules of `class: floor` are
non-waivable. Before the first shelf-check step, state the floor count back — a skipped or
partial read leaves that count blank: halt and surface it, and halt likewise if the schema's
`class: floor` count disagrees with the pin.
