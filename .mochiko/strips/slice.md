# Strip notes — `commands/slice.md`

Entry formats: `strips/README.md`. Wave context: the slice cluster wave (BACKLOG item 7, the
second one-shot-command wave after specify's at v0.13.0). The wave also ran the **D2 conversion
assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint** against
slice's needs (no new shape gap at that wave, when the shape was v2). **Stale as a standing claim:**
the shape is now **v5** (2026-07-30) — see the v0.35.0 section immediately below, which rebuilt this
command goal-shaped; the v0.31.0 entry's "now v4" claim is likewise frozen history.

---

## [v0.49.0] Command retired — `/mochiko:slice` dissolved into `/mochiko:specify`
- **Disposition:** superseded → `commands/specify.md` (intent stage + the spec's Delivery Slices section, co-accepted with the spec); file deleted
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D5+D6)
- **Content:** the whole v8 goal+harness command (Goal: accepted `slices.md` conforming to `slices-template.md` or a reviewed null exit in `slicer-report.md`; Harness incl. the un-homeable-story spec-amendment offer and the graduated-slice halt; Bindings incl. S#/US-#/SC-# ID discipline and the plan-next-step pointer). Full text: git history at v0.48.0.
- **Kept deliberately:** the un-homeable-story amendment offer (folded into specify's Reserved-to-user) · the slicing invariants and Feature-Done machinery (live on in `authoring-slices` + `spec-template.md`'s Delivery Slices section) · the Graduation contract (relocated into `spec-template.md`, staleness-guard bullet dropped — a section cannot drift from its own spec). The graduated-slice halt died (nothing can be graduated at spec time). Accepted cost, ruled: standalone re-slicing of an old accepted spec becomes a specify amend path.
- **Consumers assessed:** router (slice rows removed) · plan/implement (slice-scope re-keyed to the spec section) · ARCHITECTURE.md (section merged into Specify) · `SLICE_STOP` kill-switch retired.

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's conformance conditions (spec stamp, US-# homed once, foundation slice, dependency-closed order, Feature-Done) and the null exit as a first-class outcome · graduated-slice halt (amend deferred) · un-homeable-story spec-amendment offer reserved to the user · accept/amend/reject + accept/override acceptance shapes · echoed-never-minted ID rule · feature-done declared-not-verified · next-step pointers · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Decompose an accepted feature specification into graduation slices via an independent producer→reviewer team loop — a standing task-architect seat authors the slices.md overlay across bounded rounds, a cold devils-advocate seat grades the decomposition from the files, the user accepts at a named gate; spec-gated, null-exit-aware, default-FAIL, bounded, kernel-free. Downstream stages then run per slice instead of whole-spec. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Slice — Graduation-Slice Decomposition (Spec → Slices)

**Goal:** decompose an accepted `spec.md` into an accepted `slices.md` — ordered, named story groups
that graduate through `/mochiko:plan` → `/mochiko:implement` as independent units instead of one
whole-spec batch — authored and independently graded before the user accepts it. `$ARGUMENTS` = optional feature ID or
description; empty or detected-from-workspace is resolved at G1.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate.
Every dispatch carries its own brief in the spawn or send prompt — the seat's role and skill
(named as a hint, the agent decides fit), the exact inputs to Read, where the output lands
(write vs return), the bar it must clear, its peer edges and holds, and the independence
reminder that matches the seat (author: never grade your own output; grader: read the artifact
itself, default FAIL, quote evidence) — the seat owns none of this context and gets all of it
from you; on a retry, a peer-routed gap list is pointed at and the round opened, a relayed one
pasted verbatim. This file is self-contained: slice's whole
contract lives here. **First-spawn probe:** the `task-architect` producer — always the first seat
filled, since nothing reviews before an overlay or a null exit exists.

## Goal

Either `slices.md` exists conforming to `templates/slices-template.md` — exact spec stamp, every
`US-#` homed exactly once, a designated foundation slice, dependency-closed Slice-order,
cross-cutting extend obligations placed, Feature-Done complete — **or** the **null exit** was taken
(no file; the whole-spec reasoning disclosed in `slicer-report.md`); and G4 acceptance has cleared on
whichever of the two shapes was produced.

**Not done:** no `slices.md` and no recorded null exit · a blocking gap open · a departure with no
trail line · out of rounds · G4 unaccepted.

## Seats & checks

| seat | agent × skill | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `task-architect` × `authoring-slices` | authors `slices.md` + `slicer-report.md` from their templates, no placeholder tokens; may instead take the **null exit** — recommend whole-spec, write no `slices.md`, disclose the reasoning in the report; never grades | one **named standing seat** across rounds; **probe seat** | hands each round's output straight to the reviewer; round > 1 reaches it with the gap list in hand |
| reviewer | `devils-advocate` × `review-slices` | grades `slices.md` **and** the `spec.md` it indexes, from the files and never the producer's report → `advocate-report.md`; on a null-exit round grades the **depth call** from `spec.md` + the disclosed reasoning; never authors | cold at first review, standing after — round > 1 re-Reads the revised files | peer-edged with the producer |

**Validation model:** the loop's bounded in-loop critique, every round, from a **single reviewer** —
unsized by design. Its output is **lead-adjudicated input** (the `review-*` family boundary) and
every verdict is yours. No seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn is
the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`** — a
nameless spawn is a one-shot subagent, the forbidden transport; every later round is a
`SendMessage` to that same named seat. Verify from the roster: the `members` array in
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session ID)
must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an agent team;
failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn prompt names
the skill and role itself. Tell the user up front they can watch or message any teammate; announce
each seat in one line when filled; never narrate or reply to teammate housekeeping. A peer-routed
gap list is a **hand-off, not a start signal** — the producer revises only when you open the next
round, and your brief carries that hold.

**Seat lifecycle:** at each gate pause, count each standing seat's completed rounds and recycle at
~≥3 (counted, never observed; the user may order a recycle at any gate). A respawn is a reset:
briefed from the on-disk artifact set alone, versioned successor name (`producer-2`), never the
dead seat's bare name. End-of-need shutdown; no ritual sends.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS`; the resolved `<feature>` (an explicit ID, else the most
  recent in-progress feature under `.mochiko/specs/`, confirmed with the user before the run opens);
  `spec.md` present **and accepted**; whether
  `slices.md` already exists and whether any slice has graduated; `CLAUDE.md`'s governance region ·
  rules: the user · decides: whether the run opens, and on what feature. A
  missing or unaccepted spec **blocks** — point the user to `/mochiko:specify`. **Already
  decomposed:** no slice has graduated (no `slices/<id>/` stage artifacts) → offer re-decomposition
  (overwrite) or stop; **any slice already graduated → halt and escalate** — amending a live
  decomposition is deliberately unsupported, a recorded deferral in `BACKLOG.md`, not an oversight. A
  missing governance region is surfaced (offer `/mochiko:setup`) — governing context, never a
  blocking gate, and never auto-resolved.
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this spec
  — **reversibility** (rework cost if wrong) · **blast radius** (how much downstream work reads the
  decomposition as authoritative) · **precedent** (first-of-kind, or mirroring an audit-cleared
  pattern) · **input confidence** (scored on the artifact under review; a user ruling discounts
  ambiguity risk only, and one introducing new surface raises consistency risk) — plus the process
  you compose from it — the stated default below, or your departures from it · rules: the user ·
  decides: the run's composed process. Rigor scales with cost-of-being-wrong, never task size.
- **G2 clarification** — evidence: a reviewer gap you judge only the user can settle, or a producer
  question it cannot resolve · rules: the user · decides: the answer fed forward. You route each
  finding by judgment: **a genuine judgment call is ruled here**; a **"Research this"** gap
  answerable by investigation routes to a native `Explore` pass, never to the user; an
  **un-homeable cross-cutting story is a spec-amendment finding** — surface it and offer a
  `/mochiko:specify` amendment, never force a placement.
- **G3 escalation** — evidence: a cap trip, a gap set unchanged round-over-round, the kill-switch, or
  a `critical-gaps` verdict · rules: the user, on the last gap list plus the stop reason · decides:
  continue-refining / accept-with-noted-gaps / abort — **the run stays FAIL unless the user
  explicitly accepts**.
- **G4 acceptance** — evidence: your clearing verdict, in one of two shapes · rules: the user ·
  decides: for a **decomposition** (slice count and names, foundation choice, story coverage, SC
  coverage, seams, any noted gaps) — **accept** / **amend** (changes become the gap list; still
  bounded) / **reject** (abort; the draft stays under `.mochiko/specs/<feature>/`); for a **null
  exit** (the whole-spec recommendation and its reviewed reasoning) — **accept** (the pipeline runs
  whole-spec exactly as before, the done-condition satisfied with no `slices.md`) / **override**
  (re-enter the loop directed to decompose; still bounded).
- **Floor gates:** **G1**, the run-start weight card, **G3**, **G4**, and **G2** on its user-ruled
  limbs — the judgment-call ruling and the un-homeable-story amendment offer (its research route to
  `Explore` stays yours). That is all five: each reads `rules: the user`, so none is departable
  however you compose the run, and there is no non-floor gate here to name. Batch rulings into the
  fewest checkpoints that respect them. **No lead-penned surface takes a standing cold grade:** the
  uncertainty carrier is producer-authored — were you to pen a deliverable surface, it would take
  one cold-seat grade non-discretionarily, waivable only by recorded user waiver at the weight card.
- **Bounds:** cap **3** rounds, you count them; no-progress exit when the gap set is unchanged
  round-over-round; kill-switch — stop and escalate if `.mochiko/specs/<feature>/SLICE_STOP` exists,
  checked before each seat send; a G4 amend or override re-enters the same bounded loop; out of
  rounds = escalate, never done. Any bound this run declares — including a declared cost range —
  has you as its named counter, **rises only at a user checkpoint**, and is re-declared only on the
  record; busting a bound escalates, never silently continues.
- **Invariants:** a **wrong-depth** finding flips the outcome shape (decompose ↔ whole-spec) — it
  counts as a round, not a special case. **No devolved branch** — the review is a judgment
  grade, never all-deterministic-CLI, so no gate is skipped and every verdict is yours. Hold every
  revision targeted (fix the flagged gaps; don't regress passing slices). **Feature-done is declared,
  not verified** — the Feature-Done section executes at feature-close, once every slice ships.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a timed
  prompt. The deliverable is written as the work progresses, never reconstructed at the end.

## Bindings

- **Artifacts** under `.mochiko/specs/<feature>/`, alongside the spec they index: `slices.md`,
  producer-authored from `templates/slices-template.md` · `slicer-report.md` and
  `advocate-report.md` (carrying the reviewer's gap IDs), each from its template. IDs: slices `S#`;
  the `US-#` and `SC-#` namespaces are `spec.md`'s — echoed, never minted. Round reports are cleaned
  by default unless the user asks to retain them; **never offer to delete `slices.md`** — it is the
  deliverable.
- **Uncertainty carrier:** producer-authored — `slicer-report.md`'s Open Questions, and on a
  null-exit round its whole-spec disclosure.
- **Fact route:** `spec.md` and the artifacts themselves; a knowledge gap goes to a native `Explore`
  pass.
- **Run-start declaration:** one line at the head of the run's standing artifact — `slices.md` once
  it exists, `slicer-report.md` until then and on a null exit, the surface Recovery already notes the
  resume stage on, moved across whenever the outcome shape flips — for a default run; a run that
  departs from the stated default, or declares non-default bounds, writes a departure record
  at `.mochiko/specs/<feature>/slice-contract.md` beside the reports
  instead — the done-condition and bounds as (re-)declared, departures taken, and the counter
  state Recovery reads on resume. Counted unit: the **round**, the unit the Bounds already count.
- **Departure trail:** one line per departure from the stated default, appended under that same
  declaration as it is taken and carried into G4's evidence — never your context alone; the trail
  names the grading that actually ran. Departure is by record, never by silence.
- **Next step:** `/mochiko:plan <feature> --slice <foundation-id>`, or plain `/mochiko:plan
  <feature>` after an accepted null exit.

## Recovery

Note the resume stage on `slices.md`, or on `slicer-report.md` before the overlay exists, with the
run's counter state — rounds consumed · bounds declared · departures taken. Sessions and teams do
not survive `/resume`, and a shared account limit can throttle the team and the main session
together — escalation then has nowhere to go but pause. Resume from workspace evidence, never a
context `phase` field, respawning only what the stage needs — a respawned producer re-reads
`spec.md` + the gap list, and a respawn is cold by design.

| Evidence | Resume at |
|----------|-----------|
| no `spec.md`, or unaccepted | G1 (entry blocked) |
| `slices.md` present with graduated slices (`slices/<id>/` artifacts exist) | G1 (halt — amend deferred) |
| spec accepted; no `slices.md` and no `slicer-report.md` | loop (produce, round 1) |
| `slices.md` (or a null-exit `slicer-report.md`) present, no `advocate-report.md` this round | loop (review) |
| `advocate-report.md` not `ready`, within the cap | loop (produce) |
| `advocate-report.md` `ready`, not yet accepted | G4 |
| accepted | finalize — report the outcome (slice / foundation / story counts and SC coverage, or the accepted whole-spec recommendation), the round count, the round reports, a suggested commit (`docs: slice <feature>`), and the next step |
| `SLICE_STOP` present | escalate (G3) |
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated shape/loop-discipline reads and "in the mochiko command shape" framing left; G2's preference/knowledge-gap taxonomy vocabulary reworded to plain lead-judgment routing; "P11 is producer-authored" reworded.
- **Kept deliberately:** all gates/bounds/bindings/recovery — plus inlined weight-card factors, floor rules, transport, lifecycle, mesh hold, ground rules, counter-state recovery.
- **Consumers assessed:** none.

---
**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Design-record citation, preamble
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
Design record:
`.mochiko/brainstorms/vertical-graduation/synthesis.md`.
```
- **Kept deliberately:** the goal line's substance — slices graduate through plan and implement as independent units, authored and independently graded before the user accepts.

## [v0.43.0] The `<!-- shape-form: v7 -->` marker retired from the preamble
- **Disposition:** superseded → deleted. The marker was added by this same version's conversion
  entry below and retires in the same version, at the wave close.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-01 wave-close
  ratifications row, *shape-form marker retirement when the last command converts*; the trigger
  was written into the marker clause itself). **Ground and full record:**
  `.mochiko/strips/command-shape.md` [v0.43.0 wave close], entry 1 — *The form marker and its
  Conformance bullet retired* — not restated here.
- **Content (verbatim):** `<!-- shape-form: v7 -->`
- **Kept deliberately:** the entire preamble otherwise — goal line, obligated reads, probe seat —
  and every P18–P20 binding the marker used to gate. The slots bind unconditionally now; nothing
  the marker declared was lost, because the marker declared only which grading branch to take, and
  there is one branch.
- **Consumers assessed:** `validation-command-shape` check 20 was the sole grep consumer and its
  form branch retired in the same ceremony. All six commands swept together — a marker left in any
  one of them would be the only file in the library still declaring a form.
- **Measured:** `commands/slice.md` **9,929 → 9,904 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the v6→v7 conversion

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01 — the
lead-owned-process-flexibility row plus the shape-v7 wave-close ratification row), with **D4** ruling
**convert-on-touch** and all six commands staying v6-form. The user **widened that to all six
commands in one wave on 2026-08-01**, so slice converts here rather than at its next touch. The
audit-cleared precedent is `implement` (this wave, `.mochiko/strips/implement.md` [v0.43.0]), whose
two check-6 ceiling terms — **+120** on Constraints where P18 binds, **+110** on Bindings where the
P19/P20 pair binds — this conversion is the second body measured against. BACKLOG:
"convert-on-touch residuals".

**Post-conversion measurement, all blocks, body-only in words** (`## Heading` lines excluded, per
check 6): preamble **118 → 118/130** (the conversion's +4 was the form marker, retired at the wave close) · Goal **125 → 83/150** · Seats & checks **176/190** (unchanged)
· Constraints **458 → 572/750** · Bindings **112 → 215/236** · Recovery **160/172** (unchanged). Term
derivation as check 6 requires: **G = 4 → 5** — the four prior gate lines plus the run-start weight
card, all five carrying the complete three-part `evidence:`/`rules:`/`decides:` form — so Constraints
is 90·(5+2) = 630 **plus the +120 P18 term** = 750. **S = 2** and **R = 8**, both unchanged. **A = 3**,
unchanged (`slices.md` · `slicer-report.md` · `advocate-report.md`), and slice binds no KM landing, so
Bindings is 90 + 12·3 **plus the +110 P19/P20 term** = 236. Neither v7 term needed a re-key: at 572/750
and 215/236 this conversion lands inside the band implement calibrated them on, and a conversion that
merely fits is not a re-key case.

> **Two counting notes, recorded so the next auditor does not re-derive them.**
> 1. **The preamble baseline is 118, not the 110 this file's [v0.35.0] section published — measured at
>    the landing commit, not inferred.** On `git show b32dd82:plugins/mochiko/commands/slice.md` (the
>    v0.35.0 landing, "Land goal-shape wave v0.35.0") that preamble measures **118 title-included ·
>    110 title-excluded**, so its note was written title-**excluded**; and the preamble text is
>    byte-identical from b32dd82 through this conversion (v0.37.0 touched G1 only), so the baseline
>    transfers unchanged. The v0.43.0 precedent measures title-**included** — implement's published
>    **114** reproduces only with the title counted (103 title-excluded) — and that is the graded
>    reading here. *(This sentence cited **118** until the wave-close sweep. 118 is implement's
>    marker-era title-included figure: its note never published it, and it reproduces from
>    implement's file under no convention, so the evidence offered here was false even though the
>    conclusion it supported was right. Note the collision that hid it — **118 is slice's own**
>    title-included preamble, appearing four more times in this blockquote, so precedent and slice
>    read as coincidentally sharing a figure. The precedent's number is 114, not slice's 118.)*
>    **The convention is per-note, not per-wave, so do not generalize this correction.** At the *same*
>    commit, `brainstorm`'s note published **107**, which is its title-**included** figure (100
>    excluded): two notes from one wave, opposite rules. A later conversion must re-measure its own
>    baseline at b32dd82 rather than apply a blanket ±title correction in either direction.
>    slice's preamble measures **118/130, 9.2% headroom** *(published as 122/130, 6.2% while the
>    4-word form marker stood; re-measured at the wave-close sweep that retired it — which also
>    retired the claim it carried: at 9.2% the preamble is the file's **fourth**-tightest block,
>    behind Recovery 160/172 (7.0%), Seats 176/190 (7.4%) and Bindings 215/236 (8.9%). Point a
>    later addition's headroom question at Recovery, not here.)*
> 2. **`slice-contract.md` is not counted in A**, on implement's recorded ground: P19 names it as a
>    **departing** run's per-run carrier, it is neither a deliverable nor a round report, and it exists
>    only on a departing run. Counting it (A = 4) would raise the Bindings ceiling to 248 and so only
>    loosen the check; the conservative reading is the one measured here.

## [v0.43.0] The Goal's end state loses its reviewer-clearance clause and its lead-read clause

- **Disposition:** superseded → deleted from the Goal. Both clauses' rules are unchanged at their
  ledgered homes (below); only the Goal echo left. The end state is now artifact state + acceptance:
  the conforming `slices.md`, **or** the recorded null exit, plus G4.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01: *"Goal
  blocks lose process residue. Done = artifact state + floor compliance + user acceptance"*; graded by
  `validation-command-shape` check 23, v7-form only). Both clauses are the "the review ran / returned
  PASS" class verbatim — the first names the reviewer's returned status, the second the lead's own
  read step.
- **Content (v6, verbatim — the two clauses that left):**
  ```
  `devils-advocate` recommends
  `ready` grounded in the files, or on a null-exit round on the depth call; you Read `spec.md` + the
  artifacts + the report and confirm no blocking gap remains;
  ```
- **Protected content, checked at source before removal — none of it left the file.** The [v0.35.0]
  CS-D8 survivor re-grade ledger is the authority on where each protected line lives, and it homes
  none of this text in the Goal:
  - *"Cold reviewer **arrival**; producer↔reviewer peer edge; **every verdict the lead's**"*
    (DECISIONS 2026-07-30, Layer-2 mesh + v0.31.0) → ledgered to "the **validation-model line**",
    which is untouched and still reads: "Its output is **lead-adjudicated input** (the `review-*`
    family boundary) and every verdict is yours."
  - *"The **null-exit round is graded and ruled exactly as before**"* (v0.31.0 *Kept deliberately*) →
    ledgered to "Reviewer row's depth-call clause + G4's second shape", both untouched. The reviewer
    row still reads "on a null-exit round grades the **depth call** from `spec.md` + the disclosed
    reasoning".
  - *"The **null exit** — recommend whole-spec, no file, reasoning disclosed"* (DECISIONS 2026-07-02,
    vertical-graduation) → the Goal **is** one of its five ledgered homes, as "the `or` branch" — and
    that branch survives **verbatim**, untouched by this entry. The null exit is a routing decision;
    removing it was never on the table.
- **Kept deliberately:**
  - **The whole conformance clause** — spec stamp, every `US-#` homed exactly once, foundation slice,
    dependency-closed Slice-order, cross-cutting extend obligations, Feature-Done complete — unedited.
    These are the three elements the v0.35.0 repair round had to *restore* after a draft dropped them;
    they were re-checked at source this wave and are byte-identical.
  - **The two-shape end state** — decomposition **or** null exit — and G4 acceptance on whichever was
    produced, both unedited.
- **Consumers assessed:** not a shared primitive. Two cross-file consumers checked: the grader's check
  23 (`.mochiko/strips/validation-command-shape.md` [v0.40.0]) and the other five commands, whose Goal
  blocks are unaffected — the residue clause is v7-form-only and each command converts on its own file.

## [v0.43.0] Two not-done states superseded as process residue

- **Disposition:** superseded → deleted from the Goal. Neither rule moves: both are lead-process steps
  restated as done-conditions, and their homes are elsewhere in the file, unedited.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above). Check 23's residue class reaches
  a done-condition naming "a seat's choreography" or "the validator returned PASS"; the first state is
  the reviewer's returned status, the second is the lead's own read step. Neither can be rescued as a
  floor gate: neither is a gate line at all, so check 21's `rules: the user` test never reaches them.
