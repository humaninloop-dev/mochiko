# Adversarial-review generality — session record

**Status:** accepted (2026-08-04) — solo cold review 7/7 dispositioned, verify CLEAN
**Opened:** 2026-08-04
**Topic:** (1) How adversarial review happens in each workflow stage; (2) whether the single `devils-advocate` persona is general enough to carry multi-angle adversarial review, or whether angle diversity needs different carriers.

---

## Ground facts (Explore-mapped, 2026-08-04)

Seat→agent→skill mapping is single-sourced in the router (`plugins/mochiko/skills/mochiko/SKILL.md`); commands are goal+harness and do not name agents.

### Per-stage adversarial/independent review seats

| Stage | Artifact | Reviewer | Skill | Form |
|---|---|---|---|---|
| setup (G3) | `governance-intent.md` synthesis | devils-advocate (sized pair/solo) | review-governance-intent | adversarial critique |
| setup (final) | governance surface set | validator | validation-constitution | mirror checklist, binary PASS/FAIL |
| setup (post-accept) | injection behavior | disposable probes | testing-governance-injection | empirical probe |
| specify | `spec.md` + Delivery Slices + Screens & Flows (prototype walked) | devils-advocate | review-specifications | adversarial critique |
| plan (half 1) | analysis + architecture + design sets + cycle cards | devils-advocate | review-plan-artifacts | mirror checklist |
| plan (half 2) | same set, cross-artifact | principal-architect | review-feasibility | adversarial critique |
| implement | built code + TEST gates, per cycle + whole | qa-engineer | testing-end-user | evidence verification (neither form) |
| brainstorm | frozen `record.md` | devils-advocate (sized pair/solo, lens-split) | review-brainstorm | adversarial critique + cross-exam |

Meta-seat: primitive edits get author≠grader audit (validator, or matching validation-*/review-* skill).

### Devils-advocate persona (agents/devils-advocate.md)

- Declares skills: review-specifications, review-plan-artifacts, review-brainstorm, review-governance-intent. NOT review-feasibility (principal-architect's).
- Persona carries: adversarial stance (never approve with zero findings, never downgrade severity, distrust own "looks good"), severity discipline, product-framed questioning, refusal to author fixes.
- Hunt taxonomy (deliberately thin, one line): missing requirements · ambiguities · edge cases · assumption gaps · contradictions — canonical detail delegated to the skills.
- Angle diversity therefore lives in the **skills**, not the persona: each review-* skill encodes its own angle inventory (spec: 5 defect classes + 11 slice checks + 8 screens/flows checks; brainstorm: 5 hunt classes + scenario stress + standalone-fitness; governance-intent: 5 setup-specific hunt classes; plan-artifacts: mechanical coverage checks).
- Multi-angle-per-run mechanisms that exist today: sized **pair with lens split** (brainstorm: decision-quality vs record-integrity; governance-intent pair) + one-shot cross-exam; plan's **two-agent two-form split** (devils-advocate completeness + principal-architect feasibility) — the only stage where two different personas review the same artifact set.

### Stages with no adversarial-critique seat

1. implement — qa-engineer verifies evidence, but no judgment-critique of built code (no code-review adversary).
2. specify's Intent stage — lead-run synthesis, user-confirmed only; never cold-reviewed (setup's analogous synthesis IS cold-reviewed).
3. live brainstorm session — unchallenged by design until convergence; waivable.
4. feature-close / Feature-Done execution — no owning workflow (deferred to audit scoping).
5. the lead's own folds/merges — verify pass exists only in brainstorm/setup pair protocols, not specify/plan/implement.

---

## Decisions

**AR-D1 — The generality worry is checklists-as-ceiling.** `Confident`
User ruled (Q1: option B): the devils-advocate concern is not one-mind blind spots or seatless stages — it is that skill-encoded angle lists act as a ceiling: the reviewer never leaves the list, so novel/off-taxonomy failure modes pass unexamined.
*Grounding (as repaired at review F2, 2026-08-04):* the semantic claim — **no review-* skill sanctions findings outside its hunt-class taxonomy at the taxonomy itself**; every class table is presented as canonical/closed ("The five hunt classes", "the canonical hunt taxonomy"), and `review-specifications` (the AR-D4 substrate) carries no off-list license anywhere in its text. Mitigations, complete inventory: `review-brainstorm`'s lens line ("The lens sets your depth, not your jurisdiction… still report anything real you trip over outside it") sanctions off-list findings for that skill; `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:120` carries floor language for one check ("a P1 journey is the floor, never the cap"); review-brainstorm class 2 ("Missing dimension") is open-ended by construction — the **only** open-ended hunt class library-wide (review-governance-intent's "Missed dimension" class is bounded by the ten-dimension agenda, per review F5); persona's "never approve with zero findings" forces volume, not breadth. Net: mitigations concentrate in review-brainstorm — independently confirming review-specifications as the worst-case substrate (AR-D4). *(Original grounding line claimed "zero grep hits for floor language across all five" — false as a literal grep report; repaired here per review F2, statement amended at source.)*

