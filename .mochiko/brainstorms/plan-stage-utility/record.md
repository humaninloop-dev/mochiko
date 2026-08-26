# Plan-Stage Utility — Decision Record

**Status:** accepted (2026-08-26 — pair cold review, verify CLEAN at round 3, user accepted whole)
**Opened:** 2026-08-26
**Provenance:** produced by the session lead (Claude, `/mochiko:brainstorm` run of 2026-08-26)
in dialogue with the user, who ruled every decision; pair cold review by two independent
seats (decision-quality + record-integrity lenses), blind-map two-message dispatch.
**Topic:** Why is `/mochiko:plan` needed as a distinct stage? If specify and architecture
exist, why can't the pipeline go straight to implement? The aim is not to deprecate planning
but to understand its utility deeply and, where sensible, shift its process into other
workflows. The goal condition: a way to assess whether specify + the architecture store
provide enough guidance for the implement team to build.

---

## Ground facts

- **F1 — pipeline shape today.** `specify` produces the spec workspace (Intent, FR-XXX/SC-XXX,
  Screens & Flows manifest + prototype, stories, Feature Selection, map delta). `plan` turns one
  capability-batch into an implementation package at `.mochiko/features/FEAT-XXX/`:
  `requirements.md` (FR→TR analysis), design deltas (`data-model.md`, `contracts/`), the signed
  store delta where structural, `tasks.md` cycle cards, `plan.md` summary. `implement` gates on
  the accepted package — "the batch's `tasks.md` complete alongside its `plan.md`, and its
  signed store delta … missing or incomplete → block, point to `/mochiko:plan`"
  (`implement.md` Entry).
- **F2 — plan is already risk-scaled, not fixed-weight.** The plan-the-plan proposal
  (rung-justified per `mochiko:patterns-plan-minimalism`, contested by an independent seat,
  user-approved) sets the artifact list; the approved list is the run's done condition. Delta
  scope collapses the run to confirming a delta card. This is the `plan-structure-yagni`
  (2026-08-12) re-charter, built at v0.67.0.
- **F3 — plan carries responsibilities beyond artifact authoring.** Unconditional architecture
  store consult + trip check + no-delta claim; store-delta authoring with user sign-off
  (architecture-before-detail floor); baseline seeding (including reconstruct-and-confirm with
  the user on delivered code); map-entry hardening + design-implied dependency assertions; QA
  design-time test-case authoring in the `**TEST:**` grammar; independent feasibility +
  completeness grades; in-flight-feature conflict routing; delivered-feature `[MODIFY]`
  amendments; store-trip dispositions reserved to the user; the `quickstart.md` null-path
  record; the epic joint proposal + inline mint door.
- **F4 — implement's design inputs** *(corrected at review: the original text misstated the
  gap-finding fence as "spec + baselines only")*. Builders decompose cycle cards at build time
  (decomposition disclosed in cycle reports, never pre-written); design inputs are the
  feature's `plan.md`, signed store delta, other deltas, `requirements.md`, product baselines
  (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, the store with NFR targets),
  and `spec.md` for the cards' cited acceptance criteria. The gap-finding seat's admissible
  inputs are an explicit inclusion list (`mochiko:testing-gap-finding`): `spec.md`, the
  feature's `requirements.md`, Screens & Flows, `data-model.md`, `contracts/`, and the store's
  concern rows — never the code, `tasks.md`, `**TEST:**` cases, or cycle reports.
- **F5 — prior sessions already thinned plan once.** `plan-structure-yagni` found kinako
  FEAT-002 produced ~3,057 lines (12 conceptual entities — `DECISIONS.md` row, 2026-08-12) for
  a launch-check feature; the fix demoted artifacts to risk-scaled guidance behind the
  proposal gate at v0.67.0. **That fix's own success probe (FEAT-002 re-derivation) has never
  run** — the evidence base for this session's redundancy driver predates the last treatment.
  The architecture store (`product-architecture-schema`, 2026-08-19, built v0.81.0) later
  moved per-feature `architecture.md` out of plan into the standing store.

## Decisions