- **Content (v6, verbatim):**
  - `a reviewer status short of `ready``
  - `the reviewer's status taken as the gate without your read`
- **Protected content:** the second echoes the *every verdict the lead's* row traced above, whose
  ledgered home is the validation-model line — where it stays. The v0.14.0 *Verdict-ownership
  triplication* entry already deduped this rule once, to the then-Contract's clause; the v0.35.0
  goal-shape rebuild re-homed that clause to the validation-model line, and this entry retires the
  Goal's surviving third copy rather than the rule.
- **Kept deliberately:** the reviewer's grading obligation and its two shapes (reviewer row), the
  lead's verdict ownership (validation-model line), the `critical-gaps` escalation trigger (**G3**),
  and **`a blocking gap open`** — untouched in the not-done list, because it names a state of the run
  rather than a step that did not run.
- **Consumers assessed:** as above — not a shared primitive; grader check 23 and the five other
  commands, both unaffected.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part countable
  form, taking **G from 4 to 5**. Sited after **G1** so the process is composed once the feature and
  the accepted spec are resolved, and before the loop it composes.
- **`**Floor gates:**`** (P18) — the floor set, which is **all five gates**, with the ground stated
  (each reads `rules: the user`) and the non-floor absence stated rather than inferred; **G2** scoped
  to its two user-ruled limbs; and the lead-penned-surface element stated as an **absence**, slice's
  P11 being producer-authored.
