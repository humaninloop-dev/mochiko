# Charter ritual balance — decision record

**Topic:** tightening the charter-form command architecture (`feature.md` v0.68.0; `plan.md` / `implement.md` v0.69.0) — what form of *ritual guidance* the charters need, balancing the lead's freedom to build its own harness (the lead-owned-process-flexibility posture) against obligated rituals (independent seats, independent review, dispatch).

**When:** 2026-08-13 · **Status:** accepted (2026-08-13 — acceptance explicitly covered the Q3–Q5 adoption streak as deliberate delegation and the three `Contested` rulings as deliberate choices against recommendation)
**Driver:** the kinako `/mochiko:feature` dogfood run of 2026-08-13 — two visits (a grooming pass, then a from-scratch whole-map re-derivation) executed with zero agent dispatches; the user read it as a regression. Diagnosis (this session, pre-brainstorm): not a regression — designed behavior; the exposed gap is the vanished independent-review leg, not dispatch per se.

## Ground facts

All verified this session against the repo and the pasted kinako transcript.

- **F1 — the kinako run dispatched nothing.** Both visits ran lead-inline: the lead loaded `patterns-map-minimalism` and `authoring-feature-map` itself, authored the grooming findings and the full 10-entry map re-derivation proposal itself, and executed the adopted rewrite itself. The transcript's "PM seat recommends" was the lead wearing the hat — no `product-manager` agent existed in the run.
- **F2 — `feature.md` binds no agents.** `grep product-manager plugins/mochiko/commands/feature.md` → 0 hits; "PM seat" is a role label with no agent binding. The charter grants staffing explicitly: "how you staff, sequence, and run the visit is yours to shape" (Roles & Responsibilities, `feature.md:42-45`). Same zero-binding is true of `plan.md` and `implement.md` (0 grep hits each, pre- and post-charter); `specify.md` names `product-manager` twice.
- **F3 — desk staffing is ruled, not accidental.** `pm-role-and-feature-derivation` F3: "`/mochiko:feature` stewardship writes are lead bookkeeping." Its D5 scopes the desk PM seat to cap-trip grooming proposals + the what-next line; the charter marks principal-architect "dormant until the first cap-trip." The cap (3 of ~9) never tripped in the kinako run.
- **F4 — the re-derivation bypassed independent review legally.** Boundaries allow capability writes via "a user grooming ruling"; the user's "adopt" was the only check on a 10-file map rewrite. The identical derivation inside `/mochiko:specify` is graded by `devils-advocate` × `review-specifications` (derivation honesty, delta legality, dedup). The desk's ruling door carries no scale bound and no review seat. Letter of author≠grader held (the user cleared, not the author); the sound-loop's independent-reviewer leg was absent.
- **F5 — the pipeline charters kept their teeth; the desk's are thinner by design.** `plan.md`/`implement.md` charters retain per-run done conditions with the prior `**Not done — default FAIL**` lists verbatim (ADR `2026-08-13-charter-plan-implement`; audit re-key four-point definition). `feature.md`'s audit re-key is floor + *per-visit* goal contract only; its floor names "No self-graded writes" but prescribes no reviewer for desk-side writes.
- **F6 — transport-neutrality is a standing ruling.** `command-architecture-realignment` D5 (2026-08-02) superseded the hard-require-agent-teams bet: teammates vs subagents is the lead's per-seat call. Any ritual that *mandates dispatch* supersedes or narrows D5 and must say so. The same session's D1 ruled encoded choreography itself the defect — this session sits consciously on that pendulum.

## Decisions

### D1 — Disease: vanished independent review + lead-inline production at judgment weight; remedy: a kind-keyed ritual floor

**Statement:** the target defect is the sound-loop's independent-reviewer leg vanishing at judgment-heavy charter moments (F4), compounded by the lead producing the judgment artifact itself (F1). The remedy is a **kind-keyed ritual floor**, not a blanket dispatch mandate. When the D2 trigger fires, three rituals become non-waivable:

1. **Production sits with a seat, never the lead** — the producing seat plans first; the lead gives feedback, approves the plan, and distributes work to seats. *(User-authored leg — beyond the lead's recommendation. **Re-affirmed at cold review (I7) with the library-wide cost in view:** the reviewer's steelman — doctrine demands only an independent reviewer, so lead-produces + seat-reviews would satisfy author≠grader — is a recorded rejected road; the user held that a lead which authors, adjudicates the review of its own artifact, and presents it concentrates capture in one context, and that leg 1 is the delivery-manager identity itself.)*
2. **Independent review** — the produced artifact is graded by a non-author seat before the user's gate; the user's ruling alone never substitutes for the review leg at this weight.
3. **The user gate stays** — rulings reserved to the user remain theirs; the floor adds review beneath the gate, never a machine gate above it.

