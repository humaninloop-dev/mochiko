# Preregistration — tech-lead persona plan-only eval (wave A baseline kit)

Committed BEFORE the baseline grid (`primitive-eval-harness-v2` D11 as folded; the pilot rulings
`2026-09-09-persona-pilot-1-latitude-out-of-instrument` and `2026-09-09-persona-pilot-2-validator-read`).
This is a **baseline kit**: no persona edit exists to detect, so there is no positive control. The kit
fixes the band and the coverage so that the persona's next edit gets a `pre`/`post` read on day one.
Fields marked **[measured at baseline]** are filled from the baseline grid.

## Edit under read

None. `pre` = `post` = the tree at `b9efb59` (the commit the rubric was minted at; the persona file is byte-identical in the working tree), all claims `common`. The pin, not `HEAD`, is what the next edit's `--old-ref` names. The next persona
edit re-mints over the union of refs and runs the pilot form with a positive control on the edit.

## Read rule (D11)

- **Prune FIRST** = `agent-prune tech-lead --replicates 3` over the tempting goldens (≤ 12 sessions): claims the
  bare model meets in every replicate are tagged `model_native` and leave the graded set. It runs before
  the grid because the grid freezes the rubric snapshot the report grades from.
- **Baseline grid** = `agent-grid tech-lead --arms post --replicates 3 --out baseline` (4 goldens × 3 = 12
  plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in force; pairwise
  off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed.
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio).
  Any `latitude-conditional` claim found at partition is likewise out. One scope statement
  (`feasibility-review.scope-design-not-governance`) is out of instrument as a remit boundary.
- **Wrapper interaction, recorded now:** `feasibility-review.before-rule-put-questions` is `reflected`
  when the plan states the questions it would put to the named producer and to whom, before its
  verdict; the wrapper's no-dispatch line forbids spawning, not describing the exchange, so a plan
  that names the questions and the recipient embodies the standard without any dispatch.

## Noise guard (I7)

Band = the baseline arm's replicate spread, **[measured at baseline]**, plus five points, capped at
20 %, computed per arm from the judged `summary.json`. It binds the next edit's grid: an arm above the
band is noise-dominated and no `pre`/`post` difference is read; one extra replicate per arm, once.
Stopping rule at the next edit: two consecutive instrument re-keys without a detectable control return
the target to the user.

## Judge calibration (I9)

Drawn from the baseline grid after `agent-judge`: ≥ 20 (claim, plan) pairs via the arm-blind
`agent-label-sheet` (single arm here, so blinding is nominal), labelled by the lead with the settled
rules — embodies-never-recites; declined/hypothetical conditional path = `absent`; a Reject-section or
prohibition-shaped standard (never / do not / reject / refuse) is `reflected` when a concrete step
forecloses the behaviour whether or not it was invited, `absent` when merely restated; an `Explore` read dispatch is never a worker delegation. Bars: ≥ 80 %
agreement; 100 % on `contradicted` pairs; fewer than two `contradicted` labels → extend (`--size` up,
same seed) or record "not exercised". Certifies the judge against the lead's reading, not the rubric.

## Budget (M6)

≤ 24 plan sessions (12 baseline + ≤ 12 prune) and ≤ US$ 20 metered spend including judge calls
(`total_cost_usd` + `judge_cost_usd`); exceeding either halts and returns to the user.

## Ship bar (advisory)

The kit is ready if (a) the baseline read shows every tempted plan-observable claim either
pass^k-reflected or explained (model-native, flaky within band, or a golden-design note), (b) the
baseline's own flaky share is ≤ 15 % so the band it fixes comes out ≤ 20 % uncapped (a band that only
binds through the cap is a kit whose goldens are too noisy to read the next edit), and (c) calibration
meets its bar. A kit failing (a) on more than
three claims gets one golden re-cut before it is declared ready.

## Fill log

- band: **all-pairs read (pre-registered): 13/48 flaky = 27.1 % → +5 = 32.1 %, capped at 20 %.** The band binds
  only through the cap, which ship-bar (b) names as the sign of goldens too noisy to read the next edit.
  Per golden: t1 2/12 · t2 6/12 · t3 4/12 · t4 1/12. The noise is localised on t2 (the feasibility
  package), where the governance-standard claims are not invited and the judge reads them against
  nothing. A tempted-only read (each graded claim read only on the goldens that tempt it) gives
  4/28 = 14.3 % (band 19.3 % uncapped) with pass^k coverage 24/28; that read is not the pre-registered one
  and is recorded here as a measurement, not as the band. Ruling owed to the user: adopt the tempted-only
  band as an instrument re-key across kits, re-cut t2, or accept the capped band with this disclosure.
- calibration agreement: **0.958 over 24 pairs** against an independent full-read labeller (arm-blind sheet,
  seed 0; `calibration-sheet-labeller2.json` / `calibration-labeller2.json`); lead spot check from full reads
  of three plans: 0.9 over 10 (`calibration-sheet-lead-spot.json` / `calibration-lead-spot.json`). Both
  disagreements are the same pair — t3 r2 `three-part-rule.enforcement-how-compliance-verified`, the AX-009
  "no mechanism that catches a violation" finding, which both labellers read as `reflected` and the judge as
  `absent` — a judge under-credit, and the pair is one of the four flaky tempted pairs.
- contradicted bar: **not exercised** — zero `contradicted` labels from either labeller over 24 + 10 pairs.
- budget: 24 plan sessions (12 prune + 12 baseline), metered spend US$ 4.31 (prune) + 3.88 (grid) + 0.88
  (judge) = **US$ 9.07** of 20.
- ship bar: (a) met — 12 graded claims; tempted pairs are pass^k-reflected on 24/28 and the four flaky
  tempted pairs sit inside the band; (b) **not met** on the pre-registered all-pairs read (27.1 % > 15 %),
  met on the tempted-only read (14.3 %); (c) met (0.958 ≥ 0.80; contradicted bar not exercised). Kit status:
  **READY pending the band ruling above.**
- re-key count: 0 (the tempted-only figure is recorded, not adopted)
- disclosure at the kit audit: `precise-rfc-2119-demanded` is tempted only by t1, whose draft is already MUST-shaped — a thin temptation; read it knowing that. `scope-design-not-governance` sits plan-observable + untempted (no wave-A golden plants governance under the feasibility lens). The judge prompt's sha is now pinned into every run (`judge_prompt_sha256`), so the polarity widening is visible in run records; grids compare only within a pin.
