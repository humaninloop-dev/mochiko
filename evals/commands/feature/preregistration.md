# Preregistration — feature plan-only eval

Committed BEFORE the first grid (record build 2; skill-harness R6 carried over). The runner
refuses a grid without this file. Amending it after results exist is a recorded, deliberate act —
never a quiet retro-fit.

Third command on the instrument, after the `implement` pilot (D5) and `setup`. This is a
**baseline kit**: no edit to the `/mochiko:feature` pair is under read at authoring. At authoring
the pre arm was pinned to the branch base, where pre and post are text-identical and the kit
therefore carried **no positive control** — it could not tell "no regression because nothing
changed" from "no regression because the judge or render pipeline is silently broken". The kit
audit of 2026-09-19 ruled that kit-failing, and the pin moved: `794cea8` carries a genuine
one-rule delta, so the baseline grid now runs against a known difference. The kit fixes the
rubric, the fixtures, and the noise band so that the pair's next edit gets a `pre`/`post` read on
day one. Fields marked **[measured at baseline]** are filled
from the baseline grid.

## Command under test and arms

- **Pair:** `plugins/mochiko/commands/feature.md` + its rules as rendered by `mochiko-cli` from
  the migration log (`plugins/mochiko/migrations/0001–0005`); the derived view the runner reads
  for the rubric is `.mochiko/schema-views/commands/feature.yaml` — 50 rules, six sections,
  one `kind: fail` rule.
- **`post`** — the working tree's pair.
- **`pre`** — the pair at `--old-ref`. At the baseline grid the ref is **`794cea8`** (plugin
  v0.108.0), which partitions as **unchanged 49 · changed 0 · removed 0 · added 1 —
  `feat.artifact-home`**, the same positive control the `architecture` and `brainstorm` kits
  use. The next pair edit names its own pre-edit SHA — `HEAD` is never the pre arm once the
  edit lands.
- **Positive control (pre-registered, D11):** `feat.artifact-home` is absent from the `pre`
  pair and present in the `post` pair. The grid is only readable if it **LANDS** — the rule
  reads `absent` under `pre` and `reflected` pass^k under `post`, localized to that id with
  evidence quotes. A control that does not land is an instrument failure, not a command
  finding: the read stops and the runner, the rubric or a golden is re-keyed, under the
  **What this control does and does not prove (audit re-grade 2026-09-19, traced in
  `cmd_report`):** an added rule exercises the *adoption* path — `passk(post, rid)` alone.
  The regression path the tolerance band and the ship bars actually gate on is a different
  branch: `rid` in the unchanged bucket, `passk(pre, rid)` true and `passk(post, rid)` false.
  No real `--old-ref` can control that branch, here or in the sibling kits, because it fires
  only when the command has genuinely regressed. Detecting new text appearing is the easy
  direction; detecting a standing obligation quietly vanishing is the failure-prone one, and
  it stays unproven until a synthetic mutation probe runs. **Pre-registered:** before this
  kit's first *regression* read — not before its baseline grid — run one probe pair that
  weakens a single rule's text in the post tree and confirm the report names exactly that id
  in `unchanged-bucket regressions`. A probe that fails to localize is an instrument failure
  under the stopping rule.

  stopping rule below. Two other refs were checked and rejected: `49acf05`, `0a03626` and
  `5d8fc69` carry no delta at all, and `9cdac97` carries two (`feat.artifact-home` added plus
  `feat.delta-cards` changed) at the cost of a three-version gap whose unrelated changes would
  muddy attribution.
- **`nocmd`** — the bare-model control (`--control`, +9 sessions), run once at baseline: each
  golden's `control_prompt` asks for the same plan with no command invoked. The three control
  prompts were rewritten on 2026-09-19 (kit audit, second high finding): each had asked the bare
  model to keep "the product's capability map truthful", rubric vocabulary that three rules turn
  on and that the real command's `args` never carry, which would have steered the dead-zone read
  it exists to measure. Rules that read
  `reflected` pass^k under `nocmd` are the instrument's dead zone (prune candidates, D4 I5),
  disclosed in the fill log and excluded from the next edit's regression read — never deleted
  from the rubric.

## Read rule (D6 as amended — tolerance band, verify V4; v2 D11 band)

