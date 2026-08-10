# Benchmark pause state — 2026-08-10 (after CP1 + CP2, before CP3)

**Status:** paused by user ruling after CP1 (setup × 4 arms × r1, judged) and CP2
(specify × 4 arms × r1, judged). CP3 (replicates) not started. Resume point: the CP3
options in `cp2-checkpoint-report.md` — (a) full r2 wave · (b) targeted r2 (agents floor
reproduction + body confirmation; lead-recommended) · (c) stop and rule on n=1.

## Where every piece lives

- Protocol: `runs/RUN-PROTOCOL.md` (incl. checkpointed-stages amendment)
- Fixture: `fixture/persona-card.md` (frozen) · specify seed: `runs/_seed-project/`
- Variants: `variants/body/` (11) · `variants/descriptions/` (11) · `variants/agents/` (6)
- Rubrics: `rubric/` (setup 12 rows · specify 12 rows · 14 floors)
- Runs: `runs/<command>-<arm>-r1/` ×8, each with project/ + transcript + invocations + costs + meta
- Scorecards: `report/cp1-judge-scorecard.md` · `report/cp2-judge-scorecard.md`
- Checkpoint reports: `report/cp1-checkpoint-report.md` · `report/cp2-checkpoint-report.md`
- Anonymization keys: `report/cp1-anon-key.txt` · `report/cp2-anon-key.txt` (judges never read)

## Confidence ledger — what two judged runs support

### High confidence (consistent across both commands, mechanism-level evidence)

1. **Slim skill descriptions route correctly.** 0 fire misses, 0 wrong-sibling picks over
   12+ invocation moments across both commands, including the two deliberate disambiguation
   traps (analysis-iterative vs review-specifications; feature-map vs user-stories) and all
   correct non-fires. This is count-based mechanism evidence, not judge-scored — the rank-flip
   noise does not touch it. Descriptions averaged 68% smaller.
2. **Agent routing survives without example blocks.** 0 route misses over 9+ staffing
   decisions; at specify all four varied personas staffed correctly from prose-only
   descriptions; correct non-staffing recorded with basis. Example blocks are 69–81% of every
   agent description and bought zero routing accuracy in these runs.
3. **No cut arm degrades catastrophically.** Every cut arm, both stages, scored within the
   10% threshold (worst: −5.4%). All 8 runs produced complete, accepted, floor-passing
   artifact sets (one process-floor exception below). The "detail is load-bearing for basic
   competence" hypothesis found zero support.
4. **Guardrails body variants cut cost materially without breaking anything.** −18% skill-bytes
   at setup, −33% at specify; combined mean score Δ ≈ 0% vs baseline (+3.7 / −3.7).

### Medium confidence (real signal, single-occurrence or judge-score-based)

5. **Full-detail skills over-produce ceremony.** CP1: baseline was the heaviest set and lost
   points ONLY on right-sizing (the judge read 30 GI elements + mandatory add-ons as
   over-governance for the card's founder). Detail's failure mode may be too-much, not
   too-little — one occurrence, one judge.
6. **Detail's one visible payoff is judgment-margin honesty, not competence.** CP2: baseline
   won solely on planted-vague-zone honesty (only set inventing nothing) and edge-case depth
   (only H's review caught the pay-link enumeration gap — with a guardrails body). The floors,
   stories, FRs, prototypes were equivalent across arms.

### Not established (why CP3 exists)

7. **Score rankings at these margins.** Baseline ranked 4th at CP1 and 1st at CP2 — the same
   variant, same judge protocol. Single-run variance is demonstrably material at the observed
   spreads (≤5.4 pts). No per-arm score verdict is formal until r2 (D6 noise guard unfired).
8. **The agents-arm floor violation (F-X1).** One run, weak causal chain (a run-lead left no
   independent-review evidence in artifacts; staffing itself was clean). Needs r2 to separate
   variant effect from run noise. Strict-D6 reading (example blocks return) is on the table
   if it reproduces.
9. **Generalization beyond setup+specify.** Near-cap description skills in plan/implement/
   brainstorm stages untested (record open thread 1). Nature-coverage argument only.
10. **Simulation validity.** All runs single-agent simulations with model-played principal —
    real multi-seat runs with a human principal may stress descriptions/examples differently.

## What the evidence already licenses regardless of CP3 (lead read, not a ruling)

- The description arm's mechanism evidence (1) is strong enough that slim descriptions are
  shippable on their own track if the user wants an early harvest — fire-rate was the ONLY
  question for that arm; score deltas ride judge noise.
- The body arm needs r2 for a formal D6 verdict but nothing in two runs argues against it.
- The agents arm is the only one with an open adverse fact (8).
