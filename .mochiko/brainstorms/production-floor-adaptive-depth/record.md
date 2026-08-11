# Brainstorm — Adaptive depth for the production floor (revisiting production-only strictness)

**Status:** accepted (user, 2026-08-11 — "yes superseeded, and yes record accepted") · solo-cold-reviewed via blind-map dispatch (19-angle map; 11 raised — 0 Critical, 8 Important, 3 Minor → needs-revision → 11/11 dispositioned; verify CLEAN, 1 advisory reconciled) · PO-D7 supersession explicitly user-ruled at acceptance · opened 2026-08-11, wrapped 2026-08-11.
**Topic as given:** "i want to relook at the setup command and how we decided to make it only for production strict app. I think operationally, it has caused issues with the amount of constraint when starting up in greenfield projects. My intention with production type strict setup was to be more broad, covering range of practise, however, in terms of depth, it needs to be adaptive maybe have 2 levels low high. covers the broad range in both, but configurable to make it strict when it is needed"
**Session:** `/mochiko:brainstorm` (goal+harness shape) · lead + user.

---

## Prior-ruling context (what this session reopens)

This session revisits territory ruled in `production-only-focus` (PO-D1–D7, 2026-07-30, accepted, pair-reviewed):

- **PO-D2** — the tier ladder (`poc|internal|production|regulated`) died; exactly one asserted production floor; compliance became fact-triggered additive modules.
- **PO-D7** — immature-but-in-scope teams enter at the full floor; recorded waivers are the staged on-ramp. A **maturity axis was explicitly rejected** with steelman recorded: "binary floor items don't ratchet the way coverage percentages do, so a maturity ladder would give young teams a legitimate intermediate rung instead of day-one waivers." Rejected because "it reintroduces the rigor dial D2 killed."
- **PO-D4.1** — waiver expiry `Deferred` with an explicit revisit marker: "keep the permanent waiver for now. This decision I will come to revisit later."
- PO-D7's rejection notes D4.1 "is the natural place to add time-boxing **if waiver-as-normal-state proves real in dogfoods**" — the user's reported greenfield pain is a candidate instance of exactly that condition firing.

Reopening is legitimate: the revisit marker exists, and the user reports operational evidence the prior session anticipated. Amending PO-D2/PO-D7 requires explicit user ruling (both user-accepted decisions; amendment reserved to the user).

## Problem statement (evolving)

Driver, in the user's words: the production-strict setup "has caused issues with the amount of constraint when starting up in greenfield projects." Original intent restated: production-type strict setup was meant to be **broad** — covering the range of practices — while **depth** should be adaptive: two levels (low / high), broad range at both, configurable to strict when needed.

---

## Session trail

**Q1 — where did constraint bite?** Options: (a) setup interrogation ceremony · (b) floor obligations during build · (c) waiver-recording friction. **User: "b mostly."** Pain lives in the asserted floor's build-time obligations on a greenfield app, not primarily in setup's elicitation or the waiver mechanism. Sharpens the problem: this is PO-D7 territory head-on (binary floor items forced day-one), the exact seam its recorded steelman predicted.

**Q2 — which obligations hurt, concretely?** User cannot name a specific check. Clarified intent instead: "I love the coverage of checks, however, can they be lenient and progressively become stricter — hence the 'low'/'high' frame. If some checks can't be lenient, maybe move them into high state." Reading: breadth is valued and stays (every category present at both levels); depth starts lenient and **progressively** tightens; checks with no honest lenient form live only at high. "Progressive" is a new element beyond static low/high — implies a ratchet or transition, not just a config flag.

**Q3 — what moves the dial?** Options: (a) user declares · (b) fact-triggered (facts derive the level, like compliance modules) · (c) progressive ratchet with graduation triggers. **User: "definitely user declares, it is something that needs to be a very conscious and user driven decision."** Ruled as D1.

## Decisions

