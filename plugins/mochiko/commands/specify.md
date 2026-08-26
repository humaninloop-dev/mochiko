---
description: Turn a feature description into an accepted, independently stress-tested spec workspace — intent-governed, feature-derived, user-selected.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn `$ARGUMENTS` (the feature description) into an accepted spec workspace. Empty →
ask the user for the description.

## Goal

`.mochiko/specs/<spec>/` exists: `spec.md` conforming to the spec template (rendered by
`mochiko-cli template spec`, or its schema `plugins/mochiko/schemas/spec.yaml` Read raw when
the binary is absent — the shipped schema is the first-class source of truth) with
no placeholder tokens — a confirmed **Intent** section (the elicited scope / delivery /
depth-rigor / UX-bearing / constraints / out-of-scope rulings, plus the agreed capability
frame), FR-XXX requirements,
measurable SC-XXX criteria, edge cases, a **Screens & Flows** section (the SCR-XXX/FLOW-XXX
manifest with its clickable low-fi prototype under `prototype/`, or the single line "No UX
surface — prototype waived at intent."), and a **Feature Selection** section (the confirmed
capability frame, derived work rows grouped per capability, filter verdicts with reasons, the
user's selection with its deferred-SC list and the per-capability completeness view);
stories as `stories/US-*.md` files (text, acceptance scenarios, work-row mapping under a
capability — or `rejected` with the why); the staged map delta executed at spec acceptance as
one atomic batch — capabilities land or extend, work rows attach (pending; selected rows flip
`live`, the capability reading `in-flight` while live rows exist), deltas attach,
`FEATURES.md` and `.mochiko/specs/index.md` rows touch; it was independently stress-tested
from the files — spec + stories + capability/row derivation + map delta in one pass, the
served prototype walked when UX-bearing — with no blocking gap left open; and the user
accepted the whole — intent, requirements, experience, derivation, and selection together.

**Not done — default FAIL:** a blocking gap open · an Intent section never confirmed by the
user before authoring · the feature map not read at intent, or a missing map silently
tolerated · a drafted story neither homed to exactly one work row nor rejected with a
recorded why · a Screens & Flows section absent (the waiver line counts), or ruled
UX-bearing with no clickable prototype, or manifest↔prototype drift unresolved · a Feature
Selection section absent, or the selection not ruled by the user · the live map written
before acceptance · the spec never graded by anyone but its authors · user acceptance not
given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Transport floor.** When the run composes more than one seat,
  `mochiko:patterns-transport-floor` governs its composition and messaging under a split
  trigger — message legs on any multi-seat messaging, topology legs on shared writes —
  non-waivable once triggered; referenced, never restated.
- **Model tiering.** Exploration and fact-finding dispatches ride
  `mochiko:patterns-model-tiering`'s class key — locate/enumerate reads to a native
  `Explore` subagent spawned `model: haiku`, interpretive or absence-driven reads on the session
  tier — and every seat brief carries the routing rule; referenced, never restated.
- **Intent stage first.** Before any authoring: run the adaptive-probe agenda via
  `mochiko:analysis-iterative` — scope boundary · delivery intent · depth-rigor expectation ·
  UX-bearing (does the feature carry a user-facing surface to prototype) · constraints ·
  out-of-scope — with the **existing feature map an obligated read** per
  `mochiko:authoring-feature-map`'s intent-stage agenda: `FEATURES.md` plus the entries in
  the intent's territory. A missing map is surfaced — offer `/mochiko:setup`, whose
  brownfield analysis reconstructs it — never silently tolerated. Map entries marked
  `unrefined` are unratified hypotheses, never extension anchors: derivation ignores stub
  text (deriving frame-first, stories informing — below); a stub matching a framed or
  derived capability is confirmation, a
  stub matching nothing stays parked or is retired (procedure:
  `mochiko:authoring-feature-map`). Probes, never a questionnaire; close in a one-screen
  synthesis the user confirms; flag ratification streaks
  before treating adoptions as engagement. The confirmed synthesis governs the run and lands
  verbatim as the spec's Intent section.
- **Capability frame at intent (before stories).** Within the intent stage, once the map
  and intent are read, the `product-manager` seat states the capability frame — which
  capabilities the territory touches and an extend-vs-mint hypothesis per the capability
  tests (`mochiko:patterns-map-minimalism`) — as **nouns + verbs only, never enumerating
  stories**, agreed with the user. The frame is a hypothesis, not an anchor: it dictates
  neither story boundaries nor journeys (stories stay journey-driven), and stories win any
  conflict with it, resolved at the post-stories confirm step. On a thin greenfield intent
  the frame draws on the intent conversation, the product description's domain nouns, and the
  capability tests. The agreed frame lands in the confirmed Intent section.
- **Lockstep prototyping (UX-bearing only).** Stories and their screens co-evolve as one
  unit: skeleton nav frame first, then each story's screens and flows land while that story
  is under discussion — the user clicks through while the story is wet, never a batch render
  after the text settles. The whole feature is prototyped at specify time; FEAT-tags land at
  derivation as a re-tag pass over the SCR/FLOW manifest — unselected features' screens
  greyed coming-soon, a filter-rejected story's screens kept greyed and marked rejected.
- **Confirm frame, cut work rows, filter (after stories).** The `product-manager` seat, via
  `mochiko:authoring-feature-map` and `mochiko:patterns-map-minimalism`, first confirms or
  adjusts the intent-stage capability frame against what the stories revealed — stories win
  any conflict — then cuts the work rows (story-shaped is fine) grouped under their
  capabilities, extending an existing capability's extent or minting a new one per the
  extend-vs-mint tests, maps every SC-XXX to its verifying capability or row, and runs
  the filter — a story that earns no place on the map is rejected with the why recorded in
  its story file, never silently dropped. All derivation output stages in the spec workspace;
  the live map stays untouched until acceptance. A producer disagreement (a rejected story
  held load-bearing) escalates to the user.
- **Selection.** The user picks which work rows build now, from the PM's selection
  card — the rows grouped per capability, recommendation, row-level dependency order, the
  deferred-SC list, and, per capability in the spec's territory, the completeness view
  (pending work rows — cut but undelivered — plus parked stubs and kills), visible at the
  moment of choice. The PM recommends, never selects.
- **Epic proposal (optional).** When one derivation spans capabilities such that its work rows
  want one coordinated multi-feature run, the `product-manager` seat **may propose an epic** as
  part of the selection card — a proposal only, never a mint; the user takes it to
  `/mochiko:feature`'s desk to form, the only door that mints. Specify never mints an epic
  (`mochiko:authoring-epic`).
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the files themselves —
  `spec.md`, the story files, the staged capabilities and work rows — never a report; default FAIL.
  The stress-test grades spec + stories + capability/row derivation + map delta in one pass
  (derivation honesty, filter rejections justified, dedup against the actual map, granularity
  respected, capabilities and rows well-formed), its map-delta baseline the map's git state at run open. On
  a UX-bearing spec it also walks the prototype (served, or opened directly via the no-server
  degrade path): every FLOW-XXX clickable end-to-end, every SCR-XXX reachable, every P1
  acceptance scenario carrying a click path; manifest↔prototype drift is a blocking gap.
- **Reserved to the user:** the feature framing when `$ARGUMENTS` is empty · intent
  confirmation (the capability frame included) · clarification answers only they can settle · clicking each story's
  prototype screens as they land (their reactions fold back into story and screen together) ·
  ruling on an escalated filter disagreement · **the selection** (which work rows build now) · spec acceptance
  (accept / amend / reject), covering intent, requirements, experience, derivation, and
  selection whole — acceptance is what executes the map write batch; a rejected spec never
  touched the map.
- A missing `CLAUDE.md` governance region is surfaced (offer `/mochiko:setup`), never
  auto-resolved. Suggest commits; never run git mutations, never push. User acceptance is
  plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** `.mochiko/specs/<spec>/` — `spec.md` from the spec template (rendered by
  `mochiko-cli template spec`, or its schema `plugins/mochiko/schemas/spec.yaml` Read raw when
  the binary is absent — the shipped schema is the first-class source of truth)
  plus the `stories/US-*.md` files; `<spec>` a kebab-case slug derived from the
  description; never offer to delete it. Uncertainty lives in `spec.md`'s Assumptions and
  Open Questions sections.
- **Feature-map craft:** derivation, the filter, capability and work-row authoring
  (the feature-entry template — `mochiko-cli template feature-entry`, or its schema
  `plugins/mochiko/schemas/feature-entry.yaml` Read raw when the binary is absent, the shipped
  schema being the first-class source of truth), index lines (the features-index template —
  `mochiko-cli template features-index`, or its schema
  `plugins/mochiko/schemas/features-index.yaml` Read raw when the binary is absent, likewise the
  first-class source), delta grammar, SC re-homing, and the
  acceptance-time write rules — the `.mochiko/specs/index.md` row included; spec-index
  stewardship rides the skill — all per `mochiko:authoring-feature-map`; the capability
  tests, extend-vs-mint, and the frame discipline per `mochiko:patterns-map-minimalism`.
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
- **Next step:** `/mochiko:implement` — one run per capability-batch (a capability's selected
  work rows), in the rows' dependency order; the run opens with a sufficiency check at its
  entry.
