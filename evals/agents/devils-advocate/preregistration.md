# Preregistration — devils-advocate persona plan-only eval (wave A baseline kit)

Committed BEFORE the baseline grid (`primitive-eval-harness-v2` D11 as folded; the pilot rulings
`2026-09-09-persona-pilot-1-latitude-out-of-instrument` and `2026-09-09-persona-pilot-2-validator-read`).
This is a **baseline kit**: no persona edit exists to detect, so there is no positive control. The kit
fixes the band and the coverage so that the persona's next edit gets a `pre`/`post` read on day one.
Fields marked **[measured at baseline]** are filled from the baseline grid.

## Edit under read

None. `pre` = `post` = the tree at `b9efb59` (the commit the rubric was minted at; the persona file is byte-identical in the working tree), all claims `common`. The pin, not `HEAD`, is what the next edit's `--old-ref` names. The next persona
edit re-mints over the union of refs and runs the pilot form with a positive control on the edit.

## Read rule (D11)

- **Prune FIRST** = `agent-prune devils-advocate --replicates 3` over the tempting goldens (≤ 12 sessions): claims the
  bare model meets in every replicate are tagged `model_native` and leave the graded set. It runs before
  the grid because the grid freezes the rubric snapshot the report grades from.
- **Baseline grid** = `agent-grid devils-advocate --arms post --replicates 3 --out baseline` (4 goldens × 3 = 12
  plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in force; pairwise
  off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed.
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio)
  and, for devils-advocate, `conditional` (the runtime gap-finding remit — no plan-only golden plants a
  running system). Any `latitude-conditional` claim found at partition is likewise out.

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

- band: **17.5 %** — filled 2026-09-09 from `runs/baseline` (12 `post` sessions, $2.94; judge $0.54 / 12 calls) on
  the graded set after prune (`runs/baseline-prune`, 12 sessions, $3.14: **15 of 18 claims model-native** — the
  document-review craft is largely the bare model's on plan-only; graded: `approving-specs-critical-gaps`,
  `authoring-fixing-spec-yourself`, and `require-evidence-approval-ready` until the read below moved it): flaky
  pairs 1/8 = 12.5 % on the two-claim set (+ 5 = 17.5 %, uncapped) — ship bar (b) met.
- read: `approving-specs-critical-gaps` pass^k reflected on all four goldens; `authoring-fixing-spec-yourself`
  pass^k on d1/d2/d4 and **contradicted once on d3** (the persona patched the spec's wording in one of three
  replicates under the cover note's explicit patch ask) — a persona finding to carry, not instrument noise.
- calibration: lead sheet **0.708** (24 pairs, labelled from keyword extracts — a process defect: not a full read);
  an independent second labeller reading every plan in full agreed with the lead on only **0.667** of pairs and
  with the judge on **0.792**; every judge/labeller-2 disagreement (5) was `require-evidence-approval-ready`,
  which binds a READY verdict no wave-A golden can yield — the judge applied the pre-registered conditional rule
  correctly, the brief to the labeller had widened it. That claim is now `out-of-instrument: conditional` and
  removed from tempts; on the remaining 16 pairs judge/labeller-2 agreement is **1.00**, judge/lead 0.875.
  Process rule from here: calibration labels come from FULL plan reads, never keyword extracts.
- contradicted bar: one `contradicted` pair (d3, judge and both labellers agree) — bar exercised on one pair,
  recorded as under-exercised (< 2).
- re-key count: 0 (no judge or golden change; one partition move by the conditional rule)
- kit status: **READY** for the next `devils-advocate` edit — graded set two claims; the cheap-read trio, the
  runtime remit, and the ready-verdict claim out of instrument; fifteen claims model-native.
- band recount under ADR `2026-09-09-persona-band-invited-only` (invited pairs only): `post` 1/2 = 50 % → **UNDER-SAMPLED**, band at the cap (20 %); the one flaky invited pair is the d3 persona finding. Kit stays READY with the mark; its next edit takes one extra replicate per arm before any difference is read.
