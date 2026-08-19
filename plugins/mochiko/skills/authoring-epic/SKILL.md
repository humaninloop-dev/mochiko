---
name: authoring-epic
description: This skill MUST be invoked when authoring or updating an epic — the transient unit for a multi-feature batch: the manifest + spine at `.mochiko/epics/EPIC-XXX/`, mint-once/overlap guard, close semantics. SHOULD also invoke on 'epic', 'EPIC-XXX', 'mint an epic', 'epic manifest', 'epic spine', or 'multi-feature batch'. Boundary: owns the epic OBJECT — NOT its map marker/seam grammar (mochiko:authoring-feature-map), NOT plan/implement mechanics. Selection-scope only; never grades its own output.
---

# Authoring the Epic — Manifest and Spine

**The epic coordinates delivery; it never becomes what the product is.**

## Overview

An **epic** is the transient first-class delivery unit that runs a **closely related
multi-feature batch** — members planned and built as one unit through `/mochiko:plan` and
`/mochiko:implement`. It has identity (`EPIC-XXX`, sequential, same family as `FEAT-XXX`), a
home directory, a **manifest**, and a **spine**. This skill single-sources the epic's
**shape** — the manifest and spine files, the mint/overlap guard, and close semantics. The
commands that consume it (`plan`, `implement`, `feature`, `specify`) **reference this skill;
they never restate it.**

One epic type, two faces (no product/tech split): the **manifest is the product view**, the
**spine is the tech view** — plan and implement consume the spine sections. The epic's **role
is transient**: while in flight it is the active unit and member features' pending work rows
carry an inline `[EPIC-XXX]` marker on the map; at delivery the rows fold into their
capabilities' extents, the markers vanish, and the epic closes. **The directory persists as
readable record — never as a living map layer.** The map stays two-typed (durable capabilities
+ transient work rows); an epic is not a third type.

## When NOT to Use

- **Grading the epic** — the plan/implement outputs are graded by their cluster reviewers; this
  skill authors the epic object and never grades its own output.
- **The map marker or seam grammar** — the `[EPIC-XXX]` row marker and within-epic seam-owner
  bookkeeping are `mochiko:authoring-feature-map`'s; this skill *names* the seam owner in the
  spine, the map writes it.
- **Plan / implement run mechanics** — gate shapes, cycle sequencing, and landing steps live in
  `plan.md` / `implement.md`; this skill single-sources only the epic object they consume.
- **Transport composition** — worktree isolation vs single pen-holder is
  `mochiko:patterns-transport-floor`; referenced, never restated.
- **Batching delta-scope cards or non-feature product-lane work** — parked open threads; not
  epic scope today.

## The epic home — manifest + spine

Home `.mochiko/epics/EPIC-XXX/`. **No separate epics index — the directory is the registry; the
`/mochiko:feature` desk lists it.**

**`manifest.md`** — the product view:

- **Members** — each `FEAT-XXX`, linked to its `.mochiko/features/FEAT-XXX/` dir.
- **Status** — `open` / `delivered` / `closed-partial`.
- **Why-together line** — the relatedness stated at the declare-and-contest that opened it.

**Spine files** beside it — the tech view plan and implement consume. (*Spine* here is the
epic's, never the architecture store's topology spine; where both appear, name which.)

