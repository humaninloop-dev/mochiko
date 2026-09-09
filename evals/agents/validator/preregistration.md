# Preregistration — validator persona plan-only eval (pilot 2)

Committed BEFORE the first grid (`primitive-eval-harness-v2` D11 with folds C2 · I7 · I8 · I9 · M5
· M6, and the pilot-1 ruling `2026-09-09-persona-pilot-1-latitude-out-of-instrument`). The runner
refuses `agent-grid` without this file. Amending it after results exist is a recorded, deliberate
act. Fields marked **[measured at probe]** are filled from the probe run before the grid.

## Edit under read

`pre` = `7536b95` (the v0.77.0 tree: `## Delegating Cheap Reads` names a `mochiko:explorer`
dispatch) · `post` = the working tree (the v0.78.0 retarget: a native `Explore` subagent with an
explicit `model: haiku` override). The pair `when-work-needs-locate` (removed) /
`when-work-needs-locate-2` (added) is the edit; every other claim is `common`. The v0.84.0 /
v0.85.0 changes to the checklist-selection prose sit in an excluded section and are not read.

## Read rule (D11, the command target's form)

- Comparison substrate: the in-grid `pre` arm; no committed baseline (I6).
- **Common claims:** a claim regresses when `reflected` under pass^k in `pre` and not in `post`.
  Tolerance: 0 regressed = pass reading; 1–2 = investigate with evidence quotes; ≥ 3 = regression
  reading, present to the maintainer.
- **Added claim** (`when-work-needs-locate-2`): read by the positive control below.
- **Removed claim** (`when-work-needs-locate`): must read `reflected` in `pre` on the sweep golden
  and `absent` in `post` (a GHOST reading = the edit did not take).
- **Untempted claims** are excluded from the graded set by the runner and listed as disclosed.
- **Latitude-conditional claims** (pilot-1 ruling): none in this rubric — every validator claim is
  must-shaped; the two cheap-read claims are obligations under a plantable condition (a
  sweep-sized enumeration), not discretion.

## Positive control (C2 · M5) — gates the INSTRUMENT, never the persona

On `v3-sweep-enumeration`: `when-work-needs-locate-2` reads **absent in `pre`** and **`reflected`
under pass^k in `post`**; `when-work-needs-locate` reads **`reflected` under pass^k in `pre`** and
**absent in `post`**. A plan that reads all ~30 files itself in both arms reads both claims absent
in both arms — that is an instrument finding about the golden's sizing (re-key candidate), never a
persona regression. Any control claim reading `reflected` in the wrong arm is a judge or rubric
defect (the pilot-1 watch: an `Explore` dispatch credited where none is planned, or the wrong
dispatch target credited).

## Noise guard (I7)

Band = the `pre` arm's replicate spread on the probe, **[measured at probe]**, plus five points,
capped at 20 %, computed per arm from the judged `summary.json`. A grid arm above the band is
noise-dominated: no pre/post difference is read; add one replicate per arm and re-judge once.
**Stopping rule:** two consecutive instrument re-keys without a detectable control return the
persona target to the user. Re-key count starts at 0 (pilot 1's judge-prompt re-key for the
Explore-versus-worker confusion was applied before this pilot's first session and counts as
this pilot's re-key 0, not 1 — recorded here so the count is honest).

## Judge calibration (I9)

Drawn from the grid run after `agent-judge`, before any reading: ≥ 20 (claim, plan) pairs via the
arm-blind `agent-label-sheet` (map beside the sheet, honour-system, disclosed), labelled by the
lead with the embodies-never-recites rule (restated principle without a task-specific action =
`absent`; a declined or hypothetical conditional path = `absent`; a Reject-section standard is
`reflected` when a concrete action avoids the behaviour; an `Explore` read dispatch is never a
worker delegation). Bars: ≥ 80 % agreement; 100 % on pairs labelled `contradicted`; fewer than two
`contradicted` labels → extend the sheet (`--size` up, same seed) or record "contradicted bar not
exercised". Calibration certifies the judge against the lead's reading, not the rubric. Recorded at
`runs/<grid>/calibration.json`.

## Grid shape and budget (M6)

Probe = `agent-grid validator --arms pre --replicates 3 --out probe`: 4 × 3 = 12 plan sessions,
judged. Grid = 4 goldens × 3 × 2 arms = 24 plan sessions. Prune = `nopersona` over the tempting
goldens, ≤ 12. Total ≤ 48 plan sessions. Judges: Haiku embodiment checklist, Sonnet pairwise
(position-biased in pilot 1 — advisory, read with that in mind). **Budget bound: ≤ 48 plan
sessions and ≤ US$ 40 metered spend for this pilot including judge calls** (plan spend from
`total_cost_usd`; judge spend from `judge_cost_usd` / `judge_calls`); exceeding either halts and
returns to the user.

## Ship bar (advisory instrument)

Useful if it (a) passes the positive control on v3 in both directions, (b) reports zero
common-claim regressions or localizes each to a named claim with evidence, and (c) keeps the flaky
share inside the band on both arms. (a) failing after one re-key, or (c) failing twice, triggers
the stopping rule.

## Fill log

- band: **[measured at probe]** — pending
- calibration agreement: pending
- contradicted bar: pending
- re-key count: 0 (judge-prompt re-key from pilot 1 applied pre-session)
