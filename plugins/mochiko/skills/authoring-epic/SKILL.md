---
name: authoring-epic
description: This skill MUST be invoked when authoring or updating an epic — the transient unit for a multi-feature batch: the manifest + spine at `.mochiko/epics/EPIC-XXX/`, mint-once/overlap guard, close semantics. SHOULD also invoke on 'epic', 'EPIC-XXX', 'mint an epic', 'epic manifest', 'epic spine', or 'multi-feature batch'. Boundary: owns the epic OBJECT — NOT its map marker/seam grammar (mochiko:authoring-feature-map), NOT implement-run mechanics. Selection-scope only; never grades its own output.
---

# Authoring the Epic — Manifest and Spine

**The epic coordinates delivery; it never becomes what the product is.**

## Overview

An **epic** is the transient first-class delivery unit that runs a **closely related
multi-feature batch** — members designed and built as one unit through `/mochiko:implement`.
While in flight it is the active unit and member features' pending work rows carry an inline
`[EPIC-XXX]` marker on the map; at delivery the rows fold into their capabilities' extents,
the markers vanish, and the epic closes.

(*Spine* in this skill is the epic's, never the architecture store's topology spine; where
both appear, name which.)

## Rules — load the schema first

Your first action, before any epic touch: **Read `schema.yaml` (this skill's own directory)
and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same declared first
action** — schema, then common. The schema is the source of truth for this skill's binding
rules, nested in six sections, each addressable by its section ID:
`authoring-epic.sec.independence` · `authoring-epic.sec.scope` ·
`authoring-epic.sec.inputs` · `authoring-epic.sec.artifact` · `authoring-epic.sec.output` ·
`authoring-epic.sec.reserved`. Interpret it live: a rule's `kind:` names what it is, and an
absent `kind:` reads `constraint`; a rule of `class: floor` is always read and always
delivered; a `pointer:` rule binds you to that file's or skill's procedure, referenced never
restated; `${var}` substitutes from this schema's `vars:` at read time; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: authoring-common.<slug>` inherits text/labels/pointer from
`skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `authoring-epic.*` ID stays the citable ID. The floor pin: the 10 rules of
`class: floor` are non-waivable. Before the first epic-touching step, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

## The two faces

The **manifest** is the product view — who the members are, where the batch stands, and why
these members belong together. The **spine files** beside it are the tech view the implement
run consumes: the joint design-phase plan, the joint architecture + seam design, the
ordering, and the shared-baseline deltas. Per-feature detail stays in the member dirs the
downstream machinery already reads; the spine carries only what is genuinely joint.
