---
name: authoring-feature-map
description: This skill MUST be invoked when deriving or updating the repo-level feature map — the FEATURES.md index of durable capabilities plus per-capability FEAT-XXX entry files carrying transient work rows — during a specify or /mochiko:feature touch. SHOULD also invoke on 'feature map', 'FEATURES.md', 'FEAT-XXX', 'capability', 'work row', 'feature derivation', 'extend beats mint', or 'map delta'. Boundary: authors the MAP — NOT user stories (mochiko:authoring-user-stories), NOT architecture, NOT selection; mint/extend discipline lives in mochiko:patterns-map-minimalism. Never grades its own output.
---

# Authoring the Feature Map

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

The feature map is the **broad view of the whole system expressed as capabilities** — the primary capability lens on the product, the way `ARCHITECTURE.md` is the system viewed as components; together the two are the central source of truth. The map holds **one surface, two row types**. A **capability** is the durable entry — the only thing called a *feature*: what the product does, in the system's own language. A **work row** is a transient delivery increment under its capability; rows may be story-shaped, and at their delivery landing they **fold into the capability's extent and vanish**. Stories inform *which* capabilities exist and sharpen their extents; they never define them. The capability tests, extend-beats-mint, the soft cap, merge, and domains are the derivation *discipline* — they live in `mochiko:patterns-map-minimalism`; this skill carries the authoring craft.

