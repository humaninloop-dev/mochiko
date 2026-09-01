---
name: patterns-architecture-shelves
description: This skill MUST be invoked when dealing an architecture shelf at the desk — walking every dimension of the scoped shelf in retrofit-cost order, recommend-then-arbitrate, so the user forms a stance per row (decided / not-now + trigger / n-a + reason / open). Owns the breadth invariant and the three-strata precedence: floor-asserted categories bind, cards bind code, the desk governs stance. SHOULD also invoke on 'architecture shelf', 'shelf walk', 'stance', or 'not-now'.
---

# Architecture Shelves — The Opinion Carrier

**The value is breadth. Cheap to say "not a concern"; expensive to never have asked.**

## Overview

A **shelf** is an exhaustive list of the architecture dimensions a surface type has to have an
answer for — deliberately well past a dozen rows. Walking one is how a product gets a stance on
tenancy, billing, and observability without waiting for a feature to force the question. The
opinions live in **data**; the judgment lives here.

Full-stack and monorepo projects **compose** shelves — walk each surface's shelf, one store.

## Rules — load the schema first

Your first action, before any shelf walk: **Read `schema.yaml` (this skill's own directory)
raw, in full, as a declared first action.** The schema is the source of truth for this skill's
binding rules, nested in six sections, each addressable by its section ID:
`patterns-architecture-shelves.sec.trigger` · `patterns-architecture-shelves.sec.scope` ·
`patterns-architecture-shelves.sec.discipline` · `patterns-architecture-shelves.sec.inputs` ·
`patterns-architecture-shelves.sec.disclosure` ·
`patterns-architecture-shelves.sec.reserved`. Interpret it live: a rule's `kind:` names what
it is, and an absent `kind:` reads `constraint`; a rule of `class: floor` is always read and
always delivered; a `pointer:` rule binds you to that file's or skill's procedure, referenced
never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. The floor pin:
the 5 rules of `class: floor` are non-waivable. Before the first walk step, state the floor
count back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

## Walk order — the poles, taught

Retrofit cost is the whole ordering principle:

- **Early** — tenancy, identity and auth, data partitioning. Getting these wrong means a rewrite.
- **Late** — feature flags, experimentation, and anything a team can adopt in an afternoon.

Where a row sits between the poles is judgment, made fresh against the project in front of you.

## Stance vocabulary, taught

| Stance | Means | Carries |
|--------|-------|---------|
| `decided` | a ruling exists | the ruling + its rationale |
| `not-now` | real concern, consciously deferred | an **upgrade trigger** — the condition that reopens it |
| `n-a` | permanently dismissed | a **reason axis** |
| `open` | walked, no stance formed | nothing — the health view counts it |

`not-now` rows are the time bombs worth caring about: a deferral with no trigger is just a row
nobody will look at again. Push for the trigger while the reasoning is fresh.

*Considered and declined against the breadth invariant:* fact-triggered rows (only surface a
row when the project's facts suggest it) plus a visible unwalked list. It narrows breadth
quietly, which is the exact failure the invariant exists to prevent.
