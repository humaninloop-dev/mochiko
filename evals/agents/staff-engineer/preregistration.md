# Preregistration — staff-engineer persona plan-only eval (pilot 1)

Committed BEFORE the first grid (`primitive-eval-harness-v2` D11 with folds C2 · I7 · I8 · I9 · M5
· M6; the runner refuses `agent-grid` without this file). Amending it after results exist is a
recorded, deliberate act — never a quiet retro-fit. Fields marked **[measured at probe]** are
filled from the probe run below, before the grid, and the fill is recorded here with its date.

## Edit under read

`pre` = `b099154` (the v0.107.0 tree; persona without `## Delegating Bounded Work`) ·
`post` = the working tree at v0.108.0 (persona with the section). The nine `added` claims are
the section's units; `tempts` in `evals.json` names which golden invites each.

## Read rule (D11, the command target's form)

- Comparison substrate: the **in-grid `pre` arm** (never a committed baseline — the persona
  target keeps none at pilot, I6).
- **Common claims:** a claim regresses when it is `reflected` under pass^k in `pre` and not in
  `post`. Tolerance: **0** regressed claims is the pass reading; 1–2 = "investigate — read the
  evidence quotes before any verdict"; ≥ 3 = "regression reading, present to the maintainer".
- **Added claims:** graded on the goldens that tempt them; the positive control below reads them.
- **Removed claims:** none at this edit (the section was added, nothing cut); the report's
  removed-claim line must be empty.
- **Untempted claims** (tagged in `rules.json`) are excluded from the graded set by the runner
  (`agent-report` lists them as disclosed, never reads them as absent); they are the pilot's
  disclosed gap: the cheap-reads trio and `reproducing-reported-failure`.

## Positive control (C2 re-cut, M5) — gates the INSTRUMENT, never the persona

On the delegation-forcing golden (`g2-delegation-forcing`), the seven plan-observable added
claims — `when-task-front-already` (dispatch named: general-purpose worker, `model: sonnet`),
`one-task-per-spawn`, `brief-pins-one-task`, `readback-before-counts`,
`disclose-delegation-in-report`, `failed-readback-redone`, and `rung-execution-have-already`
(judgment legs kept on the seat) — must read **absent in `pre`** (the section did not exist)
**and `reflected` under pass^k in `post`**. Any of those ids reading
`reflected` in `pre` is a rubric or judge defect; any reading `absent` in `post` is an
instrument finding first (wrapper, golden, judge) and a persona finding only after the
instrument is cleared. A failed control re-keys the runner, rubric, or goldens — it is never
reported as a persona regression. The smoke golden `g1-decided-card` is excluded from the
control by construction (a seat reasoning about the cost of briefing versus doing a four-line
green phase does it itself); its `tempts` carry no delegation claim.

## Noise guard (I7)

Tolerance band = the `pre` arm's own replicate spread, **[measured at probe]**: the share of
graded claims with replicate disagreement inside `pre` over the probe run (all goldens, k = 3),
computed per arm from the judged `summary.json` (the report prints it as "flaky share per
arm"). The band is that measured share plus five points, capped at 20 %. A grid whose `pre`
or `post` flaky share exceeds the band is noise-dominated: no pre/post difference is read; add
one replicate per arm and re-judge once. The guard's trip is an instrument finding.

**Stopping rule:** two consecutive instrument re-keys (runner, rubric, goldens, or judge
changes made to make the positive control detectable) without a detectable control return
the persona target to the user for a keep / re-shape / abandon ruling. Re-key count for this
pilot starts at 0 and is logged below.

## Judge calibration (I9)

Before the grid's judged read counts: a **hand-labelled calibration set** of at least 20
(claim, plan) pairs drawn from the **grid run's plans after `agent-judge` and before any
reading** (both arms, so the `reflected` direction of the seven control claims is exercised;
the probe stays pre-only and feeds the band) via `agent-label-sheet`, which is **arm-blind** —
the labeller sees an opaque plan key and an anonymized plan copy, never the arm (the key map
sits beside the sheet and the blinding is honour-system, disclosed) — each labelled by the
lead as `reflected` / `absent` / `contradicted` with the embodies-never-recites rule (a restated
principle without a task-specific action = `absent`; a declined or hypothetical conditional
path = `absent`; a Reject-section standard is `reflected` when a concrete action avoids the
behaviour). The Haiku checklist judge must agree with the labels on **≥ 80 %** of pairs, and on
**100 %** of the pairs labelled `contradicted`. If the sampled sheet carries fewer than two
`contradicted` labels, the labeller extends the sheet (`--size` up, same seed — a new seed would re-key
the map) until it carries two, or records **"contradicted bar not exercised"** in the fill log — the bar is then
disclosed as untested, never counted as passed. Below either bar the judge prompt is re-keyed
(counts toward the stopping rule) and re-calibrated before any grid reading. Calibration
certifies the judge against **the lead's reading of the plans**, not against the rubric: a
claim the lead and the judge both read wrongly is not caught here. Labels and agreement are
recorded under `runs/<grid>/calibration.json`.

## Grid shape and budget (M6)

**Probe run** = `agent-grid staff-engineer --arms pre --replicates 3 --out probe`: 4 goldens × 3 ×
`pre` = 12 plan sessions, judged, from which the band is taken.
**Grid** = 4 goldens (`g1-decided-card` smoke · `g2-delegation-forcing` · `g3-contradiction` ·
`g4-brownfield-interface`) × 3 replicates × 2 arms (`pre` + `post`) = 24 plan sessions (the
probe's `pre` plans are NOT reused — the grid is one run). **Prune** = the one-time `nopersona`
pass over the tempting goldens, ≤ 12 plan sessions. Total **≤ 48 plan sessions**. Judges: Haiku
embodiment checklist (one call per ≤ 15 claims per plan) and Sonnet pairwise position-swapped
(two calls per pre/post pair), both advisory. **Budget bound: ≤ 48 plan sessions and ≤ US$ 40
metered spend for the whole pilot including judge calls** — plan-session spend from each run's
`total_cost_usd`, judge spend from the `judge_cost_usd` / `judge_calls` that `agent-judge` and
`agent-prune` record in `summary.json`; exceeding either halts the pilot and returns to the user.

## Ship bar (advisory instrument — informs, never gates)

The instrument is useful if, on this first real edit, it (a) passes the positive control on
`g2-delegation-forcing`, (b) reports zero common-claim regressions or localizes each to a
named claim id with evidence quotes, and (c) keeps its flaky share inside the band on both
arms. (a) failing after one re-key, or (c) failing twice, triggers the stopping rule.

## Fill log

- band: **20 %** — filled 2026-09-09 from probe run `runs/probe` (12 `pre` sessions, $2.74; judge $1.57 / 27
  calls): flaky (golden, claim) pairs in `pre` = 3 + 2 + 6 + 0 of 4 × 18 = 11/72 = 15.3 %; + 5 points =
  20.3 %, capped at 20 %. Observation recorded, not acted on (the read rule stays as pre-registered):
  g3's six flaky claims are all delegation claims g3 does not tempt — replicate disagreement
  concentrates on (golden, claim) pairs where the golden never invites the claim. A tempted-only
  read is a candidate re-key if the grid trips the guard; it is not applied pre-emptively.
  Pre-arm sanity for the control: on g2 the seven delegation claims read `absent` in every `pre`
  replicate (none flaky), as the control requires.
- calibration agreement: pending (drawn from the grid run, arm-blind)
- contradicted bar: pending
- re-key count: 0