- **D1 — Retire `/mochiko:plan` as a command; implement absorbs a conditional design phase
  behind a sufficiency check.** `Contested` (user-ruled at Q5; challenged by review survivor
  S1; re-explored at Q10 with the retain-and-collapse steelman and the full rehoming ledger
  in view; the user ruled retirement again, against the lead's recommendation of
  retain-and-collapse. The steelman: keep `/mochiko:plan`, run the sufficiency check at plan
  entry, collapse the run to card authoring + confirm on a zero-gap verdict — same
  instrument, same cheap pass path, near-empty orphan ledger, reversible. The user valued the
  single downstream run over the rehoming ledger; that ledger was accepted as obligatory
  build scope and dispositioned finding by finding at the review.)
  Implement becomes the single downstream run: (1) entry runs a sufficiency check,
  independently graded; (2) a failed check triggers an in-run design phase scoped to exactly
  the named gaps — design seats author (persona staffing the lead's call: technical-analyst,
  principal-architect, qa-engineer for `**TEST:**` cases; staff-engineer stays the builder),
  a non-author seat grades, and the user signs at a blocking checkpoint (store delta
  especially) before any cycle starts; (3) cycle cards + TEST cases are authored after the
  design phase (or directly on a zero-gap verdict), then build proceeds.
  **Carry-overs, non-negotiable:** architecture-before-detail (failed check ⇒ no code before
  user-signed design) · sound-loop floor on the design phase · adopt-first re-homed to the
  design phase and build-time decomposition (gated per mechanics d) · plan-minimalism ladder
  governs what the design phase authors · epic joint design lands in the epic implement run's
  design phase (always fired — mechanics b).
  **Mechanics (review folds):**
  (a) *Mid-run structural discovery re-fire* — a builder hitting undesigned structure
  mid-cycle halts the cycle; the design phase re-fires scoped to the discovery; the
  architecture-deviation gate anchors to the signed delta once one exists; the built-vs-signed
  diff trigger fires on any delta signed this run, whenever signed.
  (b) *Epics* — an epic run always fires the design phase for the joint spine: cross-member
  seam owners named at design time (no later-lander default inside an epic), the
  shared-baseline single-pen-holder rule carried; the epic's "joint plan-the-plan proposal"
  spine artifact re-keys to the joint design-phase plan.
  (c) *Card authorship* — cards are authored by a technical-analyst-class design seat, never
  the builder who will execute them; QA authors the `**TEST:**` cases (unchanged); the
  verification seat (never an author) reviews the cards — its grade covers buildability, and
  an infeasible judgment escalates to the user as a business-level scope decision.
  (d) *Build-time technical decisions* — build-time D-XXX / C-XXX / IP-XXX writes are never
  in-place edits: they are authored as `baseline-delta.md` entries; their judgment content is
  graded *as judgment* by the landing verification seat — an independent non-author grade
  before the user's acceptance, the As-built/Drift pattern — while the three-way diff remains
  the transcription check of faithful application only (verify V4). A commodity-category adopt-first
  ruling or an IP-XXX provisioning call is never builder-decided: it halts the cycle to the
  user's checkpoint, where `mochiko:patterns-adopt-first`'s constraint-challenge mechanism
  keeps its firing site.
  (e) *Map-entry hardening* (verify V1, R1) — the design phase, when it runs, asserts the
  design-implied dependency relations and sharpened extent onto the feature's map entry with
  provenance, and fills the entry's architecture link when a store delta is produced (the
  entry-side SPN-XXX/AX-XXX pointers, keeping the schema's both-directions navigability;
  n/a on the zero-gap path, where no delta exists); on the zero-gap path the card-authoring
  seat performs the dependency/extent assertion at card authoring; intended-vs-designed
  drift surfaces to the user at the design checkpoint or the card confirm.
  **Named cost:** a separate plan run gave design full user attention plus a natural pause
  before build; as a mid-run checkpoint there is rubber-stamp risk and long runs. Mitigation:
  the checkpoint is plain blocking text, and the user may stop after sign-off and resume the
  build later.
  **Rationale:** the drivers (Q1) are redundancy and sufficiency anxiety, not stage-hood; a
  command kept alive solely as a fallback design stop is ceremony around seats that any run
  can dispatch. Planning shifts into implement; it is not deprecated. The retain-and-collapse
  alternative was explored and declined by the user with its costs in view (Q10).

