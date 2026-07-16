# Brainstorm Record — revision of `/mochiko:brainstorm` v2.1: token-efficient review stage (v2.2)

> **Topic:** revise the end-stage review pair after its first completed run. Opened by the user as: token efficiency — "the number of reviews and devils advocate and also number of times should be reduced and efficiently used."
> **Session:** opened 2026-07-05, concluded 2026-07-16 · lead + user, running bare `mochiko:analysis-iterative` (no live team) · method: **transcript forensics** (per the v2.1 record's D1 — judge a run from its `.jsonl` transcripts, never from its self-reports) over the v2.1 dogfood run `setup-constitution-flexibility` (lead `c3daf32e…jsonl` + subagents `fact-checker`/`reviewer-a`/`reviewer-b`).
> **Status:** **ACCEPTED and built at v0.6.0 (2026-07-16).** Five decisions user-ruled; all four build-plan items executed, plus the consistency pass (router, agent mount, REGISTRY annotations). This record is itself **un-reviewed** (bare session, no cold pass); a cold review can be requested.

---

## Context for a cold reviewer

**What existed (v2.1, plugin v0.5.0):** `/mochiko:brainstorm` as lead + user (+ conditional fact-checker seated at start when the topic has a reality surface); at convergence an unconditional **review pair** — two `mochiko:devils-advocate` agents running `mochiko:validation-brainstorm`, identical briefs, counterpart withheld until findings-formed — one cross-exam round, survivor routing, dispositions, verify pass (both reviewers), plain-text acceptance. Design record: `.mochiko/brainstorms/brainstorm-v2-revision/record.md`.

**The dogfood run (2026-07-05, topic "setup constitution flexibility"):** the first *completed* end-to-end run of any brainstorm variant — 9 decisions, 15 findings raised → 11 survivors → all dispositioned → 11/11 folds verified by both reviewers → accepted. Four agent contexts: lead + fact-checker (spawned `general-purpose`) + reviewer-a + reviewer-b (both `mochiko:devils-advocate`). BACKLOG's v2.1 named checks: tail-executes ✓ (first ever), independence-before-contact ✓, survivor-volume partial (11 vs v2's ~20; ~3 needed user rulings), debate-filter weak (see F5), argument-cap ✗ (never fired), cost — closed by this session's forensics (F1), fact-checker earns-its-keep — map earned it (F6).

## Findings (from the transcripts)

