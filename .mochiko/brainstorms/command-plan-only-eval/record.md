# Command Plan-Only Eval — Decision Record

**Status:** accepted (2026-08-27 — solo cold review, verdict critical-gaps, 17/17
dispositioned; verify round 1 NOT CLEAN, all 13 lead-repaired same round; round 2 CLEAN;
user accepted with D11 ruled at acceptance)
**Opened:** 2026-08-27
**Session:** collaborative brainstorm via `mochiko:analysis-iterative` (one question per turn)

## Topic

Evaluate mochiko's commands without invoking the nested skills, subagents, and seats a real
command run orchestrates: run the command in a plan-only mode where it generates its detailed
action plan without executing it, and have an LLM judge grade that plan (detail level, and
whatever else the session rules the judge should grade). The bet: a plan-only run is a far
cheaper, less noisy eval substrate than the full simulated command runs used once before
(F2 below), while still exercising the command's `.md` + schema pair — the artifact actually
being edited.

**Relation to prior sessions (dependent session):**

- `primitive-eval-harness` (2026-08-19, **open**) — the reserved home for the eval-harness
  question. Its D1–D5 ruled a skills-first harness: goldens in the `skill-creator` format,
  deterministic checks may block / LLM judge advisory-only, local-first regression (CI
  deferred), thin scripts under a top-level `evals/` dir, one pilot skill first. Commands
  were left uncovered. This session extends the eval question to the command surface with a
  different substrate (plan-only, no execution).
- `skill-compression-tooling` (2026-08-22, accepted) — judge-protocol precedent:
  rule-coverage checklist primary + pairwise secondary, binary verdicts + quoted evidence,
  judges advisory.

## Ground facts

*(seeded from the two prior records and the current index, 2026-08-27; to be extended by
fact-finding as the session needs)*

- **F1 — the command surface under test is six `.md` + schema pairs, 320 rules.** As of
  v0.97.0 every shipped command is a canonical-scaffold `.md` plus a runtime-interpreted
  YAML schema (`plugins/mochiko/schemas/<cmd>.yaml`); provenance anchors live repo-side in
  `.mochiko/provenance.yaml` (never shipped). The schema rules are machine-enumerable —
  a natural rubric source.
- **F2 — full simulated command runs have been done once; noise was the finding.**
  `validator-scope-and-verbosity` (2026-08-10): 12 full simulated command runs, blind
  0–100 judge; on one of three arms replicate spread (5.6/7.1) exceeded the effect gap
  (0.8/1.8) — the noise guard fired and the verdict had to be user-ruled.
- **F3 — the harness session's rulings bind the frame this session works inside.**
  `primitive-eval-harness` D2 (deterministic may block, judge advisory — `Confident`),
  D3 (local-first regression, CI deferred — `Confident`), D4 (thin scripts under `evals/`,
  never shipped; stable pieces promote to the Rust crate — `Confident`). Those are ruled;
  this session composes with them rather than re-opening them.
- **F4 — cost anchors.** One full scripted session ≈ $0.55 (Sonnet 5) / ~$1.00 (Opus 5);
  a real command run orchestrates multiple seats and is a multiple of that. A plan-only
  run is one session with no subagent spawns.
- **F5 — a command run's plan is already a first-class object in the command design.**
  Commands are lead-orchestrated: the lead plans the run (seats, skills, phases, gates)
  before dispatching; plan-approval is a standing harness rule for producing seats. The
  plan-only eval elicits exactly this object without the dispatch.

## Decisions

*(as ruled, D1… — statement + rationale + confidence mark)*

- **D1 — Purpose: regression instrument on command edits.** `Assumed` (user-ruled "as you
  recommend"; re-marked from `Confident` at review disposition I7 — unelaborated adoption
  of the lead's recommendation; the user ruled the `Assumed` marks). A plan-only run per command with a recorded baseline; an edit to the
  command's `.md` or schema is compared against that baseline. Not an absolute quality
  score — F2 (own benchmark: absolute scoring noisy) and the harness session's F19 (dogfood
  + eyeball cannot separate regression from variance) both point at diff-against-baseline
  as the useful signal. **Driver (user, this session):** planned modifications to the
  command `.md` structure and to the rules; the eval exists to understand the impact of
  those edits.