- **D2 — The sufficiency check: per-row, independently graded, ten clauses.** `Assumed`
  (content user-confirmed at Q6 and amended at the review disposition batch — sources
  widened, clause 5 hardened, clauses 8–10 added, verdict ownership stated; re-marked from
  `Confident` at verify V5: the instrument has never been executed, so its calibration is
  assumed until first live evidence — the D5 falsifier's all-fail arm is a D2 mechanism
  risk).
  Graded per selected work row (per-card under delta scope — D6), **from the spec, the
  architecture store, and the product baselines** (`data-model.md`, `contracts/`,
  `constraints-and-decisions.md`), by a seat that authored none of them. A row is *sufficient*
  when all hold:
  (1) testable criteria — every acceptance scenario/SC has a stateable oracle; no stateable
  oracle = gap;
  (2) contract exposure — touched API surfaces named, each existing in baseline `contracts/`
  or flagged new (gap);
  (3) data exposure — touched entities named, existing in baseline `data-model.md` or
  new/changed (gap);
  (4) structural trigger — store consulted, trip check run, no-delta claim recordable, else
  delta needed (gap);
  (5) NFR targets — applicable store concern rows identified *and their targets stated*; a
  feature plausibly bearing NFR load (user-facing latency, data volume, auth surface,
  availability) with no identifiable concern row = gap;
  (6) commodity exposure — any storage/queueing/caching/auth/search/serialization need named
  and adopt-first answerable (unresolved = gap);
  (7) dependency order — in-batch row dependencies resolvable; unresolvable = gap;
  (8) UX trace (where the spec carries a Screens & Flows manifest — D7) — every FEAT-tagged
  SCR-XXX's data shown has a nameable serving contract surface and every FLOW-XXX action a
  mutation path (existing = cited, new = gap);
  (9) delivered-feature exposure (**selection scope only** — verify V3) — a touched surface
  owned by a delivered feature is never zero-gap: it auto-fires the design phase, and the
  `[MODIFY]` amendment is named in the sufficiency report and written as the marked delta on
  the affected feature's map entry. Under delta scope the desk's delta card is itself the
  `[MODIFY]` instrument — the marked delta on the entry carries the amendment and folds at
  landing; a delta fix discovered structural re-fires the design phase per D1 mechanics (a);
  (10) in-flight exposure (verify V2, R2) — a touched surface owned by an in-flight feature
  obliges the read of that feature's deltas and owning spec: need covered → cite the planned
  contract, no gap; adjacent → **gap** — the design phase authors the proposed delta
  sequenced behind that delivery; conflicting → reserved to the user at run-open, alongside
  trips. No locks; only silent contradiction is prohibited — the conflict routing is a
  question to the user, never a hold on the touched feature (R3).
  **Absent baselines (greenfield branch):** an absent baseline file grades its touched
  surfaces "new (gap)"; the design phase's first duty is the seed — empty scaffolds where no
  code is delivered, reconstruct-and-confirm with the user at the design checkpoint where
  delivered code exists (the v0.66.0 baseline-seed defect close,
  `.mochiko/decisions/2026-08-11-plan-baseline-seed-enforced.md`, rehomes here — its home
  command dies with plan).
  **Trips are not gaps:** a store trip (touched row standing `open`/`not-now`) rides the
  verdict report and is dispositioned by the user at run-open — warn-and-record, recorded
  deferral escape, never a silent skip.
  **Verdict ownership:** the check is binding at entry (gap list ⇒ design phase); a disputed
  clause defaults to gap and the dispute goes to the user — the grader never clears alone.
  Verdict: per-row *sufficient* or a gap list. Any gap → the design phase authors exactly
  those gaps, nothing else. Zero gaps → straight to cards + build.
  **Rationale:** per-row because the work row is the map's unit of scope, making the check
  size-adaptive by construction; sourced from the three surfaces that survive plan's
  retirement as the product's whole governing design state; seated at implement entry because
  D1 makes implement the single downstream run. The clause set enumerates every design input
  implement's builders and gap-finder consume, so a passing row means no consumer reads a
  missing artifact.

