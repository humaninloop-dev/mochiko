# QA gap-finding verification — decision record

**Status:** accepted (2026-08-19)
**Opened:** 2026-08-19
**Provenance:** authored by the session lead (brainstorm run, 2026-08-19); decisions ruled by
the user; solo cold review by an independent reviewer seat via blind-map two-message dispatch
(32-angle map, topic-only spawn); 24 raised, 18 survived (2 Critical · 11 Important · 5
Minor), verdict critical-gaps; 18/18 dispositioned (C1 explored now → D9; I1 user-ruled
reseat → D4 amended; I8/I9 ruled inline; remainder one user-ruled batch "as recommended").
Verify round 1 NOT CLEAN — 1 blocking (B1 stale D2 mitigation) + 4 non-blocking (N1–N4),
all five lead-repaired same round → bounded round 2 CLEAN (fold fidelity 18/18, reopen-born
D9/D10 fitness-clean, no new contradictions).
**Topic:** the QA agent's verification lifecycle — is deterministic re-execution after the
staff engineer redundant, and what gap-finding approach (e.g. black-box testing) should the
QA seat run instead of or alongside it.

## Ground facts

- **F1 — The QA seat's four duties today.** `plugins/mochiko/agents/qa-engineer.md`
  (`skills: testing-end-user, review-code-minimalism`): (1) design-time authoring of
  acceptance test cases in the `**TEST:**` grammar (added v0.75.0,
  `vertical-tdd-complexity-and-qa-role` D2 — QA owns the cases, the design seat owns the
  slicing); (2) runtime execution of `**TEST:**` gates against real infrastructure
  (`mochiko:testing-end-user`); (3) quality-gate execution as deterministic exit-code checks
  (lint/build/test — "always auto-resolve", never judgment); (4) the per-cycle
  code-minimalism advisory audit (`mochiko:review-code-minimalism`).

- **F2 — The verification hierarchy in an implement run.** From
  `plugins/mochiko/commands/implement.md`: staff-engineer builds each cycle test-first
  (red/green/refactor — unit tests are the builder's, per
  `vertical-tdd-complexity-and-qa-role` D2) → per-cycle verification by a seat that is never
  the implementer: the card's `**TEST:**` gate against real infrastructure + quality gates +
  the minimalism lens, ending in a human checkpoint → one whole-implementation **final
  validation** from a dependency-cold snapshot: full-repo quality gates, the accumulated
  `**TEST:**` gates of previously delivered features in the territory, seam exercises against
  real delivered sides, built-vs-approved architecture diff.

- **F3 — Everything the QA seat executes is pre-declared.** The `**TEST:**` cases are
  authored at design time (by the QA craft itself, since v0.75.0) and sit on the cycle card
  the staff engineer builds against; the staff engineer sees every assert before writing
  code. Quality gates are exit codes. Classification (CLI/GUI/SUBJECTIVE) decides
  auto-approve vs human checkpoint, not what gets probed. **No seat in the pipeline runs
  exploratory, adversarial, or black-box testing against the built thing** — no
  edge-case probing beyond the declared asserts, no negative testing beyond what the card
  happened to declare, no "what did the builder not think of" hunt.

