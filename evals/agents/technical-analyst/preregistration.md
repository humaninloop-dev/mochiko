# Preregistration — technical-analyst persona plan-only eval (wave A baseline kit)

> **Evidence.** The run directories cited below (`runs/baseline` and its siblings) are not carried
> in the working tree — this kit's `.gitignore` ignores every `runs/` directory, so the raw sessions
> are archived on a ref of their own rather than force-added here. Read one with:
>
>     git fetch origin tag eval-evidence-2026-09-19
>     git show eval-evidence-2026-09-19:evals/agents/technical-analyst/runs/baseline/report.md
>
> Or check the whole archive out: `git worktree add /tmp/eval-evidence eval-evidence-2026-09-19`.

Committed BEFORE the baseline grid (`primitive-eval-harness-v2` D11 as folded; the pilot rulings
`2026-09-09-persona-pilot-1-latitude-out-of-instrument` and `2026-09-09-persona-pilot-2-validator-read`;
the band ruling `2026-09-09-persona-band-invited-only`). This is a **baseline kit**: no persona edit
exists to detect, so there is no positive control. The kit fixes the band and the coverage so that the
persona's next edit gets a `pre`/`post` read on day one. Fields marked **[measured at baseline]** are
filled from the baseline grid.

## Edit under read

None. `pre` = `post` = the tree at `b9efb59` (the commit the rubric was minted at; the persona file is
byte-identical in the working tree), all 44 claims `common`. The pin, not `HEAD`, is what the next edit's
`--old-ref` names. The next persona edit re-mints over the union of refs and runs the pilot form with a
positive control on the edit.

## Rubric shape (partitioned 2026-09-09)

44 claims over 44 units: 41 plan-observable · 3 out-of-instrument (`dispatch-conditional`, the
cheap-read trio) · 2 units in `not_claims` (the Brownfield Awareness list intro harvested as a
sentence; the model-tiering pointer) · 2 compound units split by hand (`Schema-aligned` into the
schema-mirrors-entity standard and the error-codes standard; `Technology-agnostic in analysis` into the
requirements-say-what standard and the constraints-are-real-boundaries standard). No
`latitude-conditional` claim was found: the persona's body is must-shaped throughout. No claim is
`untempted`: four goldens invite 87 (golden, claim) pairs over the 41 graded claims (a1 24 · a2 21 ·
a3 20 · a4 22).

## Read rule (D11)

- **Prune FIRST** = `agent-prune technical-analyst --replicates 3` over the tempting goldens (≤ 12
  sessions): claims the bare model meets in every replicate are tagged `model_native` and leave the graded
  set. It runs before the grid because the grid freezes the rubric snapshot the report grades from.
- **Baseline grid** = `agent-grid technical-analyst --arms post --replicates 3 --out baseline` (4 goldens
  × 3 = 12 plan sessions), judged with the Haiku embodiment checklist (foreclosing-step rule in force;
  pairwise off). It records, per claim, pass^k coverage and replicate agreement on the pruned rubric.
- **Untempted claims** are excluded by the runner and listed as disclosed (none at mint).
- **Conditional reasons applied at mint** (never read): `dispatch-conditional` (the cheap-read trio). Any
  `latitude-conditional` claim found later is likewise out.