- **F1 · Cost, closing BACKLOG's re-measure check.** lead 355k out / 1.21M cache-write / 175 turns · fact-checker 29k / 1.23M / 51 · reviewer-a 144k / 3.66M / 82 · reviewer-b 126k / 2.53M / 80. **Total ≈654k output** — almost exactly v2's 645k and v1's ≈620k, but this time the money bought a *finished* run with the full adversarial tail (v2 died pre-convergence having reviewed nothing). The review pair alone: ~270k out + 6.2M cache-write — the dominant pool.
- **F2 · The reality surface was read three times over (verified).** fact-checker Read 14 files, reviewer-a 16, reviewer-b 11 — `setup.md`, `authoring-constitution` + references, `validation-constitution`, `loop-discipline` read independently by all three. Each reviewer rebuilt the file context the fact-checker already held.
- **F3 · Teammate↔teammate messaging exercised for the first time: 10 messages** (reviewers↔each other 6, reviewers↔fact-checker 4). v2's standing model used it zero times. Includes one pure duplication: both reviewers routed the *same* fact (the backend-skeleton claim) to the fact-checker four minutes apart (11:53, 11:58).
- **F4 · Two same-briefed reviewers delivered complementary coverage, not redundancy.** Of 6 Important survivors, only M1/M2 were found by both; **F2, F5 (the hard checklist contradiction), F7's surviving formulation came from reviewer-b alone; A1 (the scope-changing catalog ruling) from reviewer-a alone.** The uniques cluster by temperament: a's are decision-quality findings, b's are record-integrity findings — the reviewers specialized on their own. The redundancy sat in the *reading* (F2), not the finding.
- **F5 · The debate calibrated more than it killed.** Of 15 raised: 1 withdrawn (A5, on a shown misread), 1 subsumed, 2 severity-downgraded, 2 merged. Real but modest filtering for the cost of the second contact round.
- **F6 · The fact-checker's dormant window was never used by the lead** — zero lead→fact-checker messages all session. Its value was the initial map (six load-bearing facts, cited throughout) + two review-stage settlements initiated peer-to-peer by the reviewers. Seat cost ~5% of the run — not a savings pool.
- **F7 · The verify pass ran twice, one earned it.** Both reviewers verified 11/11; reviewer-b caught the two fold-introduced contradictions (the pass's exact charter); reviewer-a added soft notes only.
- **F8 · The account limit tripped at cross-exam open (09:21–11:51, 2.5h),** with 4 hot contexts right after two concurrent full cold reads — the review stage is the burst. Recovery held; contexts survived.
- **F9 · M1's origin was relay infidelity, not a checker error:** the record's fact-map section was *lead prose* summarizing the checker's map; the over-claim lived in the paraphrase and survived until end-stage review caught it.
- **F10 · `synthesis.md` was user-initiated** ("save as synthesis", 12:33, twelve minutes post-acceptance) — revealed demand for a compact pipeline-facing digest, produced as an unreviewed lead compression after the adversarial machinery closed.
- **F11 · The lead↔reviewer argument path is 0-for-2 runs.** The lead accepted 8/11 survivors without argument; the 2-exchange cap and tie-break have never fired. Watch item, not (yet) a defect — the verify pass confirmed the folds were genuine improvements.

## Decisions

### D1 — The pair survives, lens-split, contact compressed
Two reviewers stay (F4 kills a naive halving: either single seat loses Important-class findings). Codify the observed specialization: one **decision-quality lens** (hunt classes 1–4, scenario stress, steelmans), one **record-integrity lens** (class 5, fitness, reality-grounding as map-audit). Both consume the fact-checker's map as the fact substrate instead of re-reading the tree (kills F2); cross-exam collapses to **one attack message + one response each way**; **verify pass runs on the integrity reviewer only** (F7). The lens split lives in the **spawn briefs** — `validation-brainstorm` stays one document. Est. pair cost ~270k → ~150–170k.
- **Rejected:** one reviewer, full protocol (~50% cut but loses single-finder Important coverage both directions; the only remaining challenger of the record would be its author); keep-two-as-is with turn caps only (~25–30% cut, keeps paying for accidental redundancy).
- **Mark:** Confident *(adopted from lead's recommendation unmodified)*.

### D2 — Fact-checker: seat kept; map lands verbatim; substrate audited by sample
The conditional seat stays (BACKLOG's earns-its-keep check: met by the map, F6). The map becomes a **checker-authored record section pasted unedited** — the lead may write around it, never restate it (kills the F9/M1 paraphrase layer). The pair consumes it as the fact substrate; the integrity reviewer's grounding charter becomes *spot-check the map against files + check record claims against the map* — sampling, not re-derivation (keeps the M1 class catchable under D1's read-reduction). Seat remains the peer-messageable neutral referee.
- **Rejected:** demote to Explore dispatches (BACKLOG's named fallback — optimizes a ~5% pool and re-routes review-stage fact disputes through the lead, the relay pattern v2's forensics indicted); status quo (keeps the M1 injection point open).
- **Mark:** Confident *(adopted from lead's recommendation unmodified)*.

### D3 — Review sizing becomes a named human gate at convergence
When the lead confirms the wrap it states the record's weight (decisions, confidence marks, reality-surface load) and estimated review cost, recommends **pair / single / none**, and **the user rules**. Default recommendation for a heavyweight record stays full pair.
- **Precedent against, weighed openly:** v2.1 rejected *risk-graded selection* ("puts the producer in charge of its own review coverage"). Defused here: sizing is a named human gate — the lead only recommends; the user rules with the cost in view. Precedents for: the fact-checker seat is already conditional; depth-proportionality is settled repo doctrine (setup session D5).
- **Rejected:** uniform always-pair (cost-blind — a lean 3-decision session pays the heavyweight tail); deterministic decision-count threshold (arbitrary line; a lean record with one load-bearing ruling gets under-reviewed with nobody asked).
- **Mark:** Confident *(adopted from lead's recommendation unmodified — third consecutive; streak flagged to the user immediately after, per `analysis-iterative`)*.

### D4 — The assembled review-stage shape, verified against the as-ran transcripts (with two hole-fixes)
The new flow: convergence → **sizing gate (D3)** → freeze → spawn per ruling with lens briefs (D1) → cold reads → findings-formed counts → **one-shot four-message cross-exam** (a→b findings · b→a findings+attacks · a→b attacks+defenses/withdrawals · b→a defenses/withdrawals+close) with a **fact-route dedup rule** (a fact the counterpart already routed is cited, never re-routed — F3) → survivor reports; **cross-set merge/tally is lead-owned** (it drifted to reviewer-b as-ran) → routing/dispositions unchanged (2-exchange cap kept as a watch item, F11) → **verify by the integrity reviewer only** → acceptance. Hole-fixes found by the verification itself: **(a)** the "none" path gets a **recorded review waiver** in the record's Review section (who waived, at which gate, why) — otherwise the default-FAIL loop loses its validator silently, the same bug-class the dry run's F5 caught in the setup design; **(b)** single-reviewer mode names its verify owner explicitly — the sole reviewer verifies the lead's folds (sound: it grades the lead's repairs, not its own findings).
- **Mark:** Confident *(ratified after the user independently probed the machinery — agent count, devils-advocate role — and requested the recommendation)*.

### D5 — The synthesis is codified: on-request, derived-marked, fidelity-checked
`synthesis.md` becomes a sanctioned second artifact (F10's demand is real), shaped: produced **only on request after acceptance**, never auto-generated; stamped *derived — record canonical*; before it ships, the reviewer that ran the verify pass — still seated at that moment — **sample-checks synthesis↔record fidelity** (every ruling present, no confidence mark inflated, no rejected alternative resurrected). If review was waived (D3), the synthesis carries a plain **"derived, unchecked"** stamp — the same recorded-absence discipline as the review waiver.
- **Rejected:** codify bare, as the dry run did (trusts exactly the protection M1 proved weak — the user eyeballing a lead paraphrase — at the highest-stakes location: the next workflow's input, where `specify`'s critic cannot catch a flipped ruling); refuse the second artifact (rules against the user's own revealed behavior and makes every downstream consumer pay the 29k record read forever).
- **Mark:** Confident *(recommendation marked only after post-streak re-engagement; adopted with acknowledgment)*.

## Deliberate non-decisions

- **The lead's 355k — the largest single pool — is left alone, eyes open.** The spend is mostly the session itself (the user-facing product) plus the tail; D1/D4 trim the tail indirectly (fewer messages to process, one verify report, lead-owned merge it effectively did anyway). Accepted, not overlooked.
- **Limit/concurrency at review (F8):** no redesign. D1's cheaper cold reads shrink the burst that tripped the limit; recovery posture held as-ran. Watch, don't engineer.

## Open threads — named checks for the v2.2 dogfood

- **Lens-split coverage bet:** does any Important-class finding arrive via the "wrong" lens, or get missed for lack of double coverage? (The convergence-as-confidence signal — M1/M2 found-by-both — is deliberately gone.)
- **One-shot exchange bet:** does severity calibration (the F3/F4 class) survive a single attack/response round? (As-ran took 6 messages; the 4-message shape is untested.)
- **Argument cap watch (F11):** now 0-for-2. If 0-for-3, decide whether the path is dead weight or the findings keep being simply right.
- **Sizing gate:** fires sensibly; the "none" waiver text actually lands in the record; single mode's self-verify reads clearly.
- **Synthesis fidelity check:** runs when requested; the "derived, unchecked" stamp appears on the waived path.
- **Cost re-measure vs this run:** target pair ≈150–170k (from 270k), total materially under 654k, from transcripts.
- **This record is un-reviewed** — a cold pass (one reviewer, per D3's own logic: lean record, five decisions) can be requested.

## Build plan — executed 2026-07-16 (v0.6.0)

1. `plugins/mochiko/commands/brainstorm.md`: team section (lens briefs, map-verbatim discipline), convergence section (sizing gate + recorded waiver), review flow (4-message exchange, fact-route dedup, lead-owned merge, integrity-reviewer verify + single-mode owner), post-acceptance synthesis clause (D5), contract fill update (new gate; waiver variant; bounds).
2. `plugins/mochiko/skills/validation-brainstorm/SKILL.md`: map-as-substrate + sample-audit in Phase 1; one-shot message shape + dedup rule in Phase 2; verify-when-assigned; lens-scoping note (dispatch-level, skill stays whole).
3. `BACKLOG.md`: close the v2.1 dogfood item with this record's outcomes (incl. F1 cost + earns-its-keep verdicts); open the v2.2 item with the named checks above.
4. `ROADMAP.md` Key Decisions row; version bump 0.5.0 → 0.6.0 on build.

## Provenance

Session: lead + user via bare `mochiko:analysis-iterative`; opened 2026-07-05 with transcript forensics over the completed v2.1 dogfood run (`.mochiko/brainstorms/setup-constitution-flexibility/` + its four `.jsonl` transcripts), concluded 2026-07-16. Five decisions ruled live by the user. A three-adoption ratification streak was flagged after D3 per the questioning skill; the user re-engaged (parked the then-open fork, independently probed the review mechanics, then ruled D4/D5). Build deliberately not executed in-session.
