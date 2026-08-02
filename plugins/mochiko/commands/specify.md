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
depth-rigor / UX-bearing / constraints / out-of-scope rulings), prioritized P1/P2/P3 user
stories, FR-XXX requirements, measurable SC-XXX criteria, edge cases, a **Screens & Flows**
section (the SCR-XXX/FLOW-XXX manifest with its clickable low-fi prototype under
`prototype/`, or the single line "No UX surface — prototype waived at intent."), and a
**Delivery Slices** section (a graduation-slice decomposition, or the single line "Single
slice — whole spec."); it was independently stress-tested from the file — for a UX-bearing
spec that includes walking the served prototype — with no blocking gap left open; and the
user accepted the whole — intent, requirements, experience, and slicing together.

**Not done — default FAIL:** a blocking gap open · an Intent section never confirmed by the
user before authoring · a Screens & Flows section absent (the waiver line counts), or ruled
UX-bearing with no clickable prototype, or manifest↔prototype drift unresolved · a Delivery
Slices section absent (the single-slice line counts) · the spec never graded by anyone but
its author · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Intent stage first.** Before any authoring: run the adaptive-probe agenda via
  `mochiko:analysis-iterative` — scope boundary · delivery/slicing intent · depth-rigor
  expectation · UX-bearing (does the feature carry a user-facing surface to prototype) ·
  constraints · out-of-scope. Probes, never a questionnaire: crisp input collapses a probe
  to a confirmation. Close in a one-screen synthesis the user confirms; flag ratification
  streaks before treating adoptions as engagement. The confirmed synthesis governs the run —
  the authoring brief, the Screens & Flows obligation, the Delivery Slices shape, and the
  stress-test's rigor all key off it — and lands verbatim as the spec's Intent section.
- **Lockstep prototyping (UX-bearing only).** Stories and their screens co-evolve as one
  unit: skeleton nav frame first, then each story's screens and flows land while that story
  is under discussion — the user clicks the slice while the story is wet, never a batch
  render after the text settles. The whole feature is prototyped at specify time; a
  decomposed spec tags every SCR/FLOW with its slice, out-of-slice screens greyed
  coming-soon.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads `spec.md` itself —
  never the author's report — default FAIL. On a UX-bearing spec the stress-test also walks
  the prototype (served, or opened directly via the no-server degrade path): every FLOW-XXX
  clickable end-to-end, every SCR-XXX reachable, every
  P1 acceptance scenario carrying a click path; manifest↔prototype drift is a blocking gap.
- **Reserved to the user:** the feature framing when `$ARGUMENTS` is empty · intent
  confirmation · clarification answers only they can settle · clicking each story's
  prototype slice as it lands (their reactions fold back into story and screen together) ·
  a cross-cutting story no slice can home — offered back as a spec amendment, never
  force-placed · spec acceptance (accept / amend / reject), covering intent, requirements,
  experience, and slicing whole.
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
- **Prototype craft:** the Screens & Flows section and `prototype/` app per
  `mochiko:authoring-prototype` — SCR-XXX/FLOW-XXX manifest grammar, low-fi discipline
  (binding flows, advisory pixels), bun-servable static HTML with the no-server degrade
  path, design system honored where one exists; not UX-bearing takes the waiver line
  instead.
- **Governance input:** where the region is present, name the relevant
  `.claude/rules/mochiko/` files as an obligated read in the author's brief — `paths`-scoped
  rules do not fire for from-scratch authoring.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:plan` — slice-scoped per the spec's Graduation contract when a
  decomposition is present.
