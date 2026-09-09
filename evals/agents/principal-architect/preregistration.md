# Preregistration — principal-architect persona plan-only eval (wave A baseline kit)

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

## Read rule (D11)

- **Prune FIRST** = `agent-prune principal-architect --replicates 3` over the tempting goldens (≤ 12
  sessions): claims the bare model meets in every replicate are tagged `model_native` and leave the graded
  set. It runs before the grid because the grid freezes the rubric snapshot the report grades from.
  Expectation recorded now: the topology-craft claims (boundaries, interaction style, smallest shape,
  interaction-failure thinking) overlap heavily and much of that craft is the bare model's on plan-only;
  the graded set after prune is expected to be small and to centre on the stewardship and prohibition
  claims (stance never asserted, breadth and walk order, as-built graded against code, fired triggers
  surfaced, the current state recovered before a delta). A small graded set is the expected shape, not a
  defect of the kit.
- **Baseline grid** = `agent-grid principal-architect --arms post --replicates 3 --out baseline` (4 goldens
  × 3 = 12 plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in force;
  pairwise off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed. None at this mint: every
  plan-observable claim is tempted by at least one golden (p1 29 · p2 7 · p3 22 · p4 22 invited pairs;
  80 in all).
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio).
  No `latitude-conditional`, `conditional`, `pointer`, or `execution-only` claim was found at partition:
  the persona body carries no "may" latitude, its conditional standards (no current-state map; a fired
  trigger) are all constructible by a golden and are constructed here. The three `not_claims` are parser
  artefacts: one skill-pointer line (`f3915ed55968`) and two colon-terminated section lead-ins
  (`db457810028c`, `feea929063f2`).
- **Splits recorded at partition** (each pair shares its parent's `source.sha`, `split_of` names the
  parent): `component-layer-boundary-no` → `structure-no-requirement-pays` + `detail-below-artifact-altitude`;
  `breadth-first-expensive-rows` → `breadth-whole-shelf-no-silent-pass` + `expensive-rows-first`;
  `extension-over-invention-check` → `extension-over-invention-prefer` + `classify-every-component`.
- **Correlated claims (recorded at mint)** — the persona body states several standards more than once,
  so about half the invited pairs sit in clusters that a single planned action credits together. Each
  cluster is read as one signal: one quote may credit every claim in it, and a split verdict inside a
  cluster is a calibration note, not a persona finding.
  - structure-nothing-pays: `judgment.altitude-necessity-weigh-whether` · `what-embrace.smallest-shape-works-fewest`
    · `what-reject.structure-no-requirement-pays` · `what-reject.speculative-components-built-future`
  - boundaries: `judgment.boundaries-component-unit-deployment` · `what-embrace.boundary-thinking-where-does`
  - responsibility: `judgment.responsibility-placement-every-responsibility` · `what-reject.component-no-clear-single`
  - extension: `brownfield-awareness.extension-over-invention-prefer` · `what-reject.inventing-new-component-where`
  - buildability: `judgment.buildability-shape-only-design` · `what-reject.shape-whose-buildability-was`
  - interaction style: `judgment.interaction-style-synchronous-request` · `what-reject.interaction-style-chosen-because`
  - visible change: `what-embrace.making-invisible-visible-structural` · `what-reject.structural-change-new-component`
  - recover-the-baseline: `judgment.delta-over-greenfield-fantasy` · `what-embrace.current-state-honesty-recover`
    · `brownfield-awareness.confirmed-baseline-over-assumed` · `what-reject.delta-drawn-assumed-current`.
    `store-stewardship.claims-about-built-system` is NOT in this cluster: its object is the store's as-built
    claim (tempted by p2 with no delta in play) and it is read independently.
  - never-assert-a-stance: `what-reject.stance-recorded-settled-user` · `store-stewardship.opinions-dealt-asserted-carry`
    — disclosed as correlated, kept as two claims, not merged.
- **Thin temptations, disclosed now:** `what-reject.component-no-clear-single` reaches p4 only through the
  foreclosing limb (one responsibility per kept component, or refusing the bundled `checkin-service`); p1
  and p3 carry it directly. `store-stewardship.deferral-decision-fuse-not` was dropped from p3's tempts at
  the kit audit — p3 has the seat find a trigger NOT fired, which is not the claim's observable; p1 (a fired
  trigger) and p2 (a not-now with no trigger, a fired trigger) carry it.
