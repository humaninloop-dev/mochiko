---
name: authoring-architecture-store
description: This skill MUST be invoked for any write to the product architecture store at `.mochiko/product/architecture/` — spine and `AX-XXX` row grammar, element lifecycle statuses, the derived repo-root `ARCHITECTURE.md` index (single writer), landing folds plus the built-vs-approved landing diff, scoped drift probes, the orphan rule, and first-visit migration. SHOULD also invoke on 'architecture store', 'AX-XXX', 'architecture spine', 'health view', 'architecture drift', or 'built vs approved'.
---

# Authoring the Architecture Store

**One home. One writer. Intent and built state on the same surface.**

## Overview

The product's architecture lives in one place: `.mochiko/product/architecture/`. Two layers ride
together — a **topology spine** (containers, boundaries, communication styles) and a **concern
catalog** (`AX-XXX` rows: auth, tenancy, billing, observability, …). A concern row may pin to a
spine element or apply product-wide. Neither layer stands alone: concerns without structural
homes float, and a topology with no concerns says nothing about the product.

Every element carries a lifecycle status, so intent and built state sit on one surface. That kills
the **artifact** diff — there is no separate "approved" document to reconcile against a "current"
one. It does not kill the **code-vs-claim** diff: what the store says was built can still be
false, and that is exactly what the scoped drift probe exists for.

The shape is schema-backed: invoke `mochiko-cli template architecture-store`, or Read
`plugins/mochiko/schemas/architecture-store.yaml` raw when the binary is absent.

## When NOT to Use

- **Feature-level diagram craft** — altitude, container-delta diagrams, which flows earn a
  sequence diagram: `mochiko:patterns-system-design`. This skill says where a delta lands and
  what status it carries, never how to draw it.
- **Forming a stance** — which stance a row takes, what walk order applies, what a floor card
  binds: `mochiko:patterns-architecture-shelves`. This skill owns the field; that skill owns the
  judgment that fills it.
- **Minting capabilities or work rows** — a fired upgrade trigger is flagged here and routed to
  the `/mochiko:feature` growth door. The architecture lens proposes; the map machinery disposes.

## Store layout

| File | Holds | Hand-written? |
|------|-------|---------------|
| `.mochiko/product/architecture/spine.md` | topology deep view | yes |
| `.mochiko/product/architecture/concerns.md` | the `AX-XXX` ledger | yes |
| `.mochiko/product/architecture/concerns/AX-XXX-<slug>.md` | a graduated concern | yes, when one exists |
| `ARCHITECTURE.md` (repo root) | the derived index | **no — regenerated** |

The root index stays at the repo root, never inside the store directory.

`spine.md` opens with a **`Scope:` line** naming the surface types the store covers (e.g.
`Scope: backend-service`) — written by setup's scaffold, overridable at the desk. This is shelf
scope's durable home: the shelf walk reads it rather than re-asking.

## Element grammar

**Spine elements** carry `SPN-XXX` (unique store-wide), a kind — `container` · `boundary` ·
`flow` — a name, and a status. **Concern rows** carry `AX-XXX` (unique store-wide), a name, a
`Stance`, and a `Status`. Everything past that core is free-form: the schema constrains the
skeleton, never the voice.

`NFR-XXX` targets live **on the concern row they belong to** — one home per concern, stance and
pattern and target and as-built together. The ids survive unchanged; only the path moved, so
`TR-XXX → NFR-XXX` trace chains keep resolving.

`Work:` holds pointers only (`FEAT-XXX`, `EPIC-XXX`). The work itself lives on the feature map.

**Present tense, no history.** Spine and rows are written in the present tense, with no narration
of how the system got here ("we used to…", "changed in FEAT-009…"). The lifecycle statuses carry
the change story; the trail lives in the rulings and the landings, not in the prose. A row's own
`Ruling` and `Rationale` **are** the ruling and belong here; rationale that already lives in a
decision record or landing report does not — link it, never restate it.

## Element lifecycle — five statuses

```
ruled → in-flight (FEAT-XXX) | modifying (FEAT-XXX) | removing (FEAT-XXX) → built
```

- **`ruled`** — not in flight, not yet built. The resting state of *any* element, whatever its
  stance: a row sitting at `open` or `n-a` is `ruled` too. Status tracks the build lifecycle;
  stance is a separate axis and never implies one.
- **in-flight-class** — the three keyed statuses, named collectively. Each MUST name the feature
  that owns the change.
- **`built`** — the change shipped; `As-built:` says what actually exists.

**Ruled truth is never edited in place by a plan run.** A plan-time write is legal only as an
in-flight-class delta, and only after the user's sign-off on the rendered diagram plus the named
row changes — the sign-off IS the write gate. No sign-off, no store write.

## The derived index and the health view

The root `ARCHITECTURE.md` is a **projection of the store, not a second store**. Regenerate it on
**every** store write — this skill is its single writer. It is never hand-maintained, and
index-vs-store disagreement is a defect fixed by regenerating, never by editing the index.

It carries the spine thumbnail, the **full** AX summary table (every row — plan runs read the
trip check here, so a missing row is an invisible row), and **Health**. The health view is a
section of this index and **no separate artifact exists**. Five counts, each naming its rows:

