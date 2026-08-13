# PM Role & Feature Derivation — Decision Record

**Status:** accepted (2026-08-13 — acceptance explicitly covered the adoption pattern as deliberate delegation and the sibling-session supersession)
**When:** 2026-08-13
**Topic:** the product-manager role across mochiko workflows, redesigned broad — (1) should the PM seat sit in specify at all, or in the feature workflow, or elsewhere; (2) story-to-feature derivation is producing features that mirror stories instead of being informed by them (kinako evidence); (3) what advantage parent/child feature breakdown actually buys.
**Driver:** user's read of the kinako dogfood map (https://github.com/humaninloop-dev/kinako) — "user stories are mapping to features way too closely, rather than features being informed."

## Ground facts

- **F1 — kinako mapping is near 1:1.** Kinako's map: 10 entries — 3 parents (FEAT-001 runtime readiness · FEAT-004 strategy interrogation session · FEAT-008 corpus) + 7 leaves. One spec (`onboarding-journey`), 9 stories, all 9 homed onto the 7 leaves at 1–2 stories per leaf: US-1/US-2 → FEAT-002 · US-3 → FEAT-003 · US-4/US-5 → FEAT-005 · US-6 → FEAT-009 · US-7 → FEAT-010 · US-8 → FEAT-006 · US-9 → FEAT-007. No story shares a leaf with a story from a different journey stage; leaf boundaries track the story boundaries.
- **F2 — doctrine already forbids what F1 shows.** `mochiko:authoring-feature-map`: "A **feature is the built thing**: a capability of the system described in the system's own language — not a cluster or regrouping of user stories. Stories inform *which* features get built and sharpen their extents; they never define them." Red flag: "'Every story maps to its own new feature' — the filter never fired; features are capabilities, not story mirrors." The kinako outcome happened *under* this doctrine.
- **F3 — the PM seat exists in exactly one place.** `grep -rn "product-manager" plugins/mochiko` → the agent file, `commands/specify.md` (derivation + filter + selection-card seat after stories), the router skill, and layering mentions in `authoring-requirements`/`authoring-user-stories`. No PM seat in `plan.md`, `implement.md`, `setup.md`, or `feature.md` — `/mochiko:feature` stewardship writes are lead bookkeeping.
- **F4 — specify's ordering is stories-first.** `specify.md` harness: intent stage (map an obligated read) → stories drafted (lockstep with prototype) → "**Derivation + filter after stories**" — the PM derives features *from the drafted stories* against the map. Stories-first derivation was ruled at `feature-map-layer` D7.
- **F5 — open-session overlap.** `feature-map-granularity-and-reparenting` (2026-08-12, status: open) holds two threads — map-level granularity balance (narrow vs broad) and re-parenting semantics for in-flight/delivered entries. Topic thread (3) (children value) sits underneath both: if nesting's value is re-ruled, that session's threads reframe.
- **F6 — decision lineage touching this territory.** `feature-map-layer` (D7 stories-first, D14 `product-manager` wide remit `Contested`) · `feature-sizing-and-entry-points` (D2–D3 two-level nesting, leaf = pipeline unit) · `pm-requirements-stacking` (D1–D4 phasing/stub machinery) — all accepted 2026-08-09/10.

## Decisions

### D1 — Root cause of story-feature mirroring: leaf-as-pipeline-unit primary, input starvation amplifier

**Statement:** the mirroring disease is structural, two causes ranked. Primary: **the map-leaf is defined as the pipeline unit** (`authoring-feature-map` vocabulary table: leaf = "Pipeline unit … graduates through plan/implement as its own unit"), so the product map moonlights as the delivery ledger — leaf grain is dragged down to deliverable-chunk grain, and cut boundaries land where delivery chunks land. Amplifier: **input starvation at derivation** — the PM derives downstream of freshly drafted stories, on a (greenfield) map with nothing else speaking — no product thesis, no domain shape, no architecture voice — so the only available chunking signal *is* the story set; story boundaries become leaf boundaries. Seat-position-inside-specify is subsumed into the amplifier, not a separate cause. Doctrine wording is not the defect — the mirroring happened under doctrine that already forbids it (F2).

**Evidence:** kinako leaf set tracks story boundaries 1:1–2:1 (F1) while kinako *parents* (runtime readiness · interrogation session · corpus) are genuine capability names — the deriver can think in capabilities, and fails exactly at the level that must be delivery-sized. Nesting itself was minted by `feature-sizing-and-entry-points` to serve pipeline sizing pain ("too-coarse AND too-flat"), confirming the leaf level answers to delivery, not product truth.

