# PM Requirements Stacking — Decision Record

**Status:** accepted
**Opened:** 2026-08-10 · **Accepted:** 2026-08-10 (post-review, all 7 findings dispositioned, verify defect repaired)
**Topic:** how the product-manager seat in command workflows ensures feature requirements coming out of `/mochiko:specify` are stacked — balancing delivery with completeness.

## Framing

User's clarified intent (Q1): user stories and requirements out of specify are detailed and extensive — that is good and stays. Product management and delivery are pragmatic: a particular feature's delivery can happen in cycles or phases. "Stacked" = phased delivery of one feature's requirement set, phase on phase, without losing completeness. Question: where and how does the product-manager seat manage that.

## Ground facts

- **F1** — Selection at specify is feature-granular: the user picks which derived features build now from the PM's selection card; the card carries a deferred-SC list (`plugins/mochiko/commands/specify.md`).
- **F2** — The feature is the pipeline unit; slices are retired; `/mochiko:plan` runs once per selected feature (feature-map-layer D4/D17, feature-sizing D9).
- **F3** — Within-feature phasing has no first-class carrier today: the closest mechanisms are deferred SCs riding the map entry (feature-map-layer D21) and delta cards for late children on sticky-delivered parents (feature-sizing D2–D3, `/mochiko:feature` command ruled, build pending). **Corrected at review:** see F6.
- **F4** — Two-level map nesting is ruled (parent capability + leaf deliverable, leaf = pipeline unit) in feature-sizing D2–D3; build not started. **Superseded mid-session:** see F5.
- **F5** — *(mid-session merge of origin/main, v0.61.0 — user-directed, "it will most likely change some of the direction")* The feature-sizing build shipped: `/mochiko:feature` command (stewardship + lane intake + dispatch) · two-level nesting live in `mochiko:authoring-feature-map` + `feature-entry-template.md` (parent = navigation/roll-up, never built; leaf = pipeline unit; hard two-level cap; sticky-delivered; retroactive promotion; `retired` terminal with user-ruled retire) · `unrefined` capability stubs (name + one-breath hook, no extent/relations; parking never spec-bypass; derivation ignores stub text, match = confirmation; skill text says minted by `/mochiko:feature`) · parents carry no extent ("a parent's extent is its children") · obligations lines carry deferred SCs/seams · dependency relations + dependency-closure invariant · plan/implement re-keyed to feature scope.
- **F6** — *(review repair, F-1: corrects F3's "no first-class carrier" claim)* Shipped machinery already carries two of the three phasing forms. **Within one build run:** a leaf too large for one implement breath is cut into vertical-slice cycles at plan time (`mochiko:authoring-feature-map` vocabulary table + "never into pseudo-features minted for pipeline convenience"; craft in `mochiko:patterns-vertical-tdd`). **Oversize at derivation:** an extent that won't fit ~3 lines mints a parent with leaves that each pass the bar (shipped minting way 1). Only the third form — **across selection rounds** (which leaves build now; what happens to the rest) — had no carrier and needed this session's rulings.

## Decisions

### D1 — Phases ride shipped machinery; the session's ruling content is across-round phasing + the re-surfacing obligation — `Confident` *(as amended at review, F-1: reuse-and-extend, user-ruled)*

**Statement:** Three phasing forms; the first two are shipped machinery **adopted as-is, reused never re-invented**: (1) within one build run, phasing = vertical-slice cycles cut at plan time (shipped, not PM work); (2) oversize-at-derivation, phasing = mint a parent with leaves per the shipped sizing remedy (F6). D1's own ruling covers only form (3), **across selection rounds**: an extensive feature's phases are its leaves under one parent; each selection round picks the next leaves; the parent's roll-up tracks delivered vs undelivered. Layered on top: a standing PM stewardship obligation — every deferred SC and undelivered remainder must have a named return path (a leaf, or an explicit kill via the shipped user-ruled retire), re-surfaced per the scope ruled at D3a until delivered or killed.

**Rationale:** No second decomposition grammar — stacking rides shipped nesting; completeness is map-visible (parent shows undelivered leaves). Alternative B (phase sections inside one spec, consumed one per plan run) rejected — as-written it duplicated what cycles already deliver within a run (review corrective: the original rejection said "competing unit" without citing cycles). Alternative C alone (discipline only) rejected as structurally invisible; its re-surfacing obligation survives as D1's second half.

**Amendment trail:** original D1 claimed phasing "has no first-class home" (F3) and assigned all phase semantics to leaves. Review F-1 (Critical) showed forms 1–2 shipped; the user ruled "we should and must reuse what exists and extend it." D1 re-scoped accordingly; no ruling reversed — the novelty claim narrowed.

**Note:** the user asked whether two-level nesting already existed — confirmed: ruled in `feature-sizing-and-entry-points` D2–D3 (2026-08-10); shipped mid-session at v0.61.0 (F5).

### D2 — Confidence-keyed hybrid decomposition, two-lane lifecycle — `Confident`

**Statement:** At derivation the PM cuts leaves only where the requirement set supports a buildable deliverable (confident portion); the genuinely uncertain remainder parks under D1's re-surfacing obligation until it becomes a leaf or is explicitly killed (carrier per D2a). **Maturation routes through specify** *(as corrected at review, F-2)*: parked remainder becomes buildable only via a specify run's derivation — stories drafted, requirements filled, the stub confirmed and made selectable per the shipped confirmation path. `/mochiko:feature`'s role is stewardship only: parking, grooming, retire, and re-surfacing at its stewardship touches — never maturation, never dispatch of unratified scope. The lane boundary stays the shipped map-write test.

**Rationale:** Upfront-everything (A) mints speculative leaves that age badly (walked example: an "audit log" leaf later discovered to belong to a compliance feature). Rolling-only (B) reduces completeness to prose — phases stop being countable. C keeps countable completeness where the spec's detail supports it and honesty where it doesn't. Ruled after an authentication walk-through (parent + AUTH-1…5 leaves; lockout/audit parked).

**Amendment trail (F-2, Critical — user-accepted after a plain-language walk-through):** the original statement had maturation happening "in the `/mochiko:feature` lane as map deltas + dispatch, no new spec run required." That contradicted the shipped surface three ways: the lane's map-write test routes minting/status flips to `/mochiko:specify`; only specify's derivation fills a stub and makes it selectable; and a stub carries no ratified scope, so the lane cannot dispatch it. Corrected: no fast lane from parked idea to build — nothing becomes buildable without passing through requirements. Accepted cost: maturation waits for a specify run in that territory; consistent with D3's no-forced-cadence stance and the shipped no-spec-bypass design.

### D2a — Amendment after the v0.61.0 merge: uncertain remainder parks as `unrefined` stubs, mintable at derivation — `Confident`

**Statement:** D2's parking construct re-keys to the shipped machinery. The confident portion of an extensive requirement set becomes `proposed` leaf entries (derivation-filled, selectable — the existing specify path). The uncertain remainder parks as **`unrefined` stub children under the parent** — not as prose extent on the parent entry, which the shipped shape forbids ("a parent's extent is its children"). Amendment to the shipped skill text this implies: **specify's derivation may also mint stubs** (currently only `/mochiko:feature` mints them) — a derivation-minted stub carries story-trace provenance; the existing rule stands that later derivation re-derives from stories and treats the stub as confirmation, never as anchor text. D1's re-surfacing obligation and D3's ledger/escalation now read over leaves + stubs under the parent.

**Rationale:** One parking construct beats two (obligation-line parking rejected — it would make Obligations a second parking home). Story-backed remainder from a spec is more ratified than `/mochiko:feature`'s idea-parking, not less — extending stub-minting to derivation is a smaller distortion than storing remainder as parent prose. D2's confidence-keyed judgment survives; the lifecycle claim was corrected at review (F-2 — maturation routes through specify).

**Review challenge sustained against (F-3, Important — user-ruled):** the no-map-carrier alternative (remainder rides deferred-SC obligations + story trace only, no stub, no skill edit) was steelmanned and rejected: it buries the remainder one level deeper than where the PM looks — map visibility (ledger, re-surfacing, kill path on `FEATURES.md`) is the session's point. The reviewer's why-was-minting-restricted question was answered from the shipped text: the restriction's stated intent is anti-spec-bypass ("selectability stays behind specify's derivation"); a derivation-minted stub is the *output* of a spec run, not a bypass of one, and post-F-2 maturation re-enters specify regardless — the loosening does not touch the protected intent.

### D3 — Completeness ledger + dependency-triggered escalation only — `Contested`

**Statement:** Every selection card carries a per-parent completeness ledger line (delivered leaves / undelivered leaves / parked remainder / kills) — pure information. Forced disposition fires on exactly one trigger: an undelivered leaf or parked remainder **blocks another piece of work entering delivery** — then the PM escalates hard: cut it into a leaf now, or the dependent work re-scopes. No skip-count nag, no staleness backstop.

**Rationale:** The user's read — the escalation trigger is dependency, not repetition. The lead pushed back once (non-blocking remainder can rot silently forever; a weak "still real?" backstop would keep the ledger honest); the user held: dependency trigger only, silent non-delivery is real but this is the wrong place to solve it. Marked `Contested` per the push-back; the silent-rot question is recorded as an open thread, not folded in here.

**Minor amendments (review batch, F-6 + F-7, user-accepted):** *(F-7)* the escalation's "cut it into a leaf now, or the dependent work re-scopes" is a **recommendation surfaced to the user**, never a PM-forced cut — the PM-recommends-never-selects invariant governs; the forced element is that a *decision* is put in front of the user, not which way it goes. *(F-6)* phase-boundary quality bar: a leaf cut as a phase must be independently useful — a working increment on its own, per the existing vertical discipline — never a horizontal layer that only pays off when a later phase lands.

### D3a — Re-surfacing scope narrowed to territory touches — `Confident` *(review F-5, user-accepted)*

**Statement:** D1's re-surfacing obligation fires at exactly two sites: (1) the selection card of any specify run whose territory touches the parent — the shipped map-read agenda already obliges reading the territory's entries, so parked stubs and undelivered leaves surface there; (2) any `/mochiko:feature` stewardship touch on that parent (query, promotion, grooming). It does **not** fire on unrelated specs — "every selection card" (D1's original wording) was unfulfillable: a card belongs to its spec's scope and cannot carry an unrelated parent's remainder.

**Rationale:** Honest narrowing over fictional universality. Stated consequence, accepted: a territory nobody revisits keeps its remainder parked indefinitely with no nag — which is exactly D3's ruling (dependency trigger only; silent non-delivery solved elsewhere). D1/D3 now consistent instead of contradictory; open thread 1 remains the future home for territory-level staleness pressure.

### D4 — Split claims: technical seat asserts dependency facts, PM owns escalation — `Confident`

**Statement:** The dependency fact behind a D3 escalation is asserted by a technical seat — system-architect at plan time or technical-analyst at derivation — and lands as a map relation (`<leaf> blocks <feature>`) with provenance. The PM consumes relations: the escalation and the recommendation on the selection card are the PM's; the underlying dependency claim is never the PM's own assertion.

**Rationale:** Answers the user's own doubt ("unsure if only product manager's input is needed here, or tech person too"). Matches the existing remit split — PM owns which/when, architecture owns topology; keeps fiction out of map relations (each seat asserts only what it can verify). Joint per-cycle co-authored cards rejected as heavyweight; PM-sole rejected as unverifiable assertion.

**Amendment (F-4, Important — user-accepted; touches user-ruled D3):** the escalation trigger splits by carrier. An **undelivered leaf** blocking incoming work escalates on a technically asserted map relation (the path above, unchanged). A **parked `unrefined` stub** blocking incoming work cannot carry a map relation (stubs have no shape to verify against — template: no Extent, no Relations); it escalates as PM judgment explicitly flagged *unverified* ("incoming work appears to need <stub> — parked, unshaped; specify it now or re-scope"), the user rules. Stubs stay shapeless; the escalation still fires; only the claim's provenance is honest — judgment, not verified topology.

## Build surface (post-review revision)

Grounded against the shipped v0.61.0 surface — much of the session's early direction was absorbed by that build (F5) or corrected at review; what survives as genuinely new edits:

- **`mochiko:authoring-feature-map`** — across-round phase-semantics line: an extensive feature's selection-round phases are its leaves; within-run phasing stays cycles, oversize-at-derivation stays parent-minting — reuse stated explicitly (D1 as amended) · derivation-may-mint-stubs amendment, anti-spec-bypass intent preserved (D2a, F-3 sustained) · re-surfacing obligation at the two D3a sites, in the selection-card procedure and the stewardship-touch path · completeness ledger line on the selection card (per-parent: delivered/undelivered leaves, stubs, kills; D3) · dependency-escalation duty split by carrier: leaf-blocking via technically asserted map relation, stub-blocking as PM judgment flagged unverified, both surfaced as recommendations for the user's ruling (D3/D4 as amended, F-4/F-7) · phase-boundary quality bar: a phase-leaf is independently useful (F-6).
- **`specify.md`** — selection-card line gains the completeness ledger reference — one Harness-line touch (D3).
- **`feature.md`** — stewardship touches re-surface parked remainder on the touched parent (D3a site 2); no lane/maturation edits — maturation confirmed specify-only (F-2).
- **`plan.md`** — one line: the architecture stage asserts dependency relations onto the map with provenance (D4).
- Strips + author≠grader audits per the primitive-edit ceremony; lands as a normal post-brainstorm BACKLOG build item.

## Review

Solo cold review, blind-map dispatch (two-message fence held; reviewer's 18-angle Phase 0 map built from topic + goal + free repo grounding only). Verdict **FAIL / needs-revision**: 7 findings — 2 Critical, 3 Important, 2 Minor. Dispositions (Criticals + Importants one-by-one with the user, Minors batched, all user-ruled):

| # | Sev | Class | Finding | Disposition |
|---|-----|-------|---------|-------------|
| F-1 | Critical | coverage + contradiction | Record never engaged vertical-slice cycles / shipped oversize remedy; F3 ground fact false | Folded: F6 ground fact added; D1 re-scoped to across-round phasing only ("reuse what exists and extend it" — user's words) |
| F-2 | Critical | decision inconsistency | D2's lane-maturation path contradicts shipped map-write test + selectability rule | Folded: maturation routes through specify; `/mochiko:feature` stewardship-only; accepted cost recorded |
| F-3 | Important | rung-0 challenge | Stub may be empty carrier; no-map-carrier alternative unexplored; minting-restriction intent unknown | Sustained against: map visibility is the session's point; restriction intent verified anti-spec-bypass, untouched — D2a stands |
| F-4 | Important | decision inconsistency | Dependency can't be asserted on shapeless stubs | Folded into D4: trigger split — leaf via map relation, stub via flagged-unverified PM judgment |
| F-5 | Important | coverage + inconsistency | "Every selection card" unfulfillable across unrelated specs | Folded as D3a: two-site scope (territory-touching specs + stewardship touches); consequence accepted, consistent with D3 |
| F-6 | Minor | coverage | No phase-boundary quality bar | Folded into D3 amendments: phase-leaf independently useful |
| F-7 | Minor | decision | Escalation wording brushed PM-never-selects | Folded into D3 amendments: recommendation surfaced to user |

Reviewer's rejected-roads note: D1's option-B rejection shared F-1's blind spot (repaired with D1's amendment); D3's Contested honesty called the record's strongest point.

## Open threads

1. **Silent non-delivery of non-blocking remainder** — user acknowledges the rot risk but ruled this session the wrong place to solve it (D3). Candidate future capture: where does ledger-honesty pressure belong if not the selection card?