- **D3 — The FR→TR layer dies as a mandatory artifact; it does not move to specify.**
  `Confident` (user accepted at Q7, reversing the Q2 hypothesis after pushback). No
  per-feature `requirements.md`. Real technical decisions (D-XXX, C-XXX, IP-XXX) land where
  they already live — `constraints-and-decisions.md` and the store — authored by the design
  phase when the check fails, or at build-time decomposition (gated per D1 mechanics d) when
  it passed. Specify stays technology-agnostic; FR/SC unchanged.
  **Fence consequence (review fold):** `requirements.md` is a named member of
  `mochiko:testing-gap-finding`'s explicit inclusion list; its slot re-keys to the
  sufficiency report + the design-phase deltas (spec-layer artifacts, never code); the
  narrowing is recorded, its adequacy watched (Open questions).
  **Rationale:** moving TR into specify would make every spec technical, kill the deliberate
  business/technical layer split, and re-fatten specify with the same restatement this
  session is cutting.

- **D4 — Homes and gates in the plan-less pipeline.** `Confident` (user-confirmed at Q8;
  amended at the review disposition batch: card confirm re-seated, report contents widened).
  **Homes:** design-phase outputs land unchanged at `.mochiko/features/FEAT-XXX/` — deltas
  mirroring baselines, the signed store delta; `tasks.md` survives as the cards + progress
  surface; `plan.md` (the summary artifact) dies — no restatement artifact; the sufficiency
  verdict lands as a report under `templates/report-format.md` in the feature dir and is the
  durable assessment record — it additionally carries the `quickstart.md` null-path record
  and any `[MODIFY]` amendment naming (D2 clause 9).
  **User gates in the new implement:** run-open confirmation carries routing only — batch,
  scope type, attempt bounds, the sufficiency verdict with gap routing, trip dispositions,
  and in-flight conflict rulings (D2); the design checkpoint — only when the phase ran — signs the design and
  store delta, blocking, before the first cycle; the **card confirm is its own blocking
  checkpoint** after card authoring (which follows the design phase when one ran), ruling the
  slicing before build; cycle checkpoints and final acceptance unchanged.
  **Dead gates:** plan's plan-the-plan proposal approval and package acceptance, replaced by
  the run-open extension, the design checkpoint, and the card confirm. Plan's inline epic
  mint door (declare-and-contest from a bare feature list) dies with it — epic minting is
  desk-only (`/mochiko:feature`), recorded supersession.
  **Rationale:** every artifact implement consumes keeps exactly one authoring home and one
  user gate; the three checkpoints partition plan's two dead gates without loss — routing,
  design, slicing — each at the moment its subject exists.

- **D5 — Evidence honesty, grader disposition, falsifier.** `Confident` (user-confirmed at
  Q9; amended at the review disposition batch: falsifier bidirectional, evidence
  qualification added). n=0 — no run has ever executed the sufficiency check or an in-run
  design phase. **The redundancy driver's evidence is pre-treatment:** F5's bloat datapoint
  predates the v0.67.0 plan re-charter, and that fix's FEAT-002 re-derivation probe never
  ran; the staged-advisory alternative (ship the check as an advisory instrument first,
  gather discrimination evidence, then rule retirement) was put to the user within the Q10
  exploration and declined with the trade in view. First-live-run watch owed: does the check
  discriminate (some rows pass, some fail) · does the design checkpoint get real attention
  mid-run or rubber-stamp · does direct-to-build produce landing-time design surprises.
  `review-plan-artifacts` and `review-feasibility` re-scope at build time to grade the
  design-phase output and the sufficiency check's own honesty; detail deferred to the build
  wave. **Falsifier (bidirectional):** first live runs showing the check passing everything
  (no discrimination) *or failing everything* (the design phase fires almost always,
  collapsing D1's economics) weaken D1's premise — revisit. The watch lands as a BACKLOG item
  at the landing, its owner the landing ritual.
  **Rationale:** the session's decisions outrun their evidence by construction (the
  instrument cannot have run before it exists); recording that gap, its direction, and its
  revisit trigger is what keeps D1 falsifiable rather than merely ruled.

