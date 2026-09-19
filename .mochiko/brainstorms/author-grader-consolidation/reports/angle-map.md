---
report: review
phase: 0-blind-angle-map
skill: mochiko:review-brainstorm
seat: cold-reviewer (solo)
written: 2026-09-19
record_contact: none
angles: 56
classes: 8
---

## Notes of note

Built blind under `cold-review-gap-challenge` D2/D6/D7: from the topic statement and goal line
plus free repo grounding, before any contact with the session's record, index entry, or
directory. Ground read: the operating docs, the ledger's GI-004 and GI-012 blocks, both
`.claude/rules/mochiko/` files, the rendered rule sets of `patterns-sound-loop` and
`review-brainstorm`, `.mochiko/schema-views/`, and the records of `author-grader-value-tiering`,
`producer-plan-enforcement`, `orchestrator-model-selection`, and two adjacent review sessions.

An expected-coverage map, not a finding set. An angle becomes a coverage finding only after the
record is read, only where the record neither visits nor knowingly dismisses it, and only with a
materiality argument that a plausible ruling would differ had it been visited (D2 as amended).
An angle the record shows it saw and rejected is a ruling, not a gap.

## Class 1 — What independence is for (5)

- **A1 — The named catch mechanism.** What cognitive failure does a second seat catch that a
  self-check cannot? Anchoring on one's own frame, sunk-cost defence of a produced artifact,
  and self-consistency bias are three different mechanisms with three different cheap
  substitutes. A path forward built on "it catches things" cannot say which cheap form
  preserves the catch.
- **A2 — One mechanism or five.** The five binding sites grade different objects: a record's
  reasoning (cold review), a primitive's conformance to a criteria list (GI-004 audit), a
  plan's adequacy, a spec's gaps, code's shape. A conformance audit is mechanizable; a
  reasoning attack is not. Does the session distinguish them before ruling one floor over all
  five?
- **A3 — Independence vs adversariality.** "Not the author" and "hostile posture" are separate
  axes. `mochiko:validator` is an independent checklist grader; `devils-advocate` is an
  adversarial critic. A cheaper form might keep independence and drop adversariality, or the
  reverse. Which axis is the floor on?
- **A4 — Three claims under one name.** Independence is asserted as fresh context, different
  seat, and different skill (axis 5 says agent *and* skill). Which of the three carries the
  catch? `producer-plan-enforcement` D3 already splits them — same persona type, fresh spawn,
  different skill — so the three are known to be separable.
- **A5 — Non-catch purposes.** Independence also produces the audit trail GI-004's testability
  reads ("audit PASS on record"), the reconstructibility GI-006 demands, and the evidence the
  ratchet is measured against. A cheaper form that preserves catch yield but leaves less
  evidence still breaks two principles.

## Class 2 — Where the floor binds (6)

- **A6 — The tiering key itself.** If independence tiers, what is the key? Candidates: artifact
  durability, blast radius, reversibility, author confidence, surface kind, depth level. The
  driver says "value"; value is not yet a defined key anywhere in the repo.
- **A7 — Reversibility as the key.** `orchestrator-model-selection` D4 accepted a weaker
  evidence gate precisely because the change was one alias flip to revert. Reversibility is
  already a ruled discount elsewhere. Is it the key here, and does it survive the objection
  that a shipped plugin edit reaches consumers before anyone reverts it?
- **A8 — Two keys that do not obviously reconcile.** `patterns-sound-loop` fires on
  judgment-authored × governing surface. GI-004 fires on shipped-primitive edit × before the
  bump. The first is kind-keyed and library-wide; the second is ceremony-keyed and
  maintainer-side. They overlap on `plugins/mochiko/` and are disjoint elsewhere. Does the path
  forward unify the keys or leave both standing?
- **A9 — Sites where independence is absent, not excessive.** `adversarial-review-generality`
  inventoried five stages with no adversarial-critique seat: implement's built code, specify's
  Intent synthesis, the live brainstorm before convergence, feature-close, and the lead's own
  folds. A consolidation driven by cost may subtract only, leaving the recorded absences
  unaddressed.
- **A10 — The three exemptions as the existing cheap form.** Mechanical execution,
  transcription, and fix-on-sight repair already escape the floor. Is the cheaper form being
  sought a fourth exemption, a graded review, or a widening of these three? The distinction
  changes which floor rule is superseded.
- **A11 — Self-hosted vs shipped.** mochiko's primitives install into other people's projects.
  A defect in a shipped skill propagates to every consumer; a defect in this repo's operating
  docs does not. Does the floor distinguish them?

