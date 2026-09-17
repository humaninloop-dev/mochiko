# Preregistration — architecture plan-only eval

Committed BEFORE the first grid (record build 2; skill-harness R6 carried over; the band
form from `primitive-eval-harness-v2` D11 as folded and the `2026-09-09-persona-band-invited-only`
ruling, adapted to the command target below). The runner refuses a grid without this file.
Amending it after results exist is a recorded, deliberate act — never a quiet retro-fit.

Fourth command on the instrument, after the `implement` pilot (D5) and the `setup` grid.
Fields marked **[measured at first grid]** are filled from that grid's `summary.json` and
`report.md`.

## Command under test and arms

- **Pair:** `plugins/mochiko/commands/architecture.md` + the rules `mochiko-cli` renders for
  `architecture` from the migration log at `plugins/mochiko/migrations/` (derived view for
  reading: `.mochiko/schema-views/commands/architecture.yaml`, 48 rules).
- **`post`** — the working tree (plugin 0.109.0 staged; migration `0005-artifact-homes.yaml`
  present; 48 rules).
- **`pre`** — `--old-ref 794cea8` (the commit before the artifact-home wave landed the pair's
  last edit; migrations 0001–0004; 47 rules). Pin the SHA, never `HEAD`.
- **`nocmd`** — the bare-model control: no plugin, the golden's `control_prompt` in place of
  the slash command, one pass at the first grid (`--control`, +9 sessions), never re-run
  unless a golden changes. Rules that pass in every control replicate are the instrument's
  dead zone for this pair (prune candidates, D4 as amended I5) and are listed, never
  deleted.

## Read rule (D6 as amended — tolerance band, verify V4)

