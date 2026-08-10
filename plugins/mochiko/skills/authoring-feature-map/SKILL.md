---
name: authoring-feature-map
description: This skill MUST be invoked when deriving or updating the repo-level feature map — the living FEATURES.md index plus per-feature entry files — during a specify run: the intent-stage map-read agenda, deriving proposed features from drafted user stories (stories first, features derived), running the story filter (a story can be rejected, with the why recorded, rather than silently inflating the map), authoring or amending FEAT-XXX entries (capability statement, status, extent, relations, architecture link, story trace, obligations), attaching marked deltas to delivered features, and staging the map write that lands at spec acceptance. SHOULD also invoke when the work involves "feature map", "FEATURES.md", "FEAT-XXX", "propose features", "feature derivation", "feature selection", "map delta", "in-flight feature", or verifying a reconstructed-from-code feature entry on first touch. Boundary: this skill authors and maintains the MAP — capability entries and their lifecycle — NOT user stories (mochiko:authoring-user-stories), NOT architecture views (the map links to ARCHITECTURE.md components, never restates them), and NOT selection: which features build now is the user's ruling — this skill recommends, never selects. Product capabilities live on the map; defects, tooling, and process items stay in BACKLOG.md. It never grades its own output — derivation and map delta are graded with the spec by mochiko:review-specifications, run by an independent reviewer.
---

# Authoring the Feature Map

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

The feature map is the **broad view of the whole system expressed as features** — the primary capability lens on the product, the way `ARCHITECTURE.md` is the system viewed as components. Together the two are the central source of truth. A **feature is the built thing**: a capability of the system described in the system's own language — not a cluster or regrouping of user stories. Stories inform *which* features get built and sharpen their extents; they never define them.