## Class 3 — The menu of cheaper independence (7)

- **A12 — Is the menu enumerated at all.** Candidate cheaper forms: fresh-context same-persona
  peer · a cheaper model tier for the grader · a narrower criteria list · batch grading (one
  seat, N artifacts) · sampling (grade k of n) · deterministic pre-assert absorbing the
  mechanizable half. A path forward that names one without pricing the others has not chosen.
- **A13 — Mechanical substitution, the largest available lever.** `mochiko-cli check` already
  grades mechanical conformance (path, file set, headings, frontmatter, placeholders, size)
  under the AM-3 admission. Several `primitive-edits.md` criteria — scaffold conformance,
  section-set match, ID continuity, floor survival, budget — are mechanically checkable in
  principle. Moving them into the crate leaves judgment grading only what judgment must. Does
  the session cost this, and does GI-019's bright line (never grade meaning) bound how far it
  goes?
- **A14 — Grader tier as the lever.** The cheapest independence is a cheap grader.
  `orchestrator-model-selection` D5/CG3 bars a grader from running below its producer's tier,
  and bars Haiku as any seat. That bound forecloses the cheapest form unless it is superseded.
  Is it in scope?
- **A15 — Batching.** The recorded waves cite per-cluster grader counts (13/13, 12/12, 8/8,
  4/4), implying one fresh seat per cluster. One seat grading N clusters cuts spawn cost
  N-fold. What does batching cost in independence — shared context across artifacts, or
  nothing?
- **A16 — Sampling.** Grading a sample rather than every edit is the sharpest cost cut and the
  most direct collision with GI-004's ratchet ("the baseline — all shipped primitives audited —
  MUST NOT decrease"). Is sampling considered and rejected, or not considered?
- **A17 — Cost and latency are different problems.** Parallel grader spawns cut latency and
  raise cost; batching cuts cost and raises latency; mechanization cuts both. The driver names
  both as primary. Does the path forward split them, or treat them as one quantity?
- **A18 — Could cheaper be better.** A narrower criteria list may raise precision.
  `adversarial-review-generality` AR-D1 rules the opposite worry — encoded angle lists act as a
  ceiling, so a narrower list lowers off-taxonomy recall. Both effects are real; does the
  session weigh them against each other?

## Class 4 — Evidence on both sides (7)

- **A19 — Yield per site, not per wave.** The recorded outcomes are wave-level audit results.
  The five sites have separate expected yields: a cold review of a reasoning record and a
  conformance audit of a skill pair are not the same instrument. One yield figure cannot tier
  five sites.
- **A20 — Is the cost side still unmeasured.** The stalled predecessor's F5 asserts the
  platform exposes no session-readable token totals, so cost evidence is seat counts and wall
  time only. `evals/run.py` records `cost_usd` per session, sums it, and enforces a budget
  ceiling with a client-side estimate. If a cost number is now obtainable, the driver can be
  quantified instead of asserted.
- **A21 — Severity mix, not finding count.** "Zero blocking findings" waves may still have
  produced advisory findings worth their price. Which of the recorded catches would have
  shipped as a defect, and which were cosmetic?
- **A22 — The counterfactual denominator.** Any recorded case of a defect that passed an audit
  and landed? Without escape data, the yield side has a numerator and no denominator, and the
  cheaper form cannot be sized against the risk it takes.
- **A23 — The evidence that cuts against the driver.** Reviewer overlap is low: in one pair run
  only 2 of 6 Important findings were found by both reviewers. That argues detection is lossy
  and independence is under-supplied, not over-supplied. A record that cites this fact only as
  "naive halving loses findings" has used half of it.
- **A24 — Measuring the yield is itself judgment work.** Labelling findings by severity to
  compute yield needs a calibration design; a keyword-extract labelling in this repo produced
  0.667 inter-labeller agreement. A measurement plan that skips calibration produces a number
  nobody can act on.
- **A25 — The parked benchmark.** `adversarial-review-generality` AR-D3/AR-D5 fully specified a
  seeded-defect benchmark — arms, replicates, seed mix, independent seeder, independent scorer,
  precision guard — and deliberately parked it as a BACKLOG item, with the decay risk recorded.
  It measures exactly whether encoded criteria ceiling the catch. Is it re-triggered,
  re-scoped, or formally killed here?

## Class 5 — Reconciling the standing rulings (10)

