---
name: patterns-map-minimalism
description: This skill MUST be invoked when the feature map gains or grooms a capability — PM derivation, spec review, `/mochiko:feature` grooming at cap-trip — running the ranked capability tests (system's-language · durability · new-kind; noun+verbs only aids), extend-beats-mint, soft cap ~9, and merge mechanics. SHOULD also invoke on 'is this a capability', 'extend beats mint', 'feature map growth', or 'merge capabilities'. Single source of the map-minimalism discipline; sibling of the plan/code skills.
allowed-tools: Bash(mochiko-cli *)
---

# Map Minimalism — The Fewest Honest Capabilities

**The fewest capabilities that still tell the truth about the product.**

## Overview

The map's durable layer is its **capabilities** — the entries called "feature": what the product
does, in its own language, permanent. Map minimalism keeps that layer honest and few.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-map-minimalism · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-map-minimalism · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-map-minimalism --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-map-minimalism --section patterns-map-minimalism.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-map-minimalism --section patterns-map-minimalism.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-map-minimalism --section patterns-map-minimalism.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-map-minimalism --section patterns-map-minimalism.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-map-minimalism --section patterns-map-minimalism.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-map-minimalism --section patterns-map-minimalism.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

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
