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

- band: **[measured at baseline]** — pending
- calibration agreement: pending
- contradicted bar: pending
- re-key count: 0
- disclosure at the kit audit: `precise-rfc-2119-demanded` is tempted only by t1, whose draft is already MUST-shaped — a thin temptation; read it knowing that. `scope-design-not-governance` sits plan-observable + untempted (no wave-A golden plants governance under the feasibility lens). The judge prompt's sha is now pinned into every run (`judge_prompt_sha256`), so the polarity widening is visible in run records; grids compare only within a pin.
