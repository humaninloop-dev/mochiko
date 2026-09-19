# Record — author≠grader consolidated: one coherent path forward

**Status:** accepted 2026-09-19 (user: "accept and then implement") — landed: `DECISIONS.md` row
(2026-09-19) · `author-grader-value-tiering` superseded (its record + the index) · BACKLOG
"Charter ritual-floor build": gate-form wave-1 item + D9 watch item, producer-plan item
re-sequenced · ROADMAP Next "Floor builds" row touched · **wave 1 BUILT 2026-09-19 at v0.111.0**
(plan `wave1-gate-form.md`, log `build-log.md`; migration `0008-gate-form`, new skill
`validation-primitive-edit`, `common.gate-loop-bound` the corpus's first anchored common block,
`primitive-edits.md` Check section rewritten, crate 0.2.0 with the `mint-rule` grammar widening
the wave's one §7 halt produced — user-ruled; 12 audit units / 17 rounds / 8 blocking caught,
gates green, contract suite 89/89 across two runs — see the log; committed `2ec189b`) ·
**wave 2 BUILT 2026-09-19 at v0.112.0** (plan `wave2-plan-qa.md`; migration `0009-plan-qa-leg`,
new skill `review-seat-plan`, sound-loop leg 1 reworded and the wave run under the leg it ships,
`primitive-edits.md` criterion 6 to the peer-graded standard; plans P1:PASS(1) · P2:PASS(0) ·
P3:PASS(1); 14 gate units / 1 fix round under the new form, the `review-seat-plan` pair
double-graded as the D9 baseline, D7 cost limb Measured; gates green, contract suite 91/91 — see
the log; committed `56b44c2`) · **wave 3 BUILT 2026-09-20 at v0.113.0** (plan
`wave3-validator-retirement.md`; migration `0010-validator-retirement`, the `validator` persona
retired — nine agents ship, the carrier live at both sites; governance PATCH v3.1.0 → v3.1.1
user-ruled; plans P1:PASS(0) · P2:PASS(1) · P3:PASS(0); 11 gate units / 3 rounds / 4 blocking
caught, 11 PASS at close; gates green — see the log) — **all three waves built, the path
complete**; commit pending the user
**Opened:** 2026-09-19
**Supersedes:** `author-grader-value-tiering` (opened 2026-08-26, stalled at Q1 with
ground facts F1–F6 and no decisions). This session carries that record's facts forward, adds the
rulings landed since, and produces the single path the scattered pieces did not.
**Topic:** Author≠grader is ruled in pieces across five sessions and one governance principle.
Bring the pieces together into one coherent path forward — what independence is for, where it is
a floor, where a cheaper form of independence suffices, and what the build and governance moves
are.

---

## Ground facts

*F1–F6 carried from `author-grader-value-tiering` (2026-08-26), re-verified against the tree at
v0.110.0 where noted; F7 onward new to this session.*

- **F1 — Where author≠grader binds today (five sites).** (1) GI-004, NON-NEGOTIABLE: every
  shipped-primitive edit passes the author≠grader audit before the `plugin.json` bump that ships
  it. (2) Skill-library axis 5: every reviewable artifact is graded by a structurally independent
  validator. (3) `patterns-sound-loop` floor leg 2: a judgment-authored write to a governing
  surface is graded by a non-author seat before the user's gate; the user's ruling never
  substitutes for the review leg; only three exemptions (mechanical execution · transcription of
  user decisions · fix-on-sight integrity repairs); no delta-card exemption; no size threshold.
  (4) Brainstorm end-stage cold review, blind two-message dispatch, with the user's recorded
  waiver as the only skip. (5) Pipeline producer↔validator pairs (`mochiko:validator`, the
  `review-*`/`validation-*` skills, `qa-engineer`'s independence). *(Re-verified 2026-09-19:
  `patterns-sound-loop` render shows leg 2 unchanged; leg 1 amended by producer-plan-enforcement.)*

- **F2 — Prior retention ruling.** `architect-role-pushback-and-abstraction` D3 (2026-08-13)
  explicitly retained author≠grader: the additive bias that session hunted was traced to charter
  asymmetry, not to independence itself. A wholesale change supersedes that ruling — recorded
  supersession, user-ruled.