When the trigger does not fire, the lead's inline freedom stands as chartered. **Vocabulary note (review fold, I2):** Q1's "scale trigger" intent deliberately resolved into a *kind* test at D2 — size thresholds (t3) were rejected, so a small judgment-authored write on a governing surface trips the floor as surely as a 10-file rewrite; magnitude never gates it. **Price (review fold, I6):** a qualifying visit costs one producer dispatch + one review round; under-classification (the lead calling judgment work "mechanical" to stay inline) is mitigated by D4's close-report disclosure — the floor call is visible and auditable per visit — and the residual risk is accepted. Transport stays neutral: a seat may be a teammate or a subagent (`command-architecture-realignment` D5 untouched at the transport level) — but the *lead-absorbs-the-seat* reading of "how you staff … is yours to shape" dies when the D2 trigger fires; that clause takes a recorded narrowing at build.

**Rationale:** the four sound-loop rules already name the independent reviewer non-waivable; the desk lost that leg legally through the staffing-freedom clause + the "user grooming ruling" door (F4, F5). Mandating dispatch everywhere (option b) refights the choreography ruling (`command-architecture-realignment` D1); cosmetics alone (option d) leaves the review hole open. Role-claim honesty ("PM seat recommends" written by the lead) rides along: above the floor the claim must be true — a named seat exists or the lead speaks as itself.

**Confidence:** Confident — user ruled "as recommended yes" and authored leg 1's shape.

### D2 — The trigger: durable judgment artifact (library-wide, two-part test)

**Statement:** the ritual floor fires **anywhere in the library** when both parts are true of the output about to be produced:

1. **Judgment-authored** — producing it required real judgment; mechanical execution of an existing ruling, transcription of user decisions, and fix-on-sight integrity repairs (typos, status agreement, dead pointers) never trip.
2. **Governing surface** — downstream work will read it as truth: the capability map (`FEATURES.md` + entries), product baselines (`.mochiko/product/`), specs, `ARCHITECTURE.md`, governance surfaces, plugin primitives, product code. *(List is the build's to pin; these are the ruled members.)* **Blast-radius obligation (review fold, I5):** before pinning, the build produces a per-member table — surface → existing regime → net-new obligation. Members already satisfying by construction are named as such (plugin primitives via the primitive-edits ceremony, product code via implement's seats, specs via specify's loop, governance via setup's loop); the floor's net-new bite is the desk and ad-hoc holes (capability map, baselines, `ARCHITECTURE.md` folds outside landings).

Both true → D1's three legs are obligatory. Either false → lead-inline freedom stands.