- **`**Run-start declaration:**`** (P19) and **`**Departure trail:**`** (P20) in Bindings — the
  declaration on the run's standing artifact for a default run, an instantiated `slice-contract.md`
  for a departing one, and the **round** named as the counted unit (check 22), the same unit the
  Bounds already count.
- **One new not-done state** — `a departure with no trail line`, the honest-trail invariant made
  visible in the Goal as floor compliance.

**Three judgments made here rather than deferred, flagged for the grader.**

1. **The floor-gate set is all five, and the ground is *who rules*.** slice has no `rules: you` gate —
   unlike implement's cycle checkpoint, which check 21's test excludes by construction — so every gate
   line in the file reads `rules: the user` and every one is floor. **G1 was not left departable on
   implement's reasoning, because implement's reasoning does not transfer.** There, G1's confirm was
   safely departable *because the package gate is floor and re-presents the resolved feature before the
   run opens* — the invariant had a second carrier. slice has no second carrier: its G1 **is**
   structurally implement's package gate, deciding "whether the run opens, and on what feature" and
   carrying both the unaccepted-spec block and the **already-graduated halt**. Compose G1 out and no
   user stop precedes the run at all.
2. **The precedent's blocking-vs-floor lesson is consumed inside G1, not dodged.** G1's governance limb
   states "governing context, never a **blocking** gate" — that settles the *blocking* axis and says
   nothing about the *floor* axis, which `rules: the user` settles outright. Marking G1 floor therefore
   changes no behaviour: the governance-region absence stays surfaced-and-non-blocking exactly as
   written, while the ruling it opens stays the user's. **G2 is the one scoped limb:** its preference
   ruling and its un-homeable-story amendment offer are the user's, but its **"Research this" route to
   `Explore`** is excluded — that route exists precisely to keep a knowledge gap *off* the user, so it
   is not a user ruling that could be lost.