- **F4 — What the re-execution does buy today.** Independence (verification seat is never
  the implementer; evidence captured, never trusted from the builder's claim), real
  infrastructure (builder may have run against local state; the TEST gate runs
  Setup/Action/Assert fresh), regression accumulation (territory gates + seams at final
  validation), and the human checkpoint audit trail.

## Decisions

- **D1 — Deterministic layer stays; exploration is added on top.** `Confident`
  The pre-declared `**TEST:**` gate re-execution and exit-code quality gates keep their
  place as the regression/independence floor (F4's four buys are real); the QA seat gains a
  new exploratory gap-finding duty rather than trading re-execution away for it. Accepted
  cost: the QA cycle gets heavier.
  *Rationale:* re-execution's value is independence + fresh-infra + regression accumulation,
  not discovery; the diagnosed hole is the missing discovery layer, so the fix is additive,
  not substitutive.
  *Rejected roads (I11 repair):* **shift QA time to exploration** lost because demoting TEST
  gates to un-ceremonied exit-code runs weakens the author≠grader independence on the one
  layer that carries it; **replace re-execution** lost because trusting the builder's green
  run surrenders fresh-infra evidence and the "inferred outcomes" rejection the QA persona
  is built on.

- **D2 — Exploration runs at final validation only** *(as amended at review: C2 scope
  carve · I8 audit disposition · M1 epic form)*. `Confident`
  One deep gap-finding pass over the whole built feature at the final-validation stage —
  full surface visible, seams live against real delivered sides, one cost per run. Per-cycle
  verification stays deterministic-only. Accepted risk: gaps surface late (rework cost);
  mitigated by the pass's own gap-rework bound at final validation (D6's I3 amendment —
  whole-run bound, default 2 rounds; the cycle attempt economy does not reach this stage).
  **C2 amendment — run scope:** the pass runs on **selection-scope and epic runs only** —
  runs that carry a spec layer. Delta-scope and product-lane runs (whose sole expectation
  source is the card, which the fence forbids) **skip the pass, and the final-validation
  report states the skip explicitly** — never a silent no-op.
  **I8 amendment — audit-track disposition:** the pass homes in implement's final validation
  now; if the deferred `audit` workflow (BACKLOG, 2026-07-02 charter) is ever scoped to own
  feature-close verification, the pass **migrates there** — same precedent as the
  cold-checkout step's ruled migration clause. The BACKLOG item gains an annotation at
  landing; the Cluster-2 journey-gate class stays with the audit track, unabsorbed here.
  **M1 amendment — epic form:** over an epic, the pass runs **once, over the union of member
  territories**, at the epic's single final validation.
  *Rejected roads (I11 repair):* **per-cycle** lost because a slice-scoped surface yields
  shallow probes at N× cost — re-adding the per-cycle fixed overhead the v0.75.0 session
  just cut; **both-weighted** (light per-cycle sweep + deep final pass) lost because it is
  the heaviest run shape and its early-catch benefit is bounded — the deterministic per-cycle
  gate already catches declared-behavior breaks early, so the added light sweep buys little
  discovery for its N× spawn cost. The late-gap risk it would have mitigated is accepted on
  D2's face.

- **D3 — The explorer is blind to code and cards: spec-derived expectations first** *(as
  amended at review: I2 explicit fence list)*. `Confident`
  The gap-finding pass is true black-box behind a structural fence: the explorer derives its
  own expected behaviors (including negative, abuse, and edge expectations) *before*
  touching the running system, then probes against real infrastructure. Same anchoring-fence
  shape as the brainstorm cold-review blind-map dispatch: expectations built before sight of
  the artifact, enforced by dispatch order, not trust. Finds what the builder AND the test
  author both missed — the diff between spec-implied behavior and declared coverage is
  exactly the hunt surface.
  **I2 amendment — the fence is an explicit inclusion list, not a layer label.** The
  explorer's admissible inputs: `spec.md` (FR-XXX / SC-XXX / stories / edge cases), the
  feature's `requirements.md`, Screens & Flows (SCR-XXX / FLOW-XXX), **and the product
  baselines** `data-model.md` (entities, state machines, DS-XXX sensitivity), `contracts/`
  (`api.yaml`), and `nfrs.md`. All of these define externally-observable promised behavior —
  the pass stays black-box. **Excluded, structurally:** the code, the cycle cards
  (`tasks.md`), the `**TEST:**` cases, cycle reports, and the builder's tests.
  *Rejected roads (I11 repair):* **blind-to-code-only** lost because sight of the declared
  `**TEST:**` cases anchors the hunt on existing coverage — the explorer probes around what
  is already asserted instead of deriving independently; **full sight** lost because
  grey-box targeting maximizes the anchoring risk the fence exists to kill — it tends to
  confirm what exists rather than find what is absent.

- **D4 — Explorer seat: devils-advocate persona under two-message dispatch** *(superseding
  amendment at review, user-ruled at I1 — original fresh-qa-engineer-spawn ruling
  superseded)*. `Confident`
  The exploratory seat is the **`mochiko:devils-advocate` persona** — the adversarial
  gap-hunt craft — dispatched fresh under the two-message contract: first message carries
  the D3 inclusion-list references only (never card/code/TEST paths); the seat states its
  derived expectations; only then does probing begin. The new D8 skill carries the runtime
  probing procedure the persona lacks (persona carries judgment, skill carries procedure —
  five-axis #4). QA-engineer keeps every deterministic duty (F1) and the D7 fold-back
  authoring; it no longer sits the exploratory seat.
  *Why the original ruling died (I1):* a fresh qa-engineer spawn is blind to the design-time
  cases but re-derives expectations with the same persona from the same spec that authored
  them — correlated omissions are exactly the target class, and dispatch blindness cannot
  reach them. The cited blind-map precedent pairs blindness with seat independence; it never
  substitutes one for the other. This also dispositions the open BACKLOG item "`qa-engineer`
  audit affinity": the produce+grade leak is resolved by reseating, not by justification.
  *Rejected roads (I11 repair):* **fresh qa-engineer spawn** — died at review as above;
  **new dedicated explorer agent** lost because devils-advocate already carries the
  adversarial persona and review craft — a new persona would duplicate it for maintenance
  cost with no independence gain.
  *Mark note (N4):* the mark is on the seating ruling, made with the argument in full view;
  the seat's runtime-probing fit is unmeasured (Evidence honesty) and rides the
  first-live-run watch — the ruling's confidence does not extend to the fit.

- **D5 — Gap-finding lenses** *(as amended at review: I6 mutation pricing · I9 depth
  keying · D9 non-functional widening)*.
  (a) **Spec-derived probe kit** `Confident` — the blind explorer's charter includes, beyond
  happy-path exploration: adversarial inputs (invalid/boundary/malformed/misuse), illegal
  state-transition walks (from `data-model.md` state machines), contract probes (from
  `contracts/api.yaml` — wrong types, missing fields, status-code and pagination edges),
  concurrency/idempotency probes (parallel submits, replays), **and the D9 non-functional
  probes** (security/abuse, runtime NFR, observability). All derivable from the D3 inclusion
  list, all inside the fence — one seat, one charter. Runs at **both depth levels** (breadth
  invariant).
  (b) **Mutation testing** `Assumed` *(mark lowered at review, I11 — noise level unmeasured,
  n=0)* — a separate grey-box lens: a stack-appropriate mutation tool (cargo-mutants /
  mutmut / Stryker class) mutates the built code and runs the staff engineer's suite;
  surviving mutants are measured holes in the suite. Runs alongside, never inside, the blind
  explorer (it requires code sight); sits on the existing verification seat, which already
  holds code sight. The most direct instrument for "find gaps in the staff engineer's work".
  **I6 pricing:** mutants scoped to **this feature's diff only** (changed-code mutation,
  cargo-mutants `--in-diff` class), timeboxed; a flaky suite detected during the run skips
  the lens with the skip noted. **I9 keying: high depth only.** Tool absent for the stack =
  lens skipped and noted (D8) — expected routinely on mobile/desktop shelves (M3), where
  mutation tooling is sparse; the skill's lens declaration carries that note.
  *Declined:* property-based testing (writes durable harness code — heavier than the pass
  warrants now) and metamorphic relations (niche until oracle-less features exist). Both
  open threads.

- **D6 — Findings split by kind: spec-violation blocks, beyond-spec advises** *(as amended
  at review: I3 run bound · I4 adjudicator · M5 routing)*. `Confident`
  A finding demonstrating spec-required behavior broken (evidence captured, spec clause
  cited) is blocking — final validation fails. A beyond-spec finding — robustness gap,
  undeclared edge behavior, surviving mutant, observability hole — is advisory to the final
  checkpoint: the user rules fix-now / backlog booking / accept. Consistent with standing
  doctrine: deterministic spec conformance is a gate, judgment stays with the user, gates
  are never severity-triaged (the split is by *kind*, not severity). A broken `nfrs.md`
  numeric target is spec-violation kind (D9).
  **I4 amendment — adjudicator:** the finder proposes the kind; the **lead confirms the
  blocking classification at the checkpoint verdict** against the cited spec clause; a
  disputed kind **defaults advisory** and the dispute is presented to the user, who rules.
  The finder never gates alone.
  **I3 amendment — attempt accounting at final validation:** gap-rework carries a
  **whole-run bound, default 2 rounds, redeclarable only at run open** (the attempt economy
  reaches cycles only; this is its final-validation analogue). A finding that localizes to
  one cycle's territory charges that cycle's remaining attempts instead. Bound exhaustion or
  a no-progress round → halt, disposition **reserved to the user** (mirrors the epic
  carve-out reservation).
  **M5 amendment — out-of-territory routing:** a gap surfaced in a previously delivered
  feature's territory (via accumulated gates or seams) is not this run's rework — it routes
  to a **`/mochiko:feature` delta card**, cited in the report.

- **D7 — Confirmed gaps fold back as durable `**TEST:**` cases** *(as amended at review:
  I5 target artifact)*. `Confident`
  Every gap the user rules fix-now or backlog is authored — QA craft, the grammar it already
  owns — as a `**TEST:**` case, so it rides the territory accumulation at every later final
  validation. Discovery becomes permanent regression armor; the deterministic layer D1 kept
  is exactly what makes the fold worth it. Findings the user accepts as as-designed do not
  fold.
  **I5 amendment — the fold target exists as a new artifact:** a feature's durable gate set
  lives at **`.mochiko/features/FEAT-XXX/gates.md`** — minted at first fold (or at plan time
  when cards are authored), surviving graduation (rows vanish; `gates.md` persists), and
  **named as the read source of the "accumulated territory `**TEST:**` gates"** implement.md
  already references without a home. Design-time cases stay on the cards; the accumulated
  read at final validation takes the union of territory features' `gates.md` + their cards'
  cases. Minting this artifact and re-pointing the accumulated-gates references is the
  **build's first task**, not an implementation detail.

- **D8 — Carrier: one new model-invoked skill `testing-gap-finding`** *(as amended at
  review: I10 done condition · M2/M3/M4 build notes)*. `Confident`
  The skill owns the whole pass: the D3 fence inclusion list and two-message dispatch
  contract, expectation derivation, the D5a probe kit (incl. D9 probes), the D5b mutation
  lens (its own section — pricing, depth key, tool-absent and flaky-suite skip clauses,
  mobile/desktop sparsity note), the D6 finding-kind split + adjudication, and the D7
  fold-back procedure into `gates.md`.
  **I10 amendment — done condition:** the pass is complete when **every derived expectation
  has been probed or explicitly marked unprobeable (with reason), within the charter's
  timebox**; the report discloses the expectation count, probed count, and findings.
  **Zero findings = a clean pass** — no never-zero rule; the disclosure is the honesty
  mechanism.
  Touch set: `devils-advocate` gains the skill + an exploratory-runtime persona line
  (D4); `qa-engineer` persona notes the fold-back duty (M2: its `description:` has 75 budget
  chars of headroom — stay inside or carry a justified overage per the D7 pre-assert);
  `implement.md` final-validation wiring (dispatch, scope carve, bound, adjudication,
  fold-back at acceptance landing) — every dispatched seat brief carries the model-tiering
  routing rule per `mochiko:patterns-model-tiering` (M4); router row. `testing-end-user`
  stays purely the deterministic runtime.
  *Rejected roads:* extending `testing-end-user` (muddies a deterministic-execution skill,
  fence reads optional), two skills (heavier than two lenses warrant).

- **D9 — Non-functional families: security/abuse IN, runtime NFR IN, observability IN,
  accessibility declined** *(born from the C1 coverage reopen, user-ruled)*.
  - **Security/abuse probing** `Confident` — joins the D5a probe kit: authz bypass
    (cross-user resource reach), privilege escalation, injection-class inputs,
    session/replay misuse — derived from spec roles and `data-model.md` DS-XXX sensitivity
    classes (Confidential/Restricted attributes name what must not leak). Both depths.
    Spec-violation kind when a DS-XXX class or declared authz rule is demonstrably breached.
  - **Runtime NFR verification** `Confident` — `nfrs.md` numeric targets (p95, availability,
    limits) measured against the built system at final validation. **Discharges the parked
    BACKLOG rider** ("runtime NFR verification … joins the feature-close verification scope
    when audit is scoped") into this pass — the rider's home arrived; the BACKLOG annotation
    lands with I8's. A broken target is spec-violation kind (blocking).
  - **Observability probing** `Confident` *(user-chosen beyond the lead's recommendation)* —
    key flows leave logs/metrics, error paths produce actionable diagnostics.
    **Advisory-only findings, always** — no spec clause to cite, by construction.
  - **Accessibility probing — declined** (user ruling; not selected). The a11y floor line in
    the production catalog stays a build-time standard, unverified by this pass. Open
    thread: re-entry when a UX-bearing feature's acceptance demands runtime a11y evidence.

- **D10 — Mutation tooling is not kernel-class (GI-019 recorded ruling)** *(born from the
  I7 disposition)*. `Confident`
  A stack-appropriate mutation tool is an **advisory post-hoc checker used as an optional
  exit-code signal** — CLAUDE.md's explicit non-kernel carve: it never gates pipeline
  progress (D6 keeps survivors advisory), never dispatches agents, never holds skill-owned
  judgment; absent = skipped and noted (GI-020 additive install untouched — no install-time
  dependency). The **deepeval / eval-harness BACKLOG item stays open**, annotated: mutation
  adoption here is not the eval-harness brainstorm that item reserves.

## Evidence honesty

n=0 — no live run has executed the gap-finding pass; the probe kit's yield, the mutation
lens's noise level, the devils-advocate seat's fit for runtime probing, and the blindness
fence's practical hold are all unmeasured. First-live-run watch owed at build landing.

## Build surface

1. **First task (D7/I5):** mint `.mochiko/features/FEAT-XXX/gates.md` as the durable
   per-feature gate-set artifact; re-point implement.md's "accumulated territory
   `**TEST:**` gates" references to it (supersession strips where clauses change).
2. New skill `plugins/mochiko/skills/testing-gap-finding/SKILL.md` (D8; fence list,
   two-message contract, probe kit incl. D9, mutation lens priced + depth-keyed + skip
   clauses, finding kinds + adjudication, done condition, fold-back).
3. `plugins/mochiko/agents/devils-advocate.md` — `skills:` line + exploratory-runtime
   persona line (D4); its `description:` sits at 316/395 budget chars (79 headroom) — the
   D7 char-budget pre-assert applies to this touch (N3).
4. `plugins/mochiko/agents/qa-engineer.md` — fold-back duty note; `description:` inside 75
   spare budget chars or justified overage (M2).
5. `plugins/mochiko/commands/implement.md` — final-validation wiring: blind dispatch (D3/D4
   contract), scope carve + skip disclosure (C2), gap-rework bound (I3), adjudication (I4),
   epic union form (M1), out-of-territory routing (M5), fold-back at acceptance landing
   (D7), model-tiering line in seat briefs (M4).
6. Router row in `plugins/mochiko/skills/mochiko/SKILL.md`.
7. BACKLOG annotations at landing: `audit` workflow item (I8 migration clause) · NFR rider
   discharged into the pass (D9) · deepeval item annotated-still-open (D10) · qa-engineer
   audit-affinity item dispositioned by D4 reseat · first-live-run watch added.
8. Supersessions: any removed/changed clause takes the strip ceremony; the rest additive.

## Review + disposition trail

Solo cold review, blind-map two-message dispatch (32-angle map, 7 clusters, topic-only
spawn; reviewer source-verified its repo claims). **24 raised, 18 survived — 2 Critical,
11 Important, 5 Minor; verdict critical-gaps.** Dispositions (18/18):

- **C1** (coverage — functional-only menu) → **explored now**, user-ruled → **D9** (three
  families in, accessibility declined).
- **C2** (delta/lane runs have no spec) → D2 scope carve, batch "as recommended".
- **I1** (correlated blind spots on the qa-engineer seat) → **user-ruled reseat** →
  D4 superseding amendment (devils-advocate); audit-affinity BACKLOG item dispositioned.
- **I2** (fence vs probe-kit input contradiction) → D3 explicit inclusion list.
- **I3** (no attempt accounting at final validation) → D6 whole-run bound.
- **I4** (finder sets the gate boundary) → D6 adjudication clause.
- **I5** (fold-back had no target artifact) → D7 `gates.md` mint, build's first task.
- **I6** (mutation lens unpriced) → D5b diff-scope + timebox + flaky-skip.
- **I7** (external-harness adoption unruled) → **D10** GI-019 ruling; deepeval item
  annotated.
- **I8** (coverage — audit-track collision) → ruled inline: home-now-migrate-later clause
  on D2; journey-gate class stays with audit.
- **I9** (coverage — adaptive depth unvisited) → ruled inline: probe kit both depths,
  mutation high-only (D5).
- **I10** (no done condition) → D8 done-condition clause.
- **I11** (record fitness: rejected roads unargued, provenance absent, marks) → why-lost
  lines added to D1–D4; provenance header added; D5b mark lowered to `Assumed`; D4's
  original mark superseded with the ruling itself.
- **M1** epic union (D2) · **M2** qa-engineer budget note (build surface) · **M3**
  mobile/desktop tool-sparsity note (D5b/D8) · **M4** model-tiering line (D8/build) ·
  **M5** out-of-territory routing (D6). All batch "as recommended".

Reviewer kill list (6, retrievable): transport floor already triggered · flake folded into
I6 · zero-findings folded into I10 · done-condition wiring covered by build surface ·
plan.md not a consumer · D8 description-cap speculation.

## Session trail

- **Q1 — fate of the deterministic layer:** keep + add exploration (recommended option
  accepted) over shift-time / replace. → D1.
- **Q2 — placement:** final validation only, over per-cycle / both-weighted. → D2.
- **Q3 — blindness:** blind to code + cards (spec-only inputs), over blind-to-code-only /
  full sight. → D3 (fence widened at review by the I2 amendment: product baselines
  `data-model.md` / `contracts/` / `nfrs.md` admitted; no longer spec-only).
- **Q4 — seat:** fresh qa-engineer spawn under two-message dispatch, over new dedicated
  agent / devils-advocate reuse. → D4 (superseded at review by the I1 reseat ruling).
- **Q4a — user widening (mid-turn):** "don't limit to black box — propose others."
  Seven families presented (adversarial, mutation, property-based, metamorphic,
  model-based state walks, contract probing, concurrency probing). → Q5.
- **Q5 — technique selection (multi):** spec-derived probe kit + mutation testing ruled
  in; property-based + metamorphic declined as open threads. → D5.
- **Q6 — findings consequence:** split by kind — spec-violation blocks, beyond-spec
  advises. → D6.
- **Q7 — ratchet:** confirmed gaps fold back as durable `**TEST:**` cases on fix/backlog
  ruling. → D7.
- **Q8 — carrier:** one new skill `testing-gap-finding` + wiring touches. → D8.
- **Q9 — review sizing:** solo cold review chosen (recommended) over pair / waiver.
- **Q10 — disposition round 1:** C1 explore-now · I8 inline · I9 inline · I1 reseat to
  devils-advocate (all recommended options accepted).
- **Q11 — C1 menu (multi):** security/abuse + runtime NFR (recommended) + observability
  (user-added) in; accessibility declined. Batch (C2, I2–I7, I10, I11, M1–M5) applied as
  recommended. → D9, D10, amendments.

## Open threads

- Property-based testing re-entry when a feature carries hammerable invariants worth a
  durable harness (declined at D5).
- Metamorphic relations re-entry when an oracle-less feature ships (declined at D5).
- Accessibility probing re-entry when a UX-bearing feature's acceptance demands runtime
  a11y evidence (declined at D9).