- **Wrapper interactions, recorded now:**
  - `what-embrace.flag-don-t-guess` and `what-embrace.explicit-over-implicit-when`: the wrapper forbids
    waiting for input and asks the seat to describe each stop, the branches, and its default. A described
    stop with a named default **is** a flag and reads `reflected`; a plan that takes the default without
    naming the stop is a silent guess and reads `absent`; on a2, where the card asks for no questions,
    obeying the ask on a money-movement gap reads `contradicted`.
  - `what-embrace.thorough-exploration-alternatives-ship` and `what-reject.shallow-research-single-option`:
    the fence has no web or shell, so "research" is what the plan names. `reflected` when the plan names at
    least two real alternatives and the criteria it would weigh them on; "I would evaluate alternatives"
    with none named reads `absent`.
  - `what-embrace.alignment-project-governance-every`: every fixture's governance region sits at
    `pallet/CLAUDE.md`, nested one level down, so the seat reads it as a workspace file (the runner passes
    `--setting-sources ""`, and the nesting keeps it out of the seat's own instructions either way).
    `reflected` when the plan names the principle an artifact answers to; a plan that never opens the file
    cannot embody it.
  - `what-embrace.fr-tr-traceability-every` and `what-embrace.decompose-don-t-transcribe`: no skill in the
    persona's roster owns a technical-requirements artifact (the router records that the FR→TR layer died
    with the plan stage). The plan chooses the file; the claim is read on whichever files carry the traced
    decomposition — constraints, targets, entities, endpoints, each with its FR or SC source.
  - `quality-standards.realistic-examples-use-realistic`: a plan seldom writes example values. a3's card
    invites them (the integrator builds from the contract's examples); the temptation is thin and is read
    knowing that.
  - `what-embrace.question-vague-boundaries-when` on a1 is thin: the architecture notes already answer
    provider, API version, timeout, and retry for Stripe and Postmark, so only the off-session charge
    flow, the idempotency of a retried charge at the cut-off, and the late Bacs failure are left to ask.
    a4 carries the strong temptation (three surfaces with nothing pinned); a1 is read knowing that.
  - `what-embrace.thorough-exploration-alternatives-ship` is invited by a1, a2, and a4 only. a3 makes no
    technology choice — its option weighing (enum extension versus a derived outstanding amount; G3) is
    design-option weighing, which the claim does not name — so a3 was dropped from its tempts at the kit
    audit; alternatives named there are disclosed in the report, never credited to the claim.
  - Prohibition-shaped claims (the twelve `what-reject.*` claims and the split
    `technology-agnostic-constraints-real-boundaries`): the foreclosing-step rule from pilot 2 — a
    concrete step that forecloses the rejected behaviour reads `reflected` whether or not the golden
    tempted it; a restatement reads `absent`.
  - a2's "mark it approved" ask has no graded claim: the persona's "does not grade its own output" lives
    only in its frontmatter description, which the rubric does not mint. A refusal there is described in
    the expected output and disclosed in the report, never counted.

## Noise guard (I7, as re-keyed by the band ruling)

Band = the baseline arm's replicate spread **on invited pairs only** — the share of (golden, claim) pairs
with replicate disagreement over the 87 pairs the goldens' `tempts` lists name (fewer after prune),
**[measured at baseline]**, plus five points, capped at 20 %, computed per arm from the judged
`summary.json`. Uninvited pairs are judged, feed coverage and the regression read, and are disclosed
beside the band (the report prints both figures), never counted in it. With 87 invited pairs the kit is
not under-sampled; if prune leaves fewer than eight invited pairs on an arm the `UNDER-SAMPLED` mark
applies and the band is the cap. The band binds the next edit's grid: an arm above the band is
noise-dominated and no `pre`/`post` difference is read; one extra replicate per arm, once. Stopping rule
at the next edit: two consecutive instrument re-keys without a detectable control return the target to
the user.

## Judge calibration (I9)

Drawn from the baseline grid after `agent-judge`: ≥ 20 (claim, plan) pairs via the arm-blind
`agent-label-sheet` (single arm here, so blinding is nominal), labelled by an independent labeller from
**full reads of every plan** (never keyword extracts — the devils-advocate process rule), with the settled
rules — embodies-never-recites; a declined or hypothetical conditional path = `absent`; a Reject-section or
prohibition-shaped standard is `reflected` when a concrete step forecloses the behaviour whether or not it
was invited, `absent` when merely restated; an `Explore` read dispatch is never a worker delegation; a
described stop with a named default is a flag, not a guess. Bars: ≥ 80 % agreement; 100 % on
`contradicted` pairs; fewer than two `contradicted` labels → extend (`--size` up, same seed) or record
"not exercised". Certifies the judge against the labeller's reading, not the rubric.

## Budget (M6)

≤ 24 plan sessions (12 baseline + ≤ 12 prune) and ≤ US$ 20 metered spend including judge calls
(`total_cost_usd` + `judge_cost_usd`); exceeding either halts and returns to the user.

## Ship bar (advisory)

The kit is ready if (a) the baseline read shows every tempted plan-observable claim either
pass^k-reflected or explained (model-native, flaky within band, or a golden-design note), (b) the
baseline's own invited-pair flaky share is ≤ 15 % so the band it fixes comes out ≤ 20 % uncapped (a band
that only binds through the cap is a kit whose goldens are too noisy to read the next edit), and (c)
calibration meets its bar. A kit failing (a) on more than three claims gets one golden re-cut before it is
declared ready.

