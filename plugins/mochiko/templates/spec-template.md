<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by
     construction, human-legible. Section density: Overview ≤ 3 lines; stories per
     authoring-user-stories (scenarios one line each, 2–3 per story); FR/SC/edge-case
     entries one line each; entities conceptual and compact. Omit empty sections.
     Register: `full` per artifact-format.md rule 11 — this is a human sign-off surface,
     so plain English wins wherever terse and plain pull apart. -->

# {{feature_title}}

> Feature: {{feature_id}}
> Created: {{created}}
> Status: {{status}}

---

## Intent

<!-- The confirmed one-screen synthesis from the intent stage — elicited rulings, one line
     each, confirmed by the user before authoring began. Governs the authoring depth, the
     Delivery slices shape, and the stress-test rigor. Omit a line only if the probe was
     genuinely not applicable. -->

- **Scope boundary:** {{intent_scope}}
- **Delivery:** {{intent_delivery}}  <!-- increments vs whole; first shippable value; sequencing constraints -->
- **Depth / rigor:** {{intent_depth}}
- **UX-bearing:** {{intent_ux_bearing}}  <!-- yes → Screens & Flows + prototype obligated; no → the waiver line -->
- **Constraints:** {{intent_constraints}}
- **Out of scope:** {{intent_out_of_scope}}

---

## Overview

{{overview}}

---

## User Stories

{{user_stories}}

---

## Edge Cases

{{edge_cases}}

---

## Functional Requirements

{{functional_requirements}}

---

## Key Entities

{{key_entities}}

---

## Success Criteria

{{success_criteria}}

---

## Screens & Flows

<!-- Authored per mochiko:authoring-prototype from the Intent section's UX-bearing ruling.
     Not UX-bearing → the single line "No UX surface — prototype waived at intent." and
     nothing else. UX-bearing → both tables + the prototype pointer, in full. The manifest
     is binding (screens, data, actions); the prototype's pixels are advisory. Slice column
     only when Delivery Slices decomposes. -->

{{screens_and_flows}}

<!-- UX-bearing form:

| ID | Screen | Purpose | Data shown | Slice |
|----|--------|---------|------------|-------|
| SCR-001 | [name] | [one line] | [fields/collections rendered] | S1 |

| ID | Flow | Steps | Story scenario | Slice |
|----|------|-------|----------------|-------|
| FLOW-001 | [name] | SCR-001 → [action] → SCR-002 | US-1 / [scenario] | S1 |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with bun or
open `prototype/index.html` directly. Flows and data are binding; layout and styling advisory.
-->

---

## Delivery Slices

<!-- Authored per mochiko:authoring-slices from the Intent section's delivery ruling.
     Whole-spec delivery → the single line "Single slice — whole spec." and nothing else.
     Decomposed → the table + Feature-Done + the Graduation contract below, in full. -->

{{delivery_slices}}

<!-- Decomposed form:

### Slice order

| Slice | Stories | Depends on | Value seam | Rationale (≤ 2 lines) |
|-------|---------|-----------|------------|-----------------------|
| S1 *(foundation)* | US-1, US-3 | — | [journey it proves] | [shared core it establishes + why still a testable journey] |
| S2 | US-2 | S1 | [journey] | [why these graduate together] |

**Extend obligations** *(cross-cutting stories homed once; omit when none)*:
- S2 extends S1's [surface]: [obligation, one line]

### Feature-Done

| Criterion | Verified by slice |
|-----------|-------------------|
| SC-1 | S1 |

**Cross-slice seams:** [seams no single slice verifies; executed at feature-close, after the
last slice ships — declared here, never reported complete by any slice run]

### Graduation contract  *(how downstream slice-scoped runs honor this section)*

- **Slice-scoped runs** — with a decomposition present, the design → tasking → implementation
  stages run **per slice, in Slice-order**: each stage resolves the current slice (named in its
  argument, else the first slice in order lacking that stage's artifact) and scopes itself to
  that slice's stories **plus its extend obligations** — nothing else.
- **Artifact layout** — shared design artifacts live at the feature root and **accumulate**
  across slices (`requirements.md`, `constraints-and-decisions.md`, `nfrs.md`, `data-model.md`,
  `contracts/`, `quickstart.md`); per-slice artifacts live under `slices/<id>/` (`plan.md`,
  `tasks.md`, cycle reports, round reports, filled contracts).
- **Extend-mode** — a later slice's design treats the accumulated shared artifacts as
  brownfield input: read first, **extend in place, never re-derive** and never fork per-slice
  copies.
- **Graded amendment** — an **additive** extension (new entity, attribute, endpoint) is routine
  extend-mode work. A **breaking** change to design an earlier slice already shipped as code is
  an explicit amendment: surfaced as a `[MODIFY]` design change for that run's review — never a
  silent rewrite — with its migration carried in the *current* slice's cycle cards. Repeated
  breaking amendments against the same design are a re-decomposition signal, not routine.
- **Regression safety** — earlier slices' tests live in the repository; every slice's quality
  gates run the full suite, so an amendment that breaks shipped behavior surfaces by
  construction.
-->

---

## Assumptions

{{assumptions}}

---

## Open Questions

{{open_questions}}
