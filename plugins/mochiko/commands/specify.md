---
description: Turn a feature description into an accepted, independently stress-tested spec.md — intent-governed, delivery-sliced.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn `$ARGUMENTS` (the feature description) into an accepted `spec.md`. Empty →
ask the user for the description.

## Goal

`.mochiko/specs/<feature>/spec.md` exists, conforming to `templates/spec-template.md` with no
placeholder tokens — a confirmed **Intent** section (the elicited scope / delivery /
depth-rigor / constraints / out-of-scope rulings), prioritized P1/P2/P3 user stories, FR-XXX
requirements, measurable SC-XXX criteria, edge cases, and a **Delivery Slices** section
(a graduation-slice decomposition, or the single line "Single slice — whole spec."); it was
independently stress-tested from the file with no blocking gap left open; and the user
accepted the whole — intent, requirements, and slicing together.

**Not done — default FAIL:** a blocking gap open · an Intent section never confirmed by the
user before authoring · a Delivery Slices section absent (the single-slice line counts) ·
the spec never graded by anyone but its author · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Intent stage first.** Before any authoring: run the adaptive-probe agenda via
  `mochiko:analysis-iterative` — scope boundary · delivery/slicing intent · depth-rigor
  expectation · constraints · out-of-scope. Probes, never a questionnaire: crisp input
  collapses a probe to a confirmation. Close in a one-screen synthesis the user confirms;
  flag ratification streaks before treating adoptions as engagement. The confirmed synthesis
  governs the run — the authoring brief, the Delivery Slices shape, and the stress-test's
  rigor all key off it — and lands verbatim as the spec's Intent section.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads `spec.md` itself —
  never the author's report — default FAIL.
- **Reserved to the user:** the feature framing when `$ARGUMENTS` is empty · intent
  confirmation · clarification answers only they can settle · a cross-cutting story no slice
  can home — offered back as a spec amendment, never force-placed · spec acceptance
  (accept / amend / reject), covering intent, requirements, and slicing whole.
- A missing `CLAUDE.md` governance region is surfaced (offer `/mochiko:setup`), never
  auto-resolved. Suggest commits; never run git mutations, never push. User acceptance is
  plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** `.mochiko/specs/<feature>/spec.md` from `templates/spec-template.md` —
  `<feature>` a kebab-case slug derived from the description; never offer to delete it.
  Uncertainty lives in its Assumptions and Open Questions sections.
- **Slicing craft:** the Delivery Slices section per `mochiko:authoring-slices` — foundation
  designation, dependency-closed order, one home per story, Feature-Done, the Graduation
  contract; whole-spec delivery takes the single-slice line instead.
- **Governance input:** where the region is present, name the relevant
  `.claude/rules/mochiko/` files as an obligated read in the author's brief — `paths`-scoped
  rules do not fire for from-scratch authoring.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:plan` — slice-scoped per the spec's Graduation contract when a
  decomposition is present.