- **D2 — Judge grades rule-coverage primary, pairwise blind A/B secondary (advisory).**
  `Assumed` (user-ruled "as you recommend"; re-marked from `Confident` per I7, with
  `efficacy: Assumed (n=0)` per I4). Primary metric: the judge reads the generated
  plan and grades each of the command's schema rules — rubric = the D8 plan-observable
  subset (verify V4) — reflected / absent / contradicted, binary per rule, quoted
  plan-line evidence. The regression diff is the coverage set vs the
  baseline's coverage set, which localizes an edit's impact to named rules. Secondary:
  position-swapped blind A/B of baseline plan vs post-edit plan for holistic drift the
  checklist can't see. Matches the `skill-compression-tooling` precedent (rule-coverage
  primary, pairwise secondary, judges advisory) and harness D2 (judge never blocks).
  **Amended at review (I6):** a third judged axis — stub detection: per mandated plan
  field, a binary substantive-vs-nominal-stub call with quoted evidence, so a uniformly
  shallow plan cannot pass form assertions plus coverage nouns (the pairwise secondary is
  blind to shallowness present on both sides).

- **D3 — The elicited plan is an interpreted action plan, never a rule recitation.**
  `Confident` (user-stated: "i dont want the output to be a list of rules it will follow,
  more like interpretation of what it needs to do"). The plan speaks in scenario-concrete
  actions — seats spawned, skills loaded, artifacts written, gates run — not in rule IDs or
  quoted obligations. Rationale: a rule-echo plan would trivially saturate the D2 coverage
  metric while testing nothing; the schemas are model-interpreted at runtime by design
  (`command-content-schema` D1), so interpretation quality is exactly what the eval must
  measure. Consequences: (a) the judge's coverage call is an inference — does this action
  embody that rule — with the embodying plan line quoted as evidence; (b) deterministic
  assertions cannot grep for rule IDs and stay form-level (fields present, phases
  non-empty); (c) the elicitation contract must not ask for rule citations.

- **D4 — Fixtures: minimal synthetic per-scenario repos, branch-forcing by design, in the
  skill-eval scaffolding.** `Confident` (user-ruled "ratify" on the refined shape).
  Carried over from `primitive-eval-harness` D1–D5 + `skill-compression-tooling` D8:
  `skill-creator` golden format at `evals/commands/<cmd>/evals.json` (id · prompt ·
  assertions) · goldens authored by a seat that did not edit the command (author≠grader) ·
  3 replicates, pass^k · Haiku judge for coverage binaries, Sonnet for the pairwise read,
  both advisory · runner `claude -p --bare --plugin-dir` with cwd = the fixture dir (as
  amended below at C4: cwd is the assembled ephemeral workdir, not the committed scenario
  dir), `--max-turns` cap, JSON output · committed baseline results file regenerated only
  as a deliberate landing act. Command-specific refinements, ratified: **(1)** each scenario dir
  `evals/commands/<cmd>/fixtures/<scenario>/` contains exactly the files the command's own
  `.md` + schema declare as inputs — nothing else (smallness + the neutrality constraint;
  verify N5: this binds the committed scenario dir — the assembled run-time workdir is a
  distinct object and additionally carries the provisioned `plugins/mochiko`, per the C4
  amendment below);
  **(2)** scenarios are branch-forcing, not generic — the fixture seeds a known plan
  decision (e.g. two planted sufficiency gaps → the plan must show a design phase scoped to
  exactly those two; a zero-gap variant → cards + build direct), the planted branch recorded
  in the golden's assertions (seeded-defect analog, AR-D3; also the direct test of D3's
  interpretation requirement — the command must react to what it reads, recitation can't
  pass); **(3)** self-contained per-scenario dirs, no shared base fixture — a shared base
  couples baselines across commands; duplication is cheap at this size, dedupe only if
  maintenance hurts; **(4)** 2–3 scenarios per command — one happy path plus one or two
  branch-forcers. **Amended at review** (`efficacy: Assumed (n=0)` per I4): **(C4)**
  fixtures are provisioned into an ephemeral workdir — the fixture's files plus the working
  tree's live `plugins/mochiko/` — so the command's repo-relative schema reads
  (`plugins/mochiko/schemas/<cmd>.yaml`, `command-labels.yaml`, `vars:` paths) resolve and
  the pair under test is never duplicated into fixtures; probe-gated (build item 0, verify
  V5 — not yet verified). **(I5)** each
  scenario's grid carries a no-command control arm at pilot time — rules that pass without
  the pair are flagged as the instrument's dead zone (prune candidates). **(M4)** pilot
  scenario count aligned to the harness 3–5 golden band: 3 scenarios for the pilot; the
  other five commands are scoped at their own build time (verify N6 — the 2–3 band in (4)
  above is superseded for the pilot by this alignment).