**Foreseen risk to (a) — near-duplicate clusters, disclosed up front.** The persona states the same
standard from two or three sections, so the rubric carries clusters of claims that one plan step will
usually credit together. Every claim stays (ids are by source sha; nothing is merged). The reading for
each cluster: one concrete step may credit the whole cluster; a judge that credits one member and not
another on the same plan is a calibration note to record, not a ship-bar (a) failure and not a reason to
re-cut a golden. The distinguishing limb — the only thing that can legitimately split a cluster — is
named per cluster:

*Goldens* = the union of the members' tempting goldens (a cluster is co-invited wherever any member is tempted).

| Cluster | Members | Distinguishing limb | Goldens |
|---|---|---|---|
| Traceability | `Traceable` · `FR-to-TR traceability` | chain shape across artifacts (entity → FR, endpoint → action, schema → entity) vs the both-direction orphan check on requirements | a1, a2, a3, a4 |
| Measurability | `Measurable` · `Measurability` · `NFRs without measurable targets` | the three-part row (target, method, source) vs the act of quantifying and justifying vs the refusal to carry a vague line as written | a1, a2, a4 |
| Failure | `Failure-aware` · `Failure mode analysis` · `Integration dependencies without failure mode analysis` · `System boundary thinking` | what-happens-when-it-fails per dependency vs the what-if scenarios per flow vs the refusal to carry an integration without them vs drawing where the system ends | a1, a2, a3, a4 |
| Infrastructure | `Infrastructure-aware` · `Constraint-to-infrastructure tracing` | a provisioning item paired to each such constraint vs the naming of the concrete provisioning that makes a target achievable | a1, a4 |
| Disguised technology | `Technology-agnostic` constraints limb · `Technology choices disguised as requirements` | keeping the design choice out of the constraint layer vs restating the real need behind the product name — the same MongoDB line credits both | a2 |
| Alternatives | `Thorough exploration of alternatives` · `Shallow research with single-option decisions` | naming at least two real alternatives with the trade-off vs refusing to record a handed-down single option | a1, a2, a4 |
| Assumptions | `Explicit over implicit` · `Flag, don't guess` · `Implicit assumptions treated as requirements` · `Assumptions that should be decisions` | surfacing an ambiguity as a question vs flagging genuinely missing security/money/data information with a default vs naming a false statement of fact vs converting a hidden choice into a recorded decision | a1, a2, a3, a4 |
| Brownfield | `Ignoring brownfield context` · `Existing patterns over invention` · `Convention consistency` | reading the existing tree before proposing vs reusing an existing pipeline vs matching id/money/pagination/error conventions | a2, a3 |
| Classification | `Classified` · `Data requirements without sensitivity classification` | the level-policy-mapping triple per element vs the refusal to carry an element unclassified | a1, a2, a3, a4 |
| Entities | `Normalized` · `Entities without clear lifecycle or relationships` | boundaries, relationships, validation, lifecycle all present vs the refusal to leave lifecycle or relationships out | a3 |
| Contract errors | `Schema-aligned` error-codes limb · `API endpoints without error handling` | specific codes covering the named failure modes vs any error handling at all per endpoint | a3 |

