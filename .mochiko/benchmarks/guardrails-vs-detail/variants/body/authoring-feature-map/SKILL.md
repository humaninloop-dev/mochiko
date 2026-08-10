---
name: authoring-feature-map
description: This skill MUST be invoked when deriving or updating the repo-level feature map — the living FEATURES.md index plus per-feature entry files at `.mochiko/features/FEAT-XXX-<slug>.md` — during a specify run or a /mochiko:feature stewardship touch: the intent-stage map-read agenda, deriving proposed features from drafted user stories (stories first, features derived), running the story filter, authoring or amending FEAT-XXX entries, nesting parent/leaf entries (leaf = pipeline unit, two-level hard cap, sticky-delivered roll-up), minting parents at derivation or by retroactive promotion, minting `unrefined` capability stubs, attaching marked deltas to delivered features, and staging map writes. SHOULD also invoke on "feature map", "FEATURES.md", "FEAT-XXX", "parent feature", "leaf feature", "promote to parent", "capability stub", "unrefined", "propose features", "feature derivation", "feature selection", "map delta", "in-flight feature", or verifying a reconstructed-from-code entry on first touch. Boundary: this skill authors and maintains the MAP — NOT user stories (mochiko:authoring-user-stories), NOT architecture views (links to ARCHITECTURE.md, never restated), NOT selection: which features build now is the user's ruling — this skill recommends. Capabilities and extent-growth ideas live on the map; defects, tooling, and process items stay in BACKLOG.md. Never grades its own output — graded with the spec by mochiko:review-specifications, an independent reviewer.
---

# Authoring the Feature Map

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

The feature map is the **broad view of the whole system expressed as features** — the primary capability lens on the product, the way `ARCHITECTURE.md` is the system viewed as components. Together the two are the central source of truth. A **feature is the built thing**: a capability of the system described in the system's own language — not a cluster or regrouping of user stories. Stories inform *which* features get built and sharpen their extents; they never define them.

