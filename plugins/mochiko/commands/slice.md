---
description: Decompose an accepted spec into graduation slices (slices.md) — or a reviewed whole-spec null exit.
disable-model-invocation: true
---

# Slice — Graduation-Slice Decomposition

**Goal:** decompose an accepted `spec.md` into an accepted `slices.md` — ordered story groups
that graduate through `/mochiko:plan` → `/mochiko:implement` as independent units — or an
accepted null exit recommending whole-spec. `$ARGUMENTS` = optional feature ID; empty →
resolve from `.mochiko/specs/` and confirm with the user.

## Goal

Either `slices.md` exists conforming to `templates/slices-template.md` — exact spec stamp,
every `US-#` homed exactly once, a designated foundation slice, dependency-closed slice
order, cross-cutting extend obligations placed, Feature-Done complete — or the **null exit**
was taken (no file; the whole-spec reasoning disclosed in `slicer-report.md`); the outcome
was independently graded from the files; and the user accepted whichever shape was produced.

**Not done — default FAIL:** no `slices.md` and no recorded null exit · a blocking gap open ·
the outcome never graded by anyone but its author · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads `slices.md` and the
  `spec.md` it indexes from the files — never the author's report — default FAIL.
- **Reserved to the user:** the spec-amendment offer on an un-homeable cross-cutting story
  (never force a placement) · acceptance — decomposition: accept / amend / reject; null exit:
  accept / override to decompose.
- **Entry:** a missing or unaccepted `spec.md` blocks — point to `/mochiko:specify`. A
  `slices.md` already present with nothing graduated → the user decides overwrite or stop.
  Any slice already graduated (`slices/<id>/` stage artifacts exist) → halt and escalate;
  amending a live decomposition is a recorded deferral, not supported here. A missing
  governance region is surfaced (offer `/mochiko:setup`), never auto-resolved.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **Deliverable:** `.mochiko/specs/<feature>/slices.md` from `templates/slices-template.md`,
  beside the spec it indexes; never offer to delete it. IDs: slices `S#`; `US-#`/`SC-#` are
  `spec.md`'s — echoed, never minted. Null-exit reasoning and open questions live in
  `slicer-report.md`, from `templates/slicer-report-template.md`.
- **Feature-done is declared, not verified** — the Feature-Done section executes at
  feature-close, once every slice ships.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:plan <feature> --slice <foundation-id>`, or plain
  `/mochiko:plan <feature>` after an accepted null exit.