- **D5 — Pilot command: `/mochiko:implement`, end-to-end before any other command.**
  `Confident` (user-ruled "pilot on implement"). Fits the profile: the largest pair, the
  single downstream run since plan's retirement (v0.91.0), and the most-churned surface
  through the v0.92.0–v0.97.0 schema/scaffold waves — exactly where the user's planned
  `.md`-structure and rule edits will land. Other five commands follow only after the loop
  is proven once (harness D5 pattern).

- **D6 — Regression semantics under a moving rubric: three-part diff keyed on rule IDs.**
  `Assumed` (user-ruled "as you recommend"; re-marked from `Confident` per I7, with
  `efficacy: Assumed (n=0)` per I4). When an edit changes the schema itself, the
  eval partitions the two schemas' rule sets mechanically by dotted-slug ID (mint-once +
  tombstones, `command-content-schema` D11 — no judge involved in the partition):
  **unchanged rules** — coverage must not drop vs baseline; the regression signal proper.
  **Removed rules** — expected to vanish from the plan; listed for confirmation, a removed
  rule still surfacing means the edit didn't take. **Added rules** — coverage reported as
  an adoption check: did the new rule land in behavior or is it dead text. One instrument,
  three answers (nothing unrelated broke · removal took · addition took) — the user's
  "understand the impact" driver verbatim. A pure `.md`-structure edit is the degenerate
  case: rubric unmoved, all rules in the unchanged bucket, straight coverage diff.
  **Amended at review (C3):** the comparison is **within-grid** — the pre-edit pair re-runs
  as an arm in the same grid as the post-edit arm, so model/CLI/plugin/wrapper drift
  cancels instead of masquerading as regression (per-edit cost roughly doubles, ~18
  sessions); the committed baseline file additionally pins judge model, CLI version, plugin
  version, and wrapper text, and any pin change forces re-baselining as a deliberate act.
  Role split (verify V2): the **in-grid pre-edit arm is the comparison substrate** for
  every D6 criterion; the committed file is the pinned historical record, never the diff's
  denominator. Bucket grading ranges over the D8 plan-observable subset, with the
  unchanged-bucket criterion read through the preregistration tolerance band (build 2),
  not as an absolute zero (verify V4).

- **D6 amendment — changed-text rules are a fourth bucket.** `Confident` (user-confirmed).
  A rule whose ID survives but whose obligation text was edited is graded against the NEW
  text; its baseline comparison is advisory only — stability cannot be demanded of an
  obligation that itself moved.