There is **one living map, no per-spec copy**: a succinct repo-root [`FEATURES.md`](../../templates/features-index-template.md) index (one line per feature) pointing at per-feature entry files (default home: `.mochiko/features/FEAT-XXX-<slug>.md`, in the shape [`feature-entry-template.md`](../../templates/feature-entry-template.md) defines). The map has four touchpoints: **specify proposes** (this skill's core work), **plan confirms and hardens** alongside architecture, **implement's acceptance landing graduates** — status flips, delta folds, and index touches are bookkeeping edits inside that landing, never a separate close stage — and **`/mochiko:feature` stewards**: stub minting, retroactive promotion, retire, integrity grooming, plus lane intake for small feature-keyed work.

Feature delivery and spec delivery are **independent axes**: one spec can surface several features and deliberately build only a subset. The map is durable; specs are delivery events. This skill is the map judgment plus the entry authoring — the map-read agenda at intent, the derivation and filter after stories, entry and delta authoring, and the write rules. Density per the deliverable envelope ([`artifact-format.md`](../../templates/artifact-format.md)): capability statements 1–3 lines, extent and obligation entries one line each — an entry is a record, not an essay.

## Vocabulary — feature vs the units around it

| Term | Level | What it is | Owner |
|------|-------|------------|-------|
| **Feature — parent** | Product | The capability a product person names in one breath; navigation + status roll-up over its leaves, never built directly | **this skill** (map entry) |
| **Feature — leaf** | Pipeline unit | A deliverable built capability; graduates through plan/implement as its own unit (a flat entry is a leaf) | **this skill** (map entry) |
| **User story** | Spec | User value in user language; informs features, never defines them | `mochiko:authoring-user-stories` |
| **Vertical slice (cycle)** | Implementation, within one leaf | A test-first increment delivering one observable behavior | `mochiko:patterns-vertical-tdd` (downstream) |

## When NOT to Use

- **Grading the derivation or map delta** — graded with the spec by `mochiko:review-specifications` (independent reviewer, never the author)
- **Authoring or rewriting user stories** — entries trace stories by ID; a story that fits no feature is a filter verdict, not a rewrite
- **Authoring architecture** — the entry links to `ARCHITECTURE.md` components; it never restates the component view
- **Selecting which features build now** — the selection is the user's ruling; this skill prepares the selection card and recommends
- **Tracking defects, tooling, or process work** — those live in `BACKLOG.md` where KM exists (a non-KM product has no queue; lane runs accept direct requests — the stated degrade path, never silently assumed away). Extent-growth improvement ideas are the exception: they ride the map as `proposed` deltas or obligation lines — the map is the capability backlog.

## The invariants (hard rules)

1. **Exactly one home.** Every accepted story maps to exactly one feature. Other features it touches carry extend obligations on their entries — never a second home.
2. **Complete disposition.** Every drafted story is either homed to a feature or rejected by the filter with the why recorded in the story file. No silent drops, no orphans.
3. **Dependency closure.** A selected feature must be buildable given only the features ordered before it, per the map's relations. No forward dependencies.
4. **The map owns status.** `proposed / in-flight / delivered / retired` — one home, no copies. Story files derive status by following their FEAT-ID; the only story-native status is `rejected`.
5. **Delivered is sticky.** A later spec or lane run touching a `delivered` feature never regresses its status; the change rides as a marked delta until that work's landing folds it. Roll-up yields to stickiness: a delivered parent gaining an in-flight child keeps `delivered`, the child riding as a delta. `retired` is terminal: entry kept, dated, provenance intact — never deleted.
6. **Delivery writes land at acceptance; stewardship writes are direct.** During a run, proposed entries and deltas live in the spec workspace; the map write is one atomic batch at spec acceptance, and a rejected spec never touched the map. `/mochiko:feature` stewardship writes — stub minting, retroactive promotion, retire, grooming fixes — land directly, outside spec acceptance. Delivery-status writes (in-flight flips, graduations, delta folds) land only at acceptance landings. Reads happen any time.
7. **Map integrity — fix on sight.** No dangling FEAT-IDs; index lines and entry files agree on status; no orphaned deltas; every delta names its spec or lane run; every `in-flight` status or delta points at an open spec or a live lane run — live from dispatch until its acceptance landing; a delta whose lane run ended without folding is a defect, fix-on-sight. A closed spec still pointed at is a defect; a parent whose status contradicts its children's roll-up is a defect; a specs-index row contradicting the map is a defect.
8. **Entries index, never rewrite.** Story trace and SC references cite IDs; the spec's own sections stay the single source of their text.

## Red Flags — STOP and re-derive

- "Every story maps to its own new feature" — the filter never fired; features are capabilities, not story mirrors
- "This is close enough to FEAT-012, I'll just widen its statement silently" — extension of a delivered feature is a marked delta, never an in-place edit
- "That feature's in-flight, I'll design around it" — read the owning spec's artifacts; build against the planned contract or escalate a conflict, never contradict silently
- "I'll write the entry to the map now so it's not lost" — pre-acceptance derivation is unratified thought; the workspace holds it
- "The user will obviously want all of them, I'll mark them selected" — selection is the user's ruling, always
- "This entry needs eight extent lines to be honest" — then it is not one leaf; split it, or mint a parent
- "This capability wants a third level" — it doesn't get one; split the parent into two parents
- "The parent should go back to in-flight now that a new child arrived" — delivered is sticky; the child rides as a marked delta and folds when it delivers
- "The stub already describes the feature, I'll derive from its text" — stubs are unratified hypotheses; derive from stories and let a match be confirmation
- "I'll mint the remainder as a selectable stub so it's ready" — selectability is specify-derivation-only; `/mochiko:feature` parks and grooms, never matures
- "This leaf is just a phase, a thin layer is fine" — a phase-leaf must stand alone as a working increment, never a horizontal layer
- "The stub blocks this work, I'll assert it as a map relation" — stubs have no shape to verify; escalate as flagged-unverified PM judgment, not a verified relation
- "The dependency is blocking, I'll cut the remainder to unblock it" — escalation is a recommendation for the user; the PM never forces the cut
- "The reconstructed entry says X, good enough" — first touch re-verifies against the code before building on it

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Rejecting a story feels like losing requirements" | The rejection is recorded with its why — nothing is lost. Homing every story inflates the map until it stops describing the system. |
| "The map is right here, writing one entry early can't hurt" | A rejected spec must leave the truth layer clean. One early write breaks that guarantee for every future reader. |
| "Status on the story file too — easier to read" | Two status homes drift into two sources of truth. Stories derive status through their FEAT-ID; that is the design. |
| "The delta will obviously fold, no need to name the spec" | An unnamed delta is unauditable; a delta whose spec or lane run died is invisible rot. The grammar exists to make both checkable. |
| "First feature has no journey, so I'll mint a core feature" | A pseudo-feature poisons the map permanently to save one run's ordering. Carry the core as far as a real feature's extent honestly reaches. |
| "A filled-out stub saves derivation time later" | A stub is name + hook only. Extent and relations are derivation's to fill — a pre-filled stub fakes ratification and anchors the deriver on unratified text. |
| "Defects and refactors belong on the map — they're work on features" | The map states what the product does, not what needs fixing. Defects, tooling, and process live in `BACKLOG.md`; only extent-growth ideas ride the map. |

## Quality checklist

Before handing off:

- [ ] Map read completed at intent; in-flight territory read into the owning work's artifacts (spec or lane run); reconstructed entries flagged for re-verify; stubs noted as hypotheses only
- [ ] Every drafted story dispositioned: exactly one feature home, or a recorded rejection with the why
- [ ] Every proposed entry deduplicated against the actual map files, by capability — matching stubs confirmed and filled, not duplicated
- [ ] Nesting honest: two levels max; leaves the only pipeline units; every parent's status agrees with its children's roll-up (or carries the sticky-delivered delta); single-leaf parents deliberate
- [ ] Every entry within the sizing bars — one-breath name at parent/flat, extent ≤ ~3 lines at leaf — or split / parent minted
- [ ] Every SC-XXX mapped to a verifying feature; deferred SCs and one-sided seams on the owning entry's obligations line
- [ ] Every delta carries the full grammar — what grows, in-flight mark, named spec or lane run
- [ ] Relations dependency-closed for the recommended order; foundation designated as the first feature, guidance applied
- [ ] Index lines agree with entry files on status and name; leaf lines under their parent; no dangling FEAT-IDs introduced
- [ ] All delivery writes staged in the spec workspace — the live map untouched until acceptance (stewardship writes per invariant 6 excepted)
- [ ] Acceptance batch includes the specs-index row (`.mochiko/specs/index.md`), agreeing with the map
- [ ] Selection card prepared with recommendation, deferred-SC list, per-parent completeness ledger line (delivered/undelivered leaves · stubs · kills), and ordering — the ruling left to the user
- [ ] Territory-touching parents' parked stubs and undelivered leaves re-surfaced on the selection card; any dependency-blocked leaf/stub escalated as a recommendation (leaf via asserted map relation, stub via flagged-unverified judgment), never a forced cut
- [ ] Any leaf cut as an across-round phase is independently useful — a working increment, not a horizontal layer
- [ ] Derivation-minted stubs carry story-trace provenance; no stub matured or made selectable outside specify's derivation

## Related

- [`features-index-template.md`](../../templates/features-index-template.md) — owns the repo-root `FEATURES.md` index shape
- [`feature-entry-template.md`](../../templates/feature-entry-template.md) — owns the per-feature entry shape this skill fills
- `mochiko:review-specifications` — grades spec + stories + feature derivation + map delta in one pass (independent reviewer, never the author)
- `mochiko:authoring-user-stories` — upstream: the story quality the derivation reads; stories are authored inside the product frame this skill establishes
- `mochiko:patterns-vertical-tdd` — downstream: cuts one leaf's scope into cycle cards
- `mochiko:authoring-architecture` — the peer view: components that realize features; the entry's architecture link points there