- **D6 — Delta-scope branch of the check; product lanes inherited-as-is.** `Confident`
  (reopen-born from review survivor B-C1, user-ruled inline at the disposition gate).
  Implement's delta-scope entry gates on the desk-confirmed card directly — `/mochiko:feature`
  mints it and its existing review leg covers it; implement's run-open absorbs the
  card-vs-entry confirmation the delta-scope plan run used to perform. The sufficiency check
  runs per-card, not per-row: criteria testable, touched surfaces identified, store consult +
  trip check run. Product-lane runs are declared inherited-as-is: their thinness predates this
  session (shipped `implement.md` omits lanes from Entry's scope branches); this record
  neither worsens nor repairs them — noted in Open questions. Under delta scope the `[MODIFY]`
  protection rides the delta card itself, not D2 clause 9 (verify V3 — clause 9 is
  selection-scope only).
  **Rationale:** the desk already gives the delta card a mint gate and a review leg; adding a
  plan-style confirmation run would rebuild the ceremony this session retires, while the
  per-card check preserves the entry conditions implement actually needs.

- **D7 — UX sufficiency clause; the Screens & Flows trace rehomes whole.** `Confident`
  (reopen-born from review survivor B-C2, user-ruled inline at the disposition gate). D2
  gains clause 8 — where the spec carries a Screens & Flows manifest: every FEAT-tagged
  SCR-XXX's data shown has a nameable serving contract surface, and every FLOW-XXX action a
  mutation path (existing = cited, new = gap); UX-bearing cycle cards' `**TEST:**` gates name
  their FLOW-XXX paths, enforced at card authoring. The plan-era trace obligation rehomes
  whole into check + cards; nothing of it dies.
  **Rationale:** the trace exists to catch screens whose data no contract serves before code
  is built; the check's entry seat is the only pre-build moment left after D1, so the trace
  must live there or die — and UX-bearing specs are first-class (default-FAIL in specify),
  not an edge to accept losing.

## Session trail

- **Q1 — driver.** What made plan feel questionable now: felt cost, redundancy suspicion,
  structural (store landed), or sufficiency anxiety (no instrument measures whether implement
  had enough guidance)? **A:** redundancy suspicion + sufficiency anxiety; felt cost is the
  downstream effect of those two, not an independent driver.
- **Q2 — artifact-level suspects.** Which package artifacts restate what spec + store already
  say? **A:** a rehoming hypothesis, not mere redundancy: `requirements.md` (FR→TR) could move
  to specify; design deltas (`data-model.md`, `contracts/`) could move to architecture;
  `tasks.md` cycle-card authoring is work the user believes implement does (builders already
  decompose at build time). `plan.md` summary implicitly dies if the rest rehomes.
- **Q3 — shape of the design-delta rehoming.** Desk-becomes-stage vs design-at-build-time vs
  altitude split? User asked for a recommendation (noting the feature desk exists too). Lead
  recommended the altitude split plus a reframe: plan becomes conditional behind a sufficiency
  check at implement entry — structural design stays a pre-build user-signed gate,
  non-structural detail moves to build time, FR→TR mostly dies rather than moves (specify
  stays technology-agnostic), cycle cards author at implement run-open, feature desk stays a
  map desk.
- **Q4 — spine ruling on the conditional-plan shape.** **A (pushed further):** the user
  challenges keeping plan alive as a command at all just to serve as the fallback design stop —
  why not have seats inside the implement run (staff-engineer, multiple seats, or other agent
  types) do the design work when the check fails?
- **Q5 — full-retirement ruling.** Lead agreed the in-run shape is stronger than its own Q3
  recommendation and recommended retiring plan-the-command with the carry-over floors named.
  **A:** yes → D1.
- **Q6 — sufficiency-check content.** Seven-clause per-row checklist proposed. **A:**
  confirmed → D2 (as amended: ten clauses).
- **Q7 — FR→TR disposition.** Pushback on the Q2 "move to specify" hypothesis; counter:
  TR dies as a mandatory artifact. **A:** accepted → D3.
- **Q8 — homes and gates after retirement.** Proposal covering artifact homes, surviving and
  dead user gates. **A:** confirmed → D4.
- **Q9 — evidence honesty + residuals.** n=0 marker, watch, grader re-scope, falsifier.
  **A:** confirmed → D5.
- **Review interlude.** Pair cold review ran (user ruled "as recommended" at the sizing gate);
  both seats returned critical-gaps; cross-examination sustained the merged set with zero
  outright kills. The S1 survivor (retain-and-collapse never put to the user) was ruled
  **explore now**; the session re-entered iterative analysis at Q10.
- **Q10 — S1 exploration.** What does retirement buy that retain-and-collapse does not,
  and is that purchase worth the rehoming ledger the review surfaced? **A:** the user, with
  the trade table and the lead's contrary recommendation in view, ruled **retire plan** —
  D1 re-affirmed, re-marked `Contested`; the rehoming ledger accepted as obligatory build
  scope. S1 dispositioned.

## Review + disposition trail

Pair cold review, blind-map two-message dispatch, lens-split: seat A `devils-advocate`
(decision-quality, 22-angle map) · seat B `validator` (record-integrity, 28-angle map). Both
verdicts **critical-gaps**. Raised: 12 (A) + 17 (B) + 1 cross-exam-born (the D2 source-scope
contradiction, corroborated by both). Cross-examination both directions: **zero outright
kills**; one partial kill (B-C3 leg c — epic minting survives at the desk; only plan's inline
mint door dies, Minor residue); 7 merges; 2 trims (A-C3's zero-gap diff clause — the trigger
not firing there is correct behavior; A-I1's ROADMAP-convention clause — corrected to the
BACKLOG-watch convention); seat A conceded its uniform-`Confident` framing to B-I5's narrower
D1/D5-contradiction form; seat B withdrew its F4 clearance (fence misstatement, corrected
above). Post-merge: 8 Critical clusters / 10 Important / 6 Minor.

**Dispositions (all user-ruled):**
- **S1** (retain-and-collapse never surfaced — A-C2 + B-I10's collapse road): **explored now**
  → Q10 → D1 re-affirmed `Contested`. 
- **S2** (delta scope + lanes — B-C1): **ruled inline** → D6.
- **S3** (UX blindness — B-C2): **ruled inline** → D7.
- **Remainder: one user-ruled batch "as recommended"** — folds applied across D1 (mechanics
  a–d), D2 (sources, clauses 5/8/9, absent-baseline branch, trips, verdict ownership,
  rationale), D3 (fence re-key), D4 (card-confirm checkpoint, report contents, mint-door
  supersession, rationale), D5 (pre-treatment evidence, bidirectional falsifier, rationale),
  F4/F5 corrections, build surface additions, Open questions population, provenance line.
  Landing obligations from B-I7 recorded in Build surface.

A bounded verify round (seat B, record-integrity — per the skill's allocation) grades these
folds plus the reopen-born D6/D7 and the D1 amendment: internal consistency + record-fitness,
no fresh coverage hunt, no second reopen.

**Verify round 1: NOT CLEAN** — 4 blocking (V1 map-entry hardening unowned · V2 in-flight
conflict routing unowned · V3 clause 9 ↔ D6 contradiction · V4 mechanics (d) mis-claimed the
graded fold as a judgment review leg) + 4 non-blocking (V5 D2 confidence mark · V6 D6/D7
rationale labels · V7 clauses 1/7 gap forms · V8 dry-run obligation). All eight user-ruled
"as proposed" and lead-repaired same round: V1 → D1 mechanic (e) · V2 → D2 clause 10 · V3 →
clause 9 selection-scope carve + D6 note · V4 → judgment-grade re-route · V5 → D2 re-marked
`Assumed` · V6 → labeled rationales · V7 → explicit gap forms · V8 → Build surface pre-wave
obligation.

**Verify round 2: NOT CLEAN** — 2 blocking ripple defects from the V1/V2 repairs (R1
mechanic (e) dropped the architecture-link leg — the entry-side schema field would lose its
only writer · R2 clause 10 lacked a gap form, so the adjacent branch's proposed-delta work
went untriggered) + 2 non-blocking (R3 clause 10's dropped no-locks guard — folded on the
seat's recommendation · R4 a narrow clause-9 edge, noted for the build wave). Six of eight
V-repairs verified clean (V3–V8, with V4 called the strongest of the round). R1/R2/R3
lead-repaired same round; R4 recorded in the Build surface.

**Verify round 3: CLEAN.** All three repairs verified (the R1 n/a carve checked safe by
construction — a zero-gap row cannot mint store elements, clauses 2/3 fire the design phase
first); one non-blocking cosmetic (N1 — D4's run-open census now mirrors clause 10's
conflict routing, folded same round). Verify pass complete.

## Build surface

*(sketch — the build wave plans in detail; this is the session's scope statement)*

- `implement.md` rewrite: entry re-gates on ratified selection (no accepted-package
  precondition); sufficiency check at entry (D2, D6); conditional design phase with its
  blocking checkpoint and mid-run re-fire (D1 a); card authoring + card-confirm checkpoint
  (D1 c, D4); epic spine design always-fired (D1 b); build-time decision gating (D1 d);
  Reports tool gains the `templates/report-format.md` envelope binding the sufficiency
  report rides.
- `plan.md` command retirement: supersession-by-ruling strips per
  `.claude/rules/mochiko/primitive-edits.md` (charter-protected content — every obligation
  rehomed by this record's D1–D7 or explicitly superseded here); author≠grader audit; landing
  ritual whole.
- Sufficiency-check carrier: likely a new skill (name deferred); D2 is its content spec.
- Re-points across the library: `specify.md` next-step line · `feature.md` growth-row
  routing · `mochiko:authoring-epic` (epic runs enter implement directly; joint-proposal
  spine artifact re-keys per D1 b) · `mochiko:patterns-plan-minimalism` re-scope to the
  design phase · `mochiko:patterns-vertical-tdd` (design-time card authoring now inside
  implement, D1 c) · `mochiko:review-plan-artifacts` / `mochiko:review-feasibility` re-scope
  (D5) · **`architecture.md`** (its charter-protected "plan and implement own all delivery"
  line — supersession by ruling) · **`ARCHITECTURE.md`** + **`README.md`** pipeline
  references · **`mochiko:authoring-technical-requirements`** (its subject dies with D3 —
  retire or re-scope, build-wave ruling) · **`mochiko:patterns-adopt-first`** (its trigger
  "at a plan decision" lives in the budgeted `description:` field — description-class edit
  under the D7 char-budget pre-assert) · **`mochiko:testing-gap-finding`** (fence inclusion
  list re-key per D3) · router rows · `plan.yaml` schema disposition ·
  `feasibility-report-template.md` · index counts · CHANGELOG / marketplace / `plugin.json`.
- Landing obligations (KM ritual): ROADMAP standing bet "Plan absorbs tasks — start-small
  surface" ruled **superseded** by D1 (same direction, carried further; its "merged-command
  dogfood" revisit condition never fired — noted honestly) · the `plan-structure-yagni`
  first-live-run watch and its BACKLOG item close **superseded** (its subject command dies) ·
  the D5 falsifier watch lands as a new BACKLOG item.
- Build-wave note (verify R4): a delta fix that breaks a *different* delivered feature
  without being structural sits outside clause 9's delta-scope carve — the implement rewrite
  should give the regression-scope territory sweep explicit reach over it.
- Pre-wave obligation (verify V8): dry-run the D2 check against one existing kinako work row
  before the build wave — confound named (kinako's specs were authored under a pipeline where
  plan followed, so verdicts read accordingly); the result is the first discrimination
  datapoint and seeds the D5 BACKLOG watch.
- Migration: existing accepted packages are valid frozen history; a feature holding an
  accepted package enters implement with the sufficiency check trivially satisfied.

## Open questions

- **Discrimination evidence (the D5 watch, bidirectional):** does the check produce mixed
  verdicts in live runs, or degenerate to all-pass / all-fail? All-fail collapses D1's
  economics (the design phase fires always — plan reborn inside implement). Revisit D1 on
  first evidence; BACKLOG watch owed at landing.
- **Gap-finding fence adequacy (D3):** does the sufficiency report + design-phase deltas slot
  carry what `requirements.md` carried for the blind explorer's expectation derivation?
  Watch at first gap-finding pass under the new shape.
- **Product-lane runs (D6):** inherited thin — lanes appear in `implement.md` Tools and
  Boundaries but not Entry's scope branches; predates this session; unrepaired here.
- **Depth-dial:** the sufficiency check is deliberately depth-invariant (precedent:
  `architecture.md`'s "No depth-dial coupling"); revisit only if live runs show low-depth
  projects wanting a laxer check.
- **Design-checkpoint attention:** rubber-stamp risk named in D1's cost paragraph; watched
  under the D5 first-live-run watch.
