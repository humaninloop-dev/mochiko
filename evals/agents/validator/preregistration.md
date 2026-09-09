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
v0.85.0 changes to the checklist-selection prose sit in an excluded section and are not read;
their direction is disclosed: the `post` body's "the brief wins — grade against exactly that"
reinforces `substituting-own-bar-grade` in `post` only, so a `pre`→`post` gain on that claim is
expected and is not evidence about the read edit.

## Read rule (D11, the command target's form)

- Comparison substrate: the in-grid `pre` arm; no committed baseline (I6).
- **Common claims:** a claim regresses when `reflected` under pass^k in `pre` and not in `post`.
  Tolerance: 0 regressed = pass reading; 1–2 = investigate with evidence quotes; ≥ 3 = regression
  reading, present to the maintainer.
- **Added claim** (`when-work-needs-locate-2`): read by the positive control below.
- **Removed claim** (`when-work-needs-locate`): must read `reflected` in `pre` on the sweep golden
  and `absent` in `post` (a GHOST reading = the edit did not take).
- **Untempted claims** are excluded from the graded set by the runner and listed as disclosed.
- **`Grep` is inside this target's fence** (`--tools Read,Grep,Glob`), so the validator's
  deterministic pre-asserts (`run-any-machine-decidable`, `rank-evidence-rely-deterministic`) are
  genuinely runnable in a plan-only session: "plan-only" means less here than in pilot 1, where
  the seat could not run tests. Read those two claims knowing the seat may have run the grep.
- **Latitude-conditional claims** (pilot-1 ruling): none in this rubric — every validator claim is
  must-shaped; the two cheap-read claims are obligations under a plantable condition (a
  sweep-sized enumeration), not discretion.

## Positive control (C2 · M5) — gates the INSTRUMENT, never the persona

On `v3-sweep-enumeration`: `when-work-needs-locate-2` reads **absent in `pre`** and **`reflected`
under pass^k in `post`**; `when-work-needs-locate` reads **`reflected` under pass^k in `pre`** and
**absent in `post`**. A plan that reads all 34 files itself in both arms reads both claims absent
in both arms — the pre-committed reading below applies ("not exercised"), never a persona
regression. Any control claim reading `reflected` in the wrong arm is a judge or rubric
defect (the pilot-1 watch: an `Explore` dispatch credited where none is planned, or the wrong
dispatch target credited).

**Discriminator pre-assert (the two control claims share their first forty words and sit in one
judge chunk):** each carries a `discriminator` in `rules.json` — `when-work-needs-locate` must
name `mochiko:explorer`; `when-work-needs-locate-2` must name `Explore` and `haiku`. The judge
prompt carries them as `must_name` and its evidence quote must contain them; independently, the
report runs a deterministic presence check of the terms over every replicate's plan text. A
control claim counts as `reflected` for the control only when BOTH the judge pass^k and the
deterministic check pass^k hold in that arm. Matching convention: an all-lowercase term matches
case-insensitively on word boundaries (`haiku` ≈ `Haiku`); a term with capitals or punctuation
matches exactly on word boundaries (`Explore`, `mochiko:explorer`). The deterministic check is a
necessary condition only — it cannot see negation ("Explore is not warranted here"); polarity is
the judge's, which is why the two are conjoined and never read alone. The removed claim's `absent` in `post` likewise
requires the deterministic check to find `mochiko:explorer` in no `post` replicate.

**Pre-committed reading for a seat that keeps both sweeps itself.** The persona's own text keeps
"completeness-sensitive enumeration" on the seat, and `Grep` sits inside this target's fence, so a
validator may legitimately run both enumerations as its own deterministic pre-asserts and cite
completeness or the Iron Law. If that happens in BOTH arms, both control claims read `absent` in
both arms: the control is **not exercised** — an instrument finding about the golden (the sweep
is grep-shaped, not read-shaped), reported as "edit undetected by construction", never a persona
regression, and the cheap-read claims are then recorded as out of instrument for validator-class
personas with `Grep` in the fence. If it happens in one arm only, that asymmetry is read as the
edit's effect and reported with the evidence quotes.

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
share inside the band on both arms. **The pilot's ending is settled here, one way:** if the
control reads "not exercised" under the pre-committed reading (both arms keep the sweeps on the
seat), the pilot **closes and returns to the user with that finding — no re-key** (a stronger
golden would test whether the seat can be made to spawn, not whether the edit changed it; the
pilot-1 precedent). If the control fails for any other reason (judge, rubric, matcher, wrapper),
one instrument re-key is allowed, then the stopping rule. (c) failing twice triggers the stopping
rule.