**Confidence:** `Unsure` — user stated "I am unsure the root cause, that's why I want to re-imagine it" and explicitly delegated to the lead's confident read ("I will go with your read if you are confident"). Lead confidence stated and reasoned; user ownership provisional. Adoption #1 — ratification-streak watch armed.

**Review fold (C1 + I3, user-ruled):** the disease restates at the **published-surface tier**: the defect is story-grain entries living as *permanent published features* on `FEATURES.md` — under the old model a delivered leaf never leaves, so the map permanently asserts "the product does 10 things," 7 of them story-grain. Leaf *shape* was never the disease — the C1 steelman is recorded: kinako's capability tier (the 3 parents) was already clean, and its leaf shapes are harmless under D2's own principle. The cure's warrant is therefore **published-surface honesty + map convergence**, independent of the causal ranking. The primary-vs-amplifier ranking is explicitly **undiscriminated by the n=1 evidence** (both readings consistent with kinako); D11's probe is the discriminator; the ranking is provisional and the mark stays `Unsure`.

### D2 — The map holds durable capabilities plus transient work rows; "feature" is reserved for capabilities

**Statement:** the feature map keeps **one surface, two row types**. **Capabilities** are the durable entries — the only thing called "feature"; capability-grain (kinako-grain: runtime readiness · interrogation session · corpus), honest extents, permanent. **Work rows** are transient typed rows under their capability — the delivery increments currently being built; explicitly *allowed* to be story-shaped (story-shaped delivery chunks are harmless; story-shaped capabilities are the disease, per D1's reframe). At a work row's delivery landing, its content **folds into the capability's extent and the row disappears**. The map converges to pure capabilities as work ships. Delivery visibility stays on the map (selection card, `/mochiko:feature` stable-ground triage keep one place to read); the sticky-delivered invariant survives naturally as "capability `delivered`, live work rows visible, no status regression."

**Alternatives rejected:** capabilities-only map (loses at-a-glance delivery state; blinds triage and selection consumers) · keep-shape-fix-inputs-only (doctrine already forbade mirroring and lost; structure beats words).

**Consequences:** parent/leaf nesting as shipped by `feature-sizing-and-entry-points` D2–D3 reframes — the old "parent" concept becomes the capability, the old "leaf" stops being a feature and becomes a work row; supersessions owed at build. Kinako-style maps (10 "features") would restate as ~3–4 capabilities + work rows.

**Confidence:** `Confident` — user ruled "B more aligns" after a plain-language re-explanation with worked example; engaged choice, not passive adoption.

**Review fold (I3 + I5, user-ruled):** D2 **stands on its own merits regardless of D1's causal ranking** — an honest published surface (only capabilities asserted as what the product does) and a converging map are worth having under either root cause; D2's `Confident` mark rests on those merits, not on D1's `Unsure` ranking. Work rows have **two states**: *pending* — cut but undelivered; they **persist on the capability entry** as its open-obligation view (the completeness ledger from `pm-requirements-stacking` D3 re-homes here, with the desk's health report carrying the re-surfacing obligation and dependency-triggered escalation surviving PM-side as recommendation-never-forced-cut) — and *live* — selected into a run; fold-and-vanish applies at the delivery landing. Deferred work never silently disappears.

### D3 — Map growth discipline: extend-beats-mint plus a soft cap with grooming trigger

**Statement:** two rules govern top-level growth. **Extend beats mint** — a new story first tries to grow an existing capability's extent; a new capability is minted only when the product starts doing a new *kind* of thing. **Soft cap ~9** — past roughly nine top-level capabilities, a grooming pass runs (merge lookalikes, retire dead entries); the cap is a trigger, never a hard block. Within-capability roll-up is automatic under D2 (work rows fold into extent at landing; no status regression).

**Confidence:** `Confident` — user: "I agree with extend beats mint, thats key. I think the softcap is good idea too." Grouping tier ("area headers") not adopted — user unsure, reworked as D4's domain question. Minimalism-carrier skill noted as recommended, ruling pending with D4.

### D4 — Capability definition tests; domains as the meta-tier; carrier is a new `patterns-map-minimalism` skill

**Statement:** three parts, ruled as one package.

*(a) Capability tests* — four tests define what earns a capability entry: **system's language** (what the product does, never who wanted it) · **noun + verbs** (a product-owned noun plus what it can do; a new verb on an existing noun extends, never mints) · **durability** (still true and meaningful after every current story ships and is forgotten — kinako's FEAT-007 "durability and resumption" fails this; Corpus passes) · **new-kind-vs-more-of-same** (mint only when the product starts doing a new kind of thing).

