---
name: patterns-architecture-shelves
description: This skill MUST be invoked when dealing an architecture shelf at the desk — walking every dimension of the scoped shelf in retrofit-cost order, recommend-then-arbitrate, so the user forms a stance per row (decided / not-now + trigger / n-a + reason / open). Owns the breadth invariant and the three-strata precedence: floor-asserted categories bind, cards bind code, the desk governs stance. SHOULD also invoke on 'architecture shelf', 'shelf walk', 'stance', or 'not-now'.
allowed-tools: Bash(mochiko-cli *)
---

# Architecture Shelves — The Opinion Carrier

**The value is breadth. Cheap to say "not a concern"; expensive to never have asked.**

## Overview

A **shelf** is an exhaustive list of the architecture dimensions a surface type has to have an
answer for — deliberately well past a dozen rows. Walking one is how a product gets a stance on
tenancy, billing, and observability without waiting for a feature to force the question. The
opinions live in **data**; the judgment lives here.

Full-stack and monorepo projects **compose** shelves — walk each surface's shelf, one store.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules patterns-architecture-shelves · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · patterns-architecture-shelves · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules patterns-architecture-shelves --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-architecture-shelves --section patterns-architecture-shelves.sec.trigger --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-architecture-shelves --section patterns-architecture-shelves.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-architecture-shelves --section patterns-architecture-shelves.sec.discipline --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-architecture-shelves --section patterns-architecture-shelves.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-architecture-shelves --section patterns-architecture-shelves.sec.disclosure --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules patterns-architecture-shelves --section patterns-architecture-shelves.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

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