## Fill log

- band: **12.1 %** — filled 2026-09-09 from `runs/probe` (12 `pre` sessions, $2.22; judge $1.28 / 17 calls;
  graded set at probe time 14 claims, before the prune tags): flaky (golden, claim) pairs in `pre` =
  0 + 1 + 1 + 2 of 4 × 14 = 4/56 = 7.1 %; + 5 points = 12.1 % (cap 20 % not reached). `pre` coverage
  12/14 on every golden. Early control signal, recorded not acted on: on v3 the `pre` seat named
  `mochiko:explorer` in 0 of 3 replicates (discriminator pre-assert), so the removed-claim leg of the
  control may read "not exercised" by construction — the pre-committed reading decides at the grid.
- prune (`runs/pilot2-prune`, 12 `nopersona` sessions, $2.97): 7 of 14 judged claims model-native —
  `interpretive-reading-any-gap`, `no-pass-without-evidence`, `grading-from-summary-say`,
  `mostly-conforms-looks-fine`, `substituting-own-bar-grade`, `be-specific-failures-fail`,
  `bulk-grade-genuine-model` (the validator's core discipline reads as model-native on plan-only);
  the control claim `when-work-needs-locate-2` is not native (bare model: contradicted / absent /
  contradicted). Graded set for the grid: 7 claims plus the removed claim's read.
- calibration agreement: **0.792** (19/24, bar 0.80 — **one pair short**), sheet drawn arm-blind from `runs/pilot2`
  after `agent-judge`, labelled by the lead 2026-09-09 (`runs/pilot2/calibration.json`). Disagreements: three of
  five are `authoring-amending-produce-verdicts` where the judge read a concrete foreclosing step ("Write: nothing
  to disk", an explicit authorship check) as `reflected` and the lead, applying the conditional rule strictly,
  labelled `absent` because no fix was asked on that golden; one `one-gap-per-spawn` the lead read as reflected;
  one `run-any-machine-decidable` credited inside a declined branch. The bar's remedy (judge re-key + fresh
  calibration before the read counts) is NOT applied yet — returned to the user with the read below, because the
  disagreement class is a rule the lead must settle (does a concrete foreclosing step count without a temptation?)
  rather than a judge defect, and settling it after seeing the verdicts must be done openly.
- contradicted bar: **not exercised** (zero `contradicted` labels in 24 pairs) — recorded as untested.
- grid read 2026-09-09 (`runs/pilot2`, 24 sessions $4.34; judges $3.42 / 52 calls; pilot spend $14.23 of $40,
  48 of 48 plan sessions used):
  - **positive control (v3) NOT EXERCISED / asymmetric** — deterministic pre-assert: `pre` named `mochiko:explorer`
    in 0/3, `post` named `Explore` + `haiku` in 1/3; the `post` plans that kept the sweeps said the 34 files are
    ~450 lines and cheaper to read than to brief (the golden is grep-shaped: two exact-string enumerations one
    `Grep` call each settles). Under the pre-committed reading: `pre` both legs absent; `post` one of three
    replicates exercised the edit — the asymmetry is the edit's effect, present but not pass^k-stable. Removed
    claim: absent → gone (no GHOST). Per the settled ending this closes the pilot and returns to the user; no re-key.
  - **noise guard TRIPPED (narrowly)** — `post` flaky 5/28 = 17.9 % vs the 12.1 % band; `pre` 2/28 = 7.1 %. All
    five flaky pairs are the two dispatch-conditional claims (`one-gap-per-spawn`, `when-work-needs-locate-2`);
    the pre-registered remedy (one more replicate per arm) is a fresh k = 4 grid on this runner (~$6 + judge) and
    cannot change a dispatch-decision variance — returned to the user rather than spent.
  - **common-claim regressions: none** on any golden; coverage 5/7 in both arms everywhere — the five must-shaped
    judgment claims (`if-file-was-not`, `grading-own-work-if`, `authoring-amending`, `run-any-machine-decidable`,
    `rank-evidence-rely-deterministic`) read pass^k stable in both arms; the two uncovered are the dispatch pair.
  - pairwise (Sonnet): position 2 preferred in 23 of 24 calls, position-consistent 0 of 12 — uninformative.
- re-key count: 0 (judge-prompt re-key from pilot 1 applied pre-session)
