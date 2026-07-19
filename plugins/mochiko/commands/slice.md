---
description: Decompose an accepted feature specification into graduation slices via an independent producer→reviewer team loop — a standing task-architect seat authors the slices.md overlay across bounded rounds, a cold devils-advocate seat grades the decomposition from the files, the user accepts at a named gate; spec-gated, null-exit-aware, default-FAIL, bounded, kernel-free. Downstream stages then run per slice instead of whole-spec. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Slice — Graduation-Slice Decomposition (Spec → Slices)

**Goal:** decompose an accepted `spec.md` into an accepted `slices.md` — ordered, named story
groups (a designated foundation slice, dependency-closed ordering, cross-cutting extend
obligations, a declared Feature-Done section) that graduate through `/mochiko:plan` →
`/mochiko:tasks` → `/mochiko:implement` as independent units instead of one whole-spec batch —
authored and independently graded before the user accepts it. Design record:
`.mochiko/brainstorms/vertical-graduation/synthesis.md`. `$ARGUMENTS` = optional feature ID or
description; empty or detected-from-workspace is handled by triage below.

**You are the lead**, and this is a **team-form command in the mochiko command shape**: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else — the
shape's rules bind here and are not restated; this file carries only slice's parameters. You own
the loop (round counter, verdict, escalation) and every human gate. This is a
`mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill.

## Team-form parameters (shape Layer 2)

Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The **authoritative
first-spawn probe** is the producer — always the first seat filled. Transport mechanics + the
addressability check: `templates/agent-dispatch.md` (Seat transport). The no-fallback bet is the
same `Contested` dogfood-pilot ruling as the other team-form commands.

## Session constraints

- Workspace: resolve `<feature>` (an explicit ID from `$ARGUMENTS`, else the most recent
  in-progress feature under `.mochiko/specs/`). The deliverable `slices.md` lives alongside the
  spec it indexes.
- **Entry gate — spec accepted.** `.mochiko/specs/<feature>/spec.md` must be present and accepted
  (workspace evidence — there is no context-file `status`). Missing or unaccepted → block and
  point the user to `/mochiko:specify`.
- **Already-decomposed guard.** If `slices.md` already exists: no slice has graduated (no
  `slices/<id>/` stage artifacts) → offer re-decomposition (overwrite) or stop; **any slice
  already graduated → halt and escalate** — amending a live decomposition is deliberately
  unsupported (a recorded deferral, not an oversight; `BACKLOG.md`).
- Kill-switch: stop and escalate if `.mochiko/specs/<feature>/SLICE_STOP` exists — check before
  each seat send.
- **Deliverable & IDs:** `slices.md`, producer-authored per `templates/slices-template.md` (every
  `US-#` homed exactly once; the Feature-Done section's `SC-#` coverage map + seams; the spec
  stamp). Its uncertainty carrier is the producer's `slicer-report.md` (from
  `templates/slicer-report-template.md`) — its Open Questions, and on a null-exit round its
  whole-spec disclosure (the shape's producer-authored branch), not confidence marks. The
  reviewer's gap IDs live in `advocate-report.md`.

## The seats

- **producer** — `mochiko:task-architect`, one **named standing seat** across rounds. Brief it to
  author `slices.md` from `templates/slices-template.md` (+ `slicer-report.md` from its template)
  via `mochiko:authoring-slices`: `spec.md` as the input to Read, the governance obligated-read
  line (per the prerequisite), the template to fill per the skill — no placeholder tokens. Round >
  1 is a message to the same seat carrying the reviewer's gap list verbatim (fix the flagged gaps;
  don't regress passing slices). **Null exit:** it may instead recommend whole-spec — no
  `slices.md` written, reasoning disclosed in `slicer-report.md`; the round proceeds to review
  either way. It never grades.
- **reviewer** — `mochiko:devils-advocate`, spawned **cold at first review**, never in contact
  with the producer. Brief it to run `mochiko:review-slices` against `slices.md` **and** the
  `spec.md` it indexes — it Reads both files itself, never the producer's report — writing
  `advocate-report.md` (from `templates/advocate-report-template.md`): severity-bucketed gaps,
  product-framed clarifying questions, genuine strengths, and a recommended status (`ready` /
  `needs-revision` / `critical-gaps`). On a **null-exit round** it grades the depth call from
  `spec.md` + the disclosed reasoning. Round > 1 is a message to the same seat: re-Read the
  revised files. Its output is **lead-adjudicated input** (the `review-*` family boundary); there
  is no sized end-stage review here — the bounded in-loop critique is this workflow's independent
  validation (declared in the Contract). Single reviewer, never the producer.

## The flow

**Triage** *(gate G1)* — capture `$ARGUMENTS`; resolve `<feature>`. Empty (the known
`@`-reference drop) → recover via G1: ask the user to re-enter, or confirm the detected feature.
Apply the entry gate and the already-decomposed guard (Session constraints). **Governance
prerequisite:** check `CLAUDE.md` for the mochiko governance region
(`<!-- mochiko:governance:begin -->`). Present → governance reaches the producer natively at
spawn; add to its brief the one-line **obligated read** naming the `.claude/rules/mochiko/` files
relevant to what it authors. Missing → surface it (offer `/mochiko:setup`) — governing context,
never a blocking gate; don't auto-resolve.

**Decomposition loop** *(you own the counter; the decomposition is FAIL until proven otherwise)* —
initialize `round = 1` → **produce** → **review** → **verdict (you):** Read `spec.md`, `slices.md`
(when written) + both reports directly. Reviewer `ready` **and** you find no unresolved blocking
gap → acceptance. Otherwise classify each gap and route it per `loop-discipline`'s gap routing —
an **un-homeable cross-cutting story** is a spec-amendment finding (surface it, offer
`/mochiko:specify` amendment; don't force a placement); a **"Research this"** knowledge gap routes
to a native `Explore` pass, never to the user; a preference gap → G3 — then apply the bounds: cap
**3** rounds · no-progress exit when the gap set is unchanged round-over-round · kill-switch. A
**wrong-depth** finding flips the outcome shape (decompose ↔ whole-spec) — it counts as a round,
not a special case. Any trip, or `critical-gaps` → **escalate** (G4: present the last gap list +
stop reason; continue-refining / accept-with-noted-gaps / abort — the run stays FAIL unless the
user explicitly accepts). Else `round += 1`, loop to produce.

**Acceptance** *(gate G5 — reachable only on your clearing verdict)* — two shapes:

- **Decomposition produced** — present the validated overlay (slice count + names, foundation
  choice, story coverage, SC coverage, seams, any noted gaps): **accept** (→ finalize; the
  done-condition is now satisfied) / **amend** (re-enter the loop with the changes as the gap list
  — still bounded; it must clear a verdict again) / **reject** (abort; the draft stays under
  `.mochiko/specs/<feature>/`).
- **Null exit** — present the whole-spec recommendation and its reviewed reasoning: **accept** (→
  finalize; the pipeline runs whole-spec exactly as before — the done-condition is satisfied with
  no `slices.md`) / **override** (re-enter the loop directed to decompose — still bounded).

**Finalize** — report the outcome (`slices.md` + slice / foundation / story counts and SC coverage
— or the accepted whole-spec recommendation), the round count, the round reports, a suggested
commit (`docs: slice <feature>`), and the next step: `/mochiko:plan <feature> --slice
<foundation-id>` (or plain `/mochiko:plan <feature>` after a null exit). Feature-done is
**declared, not verified** — the Feature-Done section executes at feature-close, once every slice
ships. Offer a retain/clean choice for the round reports; never offer to delete `slices.md` — it
is the deliverable.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when **(1)** EITHER `slices.md` exists
  conforming to `templates/slices-template.md` (exact spec stamp; every story homed once;
  Feature-Done complete) OR the **null exit** was taken (no file; reasoning in `slicer-report.md`),
  **(2)** the reviewer recommends `ready` grounded in the files (or, on a null-exit round, on the
  depth call), **(3)** you Read `spec.md` + the artifacts + the report and confirm no blocking gap
  remains (the reviewer's status is input, never the gate), **and (4)** G5 acceptance has cleared —
  accepting either the decomposition or the whole-spec recommendation. Out of rounds = escalate,
  never done.
- **Producer ↔ validator:** `task-architect` (authoring-slices) authors the overlay, never grades;
  `devils-advocate` (review-slices) grades from the files, never authors — disjoint agents,
  disjoint skills, structurally separated (reviewer cold-spawned, gap lists lead-routed, no
  producer↔reviewer contact). A single reviewer. **Validation model:** the bounded in-loop
  critique — every round, unsized by design; no sized end-stage review (the shape's
  in-loop-critique branch).
- **Bounds:** ≤3 rounds (you count) · no-progress exit · kill-switch `SLICE_STOP` · a G5
  amend/override re-enters the same bounded loop.
- **Human gates:** G1 input recovery + entry / already-decomposed / governance surface · G3
  clarification (incl. the preference-gap decision) · G5 two-shape acceptance · escalation (G4) on
  any guard trip. **No G2** — slice is single-reviewer, so plan's feasibility-rejection slot is
  intentionally unused.

## Recovery

Pause posture (per the shape): note the resume stage on `slices.md` (or `slicer-report.md` before
the overlay exists). Resume from workspace evidence, respawning what the stage needs — a respawned
producer re-reads `spec.md` + the gap list; a reviewer respawn is cold by design:

| Evidence | Resume at |
|----------|-----------|
| no `.mochiko/specs/<feature>/spec.md` (or unaccepted) | triage (entry blocked) |
| `slices.md` present with graduated slices (`slices/<id>/` artifacts exist) | triage (halt — amend deferred) |
| spec accepted; no `slices.md` and no `slicer-report.md` | loop (produce, round 1) |
| `slices.md` (or a null-exit `slicer-report.md`) present, no `advocate-report.md` this round | loop (review) |
| `advocate-report.md` not `ready`, within the cap | loop (produce) |
| `advocate-report.md` `ready`, not yet accepted | G5 |
| accepted | finalize |
| `SLICE_STOP` present | escalate |

---

**What you own (not the seats):** input triage and the entry / already-decomposed / governance
prerequisites; the loop, the gap routing (including the un-homeable-story spec-amendment finding),
and the verdict against the default-FAIL done-condition; the human gates including the two-shape
G5 acceptance; verifying each dispatch wrote its expected files (a missing output → log and ask
retry/abort — a null-exit round expects `slicer-report.md` only); and never letting producer and
reviewer collapse into one seat. Full rules: `mochiko:loop-discipline`.
