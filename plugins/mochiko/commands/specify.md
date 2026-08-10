---
description: Turn a feature description into an accepted, independently stress-tested spec workspace — intent-governed, feature-derived, user-selected.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn `$ARGUMENTS` (the feature description) into an accepted spec workspace. Empty →
ask the user for the description.

## Goal

`.mochiko/specs/<spec>/` exists: `spec.md` conforming to `templates/spec-template.md` with
no placeholder tokens — a confirmed **Intent** section (the elicited scope / delivery /
depth-rigor / UX-bearing / constraints / out-of-scope rulings), FR-XXX requirements,
measurable SC-XXX criteria, edge cases, a **Screens & Flows** section (the SCR-XXX/FLOW-XXX
manifest with its clickable low-fi prototype under `prototype/`, or the single line "No UX
surface — prototype waived at intent."), and a **Feature Selection** section (derived
features, filter verdicts with reasons, the user's selection with its deferred-SC list);
stories as `stories/US-*.md` files (text, acceptance scenarios, FEAT-ID mapping — or
`rejected` with the why); the staged map delta executed at spec acceptance as one atomic
batch — entries land (`proposed`; selected ones flip `in-flight`), deltas attach,
`FEATURES.md` and `.mochiko/specs/index.md` rows touch; it was independently stress-tested
from the files — spec + stories + derivation + map delta in one pass, the served prototype
walked when UX-bearing — with no blocking gap left open; and the user accepted the whole —
intent, requirements, experience, derivation, and selection together.

**Not done — default FAIL:** a blocking gap open · an Intent section never confirmed by the
user before authoring · the feature map not read at intent, or a missing map silently
tolerated · a drafted story neither homed to exactly one feature nor rejected with a
recorded why · a Screens & Flows section absent (the waiver line counts), or ruled
UX-bearing with no clickable prototype, or manifest↔prototype drift unresolved · a Feature
Selection section absent, or the selection not ruled by the user · the live map written
before acceptance · the spec never graded by anyone but its authors · user acceptance not
given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Intent stage first.** Before any authoring: run the adaptive-probe agenda via
  `mochiko:analysis-iterative` — scope boundary · delivery intent · depth-rigor expectation ·
  UX-bearing (does the feature carry a user-facing surface to prototype) · constraints ·
  out-of-scope — with the **existing feature map an obligated read** per
  `mochiko:authoring-feature-map`'s intent-stage agenda: `FEATURES.md` plus the entries in
  the intent's territory. A missing map is surfaced — offer `/mochiko:setup`, whose
  brownfield analysis reconstructs it — never silently tolerated. Map entries marked
  `unrefined` are unratified hypotheses, never extension anchors: derivation ignores stub
  text and derives from the stories; a stub matching a derived feature is confirmation, a
  stub matching nothing stays parked or is retired (procedure:
  `mochiko:authoring-feature-map`). Probes, never a questionnaire; close in a one-screen
  synthesis the user confirms; flag ratification streaks
  before treating adoptions as engagement. The confirmed synthesis governs the run and lands
  verbatim as the spec's Intent section.
- **Lockstep prototyping (UX-bearing only).** Stories and their screens co-evolve as one
  unit: skeleton nav frame first, then each story's screens and flows land while that story
  is under discussion — the user clicks through while the story is wet, never a batch render
  after the text settles. The whole feature is prototyped at specify time; FEAT-tags land at
  derivation as a re-tag pass over the SCR/FLOW manifest — unselected features' screens
  greyed coming-soon, a filter-rejected story's screens kept greyed and marked rejected.
- **Derivation + filter after stories.** The `product-manager` seat, via
  `mochiko:authoring-feature-map`, derives proposed features and deltas from the drafted
  stories against the actual map files, maps every SC-XXX to its verifying feature, and runs
  the filter — a story that earns no place on the map is rejected with the why recorded in
  its story file, never silently dropped. All derivation output stages in the spec workspace;
  the live map stays untouched until acceptance. A producer disagreement (a rejected story
  held load-bearing) escalates to the user.
- **Selection.** The user picks which derived features build now, from the PM's selection
  card — recommendation, dependency order, the deferred-SC list, and, per parent capability
  in the spec's territory, the completeness ledger (delivered/undelivered leaves, parked
  stubs, kills), visible at the moment of choice. The PM recommends, never selects.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the files themselves —
  `spec.md`, the story files, the staged entries and deltas — never a report; default FAIL.
  The stress-test grades spec + stories + feature derivation + map delta in one pass
  (derivation honesty, filter rejections justified, dedup against the actual map, granularity
  respected, entries well-formed), its map-delta baseline the map's git state at run open. On
  a UX-bearing spec it also walks the prototype (served, or opened directly via the no-server
  degrade path): every FLOW-XXX clickable end-to-end, every SCR-XXX reachable, every P1
  acceptance scenario carrying a click path; manifest↔prototype drift is a blocking gap.
- **Reserved to the user:** the feature framing when `$ARGUMENTS` is empty · intent
  confirmation · clarification answers only they can settle · clicking each story's
  prototype screens as they land (their reactions fold back into story and screen together) ·
  ruling on an escalated filter disagreement · **the feature selection** · spec acceptance
  (accept / amend / reject), covering intent, requirements, experience, derivation, and
  selection whole — acceptance is what executes the map write batch; a rejected spec never
  touched the map.
- A missing `CLAUDE.md` governance region is surfaced (offer `/mochiko:setup`), never
  auto-resolved. Suggest commits; never run git mutations, never push. User acceptance is
  plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** `.mochiko/specs/<spec>/` — `spec.md` from `templates/spec-template.md`
  plus the `stories/US-*.md` files; `<spec>` a kebab-case slug derived from the
  description; never offer to delete it. Uncertainty lives in `spec.md`'s Assumptions and
  Open Questions sections.
- **Feature-map craft:** derivation, the filter, entry authoring
  (`templates/feature-entry-template.md`), index lines
  (`templates/features-index-template.md`), delta grammar, SC re-homing, and the
  acceptance-time write rules — the `.mochiko/specs/index.md` row included; spec-index
  stewardship rides the skill — all per `mochiko:authoring-feature-map`.
- **Prototype craft:** the Screens & Flows section and `prototype/` app per
  `mochiko:authoring-prototype` — SCR-XXX/FLOW-XXX manifest grammar, low-fi discipline
  (binding flows, advisory pixels), bun-servable static HTML with the no-server degrade
  path, design system honored where one exists; not UX-bearing takes the waiver line
  instead.
- **Governance input:** where the region is present, name the relevant
  `.claude/rules/mochiko/` files as an obligated read in each author's brief — `paths`-scoped
  rules do not fire for from-scratch authoring.
- **KM landing:** where `.mochiko/memory/knowledge-management.md` exists, spec acceptance is
  a landing — run its landing ritual (close/move any BACKLOG item the spec discharges, touch
  `ROADMAP.md`) and its command-boundary invariants fix-on-sight; the acceptance-time map
  write batch lands in the same moment.
- **Migration:** existing slice-form specs are valid frozen history; only new runs use this
  surface.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:plan` — one run per selected feature, in the map's dependency
  order.