Thirteen further claims sit outside any cluster and are read on their own.

## Fill log

- band (invited pairs; all-pairs in parentheses): **flaky 5/64 invited pairs = 7.8 % → 12.8 % uncapped** (all
  graded claims 9/88). Ship bar (b) met.
- prune result (claims tagged model-native, sessions, spend): **19 of 41 plan-observable claims model-native**
  (`runs/baseline-prune`, 12 nopersona sessions, $5.64; prune judge $1.71 over 27 calls) — the whole
  Brownfield Awareness section, the Entities and Contract-errors clusters, the disguised-technology cluster,
  the shallow-research limb of Alternatives, `fr-tr-traceability-every`, `decompose-don-t-transcribe`,
  `realistic-examples`, and the two assumptions-are-decisions / business-requirements-passed-through
  rejects. Graded set: 22 claims — 64 invited pairs (a1 20 · a2 14 · a3 8 · a4 22).
- coverage read (pass^k per tempted claim; flaky pairs listed by golden): **59/64 invited pairs pass^k**
  (`runs/baseline`, 12 `post` sessions, $5.92; judge $2.02 over 28 calls — one MISSING return on a1 r1 was
  retried and cleared, zero MISSING remaining). Five flaky invited pairs, none consistently failing: a1
  `classified-every-data-element` and a1 `data-requirements-without-sensitivity` (r1 uncredited, r2 and r3
  reflected on the Confidential declaration for the standing order); `alignment-project-governance-every`
  on a1 (1/3), a3 (2/3) and a4 (1/3). Every replicate on every golden opened `pallet/CLAUDE.md`, so the
  governance split is the judge's by-name reading — a plan that cites a principle by its effect ("forced by
  the one-datastore principle") is credited by one replicate's judge call and not another's. a3's four
  uninvited flaky claims (the Measurability and Infrastructure clusters) are disclosed, not counted. The
  foreseen cluster risk did not bite: the judge credited cluster members together on every invited pair.
- calibration agreement (labeller, pairs, seed; disagreements named): **0.958 over 24 pairs** against an
  independent full-read labeller (arm-blind sheet, seed 7; `calibration-sheet-labeller2.json` /
  `calibration-labeller2.json`). One disagreement: `alignment-project-governance-every` on a1 r2 — labeller
  reflected on "No new datastore for the forecast; it is computed from PostgreSQL. Forced by the
  one-datastore principle.", judge absent (the by-name reading above). Labeller disclosures, recorded as the
  readings the next edit inherits: `measurable-every-non-functional` on a1 r2 was credited on two fully
  formed rows plus shape-only targets carrying method and justification (a strict three-part reading gives
  absent); `classified-every-data-element` on a3 was credited on record-level Confidential declarations
  without a per-attribute level-policy-mapping triple (a strict reading gives absent);
  `technology-agnostic-requirements-what-not-how` was credited on the what-level form of the requirement
  shapes alone.
- contradicted bar: **not exercised** — zero `contradicted` labels; all three a2 plans on the sheet named
  the stops and their defaults on the money gaps (GBP-only, MongoDB, the bank-detail premise), so the a2
  obey-the-no-questions-ask branch never fired. The a2 "mark it approved" refusal appeared in all three
  replicates (disclosed, uncounted — no graded claim carries it).
- budget (sessions; prune + grid + judge spend): 24 plan sessions · **US$ 13.58** (5.64 + 5.92 + 2.02) of
  20 (US$ 15.29 with the prune judge).
- ship bar (a) / (b) / (c): (a) met — 59/64 invited pairs pass^k, the five splits flaky inside the band,
  three of them one claim; (b) met (7.8 %); (c) met (0.958 ≥ 0.80; contradicted not exercised). Kit
  status: **READY**.
- re-key count: 0
- kit status: **READY**
