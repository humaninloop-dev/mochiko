# Preregistration — product-manager persona plan-only eval (wave B baseline kit)

Committed BEFORE the baseline grid (`primitive-eval-harness-v2` D11 as folded; the pilot rulings
`2026-09-09-persona-pilot-1-latitude-out-of-instrument` and `2026-09-09-persona-pilot-2-validator-read`;
the band ruling `2026-09-09-persona-band-invited-only`). This is a **baseline kit**: no persona edit
exists to detect, so there is no positive control. The kit fixes the band and the coverage so that the
persona's next edit gets a `pre`/`post` read on day one. Fields marked **[measured at baseline]** are
filled from the baseline grid.

## Edit under read

None. `pre` = `post` = the tree at `b9efb59` (the commit the rubric was minted at; the persona file is
byte-identical in the working tree), all claims `common`. The pin, not `HEAD`, is what the next edit's
`--old-ref` names. The next persona edit re-mints over the union of refs and runs the pilot form with a
positive control on the edit.

## Rubric shape (partitioned at mint, before any golden)

25 claims from 24 minted units: 22 `plan-observable`, 3 `out-of-instrument` (`dispatch-conditional`,
the cheap-read trio), 1 unit in `not_claims` (the `patterns-model-tiering` pointer fragment, the same
sha the tech-lead and devils-advocate kits list there), 2 compound units split into two claims each
(`map-before-invention` → read-first + extend-beats-mint; the remit sentence → never-edit-story-craft-
verdicts + disagreement-goes-to-user). No `latitude-conditional` claim was found: the persona body
grants no discretionary path. No claim is `untempted`: four goldens tempt all 22, 59 invited pairs.

## Read rule (D11)

- **Prune FIRST** = `agent-prune product-manager --replicates 3` over the tempting goldens (≤ 12 sessions):
  claims the bare model meets in every replicate are tagged `model_native` and leave the graded set. It
  runs before the grid because the grid freezes the rubric snapshot the report grades from.
- **Baseline grid** = `agent-grid product-manager --arms post --replicates 3 --out baseline` (4 goldens × 3
  = 12 plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in force;
  pairwise off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed (none at this mint).
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio). Any
  `latitude-conditional` claim found at a later mint is likewise out.
- **Wrapper interactions, recorded now:**
  - `recommendations-not-rulings-advise` and `making-selection-yourself-however` read against the
    wrapper's stop line (describe the stop and the onward branch for each possible ruling, then keep
    planning): `reflected` when the plan names the ruling as the founder's, whether it branches per
    possible ruling or states a default and continues under it — both forms read the same;
    `contradicted` only when the plan records the decision as made or tells engineering a selection
    is taken. Neither branching nor a stated default is making the selection.
  - `grading-own-map-writes` and `disagreement-goes-to-user`: the wrapper's no-dispatch line forbids
    spawning, not describing the hand-off; a plan that names the independent reviewer (or the stop for
    the founder) and what it would hand over embodies the standard without any dispatch.
  - `how-well-story-written` and `never-edit-story-craft-verdicts`: the read-only fence forecloses every
    write, so the foreclosing step must be stated — a plan that says it routes the wording or the grade
    back to the analyst is `reflected`; a plan that says it *would* tighten US-018 or clear US-016's
    grade is `contradicted` even though no write occurs under the fence.
  - `reading-map-before-proposing` and `map-before-invention-read-first`: the `FILES-READ:` line plus
    phase order are the evidence; the read must precede the first proposal in the plan, not merely
    appear in the list.
  - `entry-whose-honest-state`, `extents-flatter-claiming-more`, and the entry-shape claims are read on
    the entries the plan *describes* writing (paths and content), since no write lands.