- **Substrate:** plan-only sessions under the pinned form-only wrapper (`evals/commands/wrapper.md`,
  sha in every run's pins; D11 — form, never content), `--allowedTools Read,Grep,Glob`,
  `--max-turns 40`, session model Sonnet. The command plans; user gates are described with the
  onward branch per ruling, never awaited (D9).
- **Judge:** Haiku coverage checklist over the 47-rule D8 observable subset, one binary per rule
  with a quoted plan line; rule ids are scrubbed from every plan before judging, so the read is
  embodiment, never lexical match (a restated obligation is `absent`); stub-detection axis per
  phase; Sonnet pairwise position-swapped, advisory. All judges advisory (harness D2) — the runner
  exits 0 on judged degradation.
- **pass^k:** a rule holds in an arm only if every replicate reflects it (`k = 3`).
- **Comparison substrate:** the in-grid `pre` arm (never a committed baseline file, which is the
  pinned historical record).
- **Unchanged bucket:** a rule regresses when it is `reflected` under pass^k in `pre` and not in
  `post`. Tolerance: **0 regressed rules** is the pass reading; 1–2 = "investigate — read the
  evidence quotes before any verdict"; ≥ 3 = "regression reading, present to the maintainer".
  Coverage-count drift without a named regressed rule is noise, not signal.
- **Removed bucket:** any removed rule still surfacing (pass^k in `post`) = "edit did not take".
- **Added bucket:** an added observable rule not reflected in any `post` replicate = DEAD-TEXT.
- **Changed bucket:** graded against the NEW text; the `pre`-versus-`post` comparison is advisory
  only — stability cannot be demanded of an obligation that itself moved.
- Recompute the buckets before reading any grid:
  `uv run evals/commands/run.py partition feature --old-ref <pre-sha>` crossed with the D8
  observable subset; the regression denominator is the unchanged *observable* rules, never all 47.

## Noise band (v2 D11 as folded — I7; the D11 band the report prints)

The command report prints, per golden, the count and ids of **flaky rules** (replicate
disagreement within an arm, observable subset, all pairs). The band is computed from those lines:

- **Band per arm** = the arm's flaky share over all (golden, rule) pairs — flaky pairs ÷ (3 goldens
  × 47 observable rules = 141 pairs) — **plus five points, capped at 20 %**. Measured at baseline
  from the `post` arm's own spread **[measured at baseline]**, never inherited from another target
  (setup's or implement's band does not transfer).
- **UNDER-SAMPLED:** an arm with fewer than 8 judged pairs carries the mark and its band is the cap
  (20 %); at 141 pairs per arm the mark cannot apply here unless judge verdicts go MISSING —
  re-run `judge` before reading, never read around a MISSING pair.
- **Guard at the next edit:** an arm whose flaky share sits above the band is noise-dominated and
  no `pre`/`post` difference is read from it; run **one extra replicate per arm, once**, re-judge,
  and re-read. A second trip is an instrument finding — re-key the runner, the rubric, or a
  golden; never a command finding.
- **Stopping rule:** two consecutive instrument re-keys without a detectable control (a known edit
  the grid cannot localize to named ids) return the command target to the user for a keep /
  re-shape / abandon ruling.
- **Invited-pair companion read (disclosed, not binding):** the command target has no `tempts`
  list and the report counts every pair, so the binding band is all-pairs. The planted set per
  golden below (121 of 141 pairs) lets a reader recount the band on invited pairs by hand from the
  per-golden flaky ids, in the spirit of `2026-09-09-persona-band-invited-only`; a rule read on a
  golden that does not plant its door is a control read (`absent` is honest there) and is not a
  regression when it stays `absent` across arms.

## Noise guard (F2's guard, verbatim discipline)

Same-variant replicate spread exceeding the variant gap = noise; run one more replicate pair
before any verdict. Operationally: if the count of flaky rules (replicate disagreement within one
arm) exceeds the count of pass^k differences between arms, the grid is noise-dominated — add one
replicate per arm and re-judge before reading anything. This is the same trip the band names;
the band gives it a pre-registered threshold.

## Grid shape

3 goldens (s1-bug-delta-lane · s2-growth-mint-lane · s3-groom-at-cap) × 3 replicates × 2 arms
(`pre` + `post`) = 18 sessions, plus the one-time `nocmd` control (3 × 3 = 9) at baseline. The
three scenarios plant every routing door and both values of the `km_file` condition, so each
door-gated rule is graded in the scenario that plants it:

| golden | plants | KM | seats |
|---|---|---|---|
| s1-bug-delta-lane | a bug on a delivered single owner (delta lane, reproduction-failing test) · an improvement with a known contract touch (baseline-delta.md before/after) · a report on in-flight territory (files to the run) · a dangling `live` row on a closed spec · a stale unrefined stub · no epics | present | multi (producer + reviewer) |
| s2-growth-mint-lane | an extend on a delivered capability (growth-door row) · a new-kind ask (route to specify, stub parked) · a no-single-owner report (product lane) · a live lane run (single-flight) · an "all in one go" epic question | absent (degrade path) | multi |
| s3-groom-at-cap | ten capabilities (cap-trip) · a lookalike pair (merge) · a dead entry (retire) · a wholesale re-derivation ask with an explicit "host it here" (ceiling + hosting) · an open epic ready to dispatch · a fired architecture trigger · an unfolded delta · an orphan store element | present | multi |

**Planted (invited) rules per golden** — the honest coverage denominator per scenario; a rule
outside a golden's set reads `absent` there by design:

- **s1 (37):** dm-health-first · dm-converge-goal · dm-map-integrity · dm-route-honestly ·
  dm-complete-card · dm-epic-stewardship · dm-km-landing · dm-close-verdict · pm-seat ·
  architect-dormancy · dispatched-runs-own-delivery · user-reserved ·
  tools-referenced-never-restated · map-files · feature-map-binding · capability-write-test ·
  stable-ground-triage · delta-cards · product-surface · dispatch-scope-split · km-relation ·
  proactive-report · reference-never-restate · author-grader · advisory-front-door ·
  model-tiering · no-git-mutations · rulings-plain-text · artifact-home ·
  capability-writes-sacred · lane-never-widens · no-delivery-harness · no-self-graded-writes ·
  no-silent-map-mutations · sound-loop-floor · transport-floor · stub-parking
- **s2 (41):** s1's set minus dm-km-landing and lane-never-widens, plus map-minimalism-binding ·
  epic-binding · dispatch-specify · single-flight-lane · growth-door · growth-routes-to-specify
- **s3 (43):** dm-health-first · dm-converge-goal · dm-map-integrity · dm-route-honestly ·
  dm-complete-card · dm-epic-stewardship · dm-km-landing · dm-close-verdict · pm-seat ·
  architect-dormancy · dispatched-runs-own-delivery · user-reserved ·
  tools-referenced-never-restated · map-files · map-minimalism-binding · feature-map-binding ·
  epic-binding · epic-dispatch · capability-write-test · product-surface · architecture-intake ·
  dispatch-scope-split · dispatch-specify · km-relation · proactive-report ·
  reference-never-restate · author-grader · advisory-front-door · model-tiering ·
  no-git-mutations · rulings-plain-text · artifact-home · capability-writes-sacred ·
  grooming-door-ceiling · out-of-remit-hosting · growth-door · growth-routes-to-specify ·
  no-delivery-harness · no-self-graded-writes · no-silent-map-mutations · sound-loop-floor ·
  transport-floor · stub-parking

Every one of the 47 observable rules is planted on at least one golden. Single-golden rules
(thin coverage, one invited pair per arm each): epic-dispatch · architecture-intake ·
grooming-door-ceiling · out-of-remit-hosting (s3) · single-flight-lane (s2) ·
lane-never-widens (s1). A flaky read on any of them is one pair, not a trend.

## Wrapper interactions, recorded now

- **User gates** (the visit-goal confirmation, card confirmation, the merge and retire rulings,
  the selection): the wrapper's stop line asks for the gate and the onward branch per ruling.
  `feat.dm-converge-goal`, `feat.user-reserved`, `feat.rulings-plain-text`, and
  `feat.capability-writes-sacred` read `reflected` when the plan names the ruling as the user's —
  whether it branches per possible ruling or states a default and continues under it, both forms
  read the same; `contradicted` only when the plan records a merge, retire, status change, or
  selection as made at the desk.
- **The read-only fence forecloses every write.** `feat.artifact-home`, `feat.delta-cards`,
  `feat.product-surface`, `feat.dm-map-integrity`, and `feat.no-silent-map-mutations` are read on
  the writes the plan *describes* — paths, content, and sequence (home rendered before the first
  write; the delta beside the baseline; the dangling row folded) — since no write lands.
- **The no-dispatch line forbids spawning, not describing.** `feat.sound-loop-floor`,
  `feat.author-grader`, `feat.no-self-graded-writes`, `feat.transport-floor`, and
  `feat.model-tiering` are read on the seat wiring the plan describes (a producing seat on an
  approved plan, a named non-author reviewer, an Explore read dispatch); a described dispatch is
  `reflected`, an omitted one `absent`.