**AR-D2 — Remedy chosen empirically, not by argument; B is the favored hypothesis.** `Unsure` (deliberately — pending test)
Candidate remedies: (A) floor-doctrine line per skill ("classes are a floor; off-taxonomy findings sanctioned"), (B) checklist-blind free-hunt pass before the class hunt (attacks anchoring mechanism), (C) angle-diverse disjoint lens briefs on sized pairs. User ruled: B "seems a good read" but must be stress-tested — run a test to see which method works best before committing to a build.

**AR-D3 — Test shape: seeded-defect benchmark.** `Confident` (user adopted recommendation)
One real artifact, defects injected by an independent seeder agent (author≠grader discipline) — mix of in-taxonomy and deliberately off-taxonomy shapes — off-taxonomy seeds must be **off-list but in product jurisdiction** (wrong domain model, mis-modeled actor incentives, missing regulatory/product-legal constraint, cross-feature P1-journey interaction); technical-zone shapes (security hole, ops blindness, licensing) dropped per review F1 — they sit in review-specifications' own excluded jurisdiction and would measure scope compliance, not the ceiling. Arms: baseline (current skill unmodified) + A (floor line) + B (free-hunt-first) + C (disjoint lens briefs), cold subagents, same artifact. Metric: recall per defect, split in-taxonomy vs off-taxonomy — off-taxonomy recall is the D1-deciding number. Fold from option B (historical replay): where real dogfood-era misses exist, use them as seed material so the injected distribution isn't purely "plantable" artificial defects — historical seeds pass the same off-list-but-in-jurisdiction filter as invented ones (verify-pass repair 1: a real dogfood miss can itself be a technical-zone shape).
*Arm-exclusivity (recorded at review F7):* deliberate — four arms already strain a ~10-seed, 2-replicate budget; a combo arm doubles interpretation load on a noise-prone benchmark. If B wins, an A+B composite (floor line + free-hunt) is a build-time option the test session may recommend without a second benchmark round.
*Pre-sweep (added at review F4):* before seeding, the seeder labels pre-existing defects it finds in the base artifact; those join the legitimate-hit key, so unmatched arm findings can be scored for precision honestly. Limit acknowledged: the pre-sweep labels only what the seeder sees — an arm's genuinely novel find lands unmatched; the borderline-escalation path (F3) routes unmatched-but-plausible findings to the user rather than auto-scoring them as noise.
*Execution parameters (pinned at review F3):* **replicates** ≥2 runs per arm, a seed counts as hit only when found in a strict majority of that arm's runs (at 2 replicates: both runs) · **seed mix** ~10 seeds, ~60/40 in-taxonomy/off-taxonomy · **scoring** by a separate cold scorer agent holding the seed key — never the seeder, never an arm reviewer (author≠grader at the scoring seat; motivated-matching risk toward the favored arm B is the named hazard); matching rule: a finding must identify the seeded defect's location and substance — naming the symptom without the substance is a miss; borderline matches escalate to the user.

**AR-D4 — Test substrate: `review-specifications` on a spec.** `Confident` (user adopted recommendation; third unelaborated adoption in a row — streak flagged to user at Q4, per ratification-streak rule)
Rationale: worst-case surface per AR-D1 grounding — solo seat, all five classes bounded, no open-ended class, no pair de-anchoring. If remedies don't move off-taxonomy recall here, no build is justified anywhere; if they do, extension to other review skills is a staged judgment call informed by this result (review-brainstorm run later only on a positive result — staged, not parallel).

**AR-D5 — Test runs in a dedicated session; this session lands the BACKLOG item.** `Confident` (ruled at a no-recommendation fork — steelmans both ways, user chose B against the closure pull)
The arm variants are draft primitives (a modified `review-specifications` SKILL.md per arm); authoring them deserves fresh context and primitive-edit discipline, not mid-session improvisation. This record closes as: worry confirmed (AR-D1), remedies designed (AR-D2), test fully specified (AR-D3/D4). Acknowledged risk, recorded honestly: a parked test is a decaying intention — the BACKLOG item carries the full test spec so a cold session can run it without re-derivation.

**AR-D6 — Decision rule pre-committed in shape, not in thresholds.** `Confident` (user adopted recommendation)
Primary metric: off-taxonomy recall. Guards (second added at review F4): (1) winning arm must not regress in-taxonomy recall; (2) **precision guard** — findings scored for precision over the labeled set (seeds + pre-swept real defects); an arm whose precision collapses relative to baseline cannot win on recall alone; per-arm finding counts reported as a visible noise signal (no hard cap — a cap invites gaming toward exactly-N findings). Null result across all arms: no build — the worry stays recorded (AR-D1), the closed taxonomies stand. No numeric margins pre-committed — four arms on one artifact won't support tight thresholds; the final ruling is the user's at test close. Anchoring risk acknowledged: even a loose rule can tempt over-reading noise as signal; the test session should treat the rule as shape, not verdict.