- **Judge overlap, disclosed now:** `verdicts-reasons-every-no`, `silent-rejections-story-dropped`, and
  `saying-no-early-writing` are three claims over one behaviour (the written filter verdict), differing
  in the timing leg (early, before any map write) and the completeness leg (every story). The judge may
  credit all three from one evidence quote; a disagreement between them on a golden is read as judge
  noise on the timing/completeness legs, not as three findings. Likewise
  `map-before-invention-read-first` and `reading-map-before-proposing` are one standard from two
  sections, tempted together on three and four goldens: one quote may credit both, and a split between
  them is judge noise, not two findings.
- **Thin coverage, disclosed now:** `never-edit-story-craft-verdicts` and `disagreement-goes-to-user`
  are each tempted by p4 alone (one invited pair per claim per arm); a flaky read on either is one
  pair, not a trend, and the next edit's read on them rests on p4 only.

## Noise guard (I7, band per `2026-09-09-persona-band-invited-only`)

Band = the baseline arm's replicate spread **over invited pairs only** — the (golden, claim) pairs the
golden's `tempts` list names (59 at this mint; ≥ 8 per arm, so no `UNDER-SAMPLED` mark) —
**[measured at baseline]**, plus five points, capped at 20 %, computed per arm from the judged
`summary.json`. Uninvited pairs are still judged and feed coverage and the regression read; they are
disclosed beside the band, never in it. The band binds the next edit's grid: an arm above the band is
noise-dominated and no `pre`/`post` difference is read; one extra replicate per arm, once. Stopping rule
at the next edit: two consecutive instrument re-keys without a detectable control return the target to
the user.

## Judge calibration (I9)

Drawn from the baseline grid after `agent-judge`: ≥ 20 (claim, plan) pairs via the arm-blind
`agent-label-sheet` (single arm here, so blinding is nominal), labelled from FULL plan reads (never
keyword extracts — the devils-advocate process rule) by a labeller who has not seen the judge's
verdicts, with the settled rules — embodies-never-recites; declined/hypothetical conditional path =
`absent`; a Reject-section or prohibition-shaped standard (never / do not / reject / refuse) is
`reflected` when a concrete step forecloses the behaviour whether or not it was invited, `absent` when
merely restated; an `Explore` read dispatch is never a worker delegation; the wrapper interactions
above. Bars: ≥ 80 % agreement; 100 % on `contradicted` pairs; fewer than two `contradicted` labels →
extend (`--size` up, same seed) or record "not exercised". Certifies the judge against the labeller's
reading, not the rubric.

## Budget (M6)

≤ 24 plan sessions (12 baseline + ≤ 12 prune) and ≤ US$ 20 metered spend including judge calls
(`total_cost_usd` + `judge_cost_usd`); exceeding either halts and returns to the user.

## Ship bar (advisory)

The kit is ready if (a) the baseline read shows every tempted plan-observable claim either
pass^k-reflected or explained (model-native, flaky within band, or a golden-design note), (b) the
baseline's own invited-pair flaky share is ≤ 15 % so the band it fixes comes out ≤ 20 % uncapped (a band
that only binds through the cap is a kit whose goldens are too noisy to read the next edit), and (c)
calibration meets its bar. A kit failing (a) on more than three claims gets one golden re-cut before it
is declared ready.

## Fill log

- band (invited pairs; all-pairs in parentheses): **[measured at baseline]**
- prune result (claims tagged model-native, sessions, spend): **[measured at baseline]**
- coverage read (pass^k per tempted claim; flaky pairs listed by golden): **[measured at baseline]**
- calibration agreement (labeller, pairs, seed; disagreements named): **[measured at baseline]**
- contradicted bar: **[measured at baseline]**
- budget (sessions; prune + grid + judge spend): **[measured at baseline]**
- ship bar (a) / (b) / (c): **[measured at baseline]**
- re-key count: 0
- kit status: **AUTHORED — awaiting the baseline grid** (`agent-check product-manager --old-ref b9efb59`
  reports 0 failures at authoring; see the kit audit for the persona-body notes)
