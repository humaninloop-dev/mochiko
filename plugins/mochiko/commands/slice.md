---
description: Decompose an accepted feature specification into graduation slices via an independent producer→reviewer team loop — a standing task-architect seat authors the slices.md overlay across bounded rounds, a cold devils-advocate seat grades the decomposition from the files, the user accepts at a named gate; spec-gated, null-exit-aware, default-FAIL, bounded, kernel-free. Downstream stages then run per slice instead of whole-spec. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Slice — Graduation-Slice Decomposition (Spec → Slices)

**Goal:** decompose an accepted `spec.md` into an accepted `slices.md` — ordered, named story groups
that graduate through `/mochiko:plan` → `/mochiko:implement` as independent units instead of one
whole-spec batch — authored and independently graded before the user accepts it. `$ARGUMENTS` = optional feature ID or
description; empty or detected-from-workspace is resolved at G1.

**You are the lead** of a team-form command in the mochiko command shape: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) and `mochiko:loop-discipline`
before anything else; brief every dispatch per `templates/agent-dispatch.md`. This file carries only
slice's parameters. **First-spawn probe:** the `task-architect` producer — always the first seat
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
every verdict is yours.

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
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this spec,
  plus the process you compose from it — the stated default below, or your departures from it ·
  rules: the user · decides: the run's composed process.
- **G2 clarification** — evidence: a reviewer gap you classify as preference, or a producer question
  it cannot resolve · rules: the user · decides: the answer fed forward. **A preference gap is ruled
  here**; a **"Research this"** knowledge gap routes to a native `Explore` pass, never to the user;
  an **un-homeable cross-cutting story is a spec-amendment finding** — surface it and offer a
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
  limbs — the preference ruling and the un-homeable-story amendment offer (its knowledge-gap route to
  `Explore` stays yours). That is all five: each reads `rules: the user`, so none is departable
  however you compose the run, and there is no non-floor gate here to name. **No lead-penned surface
  takes a standing cold grade:** P11 is producer-authored.
- **Bounds:** cap **3** rounds, you count them; no-progress exit when the gap set is unchanged
  round-over-round; kill-switch — stop and escalate if `.mochiko/specs/<feature>/SLICE_STOP` exists,
  checked before each seat send; a G4 amend or override re-enters the same bounded loop; out of
  rounds = escalate, never done.
- **Invariants:** a **wrong-depth** finding flips the outcome shape (decompose ↔ whole-spec) — it
  counts as a round, not a special case. **No devolved branch** — the review is a judgment
  grade, never all-deterministic-CLI, so no gate is skipped and every verdict is yours. Hold every
  revision targeted (fix the flagged gaps; don't regress passing slices). **Feature-done is declared,
  not verified** — the Feature-Done section executes at feature-close, once every slice ships.

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
  departs from the stated default, or declares non-default bounds, instantiates
  `templates/workflow-contract.md` as `.mochiko/specs/<feature>/slice-contract.md` beside the reports
  instead. Counted unit: the **round**, the unit the Bounds already count.
- **Departure trail:** one line per departure, appended under that same declaration as it is taken
  and carried into G4's evidence — never your context alone.
- **Next step:** `/mochiko:plan <feature> --slice <foundation-id>`, or plain `/mochiko:plan
  <feature>` after an accepted null exit.

## Recovery

Note the resume stage on `slices.md`, or on `slicer-report.md` before the overlay exists; resume
from workspace evidence, respawning what the stage needs — a respawned producer re-reads `spec.md` +
the gap list.

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
