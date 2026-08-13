---
name: patterns-map-minimalism
description: This skill MUST be invoked when the feature map gains or grooms a capability — PM derivation, spec review, `/mochiko:feature` grooming at cap-trip — running the ranked capability tests (system's-language · durability · new-kind; noun+verbs only aids), extend-beats-mint, soft cap ~9, and merge mechanics. SHOULD also invoke on 'is this a capability', 'extend beats mint', 'feature map growth', or 'merge capabilities'. Single source of the map-minimalism discipline; sibling of the plan/code skills.
---

# Map Minimalism — The Fewest Honest Capabilities

**The fewest capabilities that still tell the truth about the product.**

## Overview

The map's durable layer is its **capabilities** — the entries called "feature": what the product
does, in its own language, permanent. Map minimalism keeps that layer honest and few, firing at
the **PM's derivation** (specify), the **spec reviewer's** grade, and **`/mochiko:feature`
grooming** at cap-trip.

**Few is not sparse** — fewness never hides a capability that passes the tests; the map is small
because the product honestly does few kinds of things, never to sit under the cap.

**Boundary:** this skill owns the discipline; the capability/work-row vocabulary, file shapes, and
fold mechanism live in `mochiko:authoring-feature-map`.

## When NOT to Use

- **Work rows** — the transient delivery increments under a capability may be story-shaped; the
  tests grade the durable layer, not the rows.
- **A mint, merge, or retire ruling** — the skill shapes the recommendation; the capability layer
  is the user's to rule, never self-executed.
- **Plan or code sizing** — the two siblings, other altitudes.

## The capability tests

A candidate earns a capability only when the **three governing tests all hold** — read the current
map first, since "new kind" is claimed against what is already there.

1. **System's language** — names what the *product does*, in its own words, never who wanted it or
   why. A user's request is a story, not a capability.
2. **Durability** — still true and meaningful after every current story ships and is forgotten. (kinako's
   "durability and resumption" fails — a quality of one story's moment; "Corpus" passes — the
   product still has a corpus once those stories are gone.)
3. **New-kind-vs-more-of-same** — a *new* capability only when the product does a new *kind* of
   thing; more of a kind it already does extends, not mints.

**noun + verbs is a heuristic aid, never a gate** — a new verb on an existing noun signals
extend-not-mint, but it never blocks a cross-cutting capability (an onboarding journey) that
passes the governing tests.

## Extend beats mint

A new story **grows an existing capability first** (the new-kind test decides); a mint must argue
against extending. An unargued mint is the story-mirroring defect the map exists to stop.

## Soft cap and grooming

Past roughly **nine top-level capabilities**, a grooming pass runs — merge lookalikes, retire dead
entries. The cap is a **trigger, never a hard block**: the map may exceed nine when the product
honestly does that many kinds of things. Merge and retire are always **PM recommendations; the
user rules**. Within-capability roll-up is automatic — rows fold into extent at delivery landing
(`mochiko:authoring-feature-map`'s).

## Merge mechanics

When a user ruling merges two capabilities, the merge **preserves**:

- the **absorbing ID survives**; the merged entry flips **`retired`**, dated, with a
  **merged-into pointer** — never deleted;
- **extents union under an honesty pass** — no flattering over-claim;
- **story traces and SC references consolidate** onto the absorbing entry;
- **pending rows and unfolded deltas transfer** — no obligation dropped.

Re-parenting under a domain header is navigation-only — no status semantics.

## Domains (dormant)

Domains are the grouping tier: parts of the product's world owning their own nouns and rules
(Sessions, Knowledge). Every capability lives in **exactly one** domain, minted **only at
cap-trip** — a small map stays flat; the **PM proposes** names and the **principal-architect
co-signs** — a disagreement is an early design conversation, not a defect. **The `ARCHITECTURE.md` domain-to-components mapping line and the principal-architect's
co-sign duty are deferred to the first real cap-trip — neither is built until a live map actually
approaches the cap.**

## Sibling

`mochiko:patterns-plan-minimalism` (plan) and `mochiko:patterns-code-minimalism` (code) run the
same discipline at their altitudes.

## Quality Checklist

- [ ] Every capability passes all three governing tests (system's-language · durability · new-kind)
- [ ] noun+verbs used only as an aid, never to block a passing cross-cutting capability
- [ ] Every mint argued against extending an existing capability
- [ ] Past the soft cap ~9, grooming proposed as recommendations; the user rules merge/retire
- [ ] A merge preserved every mechanic above — ID survives, merged entry retired never deleted, honest union, traces/SCs + rows/deltas carried
- [ ] Domains flat below cap-trip; no ARCHITECTURE.md mapping line or co-sign machinery before first cap-trip