- **A26 — `producer-plan-enforcement` D3's wave gate.** D3 retires `validator` library-wide in
  a second wave, explicitly gated on wave-1 figures and on a named replacement carrier being in
  place. Wave 1 is unbuilt and open in BACKLOG. Does this session honor the gate, dissolve it,
  or supersede D3? Silence leaves two live rulings on the same retirement.
- **A27 — Axis 5 is already claimed.** D3 pre-wrote the axis-5 reword ("a fresh seat that
  authored nothing it grades, running a different skill from the author's") and the router's
  mount-doctrine line. If this session writes axis 5 differently, two records claim the same
  line and the landing has to say which wins.
- **A28 — The seat default key depends on independence.** `orchestrator-model-selection` D1
  tiers seats on one criterion: does a structurally independent seat stand between this seat's
  output and the run's verdict. Four producers were cheapened *because* independence stands
  behind them. Weakening independence retiers those four by implication. Is the dependency
  traced, or does the cost saving get double-counted?
- **A29 — The thin limb of that dependency.** D3's G7 fold concedes that for three of the four
  down-tiered seats the independent pass produces gap-finding input and no clearing verdict, so
  the standard rests on that pass plus the lead's gate plus the user's acceptance. If the
  independence behind cheap producers is already thin, cutting it further compounds.
- **A30 — Live bounds on any cheap-grader move.** CG3 (a grader never below its producer's
  tier) and CG6 (one tier per seat per loop) constrain the tier lever directly. Named or not
  named?
- **A31 — The prior retention ruling.** `architect-role-pushback-and-abstraction` D3
  (2026-08-13) explicitly retained author≠grader, tracing the additive bias it hunted to
  charter asymmetry rather than to independence. Any weakening supersedes it, by a recorded
  row.
- **A32 — What the stalled session carries forward.** `author-grader-value-tiering` has ground
  facts F1–F6 and exactly one trail answer: cost and latency/ceremony primary, yield not
  disputed. Superseding it should carry or re-ask that, not silently inherit it.
- **A33 — A cost-adding ruling on the same evidence.** `cold-review-gap-challenge` D5 has both
  reviewers in a pair build blind maps independently, justified by the low-overlap evidence.
  That is duplication bought deliberately. Is it in scope for the consolidation, or fenced out?
- **A34 — `review-brainstorm`'s own floors.** The skill carries nine `class: floor` rules
  including author-grader and never-in-the-room. Touching them is a floor supersession on a
  shipped primitive, with a migration file and a strip entry.
- **A35 — Four queued supersessions.** D3 already enumerates `validator-scope-and-verbosity`,
  `validator-worktree-isolation`, `author-grader-value-tiering`, and ADR
  `2026-08-26-validator-router-indexed-checklists` as owed supersession rows at its wave 2.
  Does this session take those rows, leave them, or split them?

## Class 6 — Governance mechanics (6)

- **A36 — The amend path is not the brainstorm's.** GI-004 is NON-NEGOTIABLE and floor-asserted
  (FLOOR-TEST). Changing it is a governance event through `/mochiko:setup`, not a decision row.
  Does the record state the path, and does it state the semver class against the precedents —
  AM-1 and AM-2 MAJOR (a principle redefined), AM-3 MINOR (an admission widened under unchanged
  text)?
- **A37 — Which limb of the ratchet is claimed.** "All shipped primitives audited MUST NOT
  decrease" has two readings: every edit graded (a cheaper grader preserves it) versus the
  audit's strength (a cheaper grader lowers it). The record has to pick, because sampling fails
  the first reading and tiering arguably fails the second.
- **A38 — Re-expression or amendment.** GI-004 has been re-expressed three times already
  (v0.76.0 for the crate, AM-2 for the schema-content unit, and the code extension) with the
  ratchet unchanged each time. Is this a fourth re-expression or a change to the principle? The
  answer sets MAJOR vs MINOR.
- **A39 — GI-006 reconstructibility.** Every primitive edit must be reconstructible from strips
  plus the log plus `DECISIONS.md` plus version stamps. Audit evidence is part of that trail. A
  cheaper audit that writes less has to say what the trail loses.
- **A40 — The release gate's text.** GI-012 lists "audits PASS" as a `plugin.json` bump
  precondition. A tiered floor changes what that phrase means at the gate. Is the gate text
  touched, and does the marketplace-sync gate care?
- **A41 — Whose ruling.** The record recommends; the user rules; `/mochiko:setup` lands the
  governance change. A record that lands a NON-NEGOTIABLE change by its own decision row has
  taken a reserved ruling.

## Class 7 — Build sequence and landing (6)

- **A42 — Sequence and its dependencies.** Which move is first, and does any move depend on the
  unbuilt `producer-plan-enforcement` wave 1? A sequence that assumes a built prerequisite is a
  plan with a hole in it.
- **A43 — Does the first wave grade itself.** `producer-plan-enforcement`'s own review caught
  exactly this: retiring the only generic default-FAIL grader before its replacement had graded
  once would make the retiring wave grade its own landing under an untested regime. A
  consolidation that cheapens grading and then grades its own landing with the cheapened form
  repeats it.
- **A44 — The landing set, complete.** `DECISIONS.md` rows · the stalled session's BACKLOG item
  moved to the trail · the producer-plan item amended or closed · `ROADMAP.md` Now/Next ·
  three-way status agreement across the brainstorms index, the record, and the decisions index
  · strips per touched primitive · `CHANGELOG.md` · `plugin.json` bump · `marketplace.json`
  synced.
- **A45 — Carriers, enumerated.** `CLAUDE.md` axis 5 and its three ceremony mentions ·
  `.claude/rules/mochiko/primitive-edits.md` (grader identity in both criteria blocks, plus
  criterion 6) · `.claude/rules/mochiko/rust-cli.md` (the code-review clause) ·
  `patterns-sound-loop`'s leg-2 rule · the `review-*` skills' independence sections · the
  ledger's GI-004 block · `.mochiko/memory/primitive-cost-budgets.md` · the router's
  validator-bearing lines. A named-but-incomplete carrier list leaves dead pointers.
- **A46 — Schema content edits are migrations.** Every rule-text change lands as a new migration
  file under `plugins/mochiko/migrations/`, validated by `mochiko-cli migrate validate`, with
  the ruling anchor where protected content moves; the derived views regenerate and are never
  hand-edited. Strip entries are owed for markdown primitives, not for schema content.
- **A47 — Superseding the stalled session properly.** Its record status, its index entry, and a
  `DECISIONS.md` row have to agree. A supersession that updates only the new record leaves a
  status contradiction that the grooming skill treats as a defect on sight.

## Class 8 — Failure modes of the path forward (9)

- **A48 — Under-classification drift.** The named hazard on the existing floor is calling
  judgment work mechanical to stay inline; the disclosure line exists to make that auditable
  visit by visit. A tier key adds a second cheap lane and therefore a second way to
  under-classify. What makes the tier choice auditable?
- **A49 — Who picks the tier.** If the author, or the lead who approved the plan, picks the
  grading tier, independence is self-granted at one remove. The existing floor puts the choice
  in the rule, not in the producer's hands.
- **A50 — Ratchet erosion over time.** A cheap default drifts as more surfaces claim it. Is
  there a re-tightening trigger, a recorded watch with observables, and a named revert?
  `orchestrator-model-selection` D4 shows the shape: two observables plus a revert trigger.
- **A51 — Reversal cost is asymmetric.** Cheapening a floor is a governance amend; restoring it
  is another. Unlike a frontmatter alias flip, this is not cheap to revert, which weakens the
  "ship and watch" argument that carried the tier ruling.
- **A52 — A new single point of failure.** If the crate absorbs the mechanizable half, a class
  of defect now rests entirely on a tool that GI-019 forbids from judging meaning. What falls
  in the gap between what `check` can assert and what the cheap grader still reads?
- **A53 — Correlated failure across seats.** Every seat here is the same model family. Two
  agent seats are independent in context and prompt, not in training or inductive bias — a
  blind spot shared by producer and grader is invisible to both. The low reviewer-overlap
  evidence suggests the correlation is not total, but the bound matters: agent independence is
  weaker than human independence, and a path forward that spends it down should say by how
  much.
- **A54 — Depth-keying and the breadth invariant.** The production floor carries a user-declared
  low/high depth level with breadth invariant at both and a one-way ratchet. Keying independence
  to depth would vary breadth, not depth. Is depth-keying considered and rejected on that
  ground, or not considered?
- **A55 — Whose cost is being cut.** The driver is the maintainer's spend and ceremony. Axis 5
  and the sound-loop floor also govern consumer runs of `/mochiko:implement` and the rest. A
  floor cheapened to fix maintainer cost changes what consumers get graded. Is the cut scoped
  to the maintainer-side ceremony, or does it reach the shipped floor?
- **A56 — Which cost, precisely.** Ceremony cost decomposes into the grader spawn, the audit
  brief the author writes, the fix rounds, and the landing paperwork. Waves with zero fix rounds
  still paid spawn and brief in full. Different components point at different fixes — brief
  templating, batching, mechanization — and a path forward aimed at the wrong component saves
  nothing.