- **Substrate:** plan-only — each session plans the visit under the pinned form-only wrapper
  (`evals/commands/wrapper.md`, sha in every run's pins) with `Read,Grep,Glob` as the whole
  tool roster; user gates are described with their branches, never awaited (D9). The desk's
  seats (the producing architect, the grading tech-lead, the drift probe, the cheap Explore
  reads) cannot be spawned under the fence, so the plan describes them — the recorded
  fidelity divergence, watched not hidden.
- **Judge:** Haiku coverage checklist over the 45-rule D8 observable subset (`observable.yaml`),
  one `reflected` / `absent` / `contradicted` per rule with a quoted plan span, rule ids
  scrubbed from the plan before judging; stub axis per numbered phase; Sonnet pairwise
  position-swapped between `pre` and `post`. All judges advisory (harness D2) — the runner
  exits 0 on judged degradation.
- **Aggregation:** pass^k over k = 3 replicates per golden per arm — a rule holds on an arm
  only if every replicate reads `reflected`.
- **Comparison substrate:** the in-grid `pre` arm, never a committed file.
- **Unchanged bucket:** a rule regresses when it is `reflected` under pass^k in `pre` and not
  in `post`. Tolerance: **0 regressed rules** is the pass reading; 1–2 = "investigate — read
  the evidence quotes before any verdict"; ≥3 = "regression reading, present to the
  maintainer". Coverage-count drift without a named regressed rule is noise, not signal.
- **Removed bucket:** a removed rule still surfacing (pass^k in `post`) = "edit did not take".
- **Added bucket:** an added observable rule not reflected in any `post` replicate = DEAD-TEXT.
- **Changed bucket:** graded against the NEW text; the pre-versus-post comparison is advisory.

## Bucket shape at authoring time (recompute before reading the grid)

`uv run evals/commands/run.py partition architecture --old-ref 794cea8`, crossed with the D8
subset (2026-09-18): **unchanged 47 (44 observable) · changed-text 0 · removed 0 · added 1
(`arch.artifact-home`, observable)**. Consequences:

- The regression denominator is the **44 unchanged observable rules**, not all 45.
- Removed is empty; changed is empty. The instrument answers unchanged-bucket regression and
  one adoption question. If the pair changes again before the grid runs, recompute.

## Positive control

The edit under read adds `arch.artifact-home` (render the store's home before the first
write; the spine's section budgets from its template). Every golden writes the store, so the
control is: `arch.artifact-home` reads **not reflected** under pass^k on `pre` and
**reflected** on `post` for at least two of the three goldens. A control that fails re-keys
the instrument (wrapper, rubric `why`, or a golden's fixture), never the pair — the same
posture as the noise guard. The control is thin by construction (one rule, one behaviour);
it says the instrument can see this edit, not that it can see every edit.

## Noise band (D11 as folded; the band ruling adapted to the command target)

The command target has no `tempts` list, so the band is computed over **all (golden, rule)
pairs** in the observable subset — 3 goldens × 45 rules = 135 pairs per arm. Band = the
`pre` arm's replicate-disagreement share over those pairs **[measured at first grid]** plus
five points, capped at 20 %, computed per arm from the judged `summary.json` (the report's
per-golden "flaky rules" lines summed over goldens, divided by 135). Fewer than eight pairs
on an arm would mark the kit UNDER-SAMPLED (band at the cap; one extra replicate per arm
before any difference is read) — not reachable here at 135 pairs. An arm above the band is
noise-dominated and no `pre`/`post` difference is read from it: add one extra replicate per
arm, once, and re-judge. Stopping rule: two consecutive instrument re-keys without a
detectable positive control return the command target to the user for a keep / re-shape /
abandon ruling.

## Noise guard (F2's guard, verbatim discipline)

Same-variant replicate spread exceeding the variant gap = noise; run one more replicate pair
before any verdict. Operationally here: if the count of flaky rules (replicate disagreement
within one arm) exceeds the count of pass^k differences between arms, the grid is
noise-dominated — add one replicate per arm and re-judge before reading anything.

## Grid shape and budget

3 goldens (`g1-first-visit-legacy-prose` · `g2-stance-batch-under-pressure` ·
`g3-as-built-claim-against-code`) × 3 replicates × 2 arms (`pre` + `post`) = 18 sessions,
plus the one-time `nocmd` control (9) = **27 plan sessions**. Budget bound (M6): ≤ 27 plan
sessions plus ≤ 9 more if the band or the noise guard demands the extra replicate per arm
(36 hard cap), and ≤ US$ 45 metered spend including judge calls (`total_cost_usd` summed over
`summary.json` plus the judge calls); exceeding either halts and returns to the user. Cost
anchor: the `setup` and `implement` grids ran at roughly US$ 1 per command session; the
desk's fixtures are the same order of size.

## Ship bar (advisory instrument — informs, never gates)

The kit is ready if (a) the positive control is detected as stated above; (b) the `pre` arm's
flaky share stays ≤ 15 % of the 135 pairs so the band it fixes comes out ≤ 20 % uncapped (a
band that binds only through the cap is a kit whose goldens are too noisy to read the next
edit); (c) the three scenarios produce visibly different coverage profiles on the
condition-gated rules — `arch.dm-author-baseline` reflected on g1 only, `arch.dm-km-landing`
reflected on g1 and g3 and absent on g2, `arch.seat-drift-probe-empirical` and
`arch.tools-drift-probe-scope` strongest on g3 — identical profiles across scenarios would
mean the fixtures do not force the branch, a fixture finding not a command finding; and
(d) no `post` plan contains `mochiko-cli rules not delivered` (see disclosures). A kit
failing (c) on more than one gated rule gets one golden re-cut before it is declared ready.

## Fill log

- pre-grid binary check (`mochiko-cli migrate status --plugin-root plugins/mochiko` exit 0
  on the host; version line): _pending_
- control (`runs/<name>`, 9 `nocmd` sessions, $): _pending_ — dead-zone rules: _pending_
- grid (`runs/<name>`, 18 sessions, $; judge $; MISSING verdicts: n): _pending_
- bucket shape recomputed at grid time: _pending_
- positive control (`arch.artifact-home` pre → post, per golden): _pending_
- band: flaky _n_/135 pairs on `pre` = _x_ % → _x+5_ % (capped 20): _pending_
- unchanged-bucket regressions (of 44): _pending_
- gated-rule profile check (ship bar c): _pending_
- validity scan (no `mochiko-cli rules not delivered`, no cap-hit, no fence breach): _pending_
- budget: _n_ plan sessions · US$ _x_ metered of 45: _pending_
- re-key count: 0
- kit status: **AUTHORED — not yet run**

## Disclosures

- **Binary range.** The installed `mochiko-cli` on the authoring host (0.1.0 · grammar 1..1)
  predates migration 0005; every `post` session on such a host would halt at fire with
  `mochiko-cli rules not delivered` and the runner's load gate would NOT catch it (the plugin
  still loads; only the rendered rules are missing). The grid must not run until the host
  binary renders both logs (`migrate status` exit 0 for the working tree and for the
  archived `794cea8` tree), and every plan is scanned for that halt string before judging; a
  plan carrying it is invalid and excluded, and a whole arm carrying it is a substrate
  failure, not a command reading.
- **Fixture-echo caveat.** g2 and g3 carry a populated store, so the store's own grammar is
  in the fixtures — stance values (`decided` · `not-now` · `n-a — handled elsewhere` ·
  `open`), lifecycle statuses, `As-built:` / `Drift:` / `Upgrade trigger:` fields, a
  shelf-coverage note, an `Owned by` pointer. That is artifact identity, not rule text, but
  coverage of the grammar-binding rules (`arch.tools-store`, `arch.na-handled-elsewhere-pointer`,
  `arch.derived-index-never-hand-maintained`) reads slightly easier on g2/g3 than on g1. The
  fixtures were scanned for the pair's rule phrasing; the hits found (a knowledge-management
  line describing the root index as the store's derived view, a "fixed on sight" heading,
  two lines naming the growth door, a changelog line that gave away g3's code divergence in
  prose) were removed. One echo is kept deliberately: the derived index's own header line
  in g2 and g3 (*regenerated on every store write. Do not hand-edit.*) is the line the store
  template's skeleton writes, so a real index carries it; `arch.derived-index-never-hand-maintained`
  reads easier on those two goldens for that reason. The `nocmd` control on g2/g3 is what
  measures the residue.
- **Under-fence divergences.** The drift probe's code read (g3), the Explore dispatches, and
  the grading seat are described, not executed; `arch.model-tiering` and
  `arch.seat-drift-probe-empirical` are therefore graded on the plan's routing statements.
  `arch.no-git-mutations` and `arch.rulings-plain-text` are graded on what the plan says it
  would do at close; the fence makes the prohibited act impossible anyway, so a `reflected`
  there is weak evidence and a `contradicted` is strong.
- **Empty-arguments entry (g1).** The command's entry with no arguments surfaces health and
  then asks what the visit is for. Under D9 the plan describes that ask and continues with
  the baseline as its proposed goal; a plan that stalls at the ask is a discarded run, and
  persistent stalls across replicates are a reportable substrate result, never a retry loop.
- **Unplanted facets** are listed at the foot of `observable.yaml` (scope override, the
  greenfield elicit limb, the touched-since-last-visit half of the probe scope on g1/g2).
- **No `.mochiko/specs` spec files.** The feature entries in g2 and g3 record their specs as
  folded at landing, so the fixtures carry no `spec.md`; the desk reads the store, the map,
  the docs, and the code, none of which needs one. The approved delta in g3 is the one spec
  artifact planted because the as-built claim is graded against it.