- **The desk's rules arrive by slash-command expansion, not by a tool call.** The command's
  `!mochiko-cli rules` lines are preprocessed by the CLI when `/mochiko:feature` expands, outside
  the model's tool roster, so the `Read,Grep,Glob` fence does not block them (the implement and
  setup grids ran the same way). A plan that halts with `mochiko-cli rules not delivered` is a
  precondition failure (see below), never a coverage reading.
- **Judge overlap, disclosed now:** `feat.dm-health-first` and `feat.proactive-report` are one
  behaviour from two sections (health before the ask); one quote may credit both, and a split
  between them is judge noise, not two findings. Likewise `feat.no-self-graded-writes`,
  `feat.author-grader`, and `feat.sound-loop-floor` share the review-leg evidence, differing in
  the plan-first leg and the user-gate leg; and `feat.dispatched-runs-own-delivery`,
  `feat.reference-never-restate`, and `feat.no-delivery-harness` share the dispatch-only evidence.

## Run precondition (binary range)

Every plan session renders the pair's rules from the provisioned plugin's migration log with the
`mochiko-cli` on PATH. The log at `794cea8` and at the working tree runs to migration 0005; both
render clean under the binary rebuilt on 2026-09-19, verified before this kit's first grid. A
binary whose grammar range
predates it halts every session at first use (`mochiko-cli rules not delivered`) and the grid
records invalid runs, not plans. Before the baseline grid, run one `plan-run feature
s1-bug-delta-lane --arm post` and confirm the init event's plugin pin and a plan with numbered
phases; a halt there is a precondition failure to fix (install a binary at or above the log's
range), not a result.

## Fixture-echo caveat

The fixtures are scanned for echoes of the pair's rule language (the routing vocabulary — doors,
lanes, tests, floors, seats, landing, health, verdict — and the ways-of-working phrases); the
scan is clean except for artifact identity the desk reads by design: the pinned
`.mochiko/memory/knowledge-management.md` in s1 and s3 carries its own landing ritual and the
orphan rule's "cleaned at the next desk visit" line (it is the file `feat.dm-km-landing` and
`feat.km-relation` point at, so its presence is the planted condition, not a hint); the s3 store's
concern row says the fired trigger is "flagged for the desk" (the architecture store's own
hand-off grammar); the s3 epic manifest names its mint "at the desk" (the epic template's
provenance field). s1 and s3 coverage on `feat.dm-km-landing`, `feat.km-relation`, and
`feat.architecture-intake` should therefore be read as slightly easier than a bare workspace
would make them; the `nocmd` control arm on those goldens is what measures the residue.

## Ship bar (advisory instrument — informs, never gates)

The kit is ready if (a) the baseline read shows every planted plan-observable rule either
pass^k-reflected on a golden that plants it or explained (model-native under `nocmd`, flaky within
band, or a golden-design note); (b) its flaky-rule set stays under 20 % of the observable subset
(fewer than 10 of 47) so the band it fixes binds uncapped — a band that only binds through the cap
is a kit whose goldens are too noisy to read the next edit; and (c) the three scenarios produce
visibly different coverage profiles on the door-gated rules (delta-cards and lane-never-widens on
s1 · growth-door, growth-routes-to-specify, and single-flight-lane on s2 · grooming-door-ceiling,
out-of-remit-hosting, architecture-intake, and epic-dispatch on s3). Identical profiles across the
three would mean the fixtures do not force the doors — a fixture finding, not a command finding.
Failing (b) triggers the record's noise falsifier (open question 3): the substrate bet weakens and
the session premise is revisited. A kit failing (a) on more than three rules gets one golden re-cut
before it is declared ready.

The grid is useful at the next edit if it localizes at least one true behavioural difference
between the pre-edit and post-edit pairs to named rule ids.

## Budget (M6)

≤ 27 plan sessions at baseline (18 grid + 9 control, once) and ≤ US$ 45 metered spend including
judge calls (`total_cost_usd` + judge spend); an extra replicate pair under the guard adds 6
sessions inside the same bound. Exceeding either halts and returns to the user.

## Fill log

- band (all pairs per arm; invited-pair recount in parentheses): **[measured at baseline]**
- control dead zone (rules pass^k under `nocmd`, per golden): **[measured at baseline]**
- coverage read (pass^k per planted rule per golden; flaky ids per golden): **[measured at baseline]**
- cap-hit runs · fence breaches · unresolvable names: **[measured at baseline]**
- stub phases flagged: **[measured at baseline]**
- budget (sessions; grid + control + judge spend): **[measured at baseline]**
- ship bar (a) / (b) / (c): **[measured at baseline]**
- re-key count: 0
- kit status: **AUTHORED — awaiting the baseline grid**