- **D7 — Plan-only fence is structural: tool strip, not instruction.** `Confident`
  (user-confirmed). The eval session runs with a restricted tool set *(the original
  deny-list form — `--disallowedTools Agent,Task,Edit,Write`, claimed to make execution
  impossible — is superseded by the I2 amendment below, verify V3: a deny-list leaves Bash
  and MCP write tools open, so the claim did not hold)*; read tools stay so the command can
  read the fixture; the wrapper instruction ("produce the plan you would follow; do not
  execute") remains as belt-and-braces, but the fence is the tool set. **Amended at review (I2):**
  the fence is an explicit **allow-list**, not a deny-list — a deny-list of
  `Agent,Task,Edit,Write` leaves Bash (and MCP write tools) able to execute and write,
  making the fence instructional wearing a structural label. Allowed: `Read,Grep,Glob`
  only; Bash excluded. Whether losing Bash-based search measurably thins plans is checked
  by the probe (build item 0); the allow-list is the ruled default.

- **D8 — Pre-grid rule partition: plan-observable vs contingency-only.** `Confident`
  (user-ruled at review disposition, C1 "as recommended"). Before the first grid, the
  pilot command's rule set is partitioned: rules a forward plan can observably embody vs
  run-time contingency rules (fail-conditions, rework paths, mid-run escalations) that no
  forward plan contains. The observable subset is the D2 rubric; the contingency remainder
  is recorded out-of-instrument in the eval's own files — declared, never silently at
  coverage 0 inside the unchanged bucket. Expected side effect: a substantially smaller
  checklist than the full ~104 rules — the magnitude is measured when the partition runs
  (build 3; verify N7). Partition authored by the fixture/goldens seat
  (the non-editor seat, D4), confirmed at build.

- **D9 — Absent-user contract.** `Confident` (user-ruled at review disposition, C2 "as
  recommended"). Every shipped command gates on the user (implement's run-open
  confirmation, design checkpoint, card confirm, final acceptance); a headless run has no
  user. The elicitation form contract therefore instructs the command to **describe** each
  user gate — the confirmation it would seek and both onward branches — never to await
  one. A run that stalls at a gate or emits a bare confirmation request is a discarded
  run, not a graded plan. Recorded fidelity divergence: under the D7 fence the run cannot
  spawn the independent sufficiency-grading seat, so the plan describes that verdict
  inline — a known divergence from a real run, watched, not hidden. Probe-gated (build
  item 0, verify V5 — to be settled there). Discard bound (verify N1): persistent stalls
  across replicates are themselves a reportable substrate result — the contract failed —
  never an unpriced retry loop.

- **D10 — Plan-quality-as-proxy is the session's named assumption.** `Assumed` (ruled at
  review disposition, I4 "as recommended"). A command can plan well and run badly; the
  instrument reads plan-level regression only, and D6's added-rules bucket makes the
  strongest claim ("did the new rule land in behavior") from a plan alone. Falsifier: one
  full real `/mochiko:implement` run against the pilot fixture, once, correlating
  plan-level coverage with observed run behavior; material divergence revisits the
  substrate bet (alongside the noise falsifier in Open questions). Named risk (Goodhart):
  scoring plans pressures command prose toward verbose self-description — in direct
  tension with the live compression program and the char budgets — with D3's
  anti-recitation register as the standing partial mitigation; watched at first live use.
  D2, D4, and D6 carry `efficacy: Assumed (n=0)` sub-marks accordingly.

- **D11 — Form-only elicitation: the harness prompt forces form, never content.**
  `Confident` (user-ruled "yes" at acceptance; promoted from the working note the cold
  review flagged as "a decision wearing a note's clothes"). The elicitation wrapper may
  mandate the plan's shape — machine-readable, mandatory fields, the D9 gate-description
  contract — but may never name the phases, seats, skills, rules, or artifacts a good plan
  would mention: a content-bearing wrapper compensates for a broken command pair and masks
  exactly the regressions D1 exists to catch. All plan content is driven solely by the
  `.md` + schema pair under test and the fixture it reads. The most validity-critical
  constraint in the design; every wrapper edit is checked against it, and the wrapper text
  is pinned in the baseline (D6 as amended). *(Local D11 — distinct from the external
  `command-content-schema` D11 cited in D8's rationale.)*

## Session trail

*(one question per turn; answers summarized)*

- **Q1 — purpose:** regression instrument vs absolute score vs both. User: regression
  instrument, as recommended; driver is upcoming `.md`-structure + rule modifications
  whose impact they want to see. → D1.
- **Q1 follow-up (user):** can prompting itself ensure the action plan is detailed? Lead's
  answer folded into the elicitation-mechanics note below.
- **Q2 — judge metric:** rule-coverage primary vs pairwise primary vs detail rubric. User:
  rule-coverage primary, pairwise secondary, as recommended. → D2. Same turn, user
  clarified the plan register: interpretation of what to do, never a list of rules to
  follow. → D3.
- **Q3 — scenario substrate:** synthetic fixture repo vs real-repo snapshot vs
  prompt-described state. User: synthetic fixture makes sense, asked for a refined
  recommendation; lead refined against the skill-eval scaffolding (four command-specific
  refinements). → D4.
- **Q4 — ratification + pilot:** user ratified the refined fixture design and picked
  `implement` as the pilot command. → D4, D5.
- **Q5 — moving rubric:** three-part diff (unchanged / removed / added, keyed on rule IDs)
  vs grade-against-new-schema-only. User: three-part diff, as recommended. → D6.
- **Q6 — residual mechanics:** changed-text fourth bucket + structural plan-only fence.
  User confirmed both. → D6 amendment, D7.
- **Q7 — review sizing (named gate):** solo recommended; user ruled solo.
- **Q8 — dispositions:** user ruled the batch "as recommended", I7 = `Assumed` re-marks,
  M2 = as recommended (D5 stands, probe on `brainstorm` first).
- **Q9 — D11 promotion + acceptance:** user ruled "yes to both, accept". → D11; record
  accepted 2026-08-27.

## Elicitation mechanics

*(mixed register, verify N2: bullets that landed in decisions or the build surface are
operative — D7's fence, D9's absent-user contract, M1's name-resolution check (blocking-
capable per build 5); the neutrality constraint was promoted to **D11** at acceptance;
the output-contract shape stays an open build detail)*