3. **The declaration and the trail share one surface, and that surface moves.** Both land at the head
   of the run's standing artifact — the same one Recovery already notes the resume stage on, so a
   resumed lead finds declaration, departures and resume state in one place. slice needs the moving
   clause that implement did not: **`slices.md` does not exist at run start and never exists on a null
   exit**, so binding P19 to the deliverable alone would leave a null-exit run with no declaration home
   at all. Hence `slicer-report.md` until the overlay exists and on a null exit, **moved across
   whenever the outcome shape flips** — a real path in this workflow, not a hypothetical: a wrong-depth
   finding flips decompose ↔ whole-spec as a normal round (Invariants), and a G4 override re-enters the
   loop directed to decompose. Without the carry-across, that flip strands the trail on a report the
   next round overwrites, which is exactly the failure check 23 grades. **`advocate-report.md` was
   rejected as a home** for implement's reason — a round report the next round overwrites is a trail
   that can vanish mid-run.

   **One residual tension, flagged rather than papered over.** Bindings states "Round reports are
   cleaned by default unless the user asks to retain them", and on a null-exit run the declaration and
   trail live on `slicer-report.md`, which is a round report. It is not edited here, because the Goal
   already requires that file to carry the null exit's disclosed reasoning — on a null-exit run it *is*
   the run's record, so cleaning it would un-do the done-condition. If the grader reads that as too
   implicit, the fix is one clause on the artifacts line, not a change to this binding.

**Recovery left untouched, deliberately** — on the precedent's ground. The counter-state clause is home
doctrine (v7 Recovery block); slice's pause line, "Note the resume stage on `slices.md`, or on
`slicer-report.md` before the overlay exists", already names the exact two surfaces P19 binds, so no
edit was owed. The 8-row evidence table is likewise unchanged: every row keys on workspace evidence,
which a composed run leaves exactly as a default run does.