**Rationale:** generalizes the repo's own primitive-edit ceremony ("editing a shipped primitive is a landing, not an ad-hoc edit") from one surface class to all governing surfaces. Checkable at the moment of writing — no reference-surface comparison needed (h2's weakness), no consumption forecasting (h3's weakness); h2 survives as rationale ("the entry door never lowers the review"), h3 as the test's intuition. Kinako's re-derivation trips cleanly; its status-wording fixes stay inline.

**Edge clause — desk delta cards (ruled at review, I8):** the original `Assumed` exemption (delta cards skip the floor because plan's delta-scope entry "confirms" the card) is **dead** — the reviewer showed that confirm is the plan lead's consistency check, not an independent non-author review, so the exemption's premise failed. Ruling: **every desk-authored delta card — bug and improvement alike — takes the D1 review leg at the desk before dispatch.** Cost accepted with eyes open: bug intake gains a review round. The lead's lighter split (bug cards exempt via their reproduction-failing test, improvement cards trip) was rejected.

**Confidence:** Confident (trigger) · Contested (delta-card clause — user ruled stricter than both the original exemption and the lead's split recommendation).

### D3 — Carrier: new discipline skill `patterns-sound-loop`, single-sourced, referenced never restated

**Statement:** the floor lands as a new model-invoked discipline skill, working name **`patterns-sound-loop`** — fourth sibling to the three minimalism skills. It carries: the D2 two-part test and governing-surface list · D1's three ritual legs · the exemptions (mechanical execution, transcription, fix-on-sight) · the delta-card review-leg rule (D2 edge clause — no exemption) · the role-claim honesty rule (a report may only name a seat that actually existed; otherwise the lead speaks as itself). Delivery: each charter's Boundaries gains **one pointer line**; the router gains a row; the charter audit re-key (`.claude/rules/mochiko/primitive-edits.md`) extends to check the pointer's presence. No restatement anywhere.

**Rationale:** three-copies-in-Boundaries drifts (rejected); router-line-only has no runtime delivery force (rejected). A skill is the library's quality surface, single-sourced like `patterns-plan-minimalism` / `patterns-code-minimalism` / `patterns-map-minimalism`, and model-invocation means any session touching a governing surface can be pulled to it by trigger description — not only charter runs.

**Confidence:** Confident.

### D4 — Seat-wiring defaults live in the skill; deviation disclosed at close

**Statement:** `patterns-sound-loop` carries the **default wiring table** per governing-surface class — map work: `product-manager` produces, `devils-advocate` reviews · architecture/baseline touches: `principal-architect` produces, `tech-lead` reviews · (build completes the table from the existing persona library and axis-5 pairing doctrine: mirror-checklist for objective criteria, adversarial for judgment artifacts). The lead may swap personas for cause; **the visit/run close report names who sat where** — staffing becomes visible and auditable. Charters stay wiring-free: no Bindings section returns (the D10 charter-anatomy amendment stands).

**Rationale:** F1/F2 — with no name written anywhere, the kinako lead absorbed the seat; names give reliability. Charter-side wiring (rejected) reverses the user's own no-Bindings amendment; no-defaults (rejected) recreates the vacuum that produced the kinako run. One home in the skill keeps single-source discipline.

**Build obligation (review fold, I3):** the wiring table's review column must name a procedure that *works on a spec-less desk map write* — the existing feature-layer checklist assumes a spec, stories, and a selection card that desk writes lack. The build names the applicable subset (capability tests via `patterns-map-minimalism`, entry/delta legality via `authoring-feature-map`, derivation honesty against whatever stories exist) or the build FAILs — the review leg may not be unexecutable for the floor's own driver case.

**Confidence:** Confident. *(Adoption-streak note: Q3–Q5 ruled "as recommended" in sequence; flagged for the cold reviewer and for explicit coverage at acceptance.)*

### D5 — Runtime delivery: command pointer + skill triggers only, for now; rules-file leg deferred

**Statement:** the floor's delivery stays as D3 ruled — the per-charter Boundaries pointer line plus the skill's own model-invocation triggers. The reviewer's C1 addition — a path-injected rules file (`.claude/rules/mochiko/sound-loop.md`, scaffolded by setup, path-scoped to the governing surfaces, firing mechanically on read) — is **deferred as purely additive**: it changes no ruled semantics and can land later without reopening this session. C1's residual risk is accepted with eyes open: delivery rides lead self-recognition, the mechanism that failed in kinako.

**Revisit trigger (named):** the **first observed floor miss in live use** — a judgment-authored governing-surface write that runs without the loop — builds the rules-file leg immediately, no new session required. The build's BACKLOG item carries this deferral so it outlives conversation context.

**Confidence:** Contested — user chose A-only against both the reviewer's and the lead's dual-delivery recommendation; deliberate deferral, trigger on record.

### D6 — Grooming door narrowed with a ceiling; out-of-remit demands adapt by importing rituals, never shedding them

**Statement:**

1. **Key responsibilities seeded.** The desk's grooming door covers **merge, retire, status change, and extent-tidying of existing entries** — plus its already-chartered remit (routing, growth-door work-row cuts, delta cards, integrity fix-on-sight). These are the desk's own; inside them, the D1 floor applies as ruled.
2. **Ceiling (narrow read — I1 upheld).** Wholesale or from-scratch re-derivation is specify's derivation work; the desk's default answer is **route to `/mochiko:specify`**. The kinako route is recorded as out-of-bounds under this reading; the adopted outcome stands ratified — nothing rolls back.
3. **Adaptive override (user-authored).** When the user explicitly asks the desk to host work outside its key responsibilities, the desk adapts rather than refuses: it **names the boundary crossing**, then serves the demand **with the home command's rituals imported** — the D1 loop at minimum (seat produces on an approved plan, independent seat reviews, user rules). Adaptation moves the door, never lowers the ritual — the same route-invariance instinct h2 contributed to D2.

**Rationale:** rigid refusal makes the desk brittle and invites the off-desk bypass I6 already prices; silent hosting recreates kinako. The middle path preserves the advisory-front-door identity — the desk can always serve — while making under-ritual hosting structurally impossible.

**Confidence:** Confident — ceiling per lead recommendation; override clause user-authored.

### D7 — Evidence marked n=1-directional; validation probe declined

**Statement:** the record's evidence base is marked honestly: **n=1, directional** — one kinako dogfood run drives the diagnosis, and the adopted 3-capability map has not been shown defective, so the review leg's corrective value is asserted, not demonstrated; the floor's justification is preventative doctrine (sound-loop consistency), not measured harm. The reviewer's probe — a retroactive cold review of the adopted kinako map plus a session-ruled first-live-run watch — is **declined**. A kinako-side review remains available any time as a purely additive act, no session reopen needed; the build's *standard* first-live-run watch convention (KM practice on every build item) is untouched by this ruling — declined here as a session probe, not forbidden as build hygiene.

**Confidence:** Confident (marker) · Contested (probe declined against lead + reviewer recommendation).

## Build surface *(cold-buildable sketch — the build ceremony refines)*

- **New skill** `plugins/mochiko/skills/patterns-sound-loop/SKILL.md`: D2 test + governing-surface list (with the I5 per-member table) · D1 three legs · exemptions (mechanical execution, transcription, fix-on-sight — **no delta-card exemption**: all desk delta cards take the review leg, D2 edge clause) · D6's import-rituals adaptation rule · role-claim honesty rule · D4 wiring table (incl. the I3 spec-less desk-review procedure, build-FAIL if unresolvable) + disclosure line. Trigger `description` engineered for model-invocation (≤1,536 chars); unbudgeted at birth, ledger note per `patterns-map-minimalism` precedent.
- **`feature.md`**: Boundaries pointer line · the grooming door gains D6's ceiling (merge/retire/status/extent-tidy of existing entries; re-derivation routes to specify; explicit-user-request hosting imports the rituals) · the staffing-freedom clause ("how you staff, sequence, and run the visit is yours to shape") takes a recorded narrowing — "below the sound-loop floor" scoping. Supersession strips owed.
- **BACKLOG build item carries D5's deferral**: the rules-file leg (`.claude/rules/mochiko/sound-loop.md`, setup-scaffolded, path-scoped) stays unbuilt; first observed floor miss in live use builds it immediately.
- **`plan.md` / `implement.md`**: Boundaries pointer line each (their run shapes already satisfy the floor; the pointer makes it doctrine, not habit).
- **Router**: one row; placement beside the minimalism trio, build's call.
- **`.claude/rules/mochiko/primitive-edits.md`**: charter audit re-key extends — floor check gains "sound-loop pointer present."
- **v8 commands** (`setup`/`specify`/`brainstorm`): no edit — already satisfy the floor by construction; the build's audit brief states this check ran.
- **Audits**: author≠grader per edited primitive; KM landing whole (DECISIONS row · BACKLOG build item · ROADMAP touch · index update).

## Open threads

- **Kinako's adopted 3-capability map stands unreviewed** — probe declined at D7 (`Contested`); a kinako-side cold review stays available any time as a purely additive act, no reopen needed.
- **Close-report floor-disclosure grammar** — one standard line ("floor: tripped/clear · seats: X produced, Y reviewed") vs free prose; build may pin, contention returns here.
- ~~Delta-card exemption~~ — resolved at review (I8): exemption dead, every desk delta card takes the review leg (`Contested`, D2 edge clause).

## Cold review

Solo `devils-advocate` × `review-brainstorm`, blind-map dispatch (37-angle Phase 0 map, topic-only spawn; record path sent only after the map was on record). Reviewer re-verified F1–F6 against the repo: all clean. **Verdict: critical-gaps** — 11 survivors: 1 Critical · 8 Important · 2 Minor; 3 candidate lines died at the materiality bar (retrievable). Strengths credited: the dispatch-to-review reframe, the refusal of blanket dispatch, rejected roads on record, self-flagged streak, honest `Assumed` mark.

| # | Finding (compressed) | Disposition |
|---|---|---|
| C1 | Floor has no runtime bite — delivery rides the same lead self-recognition that failed; path-rule leg unweighed | ruled → D5: A-only now, rules-file leg deferred as purely additive, first-miss build trigger (`Contested`) |
| I1 | Kinako re-derivation plausibly exceeded the grooming door — legality unruled, door needs a ceiling | ruled → D6: narrow read + ceiling; user-authored adaptive override (host on explicit word, rituals imported) |
| I2 | "Scale-keyed" (D1) contradicts kind test (D2); Q1 transformation unflagged | folded — D1 reworded, transformation + small-write consequence stated |
| I3 | Review leg has no spec-less procedure for desk map writes | folded — D4 build obligation, build FAILs if unresolvable |
| I4 | n=1 evidence unmarked; adopted kinako map never shown defective; no probe | ruled → D7: n=1-directional marker adopted; probe declined (`Contested`), kinako review stays available additively |
| I5 | Governing-surface list's per-member blast radius unaudited | folded — D2 per-member table obligation before pinning |
| I6 | Desk cost/bypass unpriced | folded — D1 price + disclosure mitigation + accepted residual |
| I7 | Leg 1 (producer ≠ lead) stricter than doctrine; user-authored; re-affirm with cost in view | ruled → D1 re-affirmed as-is; review-only steelman recorded as rejected road |
| I8 | Delta-card exemption leans on a confirm that is not independent review | ruled → D2 edge clause: exemption dies, all delta cards take the review leg at the desk (`Contested` — stricter than recommended) |
| M1 | D3/D4 Confident marks rest on the Q3–Q5 adoption streak | engaged via I3/I5 findings on the merits; acceptance covers the streak explicitly |
| M2 | Duplicate `## Open threads` header | lead-repaired on sight |

**Verify trail:** round 1 NOT CLEAN — 1 blocking (D3's skill spec still listed the dead delta-card exemption) + 4 non-blocking stale echoes of the I2/I8 folds (D1 heading "scale-keyed", one "above the threshold" residual, two outdated open threads); all 5 lead-repaired same round. One note-only tension recorded, no fix required: D3's model-invocation "delivery force" claim vs D5's accepted residual (delivery rides lead self-recognition) — D5 documents the tension deliberately. Round 2: bounded re-verify of the 5 repairs — **CLEAN**, no new contradictions, each locus agreeing with its authoritative source.

## Session trail

- Q1: which disease — missing review (a), missing seats as such (b), both scale-keyed (c), cosmetics (d). **User: "as recommended yes" (a + c's scale trigger) + own amendment: production planned by a seat, lead feedback/approve/distribute.** → D1.
- Q2: threshold candidates t1 (desk bright line) / t2 (route-invariance) / t3 (write weight) / t4 (reversibility); lead recommended t2-doctrine + t1-instance. **User redirected: trigger must sit at higher altitude — library-wide, not keyed to the feature workflow.** Desk-only bright line rejected as primary; generality direction affirmed. → D2 pending on the higher-altitude formulation (Q3).
- Q3: trigger formulations h1 (durable judgment artifact) / h2 (route-invariance generalized) / h3 (downstream-consumer); lead recommended h1; user asked for plain language, restated, **user: "as recommended"** → D2 (trigger Confident; delta-card exemption folded as `Assumed`).
- Q4: carrier options — new skill / per-charter paragraph / router line; lead recommended the skill incl. the name `patterns-sound-loop`. **User: "as recommended."** → D3.
- Q5: wiring options — defaults in skill / defaults in charters / none; lead recommended skill-homed defaults + close-report disclosure. **User: "as recommended."** → D4. Adoption streak Q3–Q5 flagged.
- Q6: sizing gate — lead recommended solo blind-map dispatch. **User: "as recommended."** Reviewer spawned two-message blind; 37-angle map; verdict critical-gaps, 11 survivors.
- Q7: dispositions — **user ruled Batch A (I2/I3/I5/I6) folded as recommended; C1/I1/I4/I7/I8 taken individually.** Folds executed; M2 lead-repaired.
- Q8: C1 — lead recommended dual delivery (pointer + rules file); user asked what dual looks like (clarified plainly); **user ruled A-only, B deferred as purely additive** → D5 (`Contested`, first-miss trigger).
- Q9: I1 — lead recommended narrow read + ceiling; **user ruled narrow, amending with the adaptive override: seed key responsibilities, adapt on explicit out-of-remit requests, rituals imported** → D6.
- Q10: I4 — lead recommended marker + two-half probe; user answered "1", lead clarified the reading (marker only, probe declined), **user: "confirm"** → D7.
- Q11: I7 — options re-affirm / scope-down / review-only; lead read: re-affirm. **User: "yes re-affirm 1"** → D1 leg 1 re-affirmed, steelman recorded.
- Q12: I8 — options exemption-dies / split (recommended) / keep; **user: "a exemption dies"** → D2 edge clause reruled (`Contested`). All 11 survivors dispositioned; bounded verify next.
