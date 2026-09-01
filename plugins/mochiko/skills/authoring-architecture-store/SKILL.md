---
name: authoring-architecture-store
description: This skill MUST be invoked for any write to the product architecture store at `.mochiko/product/architecture/` — spine and `AX-XXX` row grammar, element lifecycle statuses, the derived repo-root `ARCHITECTURE.md` index (single writer), landing folds plus the built-vs-approved landing diff, scoped drift probes, the orphan rule, and first-visit migration. SHOULD also invoke on 'architecture store', 'AX-XXX', 'architecture spine', 'health view', 'architecture drift', or 'built vs approved'.
---

# Authoring the Architecture Store

**Intent and built state on the same surface.**

## Overview

The product's architecture lives in one place. Two layers ride together — a **topology
spine** (containers, boundaries, communication styles) and a **concern catalog** (`AX-XXX`
rows: auth, tenancy, billing, observability, …). A concern row may pin to a spine element or
apply product-wide. Neither layer stands alone: concerns without structural homes float, and
a topology with no concerns says nothing about the product.

Every element carries a lifecycle status, so intent and built state sit on one surface. That
kills the **artifact** diff — there is no separate "approved" document to reconcile against a
"current" one. It does not kill the **code-vs-claim** diff: what the store says was built can
still be false, and that is exactly what the scoped drift probe exists for.

## Rules — load the schema first

Your first action, before any store touch: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same
declared first action** — schema, then common. The schema is the source of truth for this
skill's binding rules, nested in six sections, each addressable by its section ID:
`authoring-architecture-store.sec.independence` · `authoring-architecture-store.sec.scope` ·
`authoring-architecture-store.sec.inputs` ·
`authoring-architecture-store.sec.artifact` · `authoring-architecture-store.sec.output` ·
`authoring-architecture-store.sec.reserved`. Interpret it live: a rule's `kind:` names what
it is, and an absent `kind:` reads `constraint`; a rule carrying `when:` binds only where
its terms hold against the schema's declared `conditions:`, except that a `class: floor`
rule is always read and always delivered — `when:` gates when its obligation applies, never
whether it reaches you; a `pointer:` rule binds you to that file's or skill's procedure,
referenced never restated; `${var}` substitutes from this schema's `vars:` at read time;
labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: authoring-common.<slug>` inherits text/labels/pointer from
`skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `authoring-architecture-store.*` ID stays the citable ID. The floor pin: the
9 rules of `class: floor` are non-waivable. Before the first store-touching step, state the
floor count back — a skipped or partial read leaves that count blank: halt and surface it,
and halt likewise if the schema's `class: floor` count disagrees with the pin.

## Store layout

| File | Holds | Hand-written? |
|------|-------|---------------|
| `.mochiko/product/architecture/spine.md` | topology deep view | yes |
| `.mochiko/product/architecture/concerns.md` | the `AX-XXX` ledger | yes |
| `.mochiko/product/architecture/concerns/AX-XXX-<slug>.md` | a graduated concern | yes, when one exists |
| `ARCHITECTURE.md` (repo root) | the derived index | **no — regenerated** |

The `Scope:` line at the top of `spine.md` (e.g. `Scope: backend-service`) is written by
setup's scaffold and overridable at the desk.

## Element lifecycle

```
ruled → in-flight (FEAT-XXX) | modifying (FEAT-XXX) | removing (FEAT-XXX) → built
```

- **`ruled`** — not in flight, not yet built. The resting state of *any* element, whatever
  its stance: a row sitting at `open` or `n-a` is `ruled` too.
- **in-flight-class** — the three keyed statuses, named collectively.
- **`built`** — the change shipped; `As-built:` says what actually exists.

## Landing duties — two triggers, never conflated

| Duty | Fires when | Output |
|------|-----------|--------|
| **Landing diff** | an **approved delta existed** for the feature | built-vs-approved: "built as approved", or the named divergence |
| **Fold** | the built work **changed structure** | statuses flipped, `As-built:`/`Drift:` updated, index regenerated |

The triggers differ deliberately. An approved delta that was **descoped or silently
dropped** built nothing structural, so the fold does not fire — but the diff does, and it is
the only thing that catches the missing change at the landing instead of weeks later by the
orphan sweep.

## Drift — evidence, not memory

The probe's scoping exists so it keeps being run — an unscoped probe is the one that stops
being run. The orphan rule is the store's replacement for a hand-maintained in-flight
pointer list: the status field already carries the key, so nothing needs to be kept in sync
by hand.

## First visit to an existing repo — reconstruct and confirm

1. **Read** what the repo already says: `ARCHITECTURE.md` prose, any per-feature
   `architecture.md`, `nfrs.md`, structural decision records.
2. **Reconstruct** the spine and the AX rows from it.
3. **Confirm** with the user before the store becomes truth.
4. **Archive** the absorbed sources.