**Not a re-add of the [v0.14.0] contract fill, and flagged so the grader can rule it either way.**
That entry relocated "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/slice-contract.md`
… The filled artifact is the inspectable proof — not this command body", on the ground that the shape
retired per-run fills whose **values are constant at authoring time**. P19 revives the same filename on
the opposite ground: **OQ-2 as ruled at A2** makes a departing run's values genuinely vary per run,
which is what un-does that premise — and only for departing runs, a default run writing one line
instead. Different trigger, different ground, ruling-created, so it is logged here as an addition
rather than as a `RETURNED:` entry. implement's conversion made the identical call for
`implement-contract.md` against its own [v0.17.0] strip, and cleared audit.

### Measured file growth — the light-site half of R21

`lead-owned-process-flexibility` **R21** carries a recorded-open obligation: *a measured cost estimate
for declaration + trail + composition on one light and one heavy run* (verify N3, narrowed by **A3** to
the estimate alone). implement measured the **heavy site** and recorded that "the light site stays
unmeasured, so R21 remains open at half". slice is a light site — the shallowest-reduction command of
the v0.35.0 wave, 1,230 words against implement's 2,014 — so this section closes that half. Figures are
measured after the last edit.

**File growth.** `commands/slice.md` **8,715 → 9,904 B** (+1,189; words 1,230 → 1,405, +14.2%).
Attribution, each construct measured on its own text, UTF-8 bytes:

| construct | bytes | words |
|---|---|---|
| ~~`<!-- shape-form: v7 -->` marker~~ — added here, **retired at the wave close** | ±0 | ±0 |
| run-start weight-card gate line (P7) | +261 | +44 |
| `**Floor gates:**` — floor set + G2 scoping + P11 absence (P18) | +459 | +70 |
| `**Run-start declaration:**` (P19) | +560 | +77 |
| `**Departure trail:**` (P20) | +162 | +26 |
| Goal block, D6(b) residue strip | −253 | −42 |
| **net** | **+1,189** | **+175** |

**The light-vs-heavy read.** In absolute terms the two sites cost nearly the same — **+1,189 B here
against implement's +1,544** — because the v7 constructs are fixed-size obligations, not
proportional ones: four of the six rows above would measure within a few dozen bytes on any command.
In *relative* terms they diverge, and that is the finding: **+14.6% on slice against +11.5% on
implement**, because the same fixed bill lands on a body two-thirds the size. **The v7 conversion is
regressive — the lighter the command, the larger the share it pays.** slice's P19 is the one
genuinely larger construct (+560 B against implement's +473), and it is larger for a workflow reason,
not a drafting one: slice's declaration home moves with the outcome shape (judgment 3), which
implement's fixed `tasks.md` home never has to say. Per-run read cost: **+1,214 B on every slice run**,
the command being an obligated read once per run, on top of the shared `command-shape.md` floor every
team-form run already pays.

**Run-time cost, estimated and marked as one** — no live v7 slice run exists. A **default** run pays
one declaration line (~30–60 w, ~200–400 B) and nothing else; a departure pays ~15–25 w a line; a
**departing** run additionally reads `templates/workflow-contract.md` (**5,572 B**) and writes a filled
`slice-contract.md` of comparable size. Same three components implement measured, same conditional
structure — the flexibility is bought by the line, and a default run never touches the largest item.

---

## [v0.37.0] `@`-reference recovery superseded — the platform bug it named is resolved
- **Disposition:** superseded → user ruling (2026-08-01). The bug-attributed re-enter workaround retires; the detected-feature resolution (already in G1's evidence) gains a confirm.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** "Empty `$ARGUMENTS` (the known `@`-reference drop bug) → ask the user to re-enter it, or to confirm the detected feature."
- **Kept deliberately:** the detected-feature confirm — G1's evidence resolution now reads "the resolved `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the user before the run opens)". Only the re-enter workaround and the bug attribution left.
- **Consumers assessed:** five-command recovery — see the shared consumer list in the `strips/plan.md` v0.37.0 entry.
- **Protected-set note:** as recorded in the plan entry — record §7's protection premise for this recovery is spent now the bug is resolved; deliberate supersession, not a check-14 re-drop.

# v0.35.0 — the goal-shape rebuild, step 4 of 4 (CS-D10)

**Wave context:** command goal-shape rebuild, **step 4** — the remaining-five wave
(`brainstorm` · `specify` · **slice** · `implement` · `setup`), design
`.mochiko/brainstorms/command-succinctness-strip/record.md` (CS-D3/D4/D5 + D8 + D10; `DECISIONS.md`
2026-07-30), against **shape v5** with the obligated `mochiko:loop-discipline` read **retained** —
its drop is deferred to a named live-run trigger (pilot-checkpoint ruling 5), so a v5 command that
omits it is non-conformant, not early. Worked example: the audit-PASSed `plan` pilot (v0.34.0).
slice is **not** KM-carrying (the five KM commands are brainstorm/specify/plan/implement/setup), so
it takes no `.mochiko/memory/knowledge-management.md` read and its Bindings carry no KM landing. It
declares the **in-loop critique** branch of P6, so `templates/sized-end-stage-review.md` must not
appear in it — grepped, absent (check 1's negative direction).

**Measured: 1,611 → 1,242 words (−22.9%), 11,968 → 8,792 B (−26.5%)** — `wc`-measured after the
final edit round, per the pilot's standing habit (re-sweep every figure after *each* round; three
stale-headline instances in the pilot build all traced to a summary written before the last edit
landed). Against the wave's pre-authored floor of **1,076 w: +166 (+15.4%)** — over, the safe side
of CS-D8, since landing materially *under* a floor row would signal dropped content. slice's
reduction is the shallowest of the wave by design: it was already a *converted, twice-stripped*
command (v0.14.0 wave + v0.31.0 mesh re-conform), so the flow/Contract narrative it carried was
thinner than the pilot's 2,873 words.

Block sizes against the grader's ceilings (terms as the grader counts them — **G=4** gate lines,
**S=2** seat rows, **A=3** artifacts, **R=8** resume rows): preamble 110/130 · Goal 125/150 · Seats
& checks 176/190 · Constraints 470/540 · **Bindings 112/126 (88.9%)** · Recovery 160/172.

> **Authorship note, recorded because the audit trail depends on it.** This file was rewritten
> goal-shaped twice inside the same wave: a first goal-shaped draft landed on disk at 23:27
> (1,308 w) from a parallel wave author, colliding with this seat's assignment; this seat's own
> draft was rejected by the write-guard ("modified since read"). Rather than overwrite a peer's
> just-written text, the collision was reported to the lead and the on-disk draft was **repaired in
> place** — its prose survives wherever it was conformant. The repair set is enumerated below under
> *Defects carried by the first goal-shaped draft*. **Author ≠ grader still holds:** both authoring
> hands are producer-side; nothing here is a self-grade, and the independent
> `validation-command-shape` audit is unrun at the time of writing.

## [v0.35.0] Defects carried by the first goal-shaped draft, fixed at the repair round

Logged as wave evidence, not as strips — none of this content left the command; it was mis-sized,
mis-classed, or missing.

- **Two check-6 ceiling FAILs.** Seats & checks **195/190** — the reviewer row enumerated
  `advocate-report.md`'s contents ("severity-classified findings, product-framed clarifying
  questions, a recommended verdict (`ready` / `needs-revision` / `critical-gaps`)"), which is
  `mochiko:review-slices`' own description text plus `templates/advocate-report-template.md`'s
  structure; cut to the report name (see the skill-owned strip below). Bindings **128/126** — caused
  by the Governance-brief entry stripped below. Both now PASS.
- **Gates were not in order** — the draft kept the historical labels, which forced the sequence
  **G1 · G3 · G5 · G4**: both a gap (no G2) and a mis-ordering (acceptance before escalation)
  against the anatomy's "the gates, **in order**". Resolved by the renumber below.
- **Dropped from the declared end state:** the designated **foundation slice**, **dependency-closed
  ordering**, and **cross-cutting extend obligations** — the last absent from the file entirely.
  All three are `DECISIONS.md`-traceable (vertical-graduation, 2026-07-02) and the v0.14.0
  slicing-judgment strip kept "the converted goal names the deliverable's parts once" as the
  surviving half of its relocation. Restored into **Goal**. This is the same failure class the
  pilot's fix round named — loss inside a *compressed clause* whose surrounding sentence still
  reads complete — arriving this time in the Goal rather than in a gate line.
- **P10 gap:** no ID namespace was bound (the shape requires each artifact's path *and* ID
  namespace). Added to Bindings: slices `S#`; `US-#` / `SC-#` are `spec.md`'s, echoed never minted.
- **Generic FAIL prose in the not-done states** — "default **FAIL**" is shape doctrine (the anatomy
  states the initial state), not a state of this workflow, and check 13 grades exactly that;
  replaced with the structural state "no `slices.md` and no recorded null exit". The
  G-amend/override clause sat in the Goal and was **missing from Bounds**; moved to Bounds, its own
  block.
- **In-file duplication:** Bindings re-listed the Goal's conformance conditions (story homing,
  Feature-Done, spec stamp). Goal states the checkable condition; Bindings states the referent.
  Deduped to Bindings' path + template + ID namespaces.