1. **Open rows** — walked, no stance formed.
2. **Stale `not-now` triggers** — deferrals whose revisit condition has gone unreviewed.
3. **Fired triggers awaiting routing** — the pull half: each routes to the `/mochiko:feature`
   growth door, where the user rules the mint. Flagged here, minted nowhere.
4. **Orphan elements** — see the orphan rule below.
5. **Drift register** — rows whose `Drift:` field names a live divergence.

**Readability bar:** a reader new to the repo can place any file or component in the system from
the index alone. If they cannot, the index is too thin — regenerate it with more of the spine, not
with hand-written prose.

## Graduation — ledger first

A row starts compact in `concerns.md` and earns its own file only on **real depth**: long
rationale, local diagrams, a decision trail a few lines cannot hold. Extend beats mint. When a row
graduates, the ledger keeps it as a one-line summary plus a link — the row never vanishes from the
ledger.

## Landing duties — two triggers, never conflated

| Duty | Fires when | Output |
|------|-----------|--------|
| **Landing diff** | an **approved delta existed** for the feature — independent of what was built | built-vs-approved: "built as approved", or the named divergence |
| **Fold** | the built work **changed structure** | statuses flipped, `As-built:`/`Drift:` updated, index regenerated |

The triggers differ deliberately. An approved delta that was **descoped or silently dropped** built
nothing structural, so the fold does not fire — but the **diff does**, and it is the only thing
that catches the missing change at the landing instead of weeks later by the orphan sweep. **Never
gate the diff on whether structure was built.**

The diff runs **both directions**: was every signed add, remove, redirect, and boundary-move
actually built — *and* was anything structural built that the signed delta did not carry? Grading
built topology against the signed delta is a capability **this duty owns**, taking that delta as
its input; it is not an assumed pre-existing one. The diff **reports**: each difference is named,
approved-versus-built, and dispositioned by the user. It never silently reconciles.

The fold itself: flip in-flight-class elements to `built`, clear their FEAT-XXX keys, update
`As-built:` and `Drift:` on every touched row, regenerate the index.

## Drift — evidence, not memory

`As-built:` claims are checked against **actual code**, never against recollection. The probe is
**scoped**: rows touched since the last desk visit, plus a sample of the retrofit-expensive rows.
Never all rows every visit — an unscoped probe is the one that stops being run.

Findings land in the row's `Drift:` field and take a **user disposition** at the desk: the store
bends to the code, the code bends to the store, or the divergence is accepted and recorded. The
probe reports; it never silently reconciles.

## The orphan rule

**Every in-flight-class element keys an open feature.** An element whose `FEAT-XXX` names a
closed, retired, or nonexistent feature is an orphan: the health view flags it, and desk visits
clean it. This is the store's replacement for a hand-maintained in-flight pointer list — the
status field already carries the key, so nothing needs to be kept in sync by hand.

## First visit to an existing repo — reconstruct and confirm

The first desk visit to a repo with no ruled store content **reconstructs one from what exists**,
then confirms it. Never fails, never silently invents.

1. **Read** what the repo already says: `ARCHITECTURE.md` prose, any per-feature
   `architecture.md`, `nfrs.md`, structural decision records.
2. **Reconstruct** the spine and the AX rows from it, marking every reconstructed element as such
   — derived, not ruled.
3. **Confirm** with the user before the store becomes truth. A derived baseline inherits the
   brownfield caveat: a partial or wrong read poisons everything downstream, so the confirmation
   is a real gate, not a formality.
4. **Archive** the absorbed sources to `.mochiko/archive/product-baselines/<date>/`. Nothing is
   silently discarded.

A pipeline run that meets a store with no ruled content — scaffold-only or absent — **offers the
bootstrap** rather than failing.

## Who grades a store write

Judgment writes — baseline authoring, shelf-walk stance batches, amendments, and every
`As-built:`/`Drift:` write — take the independent review leg before the user's ruling, per
`mochiko:patterns-sound-loop`. Status flips and orphan cleanup are transcription and ride the
landing audit. Do not relabel judgment as mechanical to skip the leg.

## Quality checks

- [ ] Every element carries its required core: id, kind/name, status — and a stance on concern rows
- [ ] Ids unique store-wide; every `Work:`, NFR, and handled-elsewhere pointer resolves
- [ ] Every in-flight-class element names its FEAT-XXX; `ruled`/`built` carry no key
- [ ] No store write without its user sign-off; ruled truth never edited in place by a plan run
- [ ] Index regenerated on this write — it agrees with the store, and the AX table is complete
- [ ] Landing diff ran because an approved delta existed, not because structure was built
- [ ] Diff ran both directions — every signed change accounted for, and nothing structural was built unreported
- [ ] Present tense throughout; no history narration, no rationale restated from a decision record
- [ ] Index passes the readability bar — a new reader can place any component from it alone
- [ ] `As-built:` claims checked against code; drift findings dispositioned, not silently reconciled
- [ ] Health view's five counts current, each naming its rows
- [ ] Graduation earned by real depth; the ledger still carries the row
