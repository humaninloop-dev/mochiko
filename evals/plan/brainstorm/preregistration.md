# Preregistration — brainstorm plan-only eval

Committed BEFORE any grid (`command-plan-only-eval` D1–D11 as amended, accepted 2026-08-27;
the band and stopping-rule form of `primitive-eval-harness-v2` D11-preregistration-noise-guard with folds I7 · M5 · M6;
the probe findings in `../brainstorm-probe/probe-report.md`). The runner refuses a grid
without this file. Amending it after results exist is a recorded, deliberate act — never a
quiet retro-fit. Kit authored by an independent seat (author ≠ grader: this seat authored
neither the command pair nor its last edit; a separate auditor grades the kit). Fields
marked **[measured at baseline]** are filled in the fill log from the first grid.

Third command on the instrument, after the `implement` pilot (D5) and `setup`. The probe of
2026-08-27 already ran this command once (one session, one hand-made fixture) and settled
the invocation, the load gate, the fence, the absent-user contract, the rule-ID scrub, and
the turn cap; those findings are reused here, not repeated.

## Command under test and arms

`plugins/mochiko/commands/brainstorm.md` + its rules, rendered at fire by `mochiko-cli` from
`plugins/mochiko/migrations/` (55 rules since migration 0044, 30 from 0005 to 0043; the repo-side derived view
`.mochiko/schema-views/commands/brainstorm.yaml` is the projection the runner reads for ids
and texts). Three arms:

- **`post`** — the working tree's plugin, provisioned beside the fixture.
- **`pre`** — the plugin tree git-archived at `--old-ref`; the view at that ref supplies the
  pre rubric for the D6 partition.
- **`nocmd`** — no command invocation; the golden's `control_prompt` on the same fixture and
  wrapper, the bare-model dead-zone read (D4 as amended, I5). Every rule that passes under
  `nocmd` on a golden is a dead-zone candidate for that golden, disclosed, never pruned by
  this kit.

## Edit under read

