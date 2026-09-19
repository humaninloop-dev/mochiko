# Preregistration — specify plan-only eval

Committed BEFORE the first grid (record build 2; skill-harness R6 carried over). The
runner refuses a grid without this file. Amending it after results exist is a recorded,
deliberate act — never a quiet retro-fit.

Third command on the instrument, after the `implement` pilot (D5) and `setup`. This kit
fills the `specify` slot with no edit under read yet: the grid it pre-registers is whatever
the next `/mochiko:specify` pair edit is, and the first grid's positive control is the one
already on disk (below).

## Command under test and arms

- **Command:** `/mochiko:specify` — the `.md` at `plugins/mochiko/commands/specify.md` plus
  its rules, rendered at fire by `mochiko-cli` from the migration log; the repo-side
  projection is `.mochiko/schema-views/commands/specify.yaml` (52 rules in six sections at
  kit authoring, HEAD `49acf05`).
- **Arms:** `pre` (the pair as committed at `--old-ref`, git-archived) · `post` (the working
  tree) · `nocmd` (the bare model on the golden's `control_prompt`, no plugin loaded).
- **Session model:** Sonnet (R7). **Wrapper:** `evals/commands/wrapper.md`, form-only (D11),
  sha pinned in every run. **Fence:** `--allowedTools Read,Grep,Glob` (D7 allow-list) — the
  session plans, never executes; user gates are described with both branches (D9).

## Read rule (D6 as amended — tolerance band, verify V4)

- Comparison substrate: the **in-grid pre-edit arm** (never a committed baseline file,
  which is only a pinned historical record).
- **Judge:** Haiku rule-coverage checklist over the D8 observable subset (`observable.yaml`:
  **40 observable · 12 out-of-instrument · 52 total** (39 · 13 at authoring; `spec.filter-disagreement-escalates` moved to observable on 2026-09-19 by kit audit finding 2)), one binary per rule with a quoted
  evidence span, graded on embodiment after every known rule id is scrubbed from the plan;
  a restated principle is not evidence. Plus the stub axis per numbered phase. Sonnet
  pairwise, position-swapped, advisory.
- **pass^k:** a rule holds in an arm only if it reads `reflected` in every replicate of that
  arm (k = 3).
- **Unchanged bucket:** a rule regresses when it holds under pass^k in the pre arm and not
  in the post arm. Tolerance: **0 regressed rules** is the pass reading; 1–2 regressed
  rules = "investigate — read the evidence quotes before any verdict"; ≥3 = "regression
  reading, present to the maintainer". Coverage-count drift without a named regressed rule
  is noise, not signal.
- **Removed bucket:** any removed rule still surfacing (pass^k in post) = "edit did not
  take" finding.
- **Added bucket:** an added observable rule not reflected in any post replicate =
  DEAD-TEXT finding.
- **Changed bucket:** graded against the NEW text; pre-versus-post comparison advisory
  only — stability cannot be demanded of an obligation that itself moved.
- **Condition-gated rules** are graded where their scenario plants the condition (the
  gating table in `observable.yaml`'s header): a gated rule reading `absent` in a scenario
  that plants the other value is not a regression and is not counted.

## Noise band (v2 D11 as re-keyed 2026-09-09) and noise guard (F2)

- **Band input, per arm:** the share of (golden, observable rule) pairs with replicate
  disagreement over all graded pairs. The command target declares no `tempts`, so every
  observable rule is judged on every golden and the all-pairs share *is* the band input
  (the invited-only re-key has no separate denominator here; where a later edit adds a
  `tempts` list, the invited-only share takes over and the all-pairs figure is disclosed
  beside it). **Band = share + 5 points, capped at 20 %.** An arm with fewer than 8 graded
  pairs carries an `UNDER-SAMPLED` mark and its band is the cap; at 3 goldens × 40 rules =
  120 pairs per arm this kit cannot be under-sampled unless the partition shrinks below
  3 observable rules.
- **Guard:** the **pre arm's own spread must sit inside its band** before any pre/post diff
  is read. An arm whose flaky share exceeds its band is noise-dominated: add **one extra
  replicate per arm, once**, re-judge, and re-read; a guard that trips again halts the read
  and books an **instrument finding**, never a command finding.
- **F2 restated operationally:** if the count of flaky rules (replicate disagreement within
  one arm) exceeds the count of pass^k differences between arms, the grid is
  noise-dominated — same remedy as above, before anything is read.
- **Stopping rule:** two consecutive instrument re-keys (runner, rubric, or goldens) without
  a detectable positive control return the kit to the user for a keep / re-shape / abandon
  ruling. A failed control re-keys the instrument and is never read as a command regression.

## Positive control (first grid)

Recomputed at authoring: `uv run evals/commands/run.py partition specify --old-ref 32c1ed5`
(the v0.107.0 wave-6 end state, the last ref before the hook-enforced artifact-schema
migration) gives **unchanged 51 · changed 0 · removed 0 · added 1 — `spec.artifact-home`**.
The first grid pins `--old-ref 32c1ed5` and its detectable control is that rule: it must
read `absent` under pass^k in `pre` (the pre pair carries no home or template-budget step)
and `reflected` in at least one `post` replicate (LANDED, not DEAD-TEXT). An instrument
that cannot see that edit is re-keyed before the command is graded by it. Every later grid
recomputes the buckets against its own `--old-ref` — they move with every edit — and names
its control from the added or changed bucket in a fill-log row before the grid runs.

## Validity prerequisites (blocking, before any session)

- `uv run evals/commands/run.py check-rubric specify` prints `rubric OK`;
  `check-fixtures specify` prints `OK`.
- `mochiko-cli` on PATH renders `mochiko-cli rules specify --section preamble` against
  **both** the working tree's `plugins/mochiko` and the `--old-ref` archive. The runner's
  load gate checks only that the plugin appears in the init event; it does not read the
  plan for the halt string. A plan carrying `mochiko-cli rules not delivered` is an
  **invalid run** — discarded, excluded from every read, listed in the fill log — and a
  whole arm invalid is an instrument finding (binary grammar range short of the ref), not a
  command finding. At kit authoring the installed binary predated HEAD's migration 0005 and
  the post arm could not run; **discharged 2026-09-19** — the binary was rebuilt from
  `crates/mochiko-cli` and now renders both the working tree and the `32c1ed5` archive
  cleanly, verified before this kit's first grid. Re-check the render rather than the version
  string if an arm ever comes back wholly invalid: the version did not change across that
  rebuild.
- No run stalls at a gate or emits a bare confirmation request (D9); such a run is discarded
  and, if persistent across replicates, reported as a substrate result.

## Grid shape and budget

- **Grid:** 3 goldens (s1-online-payment · s2-webhook-retry · s3-bare-greenfield) × 3
  replicates × 2 arms (pre + post) = **18 sessions**; judges: Haiku coverage over 40 rules in
  three chunks of ≤ 15 per plan + the stub axis; Sonnet pairwise on each (golden, replicate)
  pre/post pair.
- **Control arm:** `--control` adds 9 `nocmd` sessions. Run it **once at the first grid**
  (the dead-zone read — which observable rules the bare model already satisfies from the
  fixture alone; those are disclosed as model-native and excluded from the regression read,
  never deleted from the rubric) and thereafter only on a re-key.
- **Guard replicate:** one extra replicate per arm = 6 sessions, at most once per grid.
- **Budget bound per grid:** ≤ 33 sessions (18 + 9 + 6) and ≤ **$35** all-in. The sibling
  grids ran $0.56–0.79 per plan session (setup 15–20 turns, implement 22–26 turns, no
  cap-hits at `--max-turns 40`); judge spend on those grids stayed under $5. A grid that
  projects past the bound stops and reports before the next session.

## Ship bar (advisory instrument — informs, never gates)

The grid is useful if it (a) localizes at least one true behavioral difference between the
pre and post pairs to named rule ids — on the first grid, `spec.artifact-home` LANDED — and
(b) keeps its flaky share ≤ 15 % of graded pairs per arm, the figure at which the band binds
uncapped; a share between 15 % and 20 % reads at the cap and is disclosed; above 20 % the
record's noise falsifier (open question 3) fires and the substrate bet is revisited.

A second, substrate-level question this grid answers for free: whether the condition
branches are legible to the instrument — whether s1, s2, and s3 produce visibly different
coverage profiles on the gated rules (`spec.stress-test-prototype-walk`,
`spec.whole-feature-prototype`, `spec.lockstep-prototyping` on s2; `spec.map-obligated-read`,
`spec.unrefined-stubs`, `spec.governance-briefs`, `spec.km-landing` on s3;
`spec.missing-map-surfaced`, `spec.governance-region-absent`, `spec.frame-greenfield-inputs`
on s1 and s2). Identical profiles across the three would mean the fixtures do not force the
branch — a fixture finding, not a command finding.

## Fill log

One row per grid, appended before the grid runs (control named) and completed after the
report is read. Never edit an earlier row; correct with a new one.

| Date | Run name | `--old-ref` | Control (added/changed id expected LANDED) | Arms × k | Sessions · USD | Invalid runs | Pre cov · post cov (pass^k /40) | Unchanged regressions | Added adoption · removed ghosts | Flaky share → band (pre · post) | Guard tripped · extra replicate | Verdict | Re-key count |
|------|----------|-------------|--------------------------------------------|----------|----------------|--------------|----------------------------------|-----------------------|--------------------------------|--------------------------------|-------------------------------|---------|--------------|
| _none yet_ | | | | | | | | | | | | | |

## Disclosures — what a plan cannot show

- **User rulings are described, never taken.** `ux_bearing` resolves at intent by the user;
  the fixtures lean each scenario one way (s1 and s3 name a screen, s2 names none) and the
  gated rules are graded on the leaned branch. The selection, the acceptance, and every
  clarification are gates with described branches, not outcomes.
- **The stress-test verdict is described inline** (D9's recorded divergence): under the
  fence the run cannot spawn the grading seat, so the plan states what that seat would grade
  and how a blocking gap would route, not the verdict itself.
- **Nothing is rendered or served.** Bash is denied, so `mochiko-cli home`, `mochiko-cli
  template spec`, the bun-served prototype, and the served-prototype walk are planned
  steps, not executed ones; the judge reads whether the plan places them, not whether they
  would succeed.
- **No write happens**, so the staged delta, the acceptance-time map batch, the specs-index
  row, and the knowledge-management landing are graded as planned writes with named paths.
- **Story quality, prototype fidelity, and the register** are out of reach: the plan names
  that stories are authored inside the frame and screens land in lockstep; it does not
  contain a story or a screen.
- **Fixture echo (s1 and s2).** A prior accepted spec is honestly the previous run's own
  output, so s1 and s2 carry the spec template's section vocabulary — an Intent block with a
  `Capability frame` line, a `Feature Selection` section with derived features, filter
  rejections and a Selection, a Screens & Flows manifest with a prototype pointer — plus the
  map's own status marks (`proposed (unrefined)`, `reconstructed-from-code`, `pending` rows)
  and the governance region's amend line naming `/mochiko:setup`. **Restatement scan at
  authoring:** every fixture line was checked against a 60-phrase list drawn from the pair's
  rule texts and against every 5-gram of those texts; zero 5-gram lifts. The phrase hits
  that remain are, by class: the feature-entry template's own stub status line (`minted by
  /mochiko:feature · marked unrefined`) and index mark (`proposed (unrefined)`); the
  governance region's markers and its amend and operating-docs lines as `/mochiko:setup`
  writes them; the ledger's amendment-policy route line; the pinned knowledge-management
  copy's own headings (`Landing ritual`, `fix-on-sight`) and groom pointer; the prior specs'
  `Capability frame:` Intent line; and the words `drift` and `one pass` in unrelated senses
  (rounding drift, a probe pass). One echo was removed rather than kept: the old-form spec's
  status line and index row had said "slice-form … frozen", the rule's own words, and now
  describe the artifact's shape instead. What remains is artifact identity, never an
  instruction to the run. s1 and s2 coverage on the artifact-binding rules should be read
  as slightly easier than s3's, and the `nocmd` control on s1 measures the residue.
- **Unplanted branches** are listed at the foot of `observable.yaml`; `spec.epic-mint-desk-only`
  is the weakest observable rule (its activation rides a latitude) and reads on evidence
  quotes, never on the binary alone.