### D1 — Level transition is a user declaration, never derived — `Confident`
**Statement:** The low/high level is moved only by an explicit, conscious user declaration, recorded in the governance ledger. It is never inferred from project facts and never auto-advanced by the system.
**Rationale (user's):** the moment of taking on full production strictness must be a deliberate, user-owned decision.
**Tension logged (lead):** this consciously reintroduces the question PO-D2 killed ("how much rigor do you want") — but bounded: two positions instead of a four-rung ladder, breadth invariant at both, the declaration a recorded governance event rather than a setup-time default. Amends PO-D2's rationale; the fact-triggered compliance-module machinery is untouched (modules attach at either level).

### D2 — Low→high is one-way; setup asks, recommending low for greenfield — `Confident`
**Statement:** The level declaration is a ratchet: high, once declared, never returns to low. At setup the level is elicited, never defaulted — setup recommends low for greenfield, recommends high where reality warrants (e.g. brownfield with real users), and the user rules per D1.
**Rationale:** one-way keeps "progressively stricter" honest and closes the convenience-downgrade path (the silent-bypass risk PO-D4's rationale named); recommend-then-arbitrate at setup matches the existing card-dealing doctrine.
**Q4 exchange:** ratchet + starting-default posed together; user: "one-way yes, and yes setup asks but recommends low for greenfield."

### D3 — One project-wide dial; per-check level tags carry the nuance — `Confident`
**Statement:** The level is a single project-wide state. Each floor/standard check is authored with a `low` row and a `high` row; a check with no honest lenient form is tagged high-only (absent at low, entering at the declaration). No per-category dials. The PO-D4.2 legal-mandate stratum rides above both levels — unwaivable at either.
**Rationale:** one declaration, one ledger event, whole posture shifts — matches D1's "very conscious decision"; per-check tagging carries the nuance without multiplying ledger states.
**Provenance note:** lead-recommended (a) over per-category dials (b); user adopted: "a".

### D4 — Waivers survive at both levels as the per-check fit escape; staged adoption moves to the low level — `Confident`
**Statement:** Recorded waivers remain available at both levels for per-check deviations. The staged-adoption role moves off waivers onto the low level: PO-D7's "full floor + waivers as on-ramp" is superseded — the on-ramp is now the low level, and a waiver returns to its natural size as a fit exception (a check that misfits this project), not a depth valve.
**Rationale (user's, lead-concurred):** breadth is not reduced — every category is present at low, so a misfitting check still needs an honest recorded exit. Low fixes depth mismatch; waivers fix fit mismatch — different failure modes. PO-D4's visible-deviation-beats-silent-bypass intent preserved.
**Ripple:** PO-D4.1 (deferred waiver expiry) shrinks in urgency — with low absorbing staged adoption, permanent waivers stop being the young-team default state. D4.1 stays deferred, revisit marker intact.

### D5 — Low's authoring principle: nothing expensive to retrofit — `Confident`
**Statement:** The low row of every check is authored to the retrofit-cost cut line: low may relax rigor that can be added later (coverage thresholds, SLO formalization, runbook depth) but never permits what is expensive to fix after the fact (secrets in repo, absent auth-boundary concept, unvalidated input). A check whose posture must stay strict even at low simply keeps its strict form there — identical rows at both levels are legitimate. Warn-don't-block is a fallback mechanism for checks with no distinct lenient form where high-only tagging is too harsh, never the general principle. Old `internal`-tier rows may serve as source material only, re-audited against the production-bound frame — never adopted wholesale.
**Rationale (user's):** "we are still building something that will be shipped to users. by having low, we are balancing speed of delivery. Not to the extent of doing mistakes that are expensive to fix. So, if certain postures need to be strict even in low, that's okay."
**Note:** this gives the level pair its honest identity — low ≠ old `internal`; both levels are production-bound, the dial is retrofit-safe depth, not audience or stakes.

### D6 — The flip is a setup rerun in high mode; nothing watches for the moment — `Contested`
**Statement:** The declaration event is the user consciously rerunning setup in high mode; that run re-deals the high rows and surfaces the adherence delta (the changes required to conform), which the user then works through. No pipeline surfacing obligation, no review-time advisory, no watcher of any kind — the flip moment lives entirely in the user's judgment.
**Rationale (user's):** the flip *is* the conscious decision — rerun setup strict, deal with the changes. A watcher dilutes the ownership D1 established.
**Provenance note:** lead recommended (a) surface-and-recommend (+ optional review-time advisory backstop); user chose no-watcher — deliberate, marked `Contested`. Risk logged: silent under-posture (team cruises at low after real users arrive) has no structural catch; same shape as PO-D3's S4 wrong-fact worry. Accepted by the user as the cost of undiluted ownership.
**Mechanics captured:** declaration = governance amend run (setup high mode) — ledger event + re-dealt rows + adherence delta in one ceremony, consistent with D2's recommend-then-arbitrate framing.
**#10 fold (review, user-ruled):** delta items between the high declaration and full conformance ride D4 recorded waivers until closed — the interim state is visible and auditable, no new machinery. These interim waivers are a bounded exception to D4's "not a depth valve" rationale: they exist only inside a declared high transition, each names its delta item, and each dies when the item conforms — never a standing depth posture (verify-pass advisory, reconciled).

### D7 — Compliance modules are level-blind — `Confident`
**Statement:** Fact-triggered compliance modules attach at either level and always assert at full strength; module obligations carry no low/high rows.
**Rationale:** modules are external, fact-driven constraints, not internal rigor choices; a "lenient HIPAA" would manufacture exactly the documented-violation state PO-D4.2 closed. Consistent with D3 (legal-mandate stratum above the dial).
**Provenance note:** lead-recommended strongly; user adopted: "as recommended." Third unelaborated adoption (D3, D7 + D5's principle) — streak flagged per discipline; D6's contested ruling breaks the pattern of passive acceptance.

### D8 — The dial governs asserted code standards only; pipeline process rigor stays uniform — `Confident`
**Statement:** The low/high level keys only the governance content (what is asserted about the product's code). The two runtime process sites the production-only narrowing collapsed stay collapsed at their strict form regardless of level: implement's domain-registry checkpoint stays always-blocking, and setup's synthesis-review sizing gate stays fixed-pair. Low relaxes what is asserted about the code, never how carefully mochiko itself works.
**Rationale:** the reported pain (Q1: "b mostly") was floor obligations on the code, not process weight; and D5's retrofit-cost principle argues for uniform process rigor — a bad dependency or an under-reviewed governance synthesis is exactly the expensive-to-fix-later class. Presented in plain language with the dependency-checkpoint example at the user's request; user adopted: "as recommended."

---

## Prior-ruling amendment surface (user rulings required at acceptance)

**Evidence honesty marker (#1 fold, user-ruled):** this amendment proceeds on directional operational experience — greenfield starts felt over-constrained — with no cited incident, run, or named check. Same evidence class as the PO session's own opening driver ("a positioning/design instinct, not a reaction to one failed run"), recorded explicitly rather than implied.

- **PO-D2 amended, not reversed:** the single-floor concept survives as breadth-invariance + one asserted standard *family*; the "no rigor dial" rationale is consciously amended — a bounded two-position, user-declared, one-way dial replaces the four-rung elicit-a-tier ladder. The dial asks "are you ready to be held to full depth," not "how much rigor do you want" at every setup.
- **PO-D7 superseded (user-ruled at acceptance, 2026-08-11 — "yes superseeded"):** the staged on-ramp moves from "full floor + recorded waivers" to the low level (D4); the rejected maturity-axis road is partially readmitted in its steelman's own terms — a legitimate intermediate rung instead of day-one waivers — but bounded: two positions, one-way, breadth-invariant, user-declared.
- **PO-D4 / PO-D4.1 / PO-D4.2 untouched:** waivers survive (D4), expiry stays deferred with reduced urgency (D4 ripple), legal-mandate stratum unwaivable and level-blind (D3, D7).
- **PO-D1, PO-D3, PO-D5, PO-D6 untouched:** target boundary, facts-not-standards elicitation (level declaration is itself elicited recommend-then-arbitrate, consistent), depth agenda, identity landing all stand.

## Build surface (consequences, not new decisions)

- Floor cards (`catalog/universal-floor.md`) + asserted standard rows: collapse-to-production-row reverses into two-row form — `low` / `high` per check, authored to D5's retrofit-cost line; identical rows where posture must hold; high-only tags where no honest lenient form exists. Old `internal` rows source material only.
- `governance-intent-template.md`: level declaration field (GI-family) — declared level + date + rationale line; ratchet noted (high is terminal).
- Governance ledger: level state + declaration event record.
- Setup (`setup.md` + `INTERROGATION-AGENDA.md`): level elicitation added — recommend-then-arbitrate, greenfield-recommends-low (D2); high-mode rerun = the flip ceremony surfacing the adherence delta (D6).
- Compliance modules: no change (level-blind, D7).
- Runtime sites (implement checkpoint, setup sizing gate): no change (D8).
- `validation-constitution` / `review-governance-intent`: graders learn the two-row form + level state; no level-vs-reality advisory (D6 no-watcher).
- Strips owed at build: supersession-by-ruling entries on every carrier of "single asserted production row" text (PO-D2 mechanical-edit descendants), citing this record's DECISIONS row.
- Re-inflation cost (#5 fold): the two-row form deliberately reverses part of PO's F24 collapse — accepted as bounded (two rows not four; dimension-2 tier elicitation and the pruning license stay dead).
- Tier-I build composition (#6 fold): the security-depth and ops build items in BACKLOG each gain a note — their checks receive low/high determinations at build time under D5's cut line; scoping records not reopened.
- Migration + identity (#7 fold): existing projects set up under the single floor default to high; the CLAUDE.md/ROADMAP "one asserted production floor, no tier ladder" sentence is rewritten to the two-level form at landing.
- Architecture-opinion cards (#9 fold): outside the dial — no low/high rows; PO-D3 S7 carve-out untouched.

## Review

**Sizing gate (2026-08-11):** weight stated — 8 decisions, 7 `Confident` / 1 `Contested` (D6); no embedded fact map (grounded on the prior accepted record); supersedes two pair-reviewed rulings (PO-D2 amended, PO-D7 superseded). Lead recommended single cold reviewer, blind-map dispatch; **user ruling: single** ("as recommended").

**Cold read (2026-08-11, blind-map dispatch — 19-angle Phase 0 map, topic-only spawn):** verdict **needs-revision** — 0 Critical, 8 Important, 3 Minor; 11 findings (4 coverage from the map diff, 4 decision-quality, 2 record-integrity, 1 unchallenged-assumption). Composition and per-decision hygiene graded sound.

**Dispositions (2026-08-11) — 11/11 landed.** Lead repaired the two record-integrity defects pre-disposition (#4 heading misfile — D2–D8 moved under `## Decisions`; #11 ratchet double-sense — D1 reworded "never auto-advanced by the system"). User batch: "as recommended, progressive meant binary, flip. re-affirm":
- **#1 — user-ruled (honesty marker):** no concrete run/check cited; the amendment proceeds on directional operational experience — marker recorded on the amendment surface below. Same evidence class as PO's own opening ("instinct, not a failed run"), now explicit instead of implied.
- **#2 — user-ruled (confirmed):** "progressive meant binary, flip" — the one-way declaration IS the intended shape; no gradual per-check tightening between declarations. Q2's "progressive" note stands corrected by this confirmation; D2/D5 unchanged.
- **#3 — user-ruled (re-affirmed):** D3, D7, D8 explicitly re-affirmed after the streak flag — marks stand `Confident` citing this affirmation. Full streak count: D3/D5-principle/D7/D8 were unelaborated adoptions; D6's contested ruling the counterweight.
- **#10 — ruled inline (adopted):** D6 gains — delta items between the high declaration and conformance ride D4 recorded waivers until closed; visible, auditable, no new machinery.
- **#5 — ruled inline (adopted, coverage):** re-inflation cost accepted deliberately — two rows not four, no dimension-2 revival, no pruning license; bounded reversal of part of PO's F24 cut. Noted on the build surface.
- **#6 — ruled inline (adopted, coverage):** the queued Tier-I builds (security-depth SD-D1–D6, ops OO-D1–D7) inherit the two-row obligation — their checks get low/high determinations at their own build time; a note lands on both BACKLOG items at landing. No reopening of their scoping records.
- **#7 — ruled inline (adopted, coverage):** existing projects set up under the single floor default to **high** (they already conformed to full depth — grandfathering down would be a silent downgrade); the CLAUDE.md/ROADMAP identity sentence ("one asserted production floor, no tier ladder") is rewritten at landing to carry the two-level form. Both join the build surface.
- **#8 — ruled inline (adopted, coverage):** cardinality 2 locked deliberately — D1's conscious-decision framing wants one binary moment, N levels or a continuum re-blurs it; the improve-waiver-ergonomics road rejected because the reported pain (Q1: "b mostly") was floor depth, not waiver friction — waivers were not the broken part. Both steelmans recorded here.
- **#9 — ruled inline (adopted, coverage):** architecture-opinion cards (BE-HEX/SRP/DEP and kin) sit **outside the dial** — recommend-then-arbitrate per PO-D3's S7 carve-out, unchanged at either level; the dial touches only the asserted layer.

**Verify pass (cold reviewer, 2026-08-11):** 11/11 dispositions CONFIRMED landed with matching content; heading structure parses clean; no statement-level or amendment-surface contradictions (#7 default-high vs D2 greenfield-low ruled distinct populations; #10 satisfied under D4's statement). **CLEAN.** One advisory — #10's interim waivers lean toward the depth-valve role D4's rationale retired — reconciled by a bounded-exception clause on the D6 fold (transition-scoped, item-named, dies at conformance).

## Open questions

*(populated as the session runs)*