*(b) Domains replace ad-hoc areas* — the map's grouping tier, when it exists, is **domains**: parts of the product's world owning their own nouns and rules (kinako: *Sessions*, *Knowledge*). Every capability lives in exactly one domain. **PM proposes domain names, principal-architect co-signs** — problem-space view meets seam view; a disagreement is an early design conversation, not a defect. `ARCHITECTURE.md` carries one mapping line per domain (domain → realizing components); the two views stay independent, connected by the mapping line. **Domains are minted only at cap-trip** (~9+ capabilities, per D3) — a 3-capability map stays flat; no structure before it is paid for.

*(c) Carrier* — a new `patterns-map-minimalism` skill carries the whole discipline (capability tests, extend-beats-mint, soft cap, domain rules), same library shape as `patterns-code-minimalism` and `patterns-plan-minimalism`: PM applies at derivation, spec reviewer grades it, `/mochiko:feature` grooming enforces at cap-trip.

**Confidence:** `Confident` — user ruled "Accept." on the package; the domain/architect half originated from the user's own probe ("is there some form of meta categorisation for capability that the principal architect can contribute to?"), so ownership is engaged, not passive.

**Review fold (I10 + M13, user-ruled):** domains ship **keep-but-shrunk** — the principle (domains at cap-trip, PM proposes + architect co-signs, one capability per domain) lands as a **dormant paragraph in `patterns-map-minimalism`**; the `ARCHITECTURE.md` mapping-line machinery and the principal-architect persona edit are **deferred to the first real cap-trip** (no ripple into the v0.67.0-rechartered persona until a map actually approaches ~9). The four tests are **ranked**: system's-language, durability, and new-kind-vs-more govern; **noun + verbs is a heuristic aid, never a gate** — a genuine cross-cutting capability (e.g. an onboarding journey) passes via the governing tests.

### D5 — The PM stays in specify, repositioned to the front; grooming seat in `/mochiko:feature`

**Statement:** the answer to "should the PM be part of specify at all?" is **yes — at the front, not the back**. Two touchpoints inside specify: **(1) capability framing at intent** — before any story is drafted, the PM reads the map plus the intent and states the frame (which capabilities the territory touches, extend-vs-mint hypothesis per D3/D4 tests), agreed with the user; stories are then authored inside that frame (the posture `authoring-user-stories` already names but no seat currently builds first). **(2) After stories** — confirm or adjust the frame against what the stories revealed, cut the work rows (story-shaped fine, per D2), run the filter, build the selection card. Additionally the PM is named the **grooming seat in `/mochiko:feature`** when D3's soft cap trips (merge/retire proposals; user rules). Specify remains the single ratification moment for delivery-driven map writes — no new command choreography.

**Alternatives rejected:** PM moves wholly to `/mochiko:feature` (splits the atomic acceptance-time map write; mid-run cross-command calls) · no PM seat anywhere (nobody owns the capability layer; the filter loses its independent voice).

**Rationale:** capability thinking placed before stories exist removes the anchoring surface — D1's amplifier dies structurally, not by doctrine words.

**Confidence:** `Confident` — user ruled "as recommended". Adoption #3 of a lead recommendation (D1 delegated, D5 verbatim); ratification-streak flagged to the user at the next turn per the streak discipline.

**Review fold (I4, user-ruled):** the intent-time frame is a **hypothesis, not an anchor** — it states capabilities as **nouns + verbs only and never enumerates stories**, so it cannot dictate story boundaries; story authoring stays journey-driven. **Stories win any conflict with the frame** (stories are evidence about the world; the frame is a lens), resolved at the D5.2 confirm-or-adjust step. On a thin greenfield intent the frame's sources are named: the intent conversation itself, the product description's domain nouns, and D4a's tests. The reverse-anchor risk (a wrong frame shaping stories the adjust step cannot un-author) is **priced and accepted as a bet**; D11's probe and first-live-run watch are its check.

### D6 — Parent/leaf nesting is superseded; the old advantages re-homed (consequence of D2)

