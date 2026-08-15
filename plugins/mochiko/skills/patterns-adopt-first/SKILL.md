---
name: patterns-adopt-first
description: This skill MUST be invoked at a plan decision in a commodity category (storage, locking, serialization, queueing, caching, auth, search) — the alternatives name a real off-the-shelf candidate or state none exists; custom wins only in writing against it. SHOULD also invoke on 'build vs buy', 'off-the-shelf', 'should we build this ourselves', 'shelf candidate', 'hand-rolled'. In-process/self-hostable only; SaaS buy is an IP-XXX call. Governs CHANGING the stack; `analysis-codebase` describes it.
---

# Adopt First — Build vs Off-the-Shelf at Plan Time

**A problem older than the product has probably already been solved.**

## Overview

The minimalism ladders ask whether a piece of the system should exist and how small it can be.
This discipline asks a different question of the pieces that survive: **must we build this one
ourselves?** It fires at plan time, where whole mechanisms are still on the table, and it binds
the D-XXX decision: name a real off-the-shelf candidate, and beat it in writing before
choosing custom.

The discipline is **weigh and disclose**, never adopt-always: the named candidate may lose on
merits. What may not happen is that it was never named at all.

**Scope bound.** Plan seats own **in-process libraries and self-hostable components**. Buying a
**managed service, a SaaS product, or a whole capability** is a business call: it routes to an
IP-XXX provisioning requirement and the PM/user, never here.

## When NOT to Use

- **The product's differentiating domain** — what the product exists to do is presumptively
  not a commodity; the test is for the infrastructure underneath.
- **Managed-service / SaaS / whole-capability buy** — out of scope per the bound above.
- **Build time** — cards already carry the plan's commitment; the code ladder
  (`mochiko:patterns-code-minimalism`) shapes code, it does not reopen the mechanism.
- **Project tooling defaults** — "established, never hand-rolled" for linters, CI, and build
  tooling is governance-floor doctrine, not a plan decision (see Siblings).

## The trigger — the commodity-category test

Ask of the capability being decided: **is this problem older than this product?**

Presumptively yes for infrastructure categories — **storage · locking · serialization ·
queueing · caching · auth · search**. That list is a starting set, not a catalog: judgment
extends it; absence from the list is not an exemption.

**The author's framing never gates the check.** A decision framed as "record granularity" or
"which replace primitive" is still a storage decision, still a locking decision. The
canonical miss was itself a framing artifact — a storage engine framed as a serialization
choice, and the shelf question never got asked.

## The two-part obligation

**1 — Disclosure floor.** Every commodity-category decision's alternative set names **at least
one real off-the-shelf candidate**, by product name, or carries an explicit **"no shelf
candidate exists"** line. A missing line is itself a review finding: silence is not neutral.

**2 — Rationale bite.** Choosing custom is legitimate **only against the named candidate** —
the choice rationale says, in writing, why custom beats it. A shelf row that sits in the
options table and is then dropped from the rationale without a word is the defect this clause
exists to catch.

## Custom can win — the two-sided limb

The candidate is weighed, not deferred to. Below the retrofit-cost line, a capability
**reasonably implementable in-house at under 100 lines** is a legitimate custom-wins
rationale — the same red flag the backend dependency-discipline standard already names (BE-DEP,
[../authoring-constitution/references/catalog/backend-service.md](../authoring-constitution/references/catalog/backend-service.md)),
read symmetrically. It argues against adopting a trivial dependency exactly as this discipline
argues against hand-rolling a solved one — no precedence conflict; both say weigh it.

## Naming a candidate is an external claim

A named candidate is a claim about the world outside the repo, so it carries the
external-research disclosure line — `verified: <source>` or `memory-asserted` — verified at
review. Grammar and mechanics live in
[../review-brainstorm/references/EXTERNAL-CLAIMS.md](../review-brainstorm/references/EXTERNAL-CLAIMS.md)
— the single source; never restated here.

## The retrofit-cost gate — who rules custom-over-shelf

**User-ruled when the custom build is expensive to walk back** — persisted formats, storage
engines, locking/concurrency primitives, migration-bearing shapes. **Seat-decidable with
disclosure** below that line. The split criterion is retrofit cost, deliberately the same
principle the adaptive-depth ruling ratified.

## Collision with a ratified constraint — route back, never override

A plan seat never silently overrides a ratified upstream constraint. When the shelf check
collides with one — a spec constraint that excludes the candidate — the seat MUST file a
**constraint-challenge finding** carrying three parts:

1. the constraint's **stated text**, quoted;
2. the **real requirement** it plausibly restates;
3. the **shelf candidate** it excludes.

The collision routes to the **user**, who rules: amend the spec, or sustain the constraint.
Only the colliding decision pauses — the plan proceeds elsewhere. These are expensive
decisions; a human rules them.

## Who grades what

- **Disclosure line** — `mochiko:review-plan-artifacts`: **BLOCKING**, a named sibling check
  at conformance strength.
- **Needed but should have been adopted** — `mochiko:review-feasibility` hunt class 7
  (remove-shaped excess): **blocking-capable**.
- **Rationale-bite honesty** — advisory, on the existing rung-honesty lane.
- **The retrofit-cost gate** — user-ruled by construction; no seat clears it.

## Siblings

`mochiko:patterns-plan-minimalism` (its rung 3 reads adoption as a way something already
exists), `mochiko:patterns-code-minimalism`, `mochiko:patterns-map-minimalism` — the ladders
size the artifact; this discipline asks who should have built it. The governance floor's
"established, never hand-rolled" tooling doctrine (home queued at
`authoring-constitution/references/STACK-TOOLING.md`) is the governance-axis sibling: that
home carries tooling defaults, this skill the plan-time decision — no merge, cross-pointers
only.

## Quality Checklist

- [ ] Commodity test run per decision, against the capability — never the author's framing
- [ ] Each qualifying alternative set names a real candidate, or states no shelf candidate exists
- [ ] Every named candidate carries `verified: <source>` or `memory-asserted`
- [ ] Choosing custom argues against the named candidate, by name, in the rationale
- [ ] Retrofit-cost gate evaluated — above the line the user ruled, below it disclosure present
- [ ] Constraint collisions filed as three-part findings and routed to the user, never absorbed
- [ ] Scope bound honored — managed-service/SaaS buy sent to IP-XXX, not decided here