## [v0.35.0] KEPT re-grade — "No G2" **superseded with grounds** (the wave's one contested call)

- **Disposition:** superseded → the goal-shaped anatomy's ordered gate list, via a contiguous
  **gate renumber**. The survivor-provenance entry below (`[v0.14.0] KEPT: "No G2 …"`) is retired;
  the note no longer appears in the command.
- **Tier failed:** n/a — supersession by ruling (**CS-D8** re-grade against the new anatomy,
  "never auto-carried"), executed under the step-4 briefing's instruction to re-grade this entry
  honestly.
- **Content (the retired note, verbatim as it last shipped):** "**No G2** — slice is
  single-reviewer, so plan's feasibility-rejection slot is intentionally unused."
- **Grounds, stated plainly.** The entry's Tier-2 evidence was a *reader-inference* failure:
  "without it, an auditor/reader seeing G1/G3/G4/G5 reads the G2 gap as a dropped gate." That
  failure has two preconditions — a **gap in the file's gate labels**, and a reader inferring a
  dropped gate from it. In the goal-shaped file the gates are an explicit ordered list in
  Constraints, and the shape home rules the label to be the command's own ("`G3`, or a plain name
  where the workflow numbers nothing; what makes it a gate is the three parts, not a number").
  Renumbering to **G1 entry · G2 clarification · G3 escalation · G4 acceptance** leaves the first
  precondition no path: there is no gap to misread, and the gate set is countable by the grader's
  own `G` term (4). Checked before ruling it: **nothing outside the command references slice's gate
  numbers** (`implement.md`'s and `strips/implement.md`'s "G5" are implement's own) — grepped, so
  the renumber breaks no cross-file pointer.
- **Kept deliberately:** the entry's *second* element — the record of the deliberate
  **single-reviewer** structure — is not carried by prose but by structure: P5's table has exactly
  one grading row, and the validation-model line names "a **single reviewer**". The comparison to
  plan's feasibility-rejection slot is provenance, and its home is this note.
- **Contested, and why it is flagged rather than settled here:** the honest counter-argument is
  that the renumber *is* the supersession's enabling condition — had the historical numbering been
  kept (as the first goal-shaped draft kept it), the gap would still exist and the note would still
  bind. So the call is a renumber decision first and a survivor re-grade second. It is put to the
  audit in exactly those terms. The tie-breaker taken: keeping the old labels also produced the
  **out-of-order** gate sequence above, so the numbering could not be preserved *and* conformant.
- **Line-cite drift:** `BACKLOG.md` (step-4 item) and the session record §9.4 cite this survivor as
  `.mochiko/strips/slice.md:111–115`. Those line numbers now point elsewhere — the retired entry is
  reachable by its heading, `[v0.14.0] KEPT: "No G2 …"`, at the bottom of this file.

## [v0.35.0] The flow body and the Contract section retired into the five-block anatomy

- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters` → the preamble's
  probe-seat line (the rest relocated, below) · `Session constraints` → **G1** + Bindings ·
  `The seats` → the **Seats & checks** table (both seats, their spawn timing and peer edges) ·
  `The flow`'s Triage → **G1** · its Decomposition loop → the seat rows + **G2**/**G3** + Bounds +
  the loop invariants · its Acceptance → **G4** (both shapes) · its Finalize → the Recovery table's
  accepted row + Bindings' Next step · `Recovery` → **Recovery** (unchanged in name and role, rows
  re-derived). The `Contract` section's four clauses → **Goal**
  (done-condition + not-done states), the **Seats & checks** table (producer↔validator), and
  **Constraints** (bounds + gates).
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is restructured · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** the `## Team-form parameters` / `## Session constraints` / `## The seats` /
  `## The flow` / `## Contract` / `## Recovery` sections plus the closing footer — 1,611 words of
  ordered procedure
  and appendix. Not reproduced verbatim: every *rule* inside them is resolved individually in the
  ledger below, and the deleted remainder is connective narration (step sequencing, "loop to
  produce", "initialize `round = 1`", and the lead's job restated per phase). Recoverable in full at
  `git show c47684d:plugins/mochiko/commands/slice.md`.
- **Gate renumber map (consequent, and the grounds are the entry above):** G1 entry → **G1** · G3
  clarification → **G2** · G4 escalation → **G3** · G5 acceptance → **G4**. The former G2 was never
  occupied. Every in-file reference was repointed (Goal ×2, Recovery ×2); no external pointer
  existed.
- **Kept deliberately:** every gate, bound, routing class, trigger, guard and artifact binding — see
  the ledger.

## [v0.35.0] The `What you own (not the seats)` footer deleted

- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate, the same class the pilot deleted. 106 words restating
  the entry/guard/governance prerequisites, the loop and gap routing, the verdict ownership, the
  human gates, and the producer↔reviewer collapse prohibition — each of which is now a Constraints
  line, a Seats-table cell, or shape doctrine.
- **Kept deliberately:** the one clause with no other home — "verifying each dispatch wrote its
  expected files (a missing output → log and ask retry/abort — a null-exit round expects
  `slicer-report.md` only)" — is **not** dropped. Per the pilot's precedent it survives as the
  Recovery block's evidence-driven resume, where a null-exit `slicer-report.md` with no
  `advocate-report.md` is an explicit row that advances to review, plus G3's escalation menu.

## [v0.35.0] Team-form parameters section relocated to the shape home

- **Disposition:** relocated → `templates/command-shape.md` **Layer 2** (Hard requirement — agent
  teams · Seat transport · the addressability probe · the no-fallback `Contested` bet). The
  surviving parameter is P2, the probe seat, now one clause in the preamble.
- **Tier failed:** 1 (altitude).
- **Content:** "Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The
  **authoritative first-spawn probe** is the producer — always the first seat filled. Transport
  mechanics + the addressability check: `templates/agent-dispatch.md` (Seat transport). The
  no-fallback bet is the same `Contested` dogfood-pilot ruling as the other team-form commands."
- **Note — the relocated pointer was also stale:** Seat transport left `agent-dispatch.md` for
  `command-shape.md` Layer 2 at **v5** (CS-D6), so the command's citation had been pointing at a
  section its target no longer carried. The rebuild retires the pointer rather than repairing it.

## [v0.35.0] Governance dispatch-brief line relocated to the briefing home

- **Disposition:** relocated → `templates/agent-dispatch.md` field 3 ("Input(s) to read … incl. the
  governance obligated-read line naming relevant `.claude/rules/mochiko/` files"), which every
  dispatch already honors. G1 keeps what is slice's own: the governance region as entry evidence,
  and surfaced-never-auto-resolved.
- **Tier failed:** 1 (altitude — true of every command that dispatches a producer under
  governance; the audit-PASSed `plan` carries no such line).
- **Content:** "**Governance prerequisite:** … Present → governance reaches the producer natively at
  spawn; add to its brief the one-line **obligated read** naming the `.claude/rules/mochiko/` files
  relevant to what it authors." — carried into the first goal-shaped draft as a Bindings entry, where
  it was also the cause of that block's ceiling FAIL.
- **Consumers assessed:** n/a — command-local; the home is a shared template, unedited by this wave.

## [v0.35.0] Skill-owned and shape-owned content stripped from the command body

- **Disposition:** relocated → the primitives that already own it (no new home written; each
  verified by reading the owner this run).
- **Tier failed:** 1 (altitude).
- **Content:**
  - The **`advocate-report.md` contents** enumeration — "severity-classified findings,
    product-framed clarifying questions, and a recommended verdict (`ready` / `needs-revision` /
    `critical-gaps`)". Homes: `mochiko:review-slices` (its description states the severity
    classification and the 3-state recommended verdict) + `templates/advocate-report-template.md`.
    **Kept deliberately:** the reviewer row still names the report as its output and still binds the
    two files it grades from, which is slice's own parameter.
  - The **check-8 keyed markers**: "the reviewer's status is **input, never the gate**",
    "disjoint agents, disjoint skills, **structurally separated**", and "a reviewer respawn is
    **cold by design**". Home: `command-shape.md` Layer 2 (Clearing · Independence by structure).
    All three are markers the grader's floor now greps for, so they could not survive as prose.
    **Kept deliberately:** their behavior — "every verdict is yours" on the validation-model line,
    independence shown by the table rather than asserted, and recovery's respawn treated as
    routine.
  - The entry-gate **memory-model parenthetical** — "(workspace evidence — there is no context-file
    `status`)". Home: `command-shape.md` Layer 1 Recovery ("never a context `phase` field"). This is
    the *second* instance of the relocation already logged at v0.14.0 for the Recovery preamble;
    the entry gate carried its own copy.
  - "**Single reviewer, never the producer**" → the trailing clause dropped as the no-self-grading
    rule (home: Layer 2 + `agent-dispatch.md`'s one hard line; P5's table is where it is checkable).
    **Kept deliberately:** "a **single reviewer**", which is slice's own cardinality and the
    surviving half of the retired "No G2" evidence.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/Tier-2-evidenced lines, **and** every
line traceable to a `DECISIONS.md` row. slice carries **one** `KEPT:` entry (the "No G2" note) plus
the *Kept deliberately* fields of the v0.31.0 supersession and the v0.14.0 conversion note, plus the
row trace. Grepped before any cut. **19 rows: 17 survive translated · 1 superseded with grounds · 1
not-applicable (never in slice's text) · 0 dropped.**

Per the pilot's wave discipline, the routing classes and named-cause recoveries were grepped
individually rather than trusted to a gate line that reads whole — which is how the three missing
end-state elements above were caught.

| protected line | source | resolved |
|---|---|---|
| `slices.md` overlay of ordered, named story groups; downstream stages run per slice | DECISIONS 2026-07-02 (vertical-graduation) | Preamble goal line · Goal · Bindings · frontmatter `description` |
| A designated **foundation slice** | same | Goal (end state) · G4 evidence · Recovery's finalize row. **Restored at the repair round** |
| **Dependency-closed** Slice-order | same | Goal (end state). **Restored at the repair round** |
| Cross-cutting **extend obligations** placed | same | Goal (end state). **Restored at the repair round** — absent from the first draft entirely |
| **Feature-Done declared at decomposition, verified at close** | same | Goal (section complete) + the loop invariant "declared, not verified — executes at feature-close, once every slice ships" |
| The **null exit** — recommend whole-spec, no file, reasoning disclosed | same | Goal (the `or` branch) · producer row · reviewer row (grades the depth call) · G4's second shape · two Recovery rows |
| An **un-homeable cross-cutting story** is a spec-amendment escalation, never a forced placement | same | **G2**, third routing class, with the `/mochiko:specify` amendment offer |
| Decomposition adds **one** grouping-acceptance gate | same | **G4**, two-shape |
| Staleness guard · graded amendment · extend-mode · artifact layout | same, but homed in `templates/slices-template.md`'s **Graduation contract** | Never slice's text (slice *produces* the contract; the downstream commands consume it) — no drop |
| Cold reviewer **arrival**; producer↔reviewer peer edge; every verdict the lead's | DECISIONS 2026-07-30 (Layer-2 mesh) + v0.31.0 | Reviewer row (cold at first review, standing after) · both peer-edge cells · the validation-model line |
| **No devolved branch** — slice has no deterministic-CLI verification, so the branch cannot apply; declared, not left implicit | v0.31.0 *Kept deliberately* | Loop invariants, in those terms |
| The **null-exit round is graded and ruled exactly as before** | v0.31.0 *Kept deliberately* | Reviewer row's depth-call clause + G4's second shape; the round reaches review whether or not a file was written |
| Standing producer holds extend-obligation placements, depth reasoning across a wrong-depth flip, foundation/Feature-Done coherence | v0.14.0 conversion note (the retention bet) | Seat row carries the operative fact ("one **named standing seat** across rounds"); the *rationale* stays in the conversion note below, its single home |
| A **wrong-depth** finding flips the outcome shape and counts as a round | v0.14.0 conversion note + current body | Loop invariants, in those terms |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a *named cause* (the `@`-reference drop bug) **and** a two-option prompt (re-enter, or confirm the detected feature) | record §7 protected set (the `command-altitude` retrofit-regression warning); the pilot lost this and restored it under audit | **G1**'s decides-clause, cause and both options intact — grepped explicitly because the pilot's audit found it dropped there |
| The gap-routing classes — **preference** ruled at the clarification gate · **"Research this"** knowledge → a native `Explore` pass, never the user | record D5 fold (a) graded exemplar; `loop-discipline` gap routing | **G2**, both named (with the un-homeable-story class above making three) |
| The **already-decomposed guard** — no graduated slice → offer re-decomposition or stop; any graduated slice → halt and escalate, a recorded `BACKLOG.md` deferral, not an oversight | current body; deferral recorded in `BACKLOG.md` ("Slice spec-amend mode", 2026-07-02) | **G1** · Recovery's halt row |
| Round reports cleaned by default, retainable on request; **never offer to delete `slices.md`** | current body | Bindings' artifact entry |
| **`KEPT:` "No G2"** | v0.14.0 survivor-provenance entry | **Superseded with grounds** — see the re-grade entry above. The single-reviewer half survives structurally in P5/P6 |


## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: the reviewer is still cold-spawned at first review, and the producer↔reviewer peer edge is declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 is a message to the same seat carrying the reviewer's gap list verbatim"
  - reviewer seat: "spawned **cold at first review**, never in contact with the producer"
  - Contract, Producer ↔ validator: "(reviewer cold-spawned, gap lists lead-routed, no producer↔reviewer contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — slice has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit. The **null-exit** round is unaffected: it is graded and ruled exactly as before.

## [v0.14.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** slice runs a producer↔reviewer cycle (≤3
  rounds, gap-list-driven revision, cold reviewer) whose context-retention bet is slice's own:
  a **standing producer seat** holds (1) cross-round **extend-obligation** placements so a targeted
  revision doesn't silently drop an obligation placed earlier, (2) the **depth reasoning across a
  wrong-depth flip** (decompose ↔ whole-spec — an outcome shift specify has no analogue for), and
  (3) **foundation + Feature-Done coherence** across a targeted fix. The reviewer maps to specify's
  critic seat (cold first review, same-seat after, no producer contact). Transport rides the v3 fix
  (`agent-dispatch.md` Seat transport + addressability probe).
- **Steelman recorded:** zero successful team-form runs at conversion time (the slice-scoped
  pipeline is undogfooded; specify's own checkpoint has not fired). Slice's inputs are unusually
  cheap-and-complete on disk (`spec.md` is the whole source; `slices.md` is a small ID overlay), so
  a one-shot producer reconstructs context more cheaply here than in specify — the retention payoff
  is *smaller* for slice. Slice is single-reviewer (nearer the surgical/one-shot pole) and a
  null-exit round writes no file. Ruled team-form anyway per D2's declared default + S4.
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:slice`" BACKLOG item, Slice-build follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here.

## [v0.14.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor
  all four requirements (default-FAIL done-condition, independent validation, bounded iteration,
  named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not
  restated here…" — restated loop-discipline's own enumeration.

## [v0.14.0] Per-run contract fill (`workflow-contract.md` → `slice-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the
  authoring-time-fill rule); the per-workflow values survive as the command's Contract section
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/slice-contract.md`
  with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is
  the inspectable proof — not this command body."

## [v0.14.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's done-condition / Producer↔validator clause; the
  `review-*` family boundary also lives in `review-slices`' description + REGISTRY's split note)
- **Tier failed:** 1
- **Content:** stated three times pre-wave — L8 ("The reviewer *recommends* a status; **you own the
  clearing verdict** — its status is input, never the gate"), the Contract Team clause, and the
  footer ("the reviewer grades from the files, you Read `spec.md` + the artifacts and decide … its
  status is input").

## [v0.14.0] Slicing-judgment enumeration in the goal-framing
- **Disposition:** relocated → `mochiko:authoring-slices` (single-sources the slicing judgment); the
  converted goal names the deliverable's parts once (the specify pattern)
- **Tier failed:** 1
- **Content:** "(the story→slice judgment: dependency-closed ordering, foundation designation,
  extend obligations, Feature-Done declaration)"

## [v0.14.0] Review-checks enumeration in the review step
- **Disposition:** relocated → `mochiko:review-slices` (single-sources the 13 checks); the command
  briefs the skill by name + the deliverable + the null-exit-round special
- **Tier failed:** 1
- **Content:** "the `review-slices` checks — story coverage, dependency closure, foundation
  legitimacy, sizing, extend-obligation visibility, Feature-Done SC coverage + seams, spec stamp,
  and the depth second-guess in both directions"

## [v0.14.0] Null-exit criterion parenthetical
- **Disposition:** relocated → `mochiko:authoring-slices` (null-exit criterion) + `slices-template`
  usage note 6; the command keeps only the loop-mechanics it owns
- **Tier failed:** 1
- **Content:** "(fewer than two distinct value seams)"

## [v0.14.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules);
  the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 +
  `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task
  tool (never inline agent behavior); do not modify git or push."

## [v0.14.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context
  `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`)"