- **F3 — Wave audit outcomes on record.** v0.65.0 → v0.81.0 (old record): zero-fix waves v0.65.0
  · v0.70.0 · v0.71.0 · v0.72.0 · v0.73.0 · v0.75.0; catch waves v0.67.0 (1 of 4 clusters FAIL)
  · v0.68.0 (audit caught the lead's unrecorded coordination ruling — a GI-006 breach) · v0.76.0
  (CI cargo-audit flag) · v0.79.0 (V2's F1 catch killed a false premise) · v0.81.0 (all 4 clusters
  FAIL round 1). Since then (BACKLOG trail lines, v0.97.0 → v0.110.0): v0.97.0 command scaffold
  build — V1 audit discharged the M5 re-inventory (a catch: desk FAIL-set under-extraction);
  v0.101.0–v0.103.0 skill-schema waves 10/10 · 11/11 · 7/7 PASS; the wave-6 and hook waves
  "13/13 PASS round 1, zero fix rounds" and "8/8 PASS round 1, zero fix rounds"; the four eval
  kits "FAIL→fix→PASS each" (2026-09-18/19). Roughly half of recorded waves produce zero blocking
  findings; the catch waves include one governance breach and several substantive defects.

- **F4 — Value-tiering already exists in three ruled forms.** Sound-loop's kind key exempts
  mechanical/transcription/fix-on-sight work; the brainstorm review sizing gate lets the user
  choose pair/single/none with a recorded waiver; the two-reviewer overlap finding (kinako,
  review-brainstorm eval) showed only 2 of 6 Important findings found by both reviewers — naive
  halving of reviewers loses Important-class findings.

- **F5 — Cost side is measured in one dimension only, for in-session spawns.** The platform
  exposes no session-readable token totals **for a seat spawned inside a session**; a session the
  harness launches returns a per-session `total_cost_usd`, which `evals/run.py` already reads,
  sums, and budgets against (narrowed at CG1, 2026-09-19). Cost evidence for in-session seats is
  seat counts and wall time. New since the old record:
  `orchestrator-model-selection` F13 measured an **~54K-token instantiation floor per persona
  spawn** before the seat reads a single artifact — every fresh validator seat pays it. Every
  validator seat is a fresh session-tier agent reading the full cluster cold.

- **F6 — GI-004 is governance.** Amending or softening it is a governance event (fact-profile
  amendment via `/mochiko:setup` per CLAUDE.md governance operations). This record can rule the
  intent; landing requires the governance amend path.

- **F7 — Three sessions ruled author≠grader pieces separately since 2026-08-26, none citing the
  stalled session.**
  - *`producer-plan-enforcement` (2026-09-03, D3, build PENDING — BACKLOG "Charter ritual-floor
    build", waves 1 + 2):* a seat's plan is graded by a **fresh spawn of the author's own persona
    type** (never the author's context, never the lead); **`validator` retires library-wide** in
    a second wave, gated on wave-1 plan-grade figures and a named default-FAIL carrier; CLAUDE.md
    axis 5 reworded to "graded by a structurally independent grader: a fresh seat that authored
    nothing it grades, running a different skill from the author's"; the router's mount doctrine
    becomes "never mount producing and grading skills for the same artifact on one seat";
    `devils-advocate` out of the plan loop "for now". Independence here is *seat* independence,
    not *persona* independence — an **addition** of a grade leg (plan QA), not a cut.
  - *`orchestrator-model-selection` (2026-09-19, D1/D3/D5, BUILT v0.110.0):* the seat default key
    is `strong`/`down` by one criterion — *does a structurally independent seat stand between this
    seat's output and the run's verdict?* Producers default to `sonnet` **because** a grader
    stands behind them; graders and the six strong-class seats stay `opus`; a grader never runs
    below its producer's tier (CG3). Independence became the **cost lever**: removing a grader
    would, by D1's own criterion, push its producer back to `strong`.
  - *`primitive-eval-harness-v2` (2026-09-08) + ADR `2026-09-09-persona-edit-advisory-grid`:*
    persona edits with an eval kit carry an advisory `pre`/`post` grid read cited in the audit
    brief — an instrument the grader reads as evidence, never a gate.

- **F8 — The mechanical tier has grown; the judgment residue of the audit has shrunk.** Since the
  old record: `mochiko-cli migrate validate --report` is the deterministic pre-pass cited in
  every audit brief (Python checkers retired v0.107.0); the char-budget pre-assert (D7) precedes
  the model-judgment audit; `hook-enforced-artifact-schema` (2026-09-13) rules a mechanical
  `PreToolUse` deny on artifact path · file set · headings · per-section size before the write.
  What remains for the model-judgment grader on a primitive pair is the substance criteria
  (`primitive-edits.md` command items 6–8, skill items 4–5 and 12): preserved responsibilities,
  floor survival, independence, reserved-to-user, done-condition branch.

- **F9 — Grader quality evidence is thin and full-read-dependent.** The persona-eval judge
  calibration (2026-09-09) got 0.667 inter-labeller agreement from keyword-extract labels and
  1.00 from a full-read labeller on the same set — a grader that skims is a different instrument
  from one that reads. No measurement exists of the primitive-audit grader's catch rate against
  seeded defects; the `adversarial-review-generality` benchmark item (2026-08-04, BACKLOG) is the
  only cold-runnable spec of that shape and is unrun.

- **F10 — Author≠grader is under-extracted as a FAIL condition on the two desks.** BACKLOG "Desk
  FAIL-set widening" (user-deferred 2026-08-27): `author-grader-separation` and
  `no-self-graded-writes` are named candidates for `kind: fail` flips on `architecture` and
  `feature`; user ruled a separate ruled pass, never a silent widening.

- **F11 — The stalled session's Q1 ruling.** Driving pain = **cost and latency/ceremony**, primary
  and jointly; yield not disputed — the catches are real; the price per catch is the problem.

- **F12 — A persona-less seat inherits the lead's model.** The Agent tool resolves `model:` from
  the override, else the agent definition, else the parent. A `general-purpose` grader spawned
  without an explicit `model:` under a Fable lead runs on Fable — one tier above `validator`'s
  pinned `opus`. `orchestrator-model-selection` D2's "no `inherit`" rule binds persona *files*
  only; nothing today binds a persona-less spawn call. *(Raised by the user at Q2.)*

- **F13 — The grading-lane conditions as they stand (inventory, read from the derived views at
  v0.110.0).** Ten condition families, grouped by the failure mode each targets:
  1. *Rubber stamp* — `common.author-grader-default-fail` (commands, ×5 via `extends`): "No
     output is cleared by its author; grading reads the authored surfaces themselves — never the
     author's report — default FAIL" · `validator` Iron Law: "NO PASS WITHOUT EVIDENCE READ FROM
     THE ARTIFACT ITSELF … If the file was not Read this run, the verdict is FAIL" ·
     `validation-constitution` "Inputs are all read from file, never from the author's report".
  2. *Absence-of-looking as evidence* — `review-common.default-fail` (review family, ×5 via
     `extends`): "Never default to `${verdict}` — earned only by a completed hunt; absence of
     looking is never evidence".
  3. *Grader drifts into co-author* — `review-common.author-grader`: "Never author, fix, or
     revise what you grade" · per-skill `<skill>.author-grader` floors ×8 · `validator`: refuse
     to fix, refuse to grade own work, "Independence is not yours to waive".
  4. *Soft verdict evades the gate* — `validator` binary PASS/FAIL, "'Mostly conforms' … is FAIL
     with a fix list" · `validation-constitution` "Binary PASS or FAIL — no soft language" ·
     three-state verdicts earned (`review-feasibility` feasible/needs-revision/infeasible ·
     `review-plan-artifacts` ready/needs-revision/critical-gaps · `review-brainstorm` and
     `review-governance-intent` critical-gaps) · `review-specifications` "gap-finding input,
     never a clearing PASS/FAIL verdict" · `review-code-minimalism` advisory.
  5. *Skipped items* — `validator`: "Violating the letter of the checklist is violating its
     spirit … restart from the checklist" · per-item PASS/FAIL + one-line evidence.
  6. *Grader invents its own bar* — `validator`: "Substituting your own bar … say so rather
     than invent a bar" · `review-common.never-excess` (floor/compliance/NFR obligations never
     excess).
  7. *Endless loop* — `review-common.its-command-states-them` (loop, round-cap, human-gate live
     in the command) · `implement` attempt bounds (`gap_rework_bound`, an attempt consumed per
     grade, exemption the user's call) · producer-plan D6 (one shared re-plan round, second
     miss to the user) · `brainstorm.reopen-born-verify` (one bounded verify round) ·
     `primitive-edits.md` "bounded re-audit" · the ≤3 review-round cap OMS D4 watches.
  8. *Verdict clears the run without the user* — `review-common.verdict-is-input`: "Your
     verdict is input, never a clearing — the lead owns the clearing verdict" **versus**
     `validator`: the `validation-*` family "issues your authoritative binary grade" — two
     postures, one library.
  9. *Evidence lost in conversation* — `review-common.evidence-floor` (verdict and dispositions
     land in the reviewed artifacts) · `validator` "Evidence read: <files> — absent ⇒ FAIL" ·
     deterministic pre-assert recorded as evidence (`validator`, `primitive-edits.md` D7).
  10. *Anchoring on the author's framing* — `review-brainstorm.never-in-the-room` +
      `blind-map-before-record-contact` (floors) · `review-governance-intent.never-a-participant`
      + `sequestration` · the two-message dispatch.
  Two things the inventory shows: "default FAIL" names three different mechanisms at once — a
  *posture* (not done until read), a *tamper-proof clause* (no read line = FAIL), and a
  *completeness rule* (every item re-confirmed, every round) — and the library runs two grading
  jobs, gate and input, with `validator` the only carrier of the gate form.

## Constraints

- GI-004 (NON-NEGOTIABLE) and GI-005 bind until a governance amend run supersedes them (F6).
- `patterns-sound-loop` floors — `two-part-trigger` · `leg-1-seat-produces` ·
  `leg-2-non-author-review` · `no-delta-card-exemption` — leave only by recorded
  supersession-by-ruling (`primitive-edits.md` floor survival).
- `producer-plan-enforcement` D3 wave 2 (`validator` retirement) is ruled but unbuilt; any ruling
  here on the grader carrier either confirms, amends, or supersedes it — never leaves it ambiguous.
- `orchestrator-model-selection` D1 (BUILT) keys producer tier on grader presence; any cut to a
  grader seat re-keys the producer it stood behind.

## Decisions

### D1 — Independence stays universal; the cost cut comes from the grader's *form*, never from removing the grader
**Statement:** Every one of F1's five sites keeps its structurally independent grader. No such
surface, however low-value, moves to author self-verification. *(Scope, M1 fold: this is a claim
about F1's sites, not a completeness claim about the library — five seatless stages are on record
from `adversarial-review-generality` (2026-08-04: implement's built code, specify's Intent
synthesis, the live brainstorm before convergence, feature-close, the lead's own folds) and F10
defers a sixth; none is opened or closed here.)* The price-per-catch (F11) is attacked through the
grader's form — which seat grades, at what tier, reading what, at what granularity — each a lever
ruled in the decisions that follow. The surface-value tiering road (the stalled session's bet) is
recorded-rejected; the defect-class split (mechanical to the CLI, judgment to a fresh seat) is one
lever inside this path, not a rival organizing claim. GI-004, axis 5, and the `patterns-sound-loop`
floors stand unamended; F2's retention ruling is re-affirmed.
**Rationale:** The driver was price per catch, not catch (F11). The three later rulings already
attack price from the seat side — `orchestrator-model-selection` D1 licenses a `sonnet` producer
*because* a grader stands behind it, so removing a grader costs a `strong` producer and saves
nothing net; producer-plan-enforcement D3 cheapens the grader's persona; the mechanical tier (F8)
shrinks what the grader must read. The tiering road would pay a governance amend run and a floor
supersession for a gain nobody measured (F5, F9) and, per F4, would lose Important-class findings.
**Confidence:** Confident (user: "yes A"). *Adoption streak: 1.*

### D2 — Two grading jobs, ruled deliberately: *gate* where nobody rules after the grade, *input* everywhere a user ruling sits downstream
**Statement:** The library keeps both grading jobs F13 found, and names where each applies.
**Gate** — a binary PASS/FAIL the lead cannot ship past — applies only where the artifact leaves
the repo with no human ruling behind the grade: shipped plugin primitives before the
`plugin.json` bump (GI-004) and the governance surface set at setup's validate step. **Input** —
severity-ranked findings the user rules, never a clearing — applies everywhere a user ruling
already sits downstream: brainstorm records, specs, design packages, seat plans, cycle
verification, governance intent. `review-common.verdict-is-input` becomes the stated rule for the
input job; the gate job's conditions are ruled next (D3). No site changes job by this decision;
what changes is that the split is ruled, not grown.
**Rationale:** A gate earns its cost only where nobody checks after it. Everywhere else the grade
is already input by rule 8 of F13, and a binary there is ceremony (F11). The one-job roads were
priced: *input only* costs a GI-004 amend run and retires the only self-enforcing check on the
ship path; *gate only* multiplies the ceremony F11 named.
**Confidence:** Confident (user: "as recommended"). *Adoption streak: 2.*

### D3 — At the primitive-edit gate, "default FAIL" keeps the posture and the tamper-proof clause; the completeness rule shrinks to judgment items, confirmed once, with delta re-audits
**Statement:** At the primitive-edit gate, of the three mechanisms F13 found under one name, the
gate keeps two: the *posture* (the artifact is not done until the grader has read it) and the *tamper-proof clause*
(a verdict with no evidence-read line is FAIL, automatically). The *completeness rule* shrinks
in three ways. (1) Mechanical checks — scaffold headings, section sets, id continuity, char
budgets — are the deterministic pre-pass: **the grader runs `mochiko-cli migrate validate
--report` and the char-budget measurement itself and quotes their output as first-hand evidence;
a pre-pass result quoted from the brief is not evidence** (I3 fold) — and the grader never
re-derives by judgment what that output already asserts. (2) The grader confirms the judgment
items once per unit, each with its one-line evidence — **two sets, keyed by unit** (I1 fold): for
a markdown primitive the pair set (preserved responsibilities, floor survival, independence,
reserved-to-user, the done-condition branch); for schema content — a migration file plus its
regenerated view diff — the AM-2 five (intent stated · anchor present where required · ID
lifecycle right · floor and fail survival · register; ledger GI-004 as re-expressed at v3.0.0).
(3) A re-audit after a FAIL reads only what the fix touched plus anything the fix could have
broken, never the whole cluster again. "Restart from the checklist" leaves the grader's brief.
**This shrink binds the primitive-edit gate only** (C1 fold, user-ruled): setup's validate step
keeps `validation-constitution`'s completeness floors — `verify-every-item` ·
`rationalization-stop` · `every-set-must-pass` — unchanged; the mechanical pre-pass this shrink
relies on has no counterpart on the governance surface set.
**Rationale:** The mechanical tier grew (F8); re-checking by model what the CLI already asserted
is the ceremony F11 named. Dropping the completeness rule outright (road C) leaves "did you
check the floors?" unanswerable, which is the rubber stamp the posture exists to prevent.
**Confidence:** Confident (user: "as recommended"; C1 scope fold user-ruled). *Adoption streak:
3 — flagged; Q5 runs recommendation-free.*

### D4 — The FAIL→fix→re-grade loop bound lives in one place and is inherited, never restated
**Statement:** The loop bound is a single rule in the command common block
(`common.<slug>`, the migration log's one shared command library), its number written in the
rule's own text — not a `${var}` each command redeclares, which would be six places. Every
command that hosts a grade loop carries it by `extends:`; skills never restate it
(`review-common.its-command-states-them` already binds them to "its command states them").
Changing the number is one migration entry. The number itself, what loops it covers, and
whether a run may deviate are ruled at Q6–Q7. `implement`'s cycle attempt bounds
(`attempt_bound_cycle`, `gap_rework_bound`) are a build-cycle economy, not a grade loop, and
are touched only if Q6 folds them in.
**Rationale:** The user's ask at Q5: "standardized somewhere so it is inherited and maintained at
one place." The `extends: common.<slug>` idiom is the library's existing single-home mechanism
(near-dup convergence: an exact duplicate across 3+ commands is extracted), and the literal number
in the common text is what makes it one place rather than six.
**Confidence:** Confident on the home (user's own words); number ruled at D6, scope at D5.

### D5 — The one loop bound covers gate loops only
**Statement:** The D4 rule binds the gate job's FAIL→fix→re-grade loops: the primitive-edit audit
before a `plugin.json` bump and setup's validate step. Input loops (cold-review verify rounds,
seat-plan re-plan rounds) and build loops (`implement` cycle attempts and gap-rework rounds) keep
the numbers their own rulings set.
**Rationale:** A build loop is a test failing, not a grade; an input loop ends with the user
ruling, so a cap there only limits reviewer re-reads. The gate is the one place a loop can spin
with nobody watching.
**Confidence:** Confident (user: "as recommended"). *Adoption streak: 1.*

### D6 — One re-audit, fixed; a second FAIL stops and goes to the user
**Statement:** The D4 rule's number is **one**: FAIL → fix → one re-audit (a delta read per D3).
**The re-audit is the same grader seat resumed, never a fresh spawn** — the delta read depends on
the first read being in context, and independence is unaffected because the seat authored
nothing in either round (I5 fold). A second FAIL halts the landing and comes to the user with
both fix lists; the user rules — **fix again, or drop the edit** (C2 fold, user-ruled: the
"ship with the finding recorded" option is struck — a bump carrying an unfixed FAIL is a bump
without audits PASS under GI-012; overruling a grader the user judges wrong rides the ledger's
existing waiver path, nothing new). No run may raise the bound at open or anywhere else.
**Rationale:** Every catch on record (F3) resolved in one fix round. A lane to raise the cap on a
gate is a way to keep a stubborn FAIL from the user, which is the one outcome a gate exists to
prevent. `implement`'s redeclare-at-open lane is a build-loop device (D5) and does not carry over.
**Confidence:** Confident (user: "as recommended"; C2 strike user-ruled). *Adoption streak: 2.*

### D7 — The gate grader is a plain fresh seat carrying the brief, pinned to a tier explicitly; `validator` retires as already ruled
**Statement:** The gate grader is a persona-less fresh seat (`general-purpose`) that authored
nothing it grades. **The gate contract is not composed per spawn** (C3 fold, user-ruled): it
ships as a small plugin skill in the validation family — working name
`validation-primitive-edit`, sibling of `validation-constitution` — whose rules live in the
migration log and carry the D3 judgment sets, the tamper-proof clause (no evidence-read line =
FAIL), "read the files, never the author's report", the fix-list form, and a pointer at the D4
bound. Delivery is two-way, the `review-seat-plan` shape (producer-plan D4): the seat invokes the
skill, and the dispatcher pastes `mochiko-cli rules validation-primitive-edit`'s render verbatim
into the brief. The dispatcher writes only the unit, the file paths, and the pre-pass command to
run. A brief whose contract section was hand-written rather than rendered is a floor miss, on
the same terms as an omitted `model:`. This also satisfies the wave-3 axis-5 wording as ruled —
the seat runs a skill different from the author's (I6 fold). **Every such spawn carries an
explicit `model:` alias** — equal to the tier the graded work was produced at, never below
(`orchestrator-model-selection` CG3), and `opus` when the lead itself made the edit; an omitted
`model:` on a persona-less grader spawn is a floor miss (closes F12). This names the
"default-FAIL carrier" `producer-plan-enforcement` D3 wave 2 waits on — the rendered skill on a
plain seat — so `validator` retires at that wave with no further condition beyond wave-1
figures. `tech-lead` remains available as a disclosed lead deviation for cause, never the default.
**Rationale:** The decision rests on the prior user ruling — "a persona whose only craft is
checklist grading is one persona too many" — which carries it alone. The cost limb is separate
(I2 fold): F13's ~54K figure was measured on persona spawns only and its composition includes
project context every seat pays, so the persona-attributable share is unknown and the rendered
skill adds tokens back; the limb is `Assumed` until the wave-2 double-grade spawn (D9) is read
the F13 way. The explicit pin is what stops the plain seat being dearer (F12).
**Confidence:** Confident on the shape (user: "as recommended"; C3 fold user-ruled) · cost limb
**Measured 2026-09-19 (wave 2 double-grade, F13 method, both seats `claude-opus-5`, same cached
project context 27,111):** the plain gate seat carrying the pasted render and 13 units cost
27,634 cache-creation tokens at first turn; the `validator` persona with its skill preload and
one unit cost 22,404 — **no instantiation saving from dropping the persona**; the saving the path
delivers is D11's one-seat-per-wave (one instantiation for 14 units against one per cluster).
The I2 `Assumed` mark is discharged by that reading; the shape stands on the prior user ruling.
*Adoption streak: 3 — flagged; Q9 runs recommendation-free.*

### D8 — Input-job conditions stand as they are
**Statement:** For the input job (D2) nothing changes: the headline verdict word, the
completed-hunt rule (`review-common.default-fail` — "absence of looking is never evidence"), the
blind angle map where a skill carries one, the evidence floor, never-fix-what-you-grade, and the
cold/blind dispatch floors all stay. The ground-up relook reached them and left them.
**Rationale:** User-ruled at a recommendation-free fork. The input job's cost is one seat per
review, its findings are ruled by the user, and its conditions are what make the findings worth
ruling on; no cost lever there was priced worth the loss.
**Confidence:** Confident (user: "keep as is"; recommendation-free question). *Streak reset.*

### D9 — Every gate audit leaves one outcome line where its landing is recorded
**Statement:** Each gate audit writes one line — `audit: <unit> · <seat> · <tier> · <n> files ·
<n> rounds · <n> blocking[ · cost: $<x>]` — into the record of the landing it belongs to: the
wave's `build-log.md` entry when the edit ships from a session, the `.mochiko/decisions/` ADR
when it is an ad-hoc defect close. The `cost:` field is present whenever the audit runs as a
launched session, read from the SDK's per-session `total_cost_usd` as `evals/run.py` already
does (CG1, ruled inline); an in-session spawn carries no cost field. **Baseline (C4 fold,
user-ruled): at wave 2 one unit is graded by both forms** — `validator` on the old full-read
form and the plain seat on the new — and the two finding sets are recorded side by side in that
unit's line; they are the baseline the later lines are read against, and that plain-seat spawn
is the I2 cost measurement. **Watch (I7 fold):** a BACKLOG item carries the trigger stated in
advance — blocking findings per wave fall by more than half against the F3 baseline with no
independent escape signal, or any defect reaches a release that a full-read audit would have
caught — and the revert: the gate form returns to a full read by recorded ruling, one migration
entry. The seeded-defect benchmark (`adversarial-review-generality` D3, BACKLOG) stays where it
is, unrun, as the deeper instrument if the trigger fires.
**Rationale:** The stalled session's driver (F11) was price per catch, and F5 says nobody has a
number for an in-session spawn. One line per audit is the cheapest instrument that yields one;
the double-grade is the only point in D10's sequence where both instruments exist, and without
it fewer findings reads as success and as a weakened grader alike (C4).
**Confidence:** Confident (user: "as recommended"; C4 fold user-ruled). *Adoption streak: 1.*

### D10 — Three waves: gate form first, then the plan-QA leg, then `validator` retirement; each wave audited under the form in force when it starts
**Statement:** **Wave 1 (this record's build)** ships the gate form — D3 brief, D4–D6 bound, D7
plain-seat grader with explicit tier, D9 outcome line — and is audited under the old form
(`validator`, full read): the old form's last full wave — its final outing is the single wave-2
unit below. **Wave 2** is `producer-plan-enforcement`
wave 1 (the plan-QA leg), unchanged in content, audited under the new gate form: the plain
seat's first live outing, yielding both the plan-grade figures that ruling waits on and the first
D9 lines — **and carrying the D9 double-grade: one wave-2 unit graded by `validator` (old form)
and the plain seat (new form), sets side by side** (C4/I4 fold). **Wave 3** is
`producer-plan-enforcement` wave 2 (`validator` retirement, CLAUDE.md sweep, axis 5 reword), its
gate now satisfied: figures from wave 2, carrier named by D7. No wave grades its own regime.
**Rationale:** The cost cut lands earliest; the new grader grades something real before the old
one retires; the wave that retires `validator` is not the wave that first runs without it; and
wave 2 is the only point where both instruments exist to grade one unit twice.
**Confidence:** Confident (user: "as recommended"; C4 fold user-ruled). *Adoption streak: 2.*

### D11 — One gate grader per wave takes every unit in the wave *(reopen-born, CG2)*
**Statement:** A wave's gate audits are run by one grader seat over all the wave's units, not one
fresh seat per cluster. Independence holds because the seat authored none of them. Each unit
keeps its own verdict block and its own D9 line, tagged with the seat, so no verdict hides in
the batch. The seat splits into two only when the units' files would not fit its context, and
says so in the lines. Re-audits (D6) resume the same seat.
**Rationale:** The instantiation cost is paid per spawn (F5); N clusters graded by N seats pay it
N times for no independence gain — independence is about who wrote the unit, not how many units
one reader holds. The per-unit line is what keeps each verdict answerable on its own; context
bleed between units is the one real risk and the line contains it.
**Confidence:** Confident (user: "as recommended"). *Adoption streak: 3 — flagged; no further
question in the session, so no recommendation-free fork follows; named to the user at
acceptance. Reopen-born: one bounded verify round, no fresh cold read, no second reopen
(`brainstorm.reopen-born-verify`).*

## Build surface — wave 1 (the gate form)

Scoped to what D3–D7 and D9 change; wave 2 and wave 3 are `producer-plan-enforcement` D8–D9 as
already ruled, re-sequenced by D10 only.

1. **Migration entry** (one file under `plugins/mochiko/migrations/`, anchored to this record):
   - `common.gate-loop-bound` — new rule in the command common block, text carrying the number:
     "A gate verdict of FAIL allows one fix and one re-audit reading only what the fix touched;
     a second FAIL halts the landing and goes to the user with both fix lists — no run raises
     this bound." Labels `user-gate`, `independence`. Extended by `setup` (its validate loop is
     the only plugin-command gate site). *Single extender by ruling — see OQ1.*
   - `patterns-model-tiering` — new rule: a persona-less grader or reviewer spawn carries an
     explicit `model:` alias, never omitted; the alias is the tier the graded work was produced
     at, never below, `opus` when the lead produced it (D7; closes F12).
   - `setup` — the validate step's rule text points its loop bound at `common.gate-loop-bound`
     and its grader at "a plain fresh seat carrying the `validation-constitution` checklist"
     (the persona name leaves at wave 3, not here); `validation-constitution`'s floors are
     untouched (C1).
   - **`validation-primitive-edit` — new validation-family skill schema** (C3): the gate
     contract as rules — the D3 judgment sets keyed by unit, the tamper-proof clause,
     read-the-files-never-the-report, the fix-list form, a pointer at `common.gate-loop-bound`,
     the D11 one-seat-per-wave shape — on the review six-set (`independence` · `scope` ·
     `inputs` · `verdict` · `output` · `reserved`, as `validation-constitution` uses, explicit
     empty markers where a section has no rules) with the floor-count pin and read-back the
     skill-pair criteria require — plus its `SKILL.md`, router row, and first-seed budget (the
     `review-seat-plan` pattern, producer-plan D4).
   - The migration's `common.gate-loop-bound` text names its consumers —
     `primitive-edits.md` and `validation-primitive-edit` — so a rename or tombstone surfaces
     the pointer sweep (M4); the number itself stays in the one rule.
   - `mochiko-cli migrate validate --report` green; views re-emitted.
2. **`.claude/rules/mochiko/primitive-edits.md` — Check section rewritten** to D3/D6/D7/D9/D11:
   the grader runs the deterministic pre-pass itself and quotes it (I3); the criteria lists
   mark each item **CLI-asserted** or **judgment** (a build-time inventory against what
   `migrate validate` actually checks today — the split is not to be guessed), the judgment
   items keyed by unit (I1); the gate contract is `validation-primitive-edit`'s render pasted
   verbatim, never hand-written (C3); the grader is a plain fresh seat with an explicit tier,
   one seat per wave (D11); one re-audit by the same seat resumed, reading the fix delta (I5); a
   second FAIL to the user, fix-again or drop (C2); "restart from the checklist" and the
   full-cluster re-read leave; the D9 line is named as part of the landing; the bound is cited
   by id, its number lives in the migration (M4). `mochiko:validator` references become "the gate
   grader" here (the persona itself retires at wave 3). *Governance-surface status of this file
   — see OQ2.*
3. **CLAUDE.md** — untouched this wave except where `primitive-edits.md` is described; the
   `mochiko:validator` name sweep and the axis 5 reword are wave 3 (`producer-plan-enforcement`
   D8) as ruled.
4. **Strips** — the migration carries its own prior content; `primitive-edits.md` is a repo rule,
   not a plugin primitive, so no strip entry; any plugin-primitive prose removed takes the
   ceremony.
5. **Audit of this wave — old form, last full wave:** `validator` grades the `setup` pair, the
   `patterns-model-tiering` pair, and the new `validation-primitive-edit` pair (full read);
   **the migration file plus its regenerated view diff is its own graded unit on the AM-2
   five** (I1); a separate seat grades `primitive-edits.md` for coherence + preserved
   responsibilities. The D9 line is written for these audits too, as the baseline's first
   old-form samples, the wave-2 double-grade carrying the last.
6. **Landing:** `DECISIONS.md` row · `author-grader-value-tiering` → superseded (its record's
   Status line + the index entry) — **written once here; the wave-3 supersession list
   (`producer-plan-enforcement` D3's four rows) drops it** (M2) · **this session's own index
   entry updated to agree with the record and the decisions index** (M3) · BACKLOG: this wave as
   an item under "Charter ritual-floor build", the D9 watch item with its trigger and revert
   (I7), the producer-plan item annotated (wave 2 gate: carrier named by D7; order per D10; wave
   2 carries the double-grade) · ROADMAP Next "Floor builds" row touched · `CHANGELOG.md` ·
   `plugin.json` + `marketplace.json` bump · the stalled session's F1–F6 live on here.
7. **Wave 2 rider (C4/I2):** the double-graded unit's plain-seat spawn is read for cache-creation
   input tokens the F13 way and the figure lands beside the two finding sets; D7's cost limb
   moves from `Assumed` on that reading.

**Sound-loop check on the build itself:** the migration and the rule file are judgment-authored
writes to governing surfaces — leg 1 (a producer seat on an approved plan) and leg 2 (the wave-1
audit above) apply; the lead does not author the migration inline.

## Evidence honesty

- F3's "roughly half of waves zero-blocking" is a reading of trail lines, not a kept count —
  D9 exists to replace it with one.
- F5's ~54K instantiation floor is `orchestrator-model-selection` F13's measurement, carried,
  not re-measured here.
- F12 (persona-less seats inherit the lead's model) is read from the Agent tool's own
  description of `model:` resolution, not probed in this session.
- F13's figure was measured on persona spawns only and includes project context every seat
  pays; the persona-attributable share is not known, so D7's cost limb is `Assumed` until the
  wave-2 plain-seat spawn is read the same way (I2).
- No cost figure for the new gate form exists yet; the path's cost claim is an argument from
  seat count and read scope until the wave-2 double-grade and five D9 lines exist. For a
  launched-session audit a dollar figure is available (CG1) and the line carries it.

## Open questions

- **OQ1 — single extender vs the 3+ extraction bar.** `common.gate-loop-bound` is extended by
  `setup` alone (the primitive-edit audit is maintainer-side, bound through
  `primitive-edits.md` citing the rule id). The near-dup convergence bar extracts at 3+
  commands; this is a ruled single home (D4, the user's words), disclosed as the exception.
  `Assumed` that a ruled home needs no allowlist edge.
- **OQ2 — `primitive-edits.md` is a `.claude/rules/mochiko/` file, part of the governance
  surface set.** Precedent (producer-plan D8 criterion-6 rewrite; the persona-grid ADR) edited it
  at build with an audit, not through `/mochiko:setup`. `Assumed` the same path; GI-004's own
  text does not change.
- **OQ3 — desk FAIL-set widening (F10)** stays deferred exactly as the user ruled 2026-08-27; not
  folded in. Note for that pass: the desks' review legs are input-job (D2), so
  `author-grader-separation` there guards the run's seat wiring, not a gate verdict.
- **OQ4 — gate grader tier when a `sonnet` seat produced the edit — `Deferred` (CG3).** No
  `sonnet`-default seat produces a plugin primitive or a governance surface today (the four
  `sonnet` seats are requirements-analyst · technical-analyst · product-manager ·
  product-engineer); the live question is the reverse — whether a `sonnet` producer should ever
  be admitted on a gate artifact — and it is deferred to the touch that reads the D9 lines.
- **OQ5 — `common.author-grader-default-fail` keeps its text** ("default FAIL") on the commands
  hosting input jobs. Flagged for the cold reviewer as possibly ambiguous across the two jobs;
  **cleared at review** — raised and killed at cross-examination, the phrase reads as the posture
  on both jobs. Closed.

## Session trail

- **Session open (2026-09-19).** Index read; `author-grader-value-tiering` found open and stalled
  at Q1 (2026-08-26); three later sessions ruled author≠grader pieces without citing it (F7). The
  user's brief: "based on the context already on author not grader, i want to do a superseding
  session bringing together the context already into one coherent path forward." Home rendered
  (`mochiko-cli home .mochiko/brainstorms/author-grader-consolidation/record.md` → declared
  deliverable, no size bound, `reports/` open). Slug `author-grader-consolidation`.
- **Q1 — organizing claim.** Offered A (independence universal, cut cost via grader form —
  recommended) · B (tier independence by surface value — the stalled session's bet) · C (tier by
  defect class — mechanical to CLI, judgment to a fresh seat). Lead case for A: the driver is price
  per catch; the price is already attacked from the seat side (F7); B pays governance ceremony for
  an unmeasured gain and loses findings per F4; C is a lever inside A. **User: "yes A"** → D1.
- **Q2 — grader carrier (PARKED).** Offered A (generic fresh seat, criteria in brief —
  recommended) · B (`tech-lead` on governance-class surfaces) · C (keep `validator`, supersede
  producer-plan D3). User raised two things instead of ruling: (1) *cost* — a generic seat
  inherits the lead's model (F12), so A as offered runs Fable, above `validator`'s `opus`; lead
  conceded, mitigation = an explicit `model:` alias on every persona-less grader spawn, never
  omitted; (2) *scope* — "relook at the default fail and other such conditions from ground up".
  Lead parked Q2: the carrier's brief *is* those conditions, so the conditions rule first.
  Inventory taken → F13. Q3 goes ground-up.
- **Style ruling (user, before Q3 was re-put):** "ask in simple language, succinctly with example,
  throughout session." Q3 re-put in that form; the form holds for the rest of the session.
- **Q3 — what a review is for.** Offered A (two jobs, gate where nobody rules after, input where
  the user rules — recommended) · B (input only; GI-004 amend) · C (gate only). **User: "as
  recommended"** → D2.
- **Q4 — what "default FAIL" keeps at the gate.** Offered A (keep posture + tamper-proof clause,
  shrink completeness: CLI holds mechanical, judgment items once, delta re-audit — recommended)
  · B (all three as today) · C (drop completeness). **User: "as recommended"** → D3. *Streak at
  3, flagged; Q5 recommendation-free.*
- **Q5 — how many loops before FAIL reaches the user (recommendation-free).** Offered A (one
  number everywhere) · B (per-place numbers, write the plugin audit's down) · C (no cap). **User:
  "i would like it to be standardized somewhere so it is inherited and maintained at one
  place"** — A with a single-home requirement → D4 (home ruled; number + scope → Q6/Q7). Lead
  checked the common block: rules there resolve `${vars}` per command (`explore_model` is
  declared six times), so the one-place form is a literal number in the common rule's text.
  Streak reset (user-composed answer).
- **Q6 — which loops the one number covers.** Offered A (gate loops only — recommended) · B (gate
  + input) · C (all three incl. build). **User: "as recommended"** → D5.
- **Q7 — the number and a deviation lane.** Offered A (one re-audit, fixed, second FAIL to the
  user — recommended) · B (one by default, raisable at run open) · C (two, fixed). **User: "as
  recommended"** → D6.
- **Q8 — who grades at the gate (Q2 un-parked).** Offered A (plain fresh seat, brief carries
  everything, explicit `model:` pin, `validator` retires — recommended) · B (`tech-lead`) · C
  (keep `validator` trimmed). **User: "as recommended"** → D7. *Streak at 3, flagged; Q9
  recommendation-free.*
- **Q9 — what an input reviewer owes (recommendation-free).** Offered A (keep as is) · B (drop the
  verdict word, keep hunt + map) · C (keep the word, drop the completed hunt). **User: "keep as
  is"** → D8.
- **Q10 — how we know the path paid off.** Offered A (one outcome line per gate audit —
  recommended) · B (seeded-defect test) · C (none). **User: "as recommended"** → D9.
- **Q11 — landing order.** Offered A (gate form → plan-QA → retirement, each wave audited under
  the prior form — recommended) · B (plan-QA first, then gate + retirement together) · C (one
  wave). **User: "as recommended"** → D10. Build surface, evidence honesty, and OQ1–OQ5 written;
  record frozen for cold review.

## Review & dispositions

**Cold review (2026-09-19):** solo seat, `mochiko:devils-advocate` on `mochiko:review-brainstorm`,
in-process teammate transport (agent-teams enabled; team config `backendType: in-process`),
persona default `opus` (one tier below the Fable lead — disclosed; same
asymmetry as the 2026-09-19 predecessor sessions). Blind two-message dispatch: 56-angle map in 8
classes written before record contact ([reports/angle-map.md](reports/angle-map.md)); index
fence held. Verdict **`critical-gaps`** — 25 raised → 18 survived: 4 Critical · 9 Important ·
5 Minor, of which 3 coverage (CG1 Important · CG2 Important · CG3 Minor); 7 killed at
cross-examination incl. two of the record's own flags (OQ5, D2's setup classification) that
cleared on check ([reports/review.md](reports/review.md)). D1, D2, D5, D8 survived with no
finding. Lead re-read of the five load-bearing claims against the tree before disposition: C1's
three `validation-constitution` floors · CG1's `total_cost_usd` in `evals/run.py` · I1's AM-2
five-criteria unit (ledger l.125) · I2's F13 composition (personas only measured) · CG3's premise
(the four `sonnet`-default seats produce no gate artifact) — all confirmed.

**Dispositions (user-ruled):**
- **Batch — "as recommended"** (I1 · I2 · I3 · I5 · I7 · M1 · M2 · M3 · M4): folds applied below
  under each anchor once the Criticals and coverage routes are ruled.
- **Criticals — one by one** (C1–C4): each challenges a user ruling; rulings follow.
- **C1 — "as recommended" (option A):** D3's shrink narrows to the primitive-edit gate; setup's
  validate step keeps `validation-constitution`'s completeness floors unchanged. Fold applied to
  D3 below.
- **C2 — "as recommended" (option A):** the "ship with the finding recorded" option leaves D6;
  after a second FAIL the user's choices are fix again or drop; overruling a wrong grader rides
  the existing ledger waiver path, nothing new. Fold applied to D6 below.
- **C3 — "as recommended" (option A):** the gate contract ships as a small plugin skill in the
  validation family, rules in the migration log, rendered and pasted verbatim into the brief;
  the dispatcher writes only unit · paths · pre-pass output; a hand-written contract section is
  a floor miss. Closes I6 (the plain seat runs a skill). Fold applied to D7 and the build
  surface below.
- **C4 — "as recommended" (option A):** one wave-2 unit is graded by both forms — `validator` on
  the old, the plain seat on the new — the two finding sets recorded side by side in the D9 line
  as the baseline later lines read against; one extra seat, once. Closes I4; the plain-seat
  spawn there is I2's cost measurement. Fold applied to D9 and D10 below.
- **Coverage routes — "as recommended":** CG1 **rule inline** (F5 narrowed; cost field on the D9
  line for a launched-session audit) · CG2 **explore now** (re-entered `mochiko:analysis-iterative`
  on batching — Q12 below; the decision lands as D11, one bounded verify round per
  `brainstorm.reopen-born-verify`) · CG3 **defer** to the touch that reads the D9 lines (OQ4
  re-marked `Deferred`).
- **Q12 — how many units one gate grader takes (CG2 reopen).** Offered A (one seat per wave, all
  units, per-unit line, split only on context fit — recommended) · B (one per cluster, today) ·
  C (one per fixed N). **User: "as recommended"** → D11.
- **Folds applied (lead, 2026-09-19):** D1 (M1) · D3 (C1 · I1 · I3) · D6 (C2 · I5) · D7 (C3 ·
  I6 · I2) · D9 (C4 · I7 · CG1) · D10 (C4/I4) · D11 new (CG2) · F5 (CG1) · build surface items
  1, 2, 5, 6 + rider 7 (C3 · I1 · I3 · M2 · M3 · M4 · C4/I2) · evidence honesty (I2 · CG1) · OQ4
  `Deferred` (CG3) · OQ5 closed. 18/18 dispositioned. Verify round dispatched to the same seat:
  bounded — fold fidelity + record fitness, plus D11's reopen-born check; no fresh cold read.
- **Verify round 1 (same seat) — NOT CLEAN: 1 blocking + 5 nits**, 18/18 folds landed, no
  pre-fold position left standing, all four consistency pairs coherent, D11 reopen-born check
  passed (one fitness nit). Lead-repaired 2026-09-19, none reopening a ruling: **B1** D10's
  "last outing" reconciled with the wave-2 double-grade (old form's last *full wave*; final
  outing = the single wave-2 unit; build item 5 heading matched) · **N1** D3 opens
  gate-specific · **N2** `validation-primitive-edit` section set named (review six-set + floor
  read-back) · **N3** D4 confidence mark current · **N4** D11 streak mark added (3, flagged) ·
  **N5** transport corrected to in-process teammate. Delta-check dispatched, bounded to the six
  repairs.
- **Delta-check (same seat) — 0 blocking, 2 nits**, all six repairs landed, neighbours coherent.
  Lead-repaired: **R1** build item 5's closing phrase (wave-1 audits are the baseline's *first*
  old-form samples, the wave-2 double-grade the last) · **R2** D3's heading narrowed to match its
  statement. **Round closed seat-unverified on R1/R2, disclosed** (one-phrase wording repairs;
  the bound held at one verify round + one delta-check, as the 2026-09-19 predecessor sessions).
  Record goes to the user for acceptance.
- **Acceptance (user, 2026-09-19): "accept and then implement."** D11's streak of 3 and the
  reviewer's one-tier-below asymmetry were disclosed at the ask. Landing ritual run (Status line
  above); wave 1 build opened in this session.
