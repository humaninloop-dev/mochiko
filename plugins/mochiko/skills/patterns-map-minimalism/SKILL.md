---
name: patterns-map-minimalism
description: This skill MUST be invoked when the feature map gains or grooms a capability — PM derivation, spec review, `/mochiko:feature` grooming at cap-trip — running the ranked capability tests (system's-language · durability · new-kind; noun+verbs only aids), extend-beats-mint, soft cap ~9, and merge mechanics. SHOULD also invoke on 'is this a capability', 'extend beats mint', 'feature map growth', or 'merge capabilities'. Single source of the map-minimalism discipline; sibling of the plan/code skills.
---

# Map Minimalism — The Fewest Honest Capabilities

**The fewest capabilities that still tell the truth about the product.**

## Overview

The map's durable layer is its **capabilities** — the entries called "feature": what the product
does, in its own language, permanent. Map minimalism keeps that layer honest and few.

## Rules — load the schema first

Your first action, before any capability judgment: **Read `schema.yaml` (this skill's own
directory) raw, in full, as a declared first action.** The schema is the source of truth for
this skill's binding rules, nested in six sections, each addressable by its section ID:
`patterns-map-minimalism.sec.trigger` · `patterns-map-minimalism.sec.scope` ·
`patterns-map-minimalism.sec.discipline` · `patterns-map-minimalism.sec.inputs` ·
`patterns-map-minimalism.sec.disclosure` · `patterns-map-minimalism.sec.reserved`. Interpret
it live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a rule
carrying `when:` binds only where its terms hold against the schema's declared `conditions:`,
except that a `class: floor` rule is always read and always delivered — `when:` gates when its
obligation applies, never whether it reaches you; a `pointer:` rule binds you to that file's
or skill's procedure, referenced never restated; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. The floor pin: the 3 rules of `class: floor` are
non-waivable. Before the first capability-judgment step, state the floor count back — a
skipped or partial read leaves that count blank: halt and surface it, and halt likewise if the
schema's `class: floor` count disagrees with the pin.

## The capability tests, taught

1. **System's language** — names what the *product does*, in its own words, never who wanted it
   or why. A user's request is a story, not a capability.
2. **Durability** — still true and meaningful after every current story ships and is forgotten.
   (kinako's "durability and resumption" fails — a quality of one story's moment; "Corpus"
   passes — the product still has a corpus once those stories are gone.)
3. **New-kind-vs-more-of-same** — a *new* capability only when the product does a new *kind* of
   thing; more of a kind it already does extends, not mints.

## Domains, taught

Domains are parts of the product's world owning their own nouns and rules (Sessions,
Knowledge). A PM/architect disagreement over a domain name is an early design conversation,
not a defect.