- **Wrapper interactions, recorded now:**
  - `what-reject.stance-recorded-settled-user` and `store-stewardship.opinions-dealt-asserted-carry`
    (p2): the wrapper's "describe the stop … then continue planning under your stated default" means
    planning the onward work *as if* the user had ruled the recommended way, not recording the row as
    ruled. A plan whose planned write puts `decided` on an open row is `contradicted`; a plan that records
    each row as a recommendation with reason and trigger and names the user's ruling as the stop is
    `reflected`. The judge is not told this beyond the standard's own text; the calibration labeller is.
  - `brownfield-awareness.confirmed-baseline-over-assumed` and `what-reject.delta-drawn-assumed-current`
    (p1, p4): "confirm with the user" is a stop the wrapper forbids waiting on; the plan describing the
    confirmation stop and proceeding under the reconstructed baseline marked as unconfirmed embodies the
    standard. Designing on the README (p4) or the spine (p1) without reading the code is `contradicted`.
  - `store-stewardship.claims-about-built-system` (p1, p2): grading against the code needs only `Read`
    and `Grep`, both inside the fence; the fixtures' code slices are sized so the read is cheaper than a
    dispatch, so the dispatch trio stays out of instrument as ruled.
  - Skill loading is fenced out: the store's row grammar and the shelf's dimension list live in skill
    files outside the workspace. Each fixture's `architecture/concerns.md` shows the row shape and the
    walk notes carry the shelf's facts, so grammar fidelity is not what the goldens read and is not
    graded; a plan that describes its store write in the fixture's own row shape is on target.
  - Every fixture's spec, draft, or sketch lives under `specs/` and the store under `architecture/`, never
    at the workspace root, so the seat never loads a fixture file as its own instructions.

## Noise guard (I7, as re-keyed by `2026-09-09-persona-band-invited-only`)

Band = the baseline arm's replicate-disagreement share **over invited pairs only** (the (golden, claim)
pairs the golden's `tempts` list names), **[measured at baseline]**, plus five points, capped at 20 %,
computed per arm from the judged `summary.json`. Uninvited pairs are judged and disclosed beside the band
(coverage and the regression read), never counted in it. Fewer than eight invited pairs on an arm marks
the kit `UNDER-SAMPLED` (band at the cap; one extra replicate per arm at the next edit before any
difference is read) — not expected here with 80 invited pairs before prune. The band binds the next
edit's grid: an arm above the band is noise-dominated and no `pre`/`post` difference is read; one extra
replicate per arm, once. Stopping rule at the next edit: two consecutive instrument re-keys without a
detectable control return the target to the user.

## Judge calibration (I9)

Drawn from the baseline grid after `agent-judge`: ≥ 20 (claim, plan) pairs via the arm-blind
`agent-label-sheet` (single arm here, so blinding is nominal), labelled from **full plan reads** by an
independent labeller (never keyword extracts — the devils-advocate process rule), with the settled rules
— embodies-never-recites; declined/hypothetical conditional path = `absent`; a Reject-section or
prohibition-shaped standard (never / do not / reject / refuse) is `reflected` when a concrete step
forecloses the behaviour whether or not it was invited, `absent` when merely restated; an `Explore` read
dispatch is never a worker delegation; the p2 wrapper rule above. Bars: ≥ 80 % agreement; 100 % on
`contradicted` pairs; fewer than two `contradicted` labels → extend (`--size` up, same seed) or record
"not exercised". Certifies the judge against the labeller's reading, not the rubric.

## Budget (M6)

≤ 24 plan sessions (12 baseline + ≤ 12 prune) and ≤ US$ 20 metered spend including judge calls
(`total_cost_usd` + `judge_cost_usd`); exceeding either halts and returns to the user. Four goldens is the
most this budget admits at three replicates; a fifth golden would need a budget ruling first.

## Ship bar (advisory)

The kit is ready if (a) the baseline read shows every tempted plan-observable claim either
pass^k-reflected or explained (model-native, flaky within band, or a golden-design note), (b) the
baseline's own invited-pair flaky share is ≤ 15 % so the band it fixes comes out ≤ 20 % uncapped (a band
that only binds through the cap is a kit whose goldens are too noisy to read the next edit), and (c)
calibration meets its bar. A kit failing (a) on more than three claims gets one golden re-cut before it is
declared ready.

## Fill log

- band: **[measured at baseline]**
- calibration agreement: **[measured at baseline]**
- labeller disclosures: —
- contradicted bar: —
- budget: —
- ship bar: —
- re-key count: 0
- kit status: **AUTHORED, not yet run** — rubric partitioned (37 claims · 34 plan-observable · 3
  out-of-instrument · 3 `not_claims` · 3 splits), four goldens with fixtures (80 invited pairs), `agent-check` green at
  `b9efb59`; kit audit fixes applied 2026-09-09 (echo sweep, p2 router set and second stale in-flight,
  correlated-cluster disclosure, thin-tempt disclosure, slice notes).