Derivation is **frame-first**: before any story is drafted, the product frame names the capabilities the territory touches as a **hypothesis** — nouns and verbs only, never a list of stories — and stories are authored inside it; where a story and the frame conflict, the **story wins** and the frame adjusts. There is **one living map, no per-spec copy**: a succinct repo-root [`FEATURES.md`](../../templates/features-index-template.md) index (one line per capability, work rows as sublines) over per-capability entry files (`.mochiko/features/FEAT-XXX-<slug>.md`, shaped by [`feature-entry-template.md`](../../templates/feature-entry-template.md)). Four touchpoints: **specify proposes** — frame at intent, then confirm-frame, cut rows, filter, and selection after stories (this skill's core work); **plan confirms and hardens** alongside architecture; **implement's acceptance landing folds** delivered rows into extent — status and index edits are bookkeeping inside that landing, never a separate close stage; **`/mochiko:feature` stewards** — stub minting, retire, integrity grooming, and the growth door that cuts rows on existing capabilities.

Capability delivery and spec delivery are **independent axes**: one spec can surface several capabilities and build only a subset of their rows. The map is durable; specs are delivery events. A work row is a **map-side** increment; the product-baseline deltas under `.mochiko/product/` (the appliable before/after form) are a different altitude — plan and implement's surface, untouched here. Density per the deliverable envelope ([`artifact-format.md`](../../templates/artifact-format.md)): capability statements 1–3 lines, extent, work-row, and obligation entries one line each — a record, not an essay.

## Vocabulary — capability vs the units around it

| Term | Level | What it is | Owner |
|------|-------|------------|-------|
| **Capability** (a *feature*) | Product | The durable thing the product does, in its own language; permanent, honest extents; the only entry called a feature | **this skill** (map entry) |
| **Work row** | Delivery increment | A transient row under a capability — one built increment, story-shaped allowed; `pending` (cut, undelivered) or `live` (in a run); folds into the capability's extent at its landing and vanishes | **this skill** (map entry) |
| **User story** | Spec | User value in user language; informs capabilities, never defines them | `mochiko:authoring-user-stories` |
| **Vertical slice (cycle)** | Implementation, within one capability-batch run | A test-first increment delivering one observable behavior | `mochiko:patterns-vertical-tdd` (downstream) |

A **capability-batch** is the pipeline unit: each capability with selected work rows gets one plan/implement run covering exactly those rows.

## When NOT to Use

- **Grading the derivation or map delta** — graded with the spec by `mochiko:review-specifications` (independent reviewer, never the author)
- **Applying the mint-vs-extend discipline** — the capability tests, extend-beats-mint, the soft cap, merge, and domains live in `mochiko:patterns-map-minimalism`; this skill authors what that discipline rules in
- **Authoring or rewriting user stories** — entries trace stories by ID; a story that fits no capability is a filter verdict, not a rewrite
- **Authoring architecture** — the entry links to `ARCHITECTURE.md` components; it never restates the component view
- **Selecting which work rows build now** — the selection is the user's ruling; this skill prepares the selection card (rows grouped per capability) and recommends
- **Tracking defects, tooling, or process work** — those live in `BACKLOG.md` where KM exists (a non-KM product has no queue; lane runs accept direct requests — the stated degrade path, never silently assumed away). Extent-growth improvement ideas are the exception: they ride the map as pending work rows or obligation lines — the map is the capability backlog.

## The invariants (hard rules)

1. **Exactly one home.** Every accepted story maps to exactly one capability. Other capabilities it touches carry extend obligations on their entries — never a second home.
2. **Complete disposition.** Every drafted story is either homed to a capability or rejected by the filter with the why recorded in the story file. No silent drops, no orphans.
3. **Dependency closure.** A selected work row must be buildable given only the rows ordered before it, per the map's relations. Closure runs at the row level: a row in one capability depending on a row in another orders those two capability-batch runs. No forward dependencies.
4. **The map owns status.** Capability status is `proposed / in-flight / delivered / retired` — one home, no copies. Work rows carry a delivery *state* (`pending` / `live`), not a status of their own. Story files derive status by following their FEAT-ID; the only story-native status is `rejected`.
5. **Delivered is sticky; delivered rows fold.** A `live` work row folds into its capability's extent at its delivery landing, then vanishes; a `pending` row persists on the entry as open obligation — deferred work never silently disappears. A `delivered` capability keeps its status while it carries live rows; the change rides on the row until the row's landing folds it — status never regresses. `retired` is terminal: entry kept, dated, provenance intact — never deleted. An epic member's pending or live work row carries an inline `[EPIC-XXX]` marker that vanishes with the row at its graduation fold.
6. **Capability writes are sacred; delivery writes land at acceptance; stewardship writes are direct.** Minting, merging, retiring, or changing a capability's status is the sacred layer — it lands only through specify or a user grooming ruling, never as desk bookkeeping. Cutting work rows is delivery bookkeeping the desk may do. During a run, proposed entries and cut rows live in the spec workspace; the delivery-status write (in-flight flips, row folds) is one atomic batch at that work's acceptance landing, and a rejected spec never touched the map. `/mochiko:feature` stewardship writes — stub minting, retire, grooming fixes — land directly. Reads happen any time.
7. **Map integrity — fix on sight.** No dangling FEAT-IDs; index lines and entry files agree on status; every work row names its spec or lane run; every `live` row or `in-flight` status points at an open spec or a live lane run — live from dispatch until its acceptance landing; a row whose run ended without folding is a defect, fix-on-sight. A closed spec still pointed at is a defect; a specs-index row contradicting the map is a defect.
8. **Entries index, never rewrite.** Story trace and SC references cite IDs; the spec's own sections stay the single source of their text.

## Red Flags — STOP and re-derive

- "Every story maps to its own new capability" — the extend-beats-mint filter never fired; capabilities are durable, not story mirrors (the tests are in `mochiko:patterns-map-minimalism`)
- "This story-grain increment should be a permanent feature" — story-shaped *work rows* are fine; a story-shaped *capability* is the disease. Cut it as a work row, never mint it as a standing feature
- "This is close enough to FEAT-012, I'll just widen its statement silently" — growth on a delivered capability rides as a marked work row, never an in-place edit
- "That capability's in-flight, I'll design around it" — read the owning run's artifacts; build against the planned contract or escalate a conflict, never contradict silently
- "I'll write the entry to the map now so it's not lost" — pre-acceptance derivation is unratified thought; the workspace holds it
- "The user will obviously want all the rows, I'll mark them selected" — selection is the user's ruling, always
- "The frame already names the stories, I'll cut rows to match it" — the intent frame is a nouns-and-verbs hypothesis, never a story list; where a story and the frame conflict, the story wins and the frame adjusts
- "This row delivered, but I'll leave it on the entry as a record" — a delivered row folds into the extent and vanishes; the extent is the record. Only `pending` rows persist
- "This entry needs eight extent lines to be honest" — then it is two capabilities (split it), or its bulk is undelivered work rows. It never grows a sub-level
- "These capabilities want a parent to group them" — capabilities are flat; grouping is domains, and domains are minted only at the soft cap (`mochiko:patterns-map-minimalism`), never before
- "The stub already describes the capability, I'll derive from its text" — stubs are unratified hypotheses; derive from stories and let a match be confirmation
- "I'll mint the remainder as a selectable stub so it's ready" — selectability is specify-derivation-only; `/mochiko:feature` parks and grooms, never matures
- "This work row is just a phase, a thin layer is fine" — a phase row must stand alone as a working increment, never a horizontal layer
- "The stub blocks this work, I'll assert it as a map relation" — stubs have no shape to verify; escalate as flagged-unverified PM judgment, not a verified relation
- "The dependency is blocking, I'll cut the remainder to unblock it" — escalation is a recommendation for the user; the PM never forces the cut
- "The reconstructed entry says X, good enough" — first touch re-verifies against the code before building on it

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Rejecting a story feels like losing requirements" | The rejection is recorded with its why — nothing is lost. Homing every story inflates the map until it stops describing the system. |
| "The map is right here, writing one entry early can't hurt" | A rejected spec must leave the truth layer clean. One early write breaks that guarantee for every future reader. |
| "Status on the story file too — easier to read" | Two status homes drift into two sources of truth. Stories derive status through their FEAT-ID; that is the design. |
| "The row will obviously fold, no need to name its run" | An unnamed row is unauditable; a work row whose spec or lane run died is invisible rot. The grammar exists to make both checkable. |
| "First capability has no journey, so I'll mint a core feature" | A pseudo-capability poisons the map permanently to save one run's ordering. Carry the core as far as a real capability's extent honestly reaches. |
| "A filled-out stub saves derivation time later" | A stub is name + hook only. Extent and rows are derivation's to fill — a pre-filled stub fakes ratification and anchors the deriver on unratified text. |
| "Defects and refactors belong on the map — they're work on features" | The map states what the product does, not what needs fixing. Defects, tooling, and process live in `BACKLOG.md`; only extent-growth ideas ride the map. |

## Quality checklist

Before handing off:

- [ ] Map read at intent and the capability frame stated as a nouns-and-verbs hypothesis; in-flight territory read into the owning work's artifacts (spec or lane run); reconstructed entries flagged for re-verify; stubs noted as hypotheses only
- [ ] Frame confirmed or adjusted against what the stories revealed — every story-vs-frame conflict resolved in the story's favor
- [ ] Every drafted story dispositioned: exactly one capability home, or a recorded rejection with the why
- [ ] Every proposed entry deduplicated against the actual map files, by capability — matching stubs confirmed and filled, not duplicated
- [ ] Capabilities flat and durable — each passes the tests in `mochiko:patterns-map-minimalism`; no parent/leaf structure, no sub-levels
- [ ] Every capability's extent honest — an extent that rounds up past ~3 lines is two capabilities or undelivered work rows, not one oversized entry
- [ ] Work rows carry a state (`pending` / `live`) and acceptance criteria; each names its spec or lane run; delivered rows folded into extent at landing; `pending` rows persist as open obligation
- [ ] Every SC-XXX mapped to a verifying capability; deferred SCs and one-sided seams on the owning entry's Obligations line; within an epic, a cross-member seam's owner is the spine's design-time assignment (`mochiko:authoring-epic`), not the later-lander default
- [ ] Relations dependency-closed at the row level for the recommended order; foundation designated first; a cross-capability row dependency orders the two capability-batch runs
- [ ] Index lines agree with entry files on status and name; work-row sublines under their capability; domain headers only if the soft cap tripped; no dangling FEAT-IDs introduced
- [ ] All delivery writes staged in the spec workspace — the live map untouched until acceptance (stewardship writes per invariant 6 excepted); no capability mint/merge/retire/status-change outside specify or a user grooming ruling
- [ ] Acceptance batch includes the specs-index row (`.mochiko/specs/index.md`), agreeing with the map
- [ ] Selection card prepared with recommendation, deferred-SC list, per-capability completeness line (pending rows · folded rows · deferred SCs), work rows grouped per capability, and ordering — the ruling left to the user
- [ ] Territory-touching capabilities' parked stubs and pending rows re-surfaced on the selection card; any dependency-blocked row or stub escalated as a recommendation (row via asserted map relation, stub via flagged-unverified judgment), never a forced cut
- [ ] Any work row cut as an across-round phase is independently useful — a working increment, not a horizontal layer
- [ ] Derivation-minted stubs carry story-trace provenance; no stub matured or made selectable outside specify's derivation

## Related

- [`features-index-template.md`](../../templates/features-index-template.md) — owns the repo-root `FEATURES.md` index shape
- [`feature-entry-template.md`](../../templates/feature-entry-template.md) — owns the per-capability entry shape this skill fills
- `mochiko:patterns-map-minimalism` — the derivation discipline this skill applies: capability tests, extend-beats-mint, the soft cap, merge mechanics, and domains
- `mochiko:review-specifications` — grades spec + stories + feature derivation + map delta in one pass (independent reviewer, never the author)
- `mochiko:authoring-user-stories` — upstream: the story quality the derivation reads; stories are authored inside the capability frame this skill establishes
- `mochiko:patterns-vertical-tdd` — downstream: cuts one capability-batch run's scope into cycle cards
- `mochiko:authoring-architecture` — the peer view: components that realize capabilities; the entry's architecture link points there