The pair's next edit landed before any grid ran, so it is the first grid's edit under read:
the 2026-10-06 brainstorm target state (`brainstorm-target-state` D1–D15, D17–D19, D22) —
migration 0044, which adds 25 rules and rewords 4, with the command `.md` reworded beside it
and `mochiko:analysis-iterative`'s two D21 repairs in the same plugin tree. `--old-ref
2e4c57c` (the branch base, `main` before the build; the view there carries the 30 rules of
0005–0043) gives, per `uv run evals/run.py command partition brainstorm --old-ref 2e4c57c`
run at the re-key: **unchanged 26 (19 observable) · changed 4 (3 observable, graded against
the new text) · removed 0 · added 25 (23 observable)**. Recompute before reading any grid — the
buckets move with every edit — and name the SHA explicitly; `HEAD` is never the pre arm.

History: at authoring the kit was a baseline kit whose edit under read was migration 0005
("`2026-09-13 hook-enforced-artifact-schema D1`", commit `5d8fc69`), read at `--old-ref 794cea8`
(unchanged 29 · changed 0 · removed 0 · added 1). No grid ran against it; it retired at the
0044 re-key under the positive-control section's own rule.

## Rubric shape (D8 partition, `observable.yaml`)

55 rules: **45 plan-observable · 10 out-of-instrument** (2 latitude/register · 1
precedence · 2 conditional · 1 structural · 4 fail-conditions). `check-rubric brainstorm`
prints `rubric OK: 45 observable, 10 out-of-instrument, 55 total`. At authoring the split was
21 · 9; the kit audit of 2026-09-19 moved `brainstorm.reopen-born-verify` to observable,
emptying the contingency class, because the same reasoning would have condemned
`brainstorm.coverage-survivor-routing`, which this kit grades observable. The 0044 re-key
(2026-10-06) took it from 22 · 8 of 30 to 45 · 10 of 55: 23 of the 25 added rules are
observable; `brainstorm.own-rules-win` is out as `precedence` (a new reason term, defined in
`observable.yaml`) and `brainstorm.size-shapes` as `conditional`.

- **Condition gating.** `km_file` is planted both ways (present in s1 and s3, absent in s2):
  `brainstorm.km-close-ritual` is graded on s1 and s3, where a backlog item exists for the
  session to close, and reads on the stated skip in s2 — an honest `absent` there is the
  same in both arms and never a regression. `seats` resolves to multi on every path (the
  review seat is a second seat), so `brainstorm.transport-floor` is graded everywhere and
  `single` is unplanted (only the user's in-run waiver reaches it).
- **Size gating (since 0044).** `size` is the user's ruling at frame hardening, which no
  fixture can make. Each golden's topic shape plants only the size the lead is expected to
  propose on the frame card: **s1 small**, **s2 standard**, **s3 standard** (reasons in
  `observable.yaml`'s header). The four size-gated observable rules are read on the goldens
  whose expected size they match — `brainstorm.small-session-review` on s1;
  `brainstorm.blind-map-dispatch`, `brainstorm.blind-map-fold` and
  `brainstorm.blind-second-list` on s2 and s3 — and an `absent` on a golden of the other
  size is by design: never a regression, never a dead-text finding. A replicate whose plan
  proposes another size is read on the size it proposed, and the fill log records it.
  Too-big is planted nowhere, so `brainstorm.size-shapes` is out-of-instrument.
- **Thin reads, named now:** `brainstorm.user-survivor-challenge`,
  `brainstorm.user-pen-boundary` and `brainstorm.reopen-born-verify` are graded on one
  described branch each at the disposition and acceptance gates; since 0044 four more join
  them — `brainstorm.frame-changes-users-word` (a hardened frame line changing on the user's
  word), `brainstorm.user-steers` (the user's standing levers each turn),
  `brainstorm.vague-answer-reasked` (a vague answer re-asked; s2's unsure user invites it)
  and `brainstorm.fix-one-card-verify-once` (the fix-and-verify branch after review
  survivors). They are the expected flaky candidates; a flaky pair on any of them is read as
  one noise event, never as a command finding on its own. `reopen-born-verify` (moved here
  from out-of-instrument on 2026-09-19) and `fix-one-card-verify-once` sit one hypothetical
  layer deeper than the others — each bounds what follows a routed survivor — so expect them
  flakier than their siblings, not their equal. An `absent` read on every replicate of every
  golden is the one outcome that reopens a thin read as a dead-rule finding.
- **Co-moving pairs:** `record-review-independence` / `author-grader-default-fail` and
  `coverage-survivor-routing` / `non-coverage-survivors` describe one seat and one gate
  respectively; a disagreement on one member usually shows on the other. Since 0044 three
  clusters join them: the frame (`frame-card` / `frame-hardens` / `size-users-ruling` — one
  card and its first reply), the map (`decision-map` / `stop-rule`, and `blind-map-fold` on
  s2 and s3 — one map from first question to empty) and the close of deciding
  (`accept-screens` / `user-record-acceptance` — one screen before the freeze and the word
  after review). The band counts each member (the runner's raw figure); the reader notes the
  cluster.

## Read rule (D6 as amended — tolerance band, verify V4; D11's form)

- Comparison substrate: the **in-grid pre-edit arm** (never a committed baseline file, which
  is only a pinned historical record). Reads are per golden, as the report prints them.
- **Unchanged bucket:** a rule regresses when it is `reflected` under pass^k in the pre arm
  and not in the post arm. Tolerance: **0 regressed rules** is the pass reading; 1–2
  regressed rules = "investigate — read the evidence quotes before any verdict"; ≥ 3 =
  "regression reading, present to the maintainer". Coverage-count drift without a named
  regressed rule is noise, not signal.
- **Removed bucket:** any removed rule still surfacing (pass^k in post) = "edit did not
  take" finding.
- **Added bucket:** an added observable rule not reflected in any post replicate =
  DEAD-TEXT finding; reflected in some but not all = visible-when-it-happens, invisible under
  pass^k — reported as such, never rounded up.
- **Changed bucket:** graded against the NEW text; pre-versus-post comparison advisory only.
- **Embodiment only.** The runner scrubs every rule id before judging; a plan line that
  restates an obligation as a principle without a scenario-concrete action reads `absent`
  (probe finding 7; harness-v2 I9 fold). A described user gate with what is confirmed and
  the onward branch per ruling is a concrete action (D9).
- **Judges advisory** (harness D2): the runner exits 0 on judged degradation; only the load
  gate, auth, a failed scripted assert, or a broken partition set a nonzero exit.

## Positive control (`primitive-eval-harness-v2` D11-preregistration-noise-guard as re-cut at C2 and M5) — gates the INSTRUMENT, never the command

On the first grid against `--old-ref 2e4c57c`, `brainstorm.frame-card` must read
**`absent` in `pre` on every golden** (the rule did not exist) **and `reflected` under pass^k
in `post`** on every golden — the plan's first message is a six-line frame card, each line
marked as the user's words or the lead's guess, with a proposed size and no confidence
percentage. It is the control because it binds at every size on every golden and is the
first thing a plan describes. Any `reflected` in `pre` is a rubric or judge defect; `absent`
or split in `post` is an instrument finding first (wrapper, golden, judge, or the binary
range below) and a command finding only after the instrument is cleared. A failed control
re-keys the runner, rubric, or goldens — it is never reported as a command regression. Once
the pair's next edit lands, that edit's added or removed rules become the control and this
one retires. (History: `brainstorm.artifact-home` at `--old-ref 794cea8` was the control
from authoring to the 0044 re-key and retired under this rule before any grid ran.)

## Noise band (`primitive-eval-harness-v2` D11-preregistration-noise-guard · I7, in the command form) and noise guard (F2)

- **Denominator: all pairs.** Command goldens carry no `tempts`; every observable rule is
  judged on every golden, so the band is the **all-pairs** flaky share per arm — pairs with
  replicate disagreement over 3 goldens × 45 observable rules = **135 pairs per arm** (well
  above the 8-pair `UNDER-SAMPLED` mark; a rubric re-cut that drops an arm below 8 pairs
  carries the mark and takes the band as the cap). The persona kits' invited-only recount
  (`2026-09-09-persona-band-invited-only`) does not apply where no golden invites a subset;
  it is the first re-key candidate if the all-pairs figure is dominated by pairs a golden
  plainly cannot reach (`km-close-ritual` on s2, the three standard-size rules on s1 and
  `small-session-review` on s2 and s3 — six pairs in all — are such pairs by design and are
  expected stable).
- **Band = the measured `pre` share + 5 points, capped at 20 %** — **[measured at
  baseline]** from the first grid's judged `summary.json`. The command report prints one
  combined pre-or-post flaky line per golden (`cmd_report`, "flaky rules (replicate
  disagreement — noise-guard input)"); the per-arm share the band needs is computed from
  `summary.json`'s per-run `coverage` verdicts — count, per arm, the (golden, rule) pairs
  whose three replicate verdicts are not all equal, over 135. A per-arm band line in
  `cmd_report` mirroring `agent-report`'s *Band input* section is the disclosed runner gap
  (a re-key candidate; no verdict depends on it).
- **Guard:** an arm whose flaky share exceeds the band is noise-dominated: no pre/post
  difference is read; add **one extra replicate per arm, once**, re-judge, then read. F2's
  discipline verbatim: same-variant replicate spread exceeding the variant gap = noise —
  operationally, if the count of flaky rules exceeds the count of pass^k differences between
  arms on a golden, that golden's read is noise-dominated.
- **Stopping rule:** two consecutive instrument re-keys (runner, rubric, goldens, wrapper,
  or judge changes made to make the control detectable) without a detectable control return
  the command target to the user for a keep / re-shape / abandon ruling. Re-key count starts
  at 0 and is logged below.

## Grid shape and budget (M6)

3 goldens (`s1-tradeoff-confident` · `s2-halfformed-unsure` · `s3-feature-shaped`) × 3
replicates × 2 arms (`pre` + `post`) = **18 plan sessions**, plus the one-time `nocmd`
control (3 × 3 = **9**) on the first grid only. Judges: Haiku coverage checklist over the
45-rule observable subset (three chunks of ≤ 15 per plan) + the stub axis; Sonnet pairwise,
position-swapped (2 calls per pre/post pair, 9 pairs). All judges advisory.

Cost anchor: the probe's `total_cost_usd` for this command was **$0.65 per session** at 25
turns (Sonnet, the smallest pair); the fixtures here are larger than the probe's, so expect
more. **Budget bound: ≤ 36 plan sessions (27 + the guard's one extra replicate per arm on
all three arms) and ≤ US$ 40 metered spend for the first grid including judge calls**;
exceeding either halts and returns to the user. Judge spend is read from the judge
sessions' `total_cost_usd` (the command runner does not yet sum it; record it by hand).

## Ship bar (advisory instrument — informs, never gates)

The kit is ready if the first grid (a) passes the positive control on every golden, (b)
localizes zero unchanged-bucket regressions or each one to a named rule id with evidence
quotes, (c) keeps the `pre` all-pairs flaky share ≤ 15 % so the band it fixes binds
uncapped (≤ 20 of 135 pairs), and (d) shows the branch is legible: the three goldens
produce visibly different coverage or evidence profiles on `question-form` (options with the
case against the stated lean on s1 versus an open question for the unsure user on s2),
`km-close-ritual` (s1/s3 versus s2), `next-step-offer` (the s3 offer described as an option
after acceptance), and the size split (`small-session-review` on s1 versus
`blind-map-dispatch` on s2 and s3). Identical profiles across the three would be a fixture
finding, not a command finding. (a) failing after one re-key, or the guard
tripping twice, triggers the stopping rule.

## Disclosures

1. **The inline questioning is the fidelity gap this instrument was built to state, not
   hide.** Since 0044 `brainstorm.lead-inline-questioning` binds the lead to run the
   questioning itself, inline, and the form of each question is `brainstorm.question-form`'s,
   its order `brainstorm.dependency-order`'s and the split between own turns and batched
   defaults `brainstorm.own-turn-or-batch`'s — and a headless run has no user. The form-only
   wrapper (D11, pinned by sha in every run) asks for the plan of the questioning, never a
   question; the judge reads these rules on the *described* sequence and its fit to the state
   the topic line carries (s1 states a lean and asks for a pressure-test; s2 states
   uncertainty; s3 states numbers as if decided). The frame card is the session's first user
   turn and its hardening the first gate: both are read as described, never asked.
   Confidence marks and ratified rulings are read on the plan's stated intent for them. No
   replicate ever asks a question, so nothing here measures the questioning's quality — only
   whether the pair still drives the lead to plan it inline and in the ruled form. The same D9
   contract covers every user gate.
2. **`seats` is multi on every path.** The record's review seat is structurally a second
   seat at both expected sizes — spawned at frame hardening at standard size, dispatched in
   two messages at small size; the single-seat path exists only through a taken waiver. A
   topic line pre-declaring "skip the review" would plant it and is recorded in
   `observable.yaml` as an unplanted branch, not taken because it also removes six review
   rules from that golden's read.
3. **Fixture-echo caveat.** The fixtures are a fictional product (Saltmarsh) at three
   moments. Prior session records necessarily carry record grammar — numbered decisions with
   statement, rationale and a confidence mark, a `Status:` line, open questions — which is
   artifact identity, not rule text. Two deliberate omissions keep the workspace from
   restating the pair: the real repo's sessions index opens with a maintenance-contract
   paragraph (open adds an entry, acceptance updates it) that restates index bookkeeping and
   is left out; the pinned knowledge-management file names "each command's landing step"
   instead of listing the commands. Governance vocabulary the region always carries
   (`Waivers: None.` in the ledger, the output-style switch line) stays, since a ratified
   project has it. A `nocmd` arm on s1 or s3 is what measures the residue. Since 0044 the
   prior records also predate the decision card (`brainstorm.decision-cards`): a plan that
   copies their numbered-decision grammar instead of the card's fixed parts reads `absent` on
   that rule in both arms. The fixtures stay as authored — a fixture change would change what
   a session sees.
4. **`km-close-ritual` on s2** reads on a stated skip; the judge may credit the skip as
   embodiment or mark it absent. Either reading is expected stable across arms; a split is
   a judge finding on that pair, not a command finding.
5. **No judge-calibration set.** The command-target form (implement, setup) carries none;
   harness-v2 I9 binds persona pilots. If the first grid's evidence quotes on the thin reads
   look unreliable, a hand-labelled sheet in the persona kits' form is the re-key, counted
   toward the stopping rule.
6. **Pairwise judge.** Position-biased in every grid so far (both command pilots, persona
   pilot 1); advisory, reported, never read for a verdict.
7. **The size is a prediction, not a plant.** No fixture can rule `size`; the expected
   proposal per golden (s1 small, s2 standard, s3 standard) is the kit author's reading of
   each topic's shape, checked by the first grid, which records every replicate's proposed
   size. A golden whose replicates disagree on the proposed size is a fixture finding for
   that golden — its topic does not plant a size cleanly — and its size-gated reads are
   taken per replicate on the size proposed, never as a command finding.

## Pre-flight (substrate watches, checked before the first priced run)

- **Binary range.** The command's rules render only if the installed `mochiko-cli` covers the
  log's grammar range; at authoring (2026-09-18) the installed binary is `mochiko-cli 0.1.0 ·
  grammar 1..1`, and `mochiko-cli rules brainstorm --section preamble --plugin-root
  plugins/mochiko` against the working tree printed two `home-bounds · home/brainstorm-session`
  check lines and **no version-triple line** — so a `post` session today would halt at
  delivery with `mochiko-cli rules not delivered`. The command runner has **no
  delivery-halt gate** (it asserts the load gate and auth only), so the halt text would be
  graded as the plan and read all-absent. Before the grid, re-run that render from the
  working tree and confirm a version-triple line; a halt is a blocked grid, never a
  regression. Candidate runner re-key: assert the absence of the halt string in every plan,
  as the skill runner does. The `pre` arm at `2e4c57c` archives a log ending at 0043 and the
  `post` arm one ending at 0045; both need a binary whose grammar range covers the log's
  (`mochiko-cli migrate status` prints it).
- **`check-fixtures brainstorm`** prints `fixture consistency: OK` (probe finding 9) — every
  index link, decision-row pointer, trail pointer, and CLAUDE.md region pointer resolves
  inside its fixture.
- **Turn cap.** The probe hit 25 of 25; the runner now allows 40 and prints a cap-hit
  warning. A cap-hit run is a warning and its plan is graded; three cap-hits on one golden
  is a fixture-size finding to log.
- **Fence.** `--allowedTools Read,Grep,Glob` under `dontAsk`; `mochiko-cli home` is denied
  at call time, so `artifact-home` is read on the plan's statement that it renders the home,
  never on a rendered result. Any attempted `Bash`/`Agent`/`Write` call is logged by the
  runner as a fence breach and disclosed.

## Fill log

- band: **[measured at baseline]** — `pre` flaky pairs ___/135 = ___ % → +5 = ___ %
  (capped 20 %); `post` ___/135 = ___ %. Filled from `runs/<grid>` on ____ (sessions $___;
  judges $___ / ___ calls).
- positive control (`frame-card`, `--old-ref 2e4c57c`): pre `absent` on s1 / s2 / s3
  ___; post pass^k `reflected` on s1 / s2 / s3 ___.
- unchanged-bucket regressions per golden: s1 ___ · s2 ___ · s3 ___.
- added-bucket reads per golden (DEAD-TEXT · visible-when-it-happens), size-gated rules on
  their expected goldens only: s1 ___ · s2 ___ · s3 ___.
- proposed size per replicate (expected s1 small · s2 standard · s3 standard): s1 ___ ·
  s2 ___ · s3 ___; goldens whose replicates disagree ___ (a fixture finding).
- dead-zone (`nocmd`, first grid only): rules pass^k under `nocmd` per golden ___.
- thin reads (`user-survivor-challenge`, `user-pen-boundary`, `reopen-born-verify`,
  `frame-changes-users-word`, `user-steers`, `vague-answer-reasked`,
  `fix-one-card-verify-once`): flaky on ___; co-moving clusters observed ___; any absent on
  every replicate of every golden? ___ (yes = dead-rule finding, not noise).
- branch legibility (ship bar d): `question-form` evidence on s1 vs s2 ___;
  `km-close-ritual` on s1/s3 vs s2 ___; `next-step-offer` on s3 ___; size split
  (`small-session-review` s1 vs `blind-map-dispatch` s2/s3) ___.
- cap-hit runs ___ · fence breaches ___ · unresolvable names ___.
- budget: ___ plan sessions · US$ ___ of 40.
- re-key count: 0
- kit status: **AUTHORED — awaiting the first grid**
- 2026-10-06 amendment, recorded before any grid: the kit is re-keyed to migration 0044
  (`brainstorm-target-state` D1–D15, D17–D19, D22; `observable.yaml` 45 · 10 of 55; the
  goldens' `expected_output` and `assertions` rewritten to the new flow, their `args`,
  `control_prompt` and fixtures unchanged), the edit under read moves to `--old-ref 2e4c57c`
  and the positive control from `artifact-home` to `frame-card`, both under this file's own
  rules. The re-key count stays 0: the stopping rule counts re-keys made to make a control
  detectable after a grid, and no grid has run (no `runs/`). Made by the build's tests-and-kits
  seat on a lead-approved plan.