---

## Review (solo cold review, 2026-08-04 — devils-advocate, review-brainstorm, spawned solo)

**Tally: 7 raised → 7 survived → dispositions below (walked one-by-one with the user).** Verdict at review: needs-revision — no decision broke; ground-facts seat/skill claims all sample-verified accurate; folds required before the BACKLOG item ships.

| # | Sev | Finding (compressed) | Disposition |
|---|---|---|---|
| F1 | Important | Off-taxonomy seed shapes (security/ops/licensing) sit in review-specifications' own excluded jurisdiction — test would measure scope compliance, not the AR-D1 ceiling | **User ruled A:** drop the technical trio from the seed set; seed off-list-but-in-jurisdiction shapes only (wrong domain model, mis-modeled actor incentives, missing regulatory/product-legal constraint, cross-feature P1-journey interaction). AR-D3 amended. |
| F2 | Important | AR-D1's "zero grep hits for floor language" false as a literal grep report; two real mitigations omitted (review-brainstorm lens line, ARTIFACT-CHECKLISTS P1-floor note) | **Folded (user-confirmed on recommendation):** grounding reworded to the verified semantic claim; mitigation inventory completed; erratum noted at source. Counterexamples strengthen AR-D4 (mitigations concentrate in review-brainstorm; review-specifications has none). |
| F3 | Important | "Test fully specified" overstated — replicates, seed mix, and scoring protocol (scorer independence, matching rule) unruled; cold session would inherit an open judgment call, breaking AR-D5's parking logic | **User ruled: pin all three.** AR-D3 amended with execution parameters (≥2 replicates/majority-hit rule · ~10 seeds ~60/40 mix · independent cold scorer with pinned matching rule, borderlines escalate to user). |
| F4 | Important | Recall-only metric rewards volume — free-hunt arm B could win by flooding; pre-existing artifact defects unlabeled, so non-seed findings unscoreable | **User ruled: fold both parts.** AR-D6 gains the precision guard + per-arm finding-count reporting (no hard cap); AR-D3 gains the seeder pre-sweep, with the novel-find limit routed through the F3 borderline-escalation path. |
| F5 | Minor | Mitigation note attributed the open-ended "Missing dimension" class to governance-intent too — that skill's class 1 is agenda-bounded (closed) | **Folded:** largely repaired by the F2 rewording; clarifying clause added — review-brainstorm class 2 is the only open-ended hunt class library-wide. |
| F6 | Minor | AR-D3/D4/D6 marked `Confident` despite being unelaborated recommendation-adoptions (fitness doctrine prescribes `Assumed`) | **User ruled: marks stand.** Confirmation grounds: the streak was broken in-session at Q5 (recommendation-free fork, user ruled against the closure pull) and the F1/F3/F4 review rulings show live engagement with the same test design. Explicit user confirmation recorded here — the marks are now confirmed, not assumed. |
| F7 | Minor | No combo arm (A+B) and no rejected-roads note recording why | **Folded:** exclusivity recorded in AR-D3 as deliberate (arm count vs noise budget); A+B composite named as a build-time option on a B win, no second benchmark round required. |

**All 7 dispositioned (7/7). Post-fold status: every Important finding repaired at source; verdict path needs-revision → folds landed.**

**Verify pass (same reviewer, fresh read from disk): CLEAN on blocking — all 7 folds confirmed landed at source, no fold-introduced contradiction, no silent narrowing.** Three non-blocking seams repaired same round: (1) historical-replay seeds now pass the F1 jurisdiction filter · (2) majority-hit rule disambiguated to strict majority (2 replicates = both runs) · (3) F1 disposition wording "primary seed set" → "seed set" (source was authoritative and stricter).

## Open threads

1. **Seatless stages (from the ground-facts map) left unruled.** Five surfaces have no adversarial-critique seat (implement's built code, specify's Intent stage, live brainstorm session [by design], feature-close execution [already deferred to audit scoping], lead folds outside pair protocols). User was offered these as separate captures at close; session closed on the D1–D6 thread only. Not decisions — candidate future captures.
2. **Extension beyond `review-specifications`** is staged on a positive test result (AR-D4) — which skills inherit the winning remedy is a test-close judgment, not pre-ruled.

## Session trail

- Q1 (failure-shape fork): user ruled B — checklists-as-ceiling (AR-D1).
- Q2 (remedy direction): user ruled test-first, B favored hypothesis (AR-D2).
- Q3 (test shape): recommendation adopted — seeded-defect benchmark (AR-D3).
- Q4 (substrate): recommendation adopted — review-specifications; **streak of three unelaborated adoptions flagged** (AR-D4).
- Q5 (timing): no-recommendation fork per streak rule; user ruled B — dedicated session (AR-D5).
- Q6 (decision rule): recommendation adopted — shape-only pre-commit (AR-D6).
- Close: user ordered close sequence; solo cold review sizing offered with the close nudge and not objected to — sized **solo** for a 6-decision, single-thread record.