- Detail is enforceable by an **output contract in the elicitation prompt**: require a
  machine-readable plan (harness `--json-schema` from the platform, or a pinned markdown
  template) with mandatory fields — phases, per-phase seats, skills loaded, inputs read,
  artifacts written, gates/checkpoints. Structure-forcing beats "be detailed".
- **Neutrality constraint:** the harness prompt may force *form*, never *content*. If the
  elicitation prompt names the phases/seats/rules the plan should mention, it compensates
  for a broken command pair and masks exactly the regressions D1 exists to catch. All
  content must be driven by the `.md` + schema pair under test.
- Consequence: with form forced, field presence and counts become deterministic assertions
  (may block, per harness D2); the judged remainder of "detail" is owned by D2's
  stub-detection axis as amended at review I6 (verify N3 — this bullet's earlier "detail
  largely stops being a judged property" phrasing was stale against that amendment).
- **Plan-only enforcement is structural, not instructional:** proposed here, ruled as D7.
- **Deterministic layer extension (review fold M1):** every seat, skill, and artifact path
  the plan names resolves mechanically against `plugins/mochiko/agents/`,
  `plugins/mochiko/skills/`, and the schema's `vars:` paths — a free, blocking-capable
  hallucination check that takes load off the judge; neutrality-safe (it forces a field,
  never its content).

## Rejected roads

*(why each lost — review fold M3)*

- **Absolute quality score** (Q1): no anchor; F2's own benchmark shows absolute scoring
  noisy — lost to diff-against-baseline.
- **Detail rubric / pairwise-primary** (Q2): holistic scores are unreliable and
  non-actionable; per-rule binaries localize an edit's impact to named rules.
- **Real-repo snapshot fixture** (Q3): plans sprawl, the judge drowns, and a big shared
  substrate couples baselines; **prompt-described state** (Q3): skips the read layer
  entirely and puts content into the harness prompt, violating the neutrality constraint.
- **Grade-against-new-schema-only** (Q5): blind to whether removals took effect; baseline
  comparison murky when texts shift.
- **Partial execution with stubbed seats** (never raised in-session; surfaced at review):
  higher fidelity, but a mock-seat harness is a build of its own, per-run cost climbs
  toward the F2 regime, and F2's noise finding erodes the fidelity advantage.
- **Manual no-judge plan diff** (surfaced at review): the cheapest shape at six commands,
  but no rule localization and drifts with reader attention; kept as a complement the
  maintainer can always do, not the instrument.
- **Full simulated command runs**: done once (F2); replicate spread exceeded effect gaps
  and cost is an order of magnitude higher — the session's founding rejection.

## Cost line

*(as amended at review — C3 within-grid, M5 lower-bound honesty)*

Pilot grid: 3 scenarios × 3 replicates × 2 arms (pre-edit + post-edit within one grid, D6
as amended) ≈ 18 sessions per evaluated edit, plus the one-time no-command control arm at
pilot (~9 more). At the F4 anchors that is ≈ $10–15 (Sonnet) / $18–27 (Opus) — **a lower
bound**: command
sessions read a 39 KB schema plus a fixture tree and emit long structured plans, so the
skill-session anchor understates; the probe's `total_cost_usd` supplies the real figure.
Judging: Haiku checklist ~$0.035/transcript, Sonnet pairwise pennies. Still an order of
magnitude under the full-simulated-run benchmark (F2), and each session is single-seat by
construction (D7).

## Build surface

*(what a build session executes; nothing here is built yet — as amended by review folds)*

0. **Probe first (I8, M2):** one plan-only run of the smallest pair (`brainstorm`, 29
   rules) against a hand-made fixture, before anything else is built. Settles empirically:
   invocability of a `disable-model-invocation: true` slash command under headless
   `claude -p` · schema-path resolution under the ephemeral-workdir provisioning (C4/D4) ·
   plugin shadowing (I1 — the user-level cache runs v0.91.0 against repo v0.97.0) · the
   allow-list fence's effect on plan quality (D7) · the absent-user contract (D9) · plan
   shape under the form contract · real per-session cost via `total_cost_usd` (M5). Pilot
   stays `implement` (D5, user-re-ruled at M2): probe small, pilot big.
1. **Runner extension** under the harness session's `evals/` home (its D4, never shipped):
   plan-only invocation — `claude -p --bare --plugin-dir <workdir>/plugins/mochiko
   --setting-sources "" --allowedTools Read,Grep,Glob --max-turns N
   --output-format json` — the `--plugin-dir` given absolute into the ephemeral workdir's
   provisioned tree, which is authoritative for both plugin loading and the command's
   repo-relative reads (verify N4: one tree, no ambiguity about which copy runs) — with
   the form-only elicitation wrapper, plan capture, and a
   **blocking load gate**: the pair's version visible in the init event, else the run is
   invalid (I1; `plugin_errors` alone is FIELD-ABSENT per the skill harness's probe).
2. **Preregistration gate (I3+M4):** the runner refuses a grid without a committed
   `preregistration.md` carrying the read rule — tolerance band on unchanged-bucket
   coverage — and F2's noise guard verbatim (same-variant replicate spread exceeding the
   variant gap = noise; one more replicate pair before any verdict).
3. **Rule-partition scripts:** (a) two schema YAMLs in, four ID-keyed buckets out
   (unchanged / removed / added / changed-text, D6); (b) the D8 observable-vs-contingency
   partition, authored by the fixture seat, committed beside the rubric.
4. **Pilot fixtures:** `evals/commands/implement/fixtures/<scenario>/` — 3 self-contained
   scenario dirs (happy path + branch-forcers, e.g. two planted sufficiency gaps · zero-gap
   direct-to-cards) plus `evals.json` goldens; authored by a seat that did not edit the
   command (D4); provisioned into ephemeral workdirs at run time (D4 as amended).
5. **Judge prompts:** Haiku rule-coverage checklist over the D8 observable subset (binary +
   quoted plan-line evidence, embodiment inference per D3, `{text, passed, evidence}`
   triples) · stub-detection axis (D2 as amended) · Sonnet pairwise blind A/B with position
   swap · deterministic name-resolution assertions (M1). Judges advisory (harness D2);
   name-resolution and load gate may block.
6. **Grid + control:** within-grid pre-edit/post-edit arms (D6 as amended) · one-time
   no-command control arm at pilot, dead-zone rules flagged (I5).
7. **Correlation check (D10 falsifier):** one full real `/mochiko:implement` run against
   the pilot fixture at pilot close, plan-coverage vs run-behavior.
8. **Baseline capture:** committed per-scenario baseline (plan + coverage results + pins:
   judge model, CLI version, plugin version, wrapper text), regenerated only as a
   deliberate landing act (D4, D6 as amended) — the pinned historical record; the diff's
   comparison substrate is always the in-grid pre-edit arm (verify V2).
9. **Cross-session fold:** on acceptance, annotate the open `primitive-eval-harness` record
   that this session extends its scope to the command surface with the plan-only substrate;
   that session still closes on its own terms.

## Open questions

*(elicited unknowns surface here — non-waivable floor)*

- Judge chunking — mitigated by the D8 partition (the rubric is the observable subset;
  magnitude measured when the partition runs, build 3); residual sizing is build detail.
- Elicitation output contract shape — `--json-schema` JSON vs pinned markdown template —
  build detail, picked at build under the form-only neutrality constraint.
- Noise profile of plan-only runs (plan-length variance across replicates) — n=0; the
  pilot's first grid is the falsifier. If replicate spread again exceeds effect gaps (F2's
  failure mode), the substrate bet weakens and the session's premise is revisited. The
  read rule + noise guard themselves are now owed via the preregistration gate (build 2).
