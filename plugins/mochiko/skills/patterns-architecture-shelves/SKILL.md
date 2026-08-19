---
name: patterns-architecture-shelves
description: This skill MUST be invoked when dealing an architecture shelf at the desk — walking every dimension of the scoped shelf in retrofit-cost order, recommend-then-arbitrate, so the user forms a stance per row (decided / not-now + trigger / n-a + reason / open). Owns the breadth invariant and the three-strata precedence: floor-asserted categories bind, cards bind code, the desk governs stance. SHOULD also invoke on 'architecture shelf', 'shelf walk', 'stance', or 'not-now'.
---

# Architecture Shelves — The Opinion Carrier

**The value is breadth. Cheap to say "not a concern"; expensive to never have asked.**

## Overview

A **shelf** is an exhaustive list of the architecture dimensions a surface type has to have an
answer for — deliberately well past a dozen rows. Walking one is how a product gets a stance on
tenancy, billing, and observability without waiting for a feature to force the question.

The opinions live in **data**; the judgment lives here. The backend shelf ships at
`plugins/mochiko/schemas/architecture-shelf-backend.yaml` — Read it raw. Stances land as `AX-XXX`
rows in the store (`mochiko:authoring-architecture-store` owns that grammar).

## When NOT to Use

- **Not a rigor dial** — the shelf never reads the governance low/high depth level. A low-depth
  project walks the same shelf as a high-depth one.
- **Not the store's file grammar** — row fields, ids, statuses, and the index belong to
  `mochiko:authoring-architecture-store`.
- **Not a per-feature design tool** — the shelf sets the product's standing posture; a feature's
  structural delta is `mochiko:patterns-system-design`.

## Shelves are dealt, never asserted

Every row is **recommend-then-arbitrate**: name the suggested default, say what it costs and what
would argue against it, then let the user rule. Architecture choice is per-project judgment, not a
standard — an asserted default here would be an opinion wearing a rule's clothes.

Every default is **memory-asserted**: common practice as the shelf's author knows it, not a
verified external claim. Say so when a row's default carries weight in the decision.

## Scope — from setup, overridable at the desk

The declared project type selects the shelves. **The desk can override it**: the user knows their
repo better than the setup answer does. Full-stack and monorepo projects **compose** shelves —
walk each surface's shelf, one store.

**Read the scope from `spine.md`'s `Scope:` line** — setup's scaffold wrote it there and the desk
overrides it there. That line is scope's durable home; do not re-ask the user at each walk.

Only the backend/service shelf is seeded today. Frontend, mobile, and desktop are **honest gaps,
not silent ones**: when the scoped surface has no seeded shelf, say so plainly, then lean on the
project's own facts and the universal floor's categories. Never deal a filtered backend list to a
non-backend surface — a frontend repo needs frontend depth, not backend leftovers.

## Walk order — retrofit cost first

Walk the shelf in order of **what costs most to retrofit**, not in file order:

- **Early** — tenancy, identity and auth, data partitioning. Getting these wrong means a rewrite.
- **Late** — feature flags, experimentation, and anything a team can adopt in an afternoon.

Retrofit cost is the whole ordering principle; where a row sits between the poles is judgment,
made fresh against the project in front of you.

## The breadth invariant

**Every row on the scoped shelf is walked.** A row may close in two seconds — "not a concern",
next — but it is never silently skipped. An unwalked row is invisible; a row walked and dismissed
is a recorded decision.

There is **no magnitude scaling**. A two-person MVP and a mature platform walk the same shelf. The
per-row stance is the magnitude valve, and `not-now` is the per-dimension depth control — the
shelf does not shrink for small projects.

*Considered and declined:* fact-triggered rows (only surface a row when the project's facts
suggest it) plus a visible unwalked list. It narrows breadth quietly, which is the exact failure
the invariant exists to prevent.

## Stance vocabulary

| Stance | Means | Carries |
|--------|-------|---------|
| `decided` | a ruling exists | the ruling + its rationale |
| `not-now` | real concern, consciously deferred | an **upgrade trigger** — the condition that reopens it |
| `n-a` | permanently dismissed | a **reason axis** (below) |
| `open` | walked, no stance formed | nothing — the health view counts it |

`n-a` is always written with its reason suffixed — `n-a — genuinely never` or `n-a — handled
elsewhere` — and the two are not interchangeable:

- **`genuinely never`** — the concern does not apply to this product, ever.
- **`handled elsewhere`** — the concern is real, and another repo, system, or team owns it. The
  **pointer to that owner is required**, not optional. Without it the row is an unowned concern
  wearing a dismissal.

`not-now` rows are the time bombs worth caring about: a deferral with no trigger is just a row
nobody will look at again. Push for the trigger while the reasoning is fresh.

## Three strata — what binds what

**1. Floor-asserted obligations bind shelf rows.** Where a universal-floor card asserts a
category, `n-a — genuinely never` is **unavailable** at the desk. Those are the rows carrying a
`floor_bound` value in the shelf data — identity and auth under `FLOOR-SEC`, for instance. Read
the data for the live set; never work from a remembered list. The legal moves are: a stance
**within** the obligation, `n-a — handled elsewhere` with its pointer, or narrowing the scope. A
genuine drop is a governance waiver, recorded in the ledger — never a shelf stance.

**2. Arbitrated constitution cards and shelf rows bind different things.** Cards bind code-layer
structure; shelf rows bind product topology and cross-cutting concerns. They usually miss each
other entirely. A genuine conflict is the user's to rule — do not resolve it at the desk.

**3. Two instruments, two axes — stated openly.** The governance depth dial governs how rigorous a
floor row's obligation is. The desk governs what stance the product takes. One dial must not
silently drive two systems, so it drives one and this skill drives the other.

## Shelf freshness

Revisit is **event-keyed, never calendrical**: after the desk skill's first live run, and
thereafter whenever a desk visit finds a default **argued against twice in one project** — that
signal means the default is stale or the segment moved. The store steward owns freshness.

## Quality checks

- [ ] Scope read from `spine.md`'s `Scope:` line and shown to the user for override, never silently assumed
- [ ] Missing shelves named as gaps; no filtered backend list dealt to another surface
- [ ] Every row on the scoped shelf walked — none skipped, no magnitude reasoning applied
- [ ] Walk ordered by retrofit cost, not file order
- [ ] Each row dealt recommend-then-arbitrate; the user ruled, the shelf did not
- [ ] Memory-asserted provenance disclosed where a default carried weight
- [ ] Every `not-now` carries an upgrade trigger; every `n-a` carries its reason axis
- [ ] Every `n-a — handled elsewhere` names the system that owns the concern
- [ ] No `n-a — genuinely never` on a floor-asserted category; drops routed to a ledger waiver
- [ ] Constitution-card conflicts routed to the user, not resolved at the desk