**Statement:** the two-level parent/leaf nesting shipped by `feature-sizing-and-entry-points` dies. Its advantage claims re-home: one-breath navigation → the capability entries themselves (few and one-breath by D4's tests) · status roll-up → D2's automatic fold (work rows fold into extent at landing) · pipeline keying → re-ruled at D7 · sticky-delivered-parent-with-deltas → "capability `delivered`, live work rows visible, no regression". **Supersessions owed at build** (statement's original "non-exhaustive" qualifier superseded at the I9 fold — the exhaustive per-clause inventory in the review fold below is authoritative): `feature-sizing-and-entry-points` D2–D4 (nesting · leaf-as-pipeline-unit · retroactive promotion) · `pm-requirements-stacking` phases-are-leaves clauses · `authoring-feature-map` vocabulary table + nesting invariants · `feature-entry-template.md` / `features-index-template.md` shapes · `feature.md` promotion remit.

**Confidence:** `Assumed` — derived by the lead as D2's direct consequence; veto affordance given at presentation, explicit confirmation rides record acceptance. **Firmed to `Confident` at the review disposition** — the user's ruling on I9 ("as recommended", covering "you explicitly confirm D6") is the explicit word.

**Review fold (C1 + I9, user-ruled):** the statement reframes honestly — this is **re-typing + transience, not teardown**: the two-tier structure survives as capability + work rows; what dies is **leaf-as-published-feature** and **leaf-as-permanent-entry**. The supersession inventory closes to **exhaustive, per-clause** (replacing "non-exhaustive"):

- `feature-sizing-and-entry-points` — D2 (two-level parent/leaf nesting) *superseded*; D3 (leaf = pipeline unit; sticky-parent delta-carry) *superseded* — delta-carry re-types onto pending/live work rows; D4 (parent minting both directions + retroactive promotion) *superseded*; D9's per-feature run-dir keying *amended* — `.mochiko/features/FEAT-XXX/` run dirs re-key to the capability; D9's two-altitude product baselines *survive untouched* (D7 fold); D15 graded folds *survive*, extended to row folds.
- `pm-requirements-stacking` — D1 across-round phasing (phases-are-leaves) *superseded* — phases become work rows; D2/D2a confident-portion-to-leaves *amended* to confident-portion-to-work-rows; **stub-maturation-specify-only survives** — the desk's growth door cuts rows on existing capabilities, which is not stub maturation; D3 completeness ledger *re-homed* (pending rows on the entry + desk health report; dependency-triggered escalation survives, recommendation-never-forced-cut); D3a re-surfacing scope *re-homed* (desk health report + specify territory touches); D4 split-claims *survives* at capability level.
- `feature-map-layer` — D7 stories-first derivation *amended* to frame-first-hypothesis + stories-inform (D5); D17 one-plan-run-per-feature *superseded* by capability-batch (D7); D4/D22 the-feature-is-the-pipeline-unit *superseded at the leaf tier* — the capability-batch is the run unit.
- `authoring-feature-map` — vocabulary table parent/leaf rows *superseded* (capability / work-row); invariant 5 sticky-delivered delta-carry *re-typed* (pending/live rows; stickiness survives as no-status-regression); invariant 6 write-timing *survives re-worded* for rows; nesting red flags + two-level sizing bars *re-keyed* to capability entries; parent-status roll-up clauses *superseded* by the fold.
- Templates — `feature-entry-template.md` parent/leaf shape *superseded* (capability entry + pending/live row blocks); `features-index-template.md` leaf sublines *superseded* (work-row sublines; domain headers dormant per D4 fold).
- `feature.md` — retroactive-promotion remit *superseded*; map-write test *superseded* by the capability-write test (D8); v8 shape *superseded* by the charter (D10).
- `specify.md` — derive-after-stories clause *amended* (frame-first + confirm, D5); leaf selection *amended* (rows grouped per capability, D7).
- `plan.md` / `implement.md` — leaf keying *superseded* (capability-batch, D7); implement's landing fold *extended* to execute row folds; baseline-delta folds *unchanged*.

### D7 — The pipeline keys to a capability-batch

**Statement:** the selection picks work rows; **each capability with selected rows gets one plan run covering exactly those rows** (e.g. "Runtime readiness — increment 1" covering the two selected preflight rows). Run size stays where today's leaf runs sit, so plan-the-plan and `patterns-plan-minimalism` machinery apply unchanged. Implement's acceptance landing folds the delivered rows into the capability's extent (D2's fold moment). Rejected: plan-per-work-row (near plan-per-story; per-run gate overhead the plan-YAGNI ruling removed) · plan-per-spec (big plans return — the disease `plan-structure-yagni` treated).

**Confidence:** `Confident` — user ruled "as recommended". Adoption #4; streak flagged to the user at D5 and continued past the flag — read as deliberate delegation, to be covered explicitly at record acceptance. D7's `Confident` rests on its own merit (run-size preservation), not on D1's ranking (I3 fold).

**Review fold (I6 + M12, user-ruled):** the **two-altitude product-baseline machinery is scoped in and survives untouched** — `.mochiko/product/` baselines (`data-model.md` · `contracts/` · `nfrs.md` · `constraints-and-decisions.md` · `quickstart.md`) and their appliable before/after deltas keep working exactly as shipped; **baseline-delta folds fire at the same acceptance landings that fold work rows**; per-feature run dirs re-key to per-capability (`.mochiko/features/FEAT-XXX/` → the capability's dir). **Cross-capability ordering:** dependency closure (invariant 3) carries to rows — selection honors row-level dependencies, and a row in capability A depending on a row in capability B orders those two capability-batch runs; the map's relations machinery is unchanged.

### D8 — `/mochiko:feature` gains the growth door; the boundary re-keys to a capability-write test

**Statement:** the capability lifecycle's missing door — **extent growth on an existing capability** ("export corpus to Obsidian": a new verb on an existing noun, one or few work rows) — lives in `/mochiko:feature`, not specify. The lane boundary re-keys from "no map write" to **"no capability write"**: capabilities are the sacred layer (minting, merging, retiring, capability-status change → specify or a user grooming ruling); work rows are delivery bookkeeping the desk may cut. At the desk, the PM seat runs the same extend-vs-mint test (`patterns-map-minimalism`, D4): *extend* → cut work row(s) with acceptance criteria on the card, dispatch a D7 capability-batch plan run ("Corpus — increment 2"); *mint or uncertain* → route to specify. Scale rule: several rows, a new UX surface, or cross-capability reach → specify regardless. Bug/improvement lane machinery survives unchanged beneath it. "The plan delta" = the row set + its card; every door ends in the same D7 run shape, so plan/implement are untouched by which door admitted the work.

**Alternatives rejected:** specify-owns-every-row-write (full intent/stories/prototype for one row — heavy process invites bypass) · adaptive micro-specify (one command wearing two unlike run shapes).

**Confidence:** `Confident` — user ruled "as recommended" (adoption #5, post-flag; deliberate delegation, covered at acceptance). Immediately followed by the user reopening scope: "relook at the feature workflow, and reimagine what it does" → Q9.

### D9 — The desk's identity: front door + health desk (A+B); what-next advice as a report line, not machinery

**Statement:** `/mochiko:feature` takes two stacked identities. **Front door (A):** every incoming demand — bug, idea, growth, new-shape capability — starts at the desk and is routed: lane, growth door (D8), or specify for new shape; the user never chooses the entry command again. **Health desk (B):** opening the desk surfaces map state before the ask — stale stubs, unfolded deltas, cap pressure (D3), domain gaps (D4) — proactive grooming, not grooming-on-trip-only. A light **what-next line** rides the health report (PM's cross-map read: parked stubs, undelivered rows, deferred criteria) — deliberately a report line, never standing roadmap machinery (C rejected as machinery; D rejected as leaving the map to rot).

**Confidence:** `Confident` — user ruled "A + B as recommended" (adoption #6). Immediately followed by a format-reimagining ask → Q10.

**Review fold (I8, user-ruled):** the front door is **advisory, not mandatory** — `/mochiko:specify` stays directly invocable (its intent stage + PM frame covers direct entry); the desk is the **default entry when the user is unsure** and the **only door for growth, bug, and improvement intake** (those lanes live nowhere else). D9's "never chooses the entry command again" softens to: the desk is a routing service you can always use, never a gate you must pass.

### D10 — `/mochiko:feature` re-formats as a charter: Delivery Manager lead, adaptive per-visit goals converging to a done condition, floor carried as responsibility

**Statement:** the command abandons the v8 Goal · Harness · Bindings anatomy (recorded supersession, **this command only** — the other five stay v8) for a six-section **charter**:

1. **Identity & Mission** — the lead is chartered **Delivery Manager of the product desk**: owns routing, pace, and follow-through on the capability layer; writes no capability truth alone. (Symmetry: plan's lead is already "delivery manager of the goal", `plan-structure-yagni` D1 — the posture extends to a standing surface.)
2. **Adaptive Goal Protocol** — open with the health report (D9) + the ask; a micro-brainstorm **converges to a one-line visit goal *and its explicit done condition***, agreed with the user; the visit then runs to that done condition. Goal-per-visit, never goal-less; a visit ending with no stated done-condition verdict is a defect.
3. **Roles & Responsibilities** — **no Bindings section exists**; the bare minimum that must always happen is carried as the Delivery Manager's *owned responsibilities* (user amendment): surface health first · converge every visit to a done condition · keep map integrity intact at close · route honestly by the capability-write test (D8) · hand dispatched runs complete cards · execute the KM landing for desk-side writes where KM exists · close the visit with a verdict. Everything beyond the minimum is the DM's per-visit judgment — the v7 lead-owned-process-flexibility philosophy applied to command anatomy. Other roles: PM seat (extend-vs-mint, grooming proposals, what-next line) · principal-architect (domain co-sign) · dispatched runs (all delivery) · the user (retire/merge rulings, route overrides, selections).
4. **Tools** — map files, `patterns-map-minimalism`, capability-write test, stable-ground triage, delta cards, dispatch targets — referenced never restated; absorbs the old Bindings' paths and templates as tools-with-homes.
5. **Ways of Working** — proactive report first · reference-never-restate · author≠grader wherever a seat produces · suggest commits, never push · single-flight product lane.
6. **Boundaries** — the non-waivable floor: capability writes sacred · no delivery harness at the desk, dispatch only · no self-graded writes · no silent map mutations.

**Audit re-key:** the author≠grader command audit grades this command against *floor present + per-visit goal contract present* instead of "default-FAIL goal present"; criteria adaptation lands with the build.

**Confidence:** `Confident` — user amended the proposal with two design moves of their own (done-condition convergence as the protocol's object; Bindings dissolved into DM responsibility) — engaged co-design, not adoption.

**Review fold (I7, user-ruled):** the charter is **reaffirmed with its costs on record**: it breaks the days-old library-wide v8 uniformity for one command (a bespoke shape the audit machinery needs a re-keyed criterion for); it supersedes a command shipped at v0.61.0 whose gates were never exercised live (first-live-run watch open — churn priced and accepted); and the charter-within-v8 steelman (Goal = the per-visit contract protocol · Harness = roles · Bindings = responsibilities) was weighed and rejected — the user's format break is a deliberate design ruling, and a standing desk is honestly a different kind of thing than a pipeline stage.

### D11 — Evidence honesty and the success probe (born from C2)

**Statement:** the session's driver evidence is marked honestly: **n=1 — one greenfield spec on one dogfood repo, with nothing delivered** — kinako's fold-at-landing behavior has never executed, so D2's convergence claim is a design bet, directional only. Two checks gate the build's validation: **the probe** — re-derive kinako's map under the new model; directional target: the 10-entry map restates as **~3–4 capabilities plus work rows**; and **the first-live-run watch** — the first implement landing that executes a row fold validates fold mechanics (both land as a BACKLOG watch item at the landing ritual). The probe is also D1's discriminator: if capability-grain derivation under the new inputs still mirrors stories, the ranking was wrong and the structural bet gets re-examined.

**Confidence:** `Confident` — user ruled "as recommended" at the review disposition.

### D12 — Capability-merge mechanics (born from M14)

**Statement:** when grooming merges two capabilities (always a user ruling, per D3): the **absorbing entry's ID survives**; the merged entry flips `retired`, dated, carrying a *merged-into* pointer — never deleted; extents union under an honesty pass (no flattering weld); story traces and SC references consolidate onto the absorbing entry; pending rows and unfolded deltas transfer. Re-parenting under a domain header is a navigation-only text edit (D4b) with no status semantics. This completes the answer the open sibling session (`feature-map-granularity-and-reparenting`) was parked on; that session closes as superseded at this record's landing.

**Confidence:** `Confident` — user ruled "as recommended" at the review disposition.

## Session trail

- **Q1** root cause → user unsure, delegated; D1 (leaf-as-pipeline-unit primary, input starvation amplifier).
- **Q2** map object model → re-asked in plain language with kinako worked example at the user's session-wide simple-language instruction; user ruled B; D2.
- **Q3** growth package → extend-beats-mint + soft cap adopted (D3); area headers killed by the user's probe — "what is an area?" + "can the principal architect contribute meta-categorisation?"
- **Q4** capability tests + domains + carrier package → "Accept."; D4.
- **Q5** PM seat → "as recommended"; D5 + D6 recorded as consequence; ratification streak flagged.
- **Q6** pipeline keying → "as recommended"; D7. First convergence.
- **Q7** review-sizing gate offered → user deferred: more angles — feature-workflow role, capability lifecycle, growth deltas. Scope reopened.
- **Q8** growth door → "as recommended" (B); D8. User then asked to reimagine `/mochiko:feature` wholesale → Q9.
- **Q9** desk identity menu → "A + B as recommended"; D9. User then asked for a full format reimagining: delivery-manager lead, adaptive goal from brainstorming, roles/responsibilities/tools/ways-of-working anatomy → Q10.
- **Q10** charter anatomy proposed with vignette + two doctrine flags (v8 supersession this-command-only; adaptive-goal risk guard) → user amended: convergence-to-done-condition made the protocol's object; Bindings dissolved, bare minimum carried as DM responsibility; D10. Second convergence.
- **Q11** review gate → "solo review as recommended". Blind-map dispatch executed (35-angle map returned before the record path was sent). Cold read → **FAIL**, 14 findings (2 Critical, 8 Important, 4 Minor).
- **Disposition** — 14/14 ruled in one user batch ("as recommended"): C1/I3 → D1/D2/D6 repaired to the published-surface framing with the steelman recorded and the ranking marked undiscriminated; C2 → new D11 (honesty marker + probe + watch); I4 → D5 frame-as-hypothesis fold; I5 → D2 pending/live rows + ledger re-home; I6/M12 → D7 baselines scoped in + ordering; I7 → D10 reaffirmed with costs; I8 → D9 advisory front door; I9 → D6 exhaustive inventory + firmed `Confident` by user word; I10/M13 → D4 domains shrunk + tests ranked; M14 → new D12 merge mechanics; M11 lead-repaired pre-disposition. Bounded verify assigned to the reviewer (fold integrity + D11/D12 record-fitness; no fresh cold read, no second reopen).
- **Verify** — NOT CLEAN: 3 non-blocking hygiene defects (the M11 duplicate `## Open threads` survived the claimed repair as a populated duplicate · D6's statement still carried the superseded "non-exhaustive" phrase · the build-surface sibling line missed the D12 cite) — all three lead-repaired same round; 13/14 folds verified executed and consistent; D11/D12 record-fitness PASS; reviewer: clearable with the touches applied, no re-review needed.

## Build surface

*Cold-buildable inventory; supersession strips owed where marked.*

- **New skill `patterns-map-minimalism`** (D4c) — capability tests (D4a, ranked per the M13 fold: noun+verbs heuristic-only), extend-beats-mint + soft cap ~9 grooming trigger (D3), merge mechanics (D12), and the **dormant domain paragraph** (D4b as shrunk at I10: cap-trip minting only, PM proposes + architect co-signs; the `ARCHITECTURE.md` mapping-line machinery and the principal-architect persona edit are deferred to the first real cap-trip). Readers: PM at derivation, spec reviewer, `/mochiko:feature` grooming.
- **`authoring-feature-map`** — core rewrite: capability/work-row vocabulary replaces parent/leaf; frame-first derivation (D5); fold-at-landing rules (D2); nesting invariants and red flags re-keyed. *Strips owed* (vocabulary table, nesting clauses, promotion machinery).
- **Templates** — `feature-entry-template.md`: capability shape + transient work-row block. `features-index-template.md`: capability lines, work-row sublines, domain headers at cap-trip. *Strips owed.*
- **`specify.md`** — intent stage gains the PM capability-framing touchpoint (D5.1); post-stories step re-worded confirm-frame + cut-rows + filter + selection (D5.2); selection = work rows grouped per capability (D7). *Strips owed.*
- **`feature.md` — full charter rewrite** (D8–D10): six-section charter anatomy (Identity & Mission — DM charter · Adaptive Goal Protocol converging to a done condition · Roles & Responsibilities carrying the bare-minimum floor as DM duties · Tools · Ways of Working · Boundaries); front-door routing + health report + what-next line (D9); growth door + capability-write test re-key (D8); retroactive-promotion remit superseded (D6); PM grooming seat at cap-trip (D5); stub parking survives at capability level. **v8-shape supersession recorded for this command only**; audit criteria re-key (floor + per-visit goal contract in place of default-FAIL goal) lands with the build. *Strips owed (heavy).*
- **`plan.md` / `implement.md`** — re-key leaf → capability-batch (D7); implement's landing executes the fold. *Strips owed.*
- **Agents** — `product-manager.md`: framing-first posture line (persona already capability-first, light touch). `principal-architect.md`: **no edit now** — domain co-sign duty deferred to first cap-trip (I10 fold).
- **Prior-ruling supersessions** — **per D6's exhaustive per-clause inventory** (the I9 fold is the authoritative list: feature-sizing D2/D3/D4 + run-dir keying · stacking D1/D2a/D3/D3a re-homes with survivals named · map-layer D7/D17/D4-D22 leaf tier · authoring-feature-map clauses · both templates · feature.md · specify/plan/implement clauses). DECISIONS.md annotations at landing.
- **Probe + watch (D11)** — BACKLOG item at landing: kinako map re-derivation probe (directional ~3–4 capabilities + rows vs 10 entries) + first-fold live-run watch; both gate build validation and discriminate D1's ranking.
- **Open-session disposition at landing** — `feature-map-granularity-and-reparenting` superseded by D3/D4/**D12** (granularity → tests + soft cap; re-parenting → domain-header moves navigation-only per D4b, merges per D12's mechanics). User ruling at landing; index updated.
- **Migration** — none owed (breaking-change-no-migration precedent, `feature-sizing-and-entry-points` D10); kinako's map restates on its next touch.

## Review — solo cold review, blind-map dispatch (2026-08-13)

Reviewer spawned topic-only (35-angle blind map returned before the record path was sent); record read cold from the file; kinako driver evidence live-verified by the reviewer against the GitHub repo. **Verdict: FAIL (critical-gaps).** 14 raised — 2 Critical, 8 Important, 4 Minor. Blocking set: C1 · C2 · I9 · I3. M11 (duplicate `## Open threads` header) lead-repaired on sight pre-disposition.

| # | Sev | Anchor | Finding (one line) | Disposition |
|---|-----|--------|--------------------|-------------|
| C1 | Critical | D1/D2/D6 | Scope-warrant contradiction: record's own principle makes kinako's leaves harmless-shaped and its capability tier was already clean — the disease as written isn't shown by the evidence | folded — D1/D2/D6 repaired to published-surface framing; steelman recorded; scope kept (user-ruled) |
| C2 | Critical | whole/D4a/D5/D8 | n=1 greenfield evidence, nothing delivered (fold untested), no evidence-honesty marker, no success probe; core mechanism is the judgment that failed | folded — new D11: honesty marker + kinako re-derivation probe + first-fold watch |
| I3 | Important | D1→D2/D6/D7 | `Unsure` foundation under `Confident` structural dependents; primary-vs-amplifier ranking undiscriminated by the evidence | folded — ranking marked undiscriminated (D1); D2/D7 own-merits lines; D6 firmed by user word |
| I4 | Important | D5 | Front-loading paradox: intent-time framing has thinner signal and may reverse-anchor story authoring | folded — D5 frame-as-hypothesis; stories win conflicts; bet priced |
| I5 | Important | D2/D6/D9 | Deferred-work tracking (stacking D3 `Contested` ledger + re-surfacing + forced-disposition) silently regresses — no re-home stated | folded — D2 pending/live rows; completeness ledger re-homed; re-homes recorded in D6 inventory |
| I6 | Important | D7/D8 | Two-altitude `.mochiko/product/` baseline-delta machinery unaddressed by the capability re-key | folded — D7: baselines scoped in, folds at same landings, run dirs re-key |
| I7 | Important | D10 | v8 uniformity break for one 3-day-old command; charter-within-v8 never steelmanned, churn unpriced | ruled — D10 reaffirmed; uniformity + churn costs on record; v8 steelman recorded |
| I8 | Important | D9 | Front door mandatory vs advisory unresolved; collides with direct `/mochiko:specify` invocation | ruled — D9 advisory front door; specify stays directly invocable |
| I9 | Important | D6/build | Supersession inventory marked non-exhaustive on an `Assumed` decision — GI-005/006 blocker | folded — D6 inventory closed exhaustive per-clause; D6 `Assumed` → `Confident` |
| I10 | Important | D4b | Domains are machinery no near-term map triggers; cheaper deferred shape exists | ruled — D4 domains keep-but-shrunk; persona edit deferred to first cap-trip |
| M11 | Minor | record | Duplicate `## Open threads` header | lead-repaired |
| M12 | Minor | D7 | Cross-capability dependency ordering unspecified for capability-batch runs | folded — D7 cross-capability ordering via row-level dependency closure |
| M13 | Minor | D4a | Noun+verbs test may misfit cross-cutting journey capabilities | folded — D4 noun+verbs demoted to heuristic; journeys pass via governing tests |
| M14 | Minor | build | Sibling supersession preempts its reparenting thread with merge mechanics undefined | folded — new D12 merge mechanics; sibling closes as superseded at landing |

Coverage diff: reviewer's angles 15/16→C2, 34→C1, 28→I5, 29→I6, 24→I7, 18→I10, 32→M14; angles 6/27 already deferred as record open threads; angles 7/11/19/23/26/31 killed at the reviewer's materiality bar (steelmans retrievable).

## Open threads

1. **PM touchpoint at plan** — the map's "plan confirms and hardens" touchpoint: does the capability frame get a confirm seat inside plan runs, or does the plan lead carry it? Deferred to build-time judgment.
2. **SCR/FLOW tag grammar** — prototype manifest tags currently key FEAT-XXX; under D2 they key capability + work row. Build detail.
3. **Setup brownfield bootstrap** — reconstruction now targets capabilities, not leaves; first brownfield run is the watch.
4. **Stub (`unrefined`) machinery** — survives as parked capability hypotheses; exact shape at build.