- Which branch-forcers beyond the sufficiency-gap pair earn a scenario slot — build-time
  scoping with the fixture author.
- Scheduling of the D10 correlation run (pilot close proposed) — build-time.

## Cold review, dispositions, and folds (2026-08-27)

- **Sizing:** solo cold reviewer (user-ruled at the named gate); seat spawned as agent
  type `mochiko:devils-advocate` (lead-confirmed from the spawn call, verify N8), running
  `mochiko:review-brainstorm` under teammate name `cold-reviewer`, via blind-map
  two-message dispatch — 47-angle map (10 families) frozen topic-only before the record
  path was sent. One disclosed negligible fence leak (three brainstorm directory names
  sighted via `find`, none opened) — accepted.
- **Fact verification:** every load-bearing claim checked exact against files (F1 320-rule
  census · F2 spread/gap figures · F3 harness rulings · F4 cost anchor · D5's size/churn
  claims) — zero broken claims.
- **Verdict as returned:** **critical-gaps** — 17 raised, 17 survived (4 Critical,
  8 Important, 5 Minor; two self-attack downgrades from Critical). Fitness: confidence
  marks FAIL (8/8 `Confident` at n=0), rejected-roads PARTIAL, remainder PASS.
- **Dispositions (user-ruled):** batch "as recommended" — C1–C4, I1–I6, I8, M1, M3–M5;
  **I7** user-ruled the `Assumed` re-marks on D1/D2/D6; **M2** as recommended — D5 stands,
  the I8 probe runs on `brainstorm` first.