- **Joint plan-the-plan proposal** — one proposal over all members (spine artifacts + each
  member's artifact list).
- **Joint architecture + seam design** — **one signed store delta** for the whole epic, rendered
  once and signed off once; **each cross-member seam names its owner explicitly** (members land
  simultaneously, so no later-lander default applies). The assignment lives here; the map writes
  it at close.
- **Ordering** — shared-foundation first, then in-epic dependency order.
- **Shared-baseline deltas** — see below.

Per-feature **design deltas stay in each member's `.mochiko/features/FEAT-XXX/` dir**, linked
from the manifest — downstream machinery keeps reading the per-feature dirs it already reads.

## Shared-baseline deltas — author once, one pen-holder

A product baseline touched by **two or more members** gets **one joint delta authored in the
spine** under a **single pen-holder**; a baseline touched by **one member** keeps its
per-feature delta. The landing folds each baseline **exactly once** — spine delta for shared
baselines, feature delta otherwise — preserving the singular-delta-per-baseline graded fold.
The **transport floor's composition steer** (`mochiko:patterns-transport-floor`) governs every
epic shared-write surface — spine files and shared baseline deltas: concurrent writers get
worktree isolation or a single pen-holder, disclosed at run open.

## Minting and the overlap guard

**Mint-once.** Every workflow resolves `EPIC-XXX` by **lookup**; re-minting does not exist. A
feature's pending rows belong to **at most one open epic at a time**. Three doors:

- **`/mochiko:feature` desk** owns the epic's life — mint, membership change, status view, close.
- **`/mochiko:plan`** invoked with a bare feature list **may mint inline** through the
  declare-and-contest — but only after resolving against open epics: any **membership overlap
  with an existing epic surfaces to the user** (join it / rule on the overlap), never a silent
  duplicate.
- **`/mochiko:specify`** selection may **propose** an epic (when one derivation spans
  capabilities), **never mint** one.

## Selection-scope only

Every member enters as **selection scope** — a spec-accepted selection or growth rows.
**Delta-scope cards** (bug/improvement deltas on delivered capabilities) **cannot join an
epic**; the graduation-shaped close (below) is thereby correct by constraint, not presumption.

## Close semantics

One acceptance landing executes **each member's graduation batch** (extent fold, status
`delivered`, the store's in-flight-class elements flipped `built` and their `FEAT-XXX` keys
cleared) **plus the epic close**: the `[EPIC-XXX]` markers vanish
with the folded rows, the manifest is **stamped `delivered` + dated**, and the **directory
stays in place as record** — no move, no trail file.

A member that **exhausts its attempt bound or hits the no-progress stop** halts
**member-scoped**. The disposition — **carve the member out** (its rows return to `pending`, the
epic continues, manifest status `closed-partial`) or **hold the whole run** — is **reserved to
the user**, never the lead's: carve-out breaks the one-unit promise.

## Red Flags — STOP

- "I'll re-mint the epic here" — mint-once; resolve `EPIC-XXX` by lookup, never a second mint
- "This feature is already in another epic — I'll add it here too" — one open epic per feature's
  pending rows; surface the overlap to the user, never a silent second home
- "Specify found a multi-capability derivation — I'll mint the epic now" — specify **proposes**,
  never mints; the desk or a plan invocation mints
- "A member failed — I'll carve it out to keep the run moving" — carve-out is the user's ruling,
  never the lead's

## Related

- `mochiko:authoring-feature-map` — owns the `[EPIC-XXX]` map marker + within-epic seam-owner
  grammar; this skill names the owner in the spine, the map writes it
- `mochiko:patterns-transport-floor` — the composition steer for epic shared-write surfaces
- `plan.md` / `implement.md` — consume the spine; reference this skill, never restate it
- `/mochiko:feature` — the desk stewarding the epic's life (mint, membership, status, close)

## Quality Checklist

- [ ] Epic resolved by lookup — no re-mint; membership overlap with an open epic surfaced to the user
- [ ] Home `.mochiko/epics/EPIC-XXX/` with `manifest.md` (members, status, why-together) + spine files
- [ ] Every cross-member seam names its owner in the spine (no later-lander default inside an epic)
- [ ] Shared baseline (2+ members) = one spine delta under a single pen-holder; single-member baselines stay per-feature
- [ ] Transport-floor steer applied to every shared-write surface, disclosed at run open
- [ ] Every member is selection scope — no delta-scope card admitted
- [ ] Close = each member's graduation batch + epic close; markers vanish, manifest stamped delivered+dated, dir persists
- [ ] Member-scoped halt disposition (carve-out / hold) left to the user