There is **one living map, no per-spec copy**: a succinct repo-root [`FEATURES.md`](../../templates/features-index-template.md) index (one line per feature) pointing at per-feature entry files (default home: `.mochiko/features/FEAT-XXX-<slug>.md`, in the shape [`feature-entry-template.md`](../../templates/feature-entry-template.md) defines). The map has three touchpoints: **specify proposes** (this skill's core work), **plan confirms and hardens** alongside architecture, and **implement's acceptance landing graduates** — status flips, delta folds, and index touches are bookkeeping edits inside that landing, never a separate close stage.

Feature delivery and spec delivery are **independent axes**: one spec can surface several features and deliberately build only a subset. The map is durable; specs are delivery events. This skill is the map judgment plus the entry authoring — the map-read agenda at intent, the derivation and filter after stories, entry and delta authoring, and the write rules. Density per the deliverable envelope ([`artifact-format.md`](../../templates/artifact-format.md)): capability statements 1–3 lines, extent and obligation entries one line each — an entry is a record, not an essay.

## Vocabulary — feature vs the units around it

| Term | Level | What it is | Owner |
|------|-------|------------|-------|
| **Feature** | Product / pipeline unit | A built capability on the map; graduates through plan/implement as its own unit | **this skill** (map entry) |
| **User story** | Spec | User value in user language; informs features, never defines them | `mochiko:authoring-user-stories` |
| **Vertical slice (cycle)** | Implementation, within one feature | A test-first increment delivering one observable behavior | `mochiko:patterns-vertical-tdd` (downstream) |

A feature too large to land in one breath of implement is cut into vertical-slice cycles downstream — never into pseudo-features minted for pipeline convenience.

## When to Use

- Shaping the lead's intent-stage conversation with the map-read agenda (below)
- Deriving proposed features and deltas from drafted user stories, and running the filter
- Authoring or amending FEAT-XXX entries and the FEATURES.md index lines
- Attaching a marked delta when a spec touches a `delivered` feature
- Staging the acceptance-time map write for a specify run, including its specs-index row (spec-index stewardship rides this skill)
- Re-verifying a reconstructed-from-code entry's extent on its first touch

## When NOT to Use

- **Grading the derivation or map delta** — graded with the spec by `mochiko:review-specifications` (independent reviewer, never the author)
- **Authoring or rewriting user stories** — entries trace stories by ID; a story that fits no feature is a filter verdict, not a rewrite
- **Authoring architecture** — the entry links to `ARCHITECTURE.md` components; it never restates the component view
- **Selecting which features build now** — the selection is the user's ruling; this skill prepares the selection card and recommends
- **Tracking defects, tooling, or process work** — those live in `BACKLOG.md`; the map carries product capabilities only

## The invariants (hard rules)

1. **Exactly one home.** Every accepted story maps to exactly one feature. Other features it touches carry extend obligations on their entries — never a second home.
2. **Complete disposition.** Every drafted story is either homed to a feature or rejected by the filter with the why recorded in the story file. No silent drops, no orphans.
3. **Dependency closure.** A selected feature must be buildable given only the features ordered before it, per the map's relations. No forward dependencies.
4. **The map owns status.** `proposed / in-flight / delivered / retired` — one home, no copies. Story files derive status by following their FEAT-ID; the only story-native status is `rejected`.
5. **Delivered is sticky.** A later spec touching a `delivered` feature never regresses its status; the change rides as a marked delta until that work's landing folds it. `retired` is terminal: entry kept, dated, provenance intact — never deleted.
6. **Writes land at acceptance.** During a run, proposed entries and deltas live in the spec workspace. The map write is one atomic batch at spec acceptance. Reads happen any time; a rejected spec never touched the map.
7. **Map integrity — fix on sight.** No dangling FEAT-IDs; index lines and entry files agree on status; no orphaned deltas; every delta names its spec; every `in-flight` status or delta points at an open spec — a closed spec still pointed at is a defect; a specs-index row contradicting the map is a defect.
8. **Entries index, never rewrite.** Story trace and SC references cite IDs; the spec's own sections stay the single source of their text.

## The intent-stage map-read agenda

The existing map is an **obligated read at the intent stage** — capability context is input to story drafting even though map writes happen only after stories exist. Drafting stories blind to the map is the named failure mode: duplicate "new" features, ignored extension points. This skill carries the agenda that shapes the lead's intent conversation:

1. **Read `FEATURES.md`**, then the full entries for every feature in or near the territory the intent claims.
2. **Surface to the conversation:** delivered capabilities the intent may extend (extension point, not new feature) · `proposed` entries the intent may be picking up (inherit their story trace and obligations) · relations that put ordering constraints on anything new.
3. **In-flight territory:** an `in-flight` or delta-carrying entry obligates a read into the owning spec's artifacts — its stories, plan, and architecture delta — so this run knows what the feature is *becoming*, not just that it is busy.
4. **Reconstructed entries:** an entry carrying the reconstructed-from-code mark has never been verified by a spec run. The first spec that touches it **re-verifies its extent against the code before building on it**, and the acceptance-time write clears the mark.
5. **Missing map:** on a repo where setup has run, a map exists. A missing map is surfaced like a missing governance region — offer `/mochiko:setup` (its brownfield analysis reconstructs the initial map) — never silently tolerated and never lazily grown mid-run.

## Derivation and the filter — stories first, features derived

After stories are drafted, derive the map delta. For each story, against the **actual map files** (never memory of them):

- **Extends a delivered feature** → a marked delta on that entry (grammar below).
- **Lands in in-flight territory** → resolve with information, not policy: need already covered by the in-flight planned extent → reference the relation, build against the planned contract, no entry write · need adjacent → a `proposed` delta marked "extends in-flight work, spec-N," sequenced behind that delivery by ordinary dependency ordering — never by a lock · need conflicting with the in-flight direction → a real product decision, escalated to the user. Silent contradiction is the only thing prohibited.
- **Implies a genuinely new capability** → a `proposed` entry, deduplicated against the map by capability, not by name.
- **Earns no place on the map** → **rejected**. The filter is the pipeline's ability to say no: not every story becomes or joins a feature, and a rejection is recorded in the story file with the why. Without the filter, every story becomes a feature and the map inflates into a story list.

**Granularity guide:** a feature is a capability a product person would name in one breath — bigger than a story, smaller than a product area. An extent that cannot be stated in ~3 lines is two features.

**SC re-homing:** at derivation, every SC-XXX maps to the feature(s) whose delivery verifies it. At selection the set splits visibly: SCs covered by selected features are this delivery's done-condition; SCs covered only by unselected features travel with the `proposed` entry's obligations line. The deferred-SC list appears on the selection card — choosing a subset is choosing which success criteria wait, shown at the moment of choice.

## Entry authoring

Author entries in the [`feature-entry-template.md`](../../templates/feature-entry-template.md) shape: FEAT-XXX ID + name · capability statement (1–3 lines, the system's own language) · status · extent (what's in and notably not in) · relations (depends-on / extends / composes-with) · architecture link (which `ARCHITECTURE.md` components realize it, navigable both directions) · story trace (accumulating provenance, IDs only) · **obligations** (deferred SCs, deferred seams — "when built, verify seam against FEAT-XXX" — and cross-cutting extend obligations). The index line is the entry compressed to one breath: FEAT-ID · name · status · capability hook.

**Delta grammar:** a delta on a `delivered` entry reads `extent grows by <X> — in-flight, <spec-slug>`; it names its spec, lives under the entry's Deltas heading, and folds into the extent lines at the owning work's acceptance landing. A delta whose spec closed without folding is an integrity defect.

**Seams:** when both sides of a cross-feature seam are selected, the later-landing feature owns its verification. When one side is unselected, the seam rides the `proposed` entry as an obligation.

## Ordering and the foundation role

Order selected features by dependency closure first; among independent features, product priority breaks ties. **Foundation is an ordering role**: the selection's first feature per dependency order. "Establishes the shared core and delivers a testable journey" is guidance for which feature goes first — not a hard invariant, because features are fixed capabilities and the map may contain none that is both. When the true shared core spans features, the first feature carries it only as far as its extent honestly reaches; plumbing lands in foundation cycles inside that feature's implement. Minting a pipeline-convenience pseudo-feature stays forbidden.

## Write rules

During the run, all derivation output — proposed entries, deltas, index-line drafts — lives in the spec workspace, where the extended spec review grades it. The reviewer's map-delta baseline is the **git state of the map at run open**. At **spec acceptance**, the write executes as one atomic bookkeeping batch: new entries land (`proposed`; the selected ones flip to `in-flight` with date and owning-spec pointer) · deltas attach to their entries · `FEATURES.md` index lines update · touched reconstructed-from-code marks clear · the specs index (`.mochiko/specs/index.md`) gains the spec's row — slug, status, FEAT-IDs touched with outcomes, one-line about. **Spec-index stewardship rides this skill:** the specs index follows the same open/close contract as the brainstorms index, and its rows must never contradict the map — a spec's closed status is derived state, true exactly when its selected FEAT-IDs read `delivered`. Later, implement's acceptance landing does the graduation half: status to `delivered`, delta folds, in-flight pointer cleared, both index lines touched.

## Quality checklist

Before handing off:

- [ ] Map read completed at intent; in-flight territory read into owning specs; reconstructed entries flagged for re-verify
- [ ] Every drafted story dispositioned: exactly one feature home, or a recorded rejection with the why
- [ ] Every proposed entry deduplicated against the actual map files, by capability
- [ ] Every entry within the granularity guide — one-breath capability, extent ≤ ~3 lines — or split
- [ ] Every SC-XXX mapped to a verifying feature; deferred SCs and one-sided seams on the owning entry's obligations line
- [ ] Every delta carries the full grammar — what grows, in-flight mark, named spec
- [ ] Relations dependency-closed for the recommended order; foundation designated as the first feature, guidance applied
- [ ] Index lines agree with entry files on status and name; no dangling FEAT-IDs introduced
- [ ] All writes staged in the spec workspace — the live map untouched until acceptance
- [ ] Acceptance batch includes the specs-index row (`.mochiko/specs/index.md`), agreeing with the map
- [ ] Selection card prepared with recommendation, deferred-SC list, and ordering — the ruling left to the user

## Red Flags — STOP and re-derive

- "Every story maps to its own new feature" — the filter never fired; features are capabilities, not story mirrors
- "This is close enough to FEAT-012, I'll just widen its statement silently" — extension of a delivered feature is a marked delta, never an in-place edit
- "That feature's in-flight, I'll design around it" — read the owning spec's artifacts; build against the planned contract or escalate a conflict, never contradict silently
- "I'll write the entry to the map now so it's not lost" — pre-acceptance derivation is unratified thought; the workspace holds it
- "The user will obviously want all of them, I'll mark them selected" — selection is the user's ruling, always
- "This entry needs eight extent lines to be honest" — then it is two features; split
- "The reconstructed entry says X, good enough" — first touch re-verifies against the code before building on it

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Rejecting a story feels like losing requirements" | The rejection is recorded with its why — nothing is lost. Homing every story inflates the map until it stops describing the system. |
| "The map is right here, writing one entry early can't hurt" | A rejected spec must leave the truth layer clean. One early write breaks that guarantee for every future reader. |
| "Status on the story file too — easier to read" | Two status homes drift into two sources of truth. Stories derive status through their FEAT-ID; that is the design. |
| "The delta will obviously fold, no need to name the spec" | An unnamed delta is unauditable; a delta whose spec died is invisible rot. The grammar exists to make both checkable. |
| "First feature has no journey, so I'll mint a core feature" | A pseudo-feature poisons the map permanently to save one run's ordering. Carry the core as far as a real feature's extent honestly reaches. |
| "Defects and refactors belong on the map — they're work on features" | The map states what the product does, not what needs fixing. Defects, tooling, and process live in `BACKLOG.md`. |

## Related

- [`features-index-template.md`](../../templates/features-index-template.md) — owns the repo-root `FEATURES.md` index shape
- [`feature-entry-template.md`](../../templates/feature-entry-template.md) — owns the per-feature entry shape this skill fills
- `mochiko:review-specifications` — grades spec + stories + feature derivation + map delta in one pass (independent reviewer, never the author)
- `mochiko:authoring-user-stories` — upstream: the story quality the derivation reads; stories are authored inside the product frame this skill establishes
- `mochiko:patterns-vertical-tdd` — downstream: cuts one feature's scope into cycle cards
- `mochiko:authoring-architecture` — the peer view: components that realize features; the entry's architecture link points there