- **Folds applied:** C1→D8 · C2→D9 · C3→D6 amendment (within-grid + pins) · C4→D4
  amendment (ephemeral-workdir provisioning) · I1→runner load gate + `--setting-sources ""`
  · I2→D7 amendment (allow-list) · I3+M4→preregistration gate + 3-scenario alignment ·
  I4→D10 (`Assumed` proxy + Goodhart risk + `efficacy: Assumed (n=0)` sub-marks on
  D2/D4/D6) · I5→control arm (D4 amendment) · I6→D2 amendment (stub-detection axis) ·
  I8→build item 0 probe · M1→deterministic name-resolution extension · M3→Rejected roads
  section · M5→cost line lower-bound annotation · I7→D1/D2/D6 re-marked `Assumed`.
- **Verify round 1 (same reviewer, bounded): NOT CLEAN** — 5 blocking (V1 cost-anchor
  arithmetic · V2 dual "baseline" ambiguity · V3 D7 self-contradiction · V4 D8 rubric
  narrowing unpropagated · V5 "probe-settled" past tense) + 8 nits (N1–N8), all 13
  lead-repaired in place same round; no fold was unfaithful, no disposition dropped, no
  ruling reopened. Round 2 bounded to the repair sites.
- **Verify round 2: CLEAN** — all 13 repairs confirmed at site (V1 arithmetic recomputed
  both anchors; V2 role split at both sites; V3 supersession marked; V4 propagated to
  D2/D6/D8/build 5 agreement; V5 forward-looking at both sites; N1–N8 each verified).
  Fitness FAIL discharged — `Assumed` marks and `efficacy: Assumed (n=0)` sub-marks
  present, Rejected roads closes the PARTIAL. Three non-blocking carried observations, two
  lead-tidied same round (chunking echo cross-ref · D4 cwd clause), the third (the D11
  token naming both an external decision and the locally proposed one) resolved at the
  D11 ruling.