## [v0.14.0] "Why this workflow is net-new" blockquote
- **Disposition:** relocated → `.mochiko/brainstorms/vertical-graduation/synthesis.md` (Problem
  Statement) + `authoring-slices` Overview + REGISTRY row 44; the design-record pointer folds into
  the converted goal
- **Tier failed:** 2 (motivational provenance, three existing homes — the shape of specify's deleted
  HIL-comparison blockquote)
- **Content:** "> Why this workflow is net-new (no HIL ancestor): the pipeline's unit was the whole
  feature — every story crossed each stage together, so P1 stories could not reach verified code
  until the entire spec was planned and tasked, and whole-spec artifacts diluted producer and
  reviewer attention. This command creates the smaller unit; the downstream commands' slice-scoped
  entries consume it. Design record: `.mochiko/brainstorms/vertical-graduation/synthesis.md`."

## [v0.14.0] KEPT: "No G2 — slice is single-reviewer, so plan's feasibility-rejection slot is intentionally unused."
- **RETIRED at v0.35.0** — superseded with grounds by the CS-D8 re-grade at the top of this file
  (the gate renumber leaves the gap-misreading no path). Kept here as provenance; the evidence below
  is the standing claim as it read at v0.14.0, not a live one.
- **Tier-2 evidence:** prevents a false-defect reading of the gate sequence — without it, an
  auditor/reader seeing G1/G3/G4/G5 reads the G2 gap as a dropped gate. The note records the
  deliberate single-reviewer structure (no feasibility-rejection slot, unlike plan). Provenance:
  slice is net-new single-reviewer by design (REGISTRY row 44; `vertical-graduation/synthesis.md`).
