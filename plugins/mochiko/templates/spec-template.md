<!-- Form: templates/artifact-format.md (the deliverable envelope) — dense by
     construction, human-legible. Section density: Overview ≤ 3 lines; stories per
     authoring-user-stories (scenarios one line each, 2–3 per story); FR/SC/edge-case
     entries one line each; entities conceptual and compact. Omit empty sections.
     Register: `full` per artifact-format.md rule 11 — this is a human sign-off surface,
     so plain English wins wherever terse and plain pull apart. -->

# {{spec_title}}

> Spec: {{spec_id}}
> Created: {{created}}
> Status: {{status}}

---

## Intent

<!-- The confirmed one-screen synthesis from the intent stage — elicited rulings, one line
     each, confirmed by the user before authoring began. Governs the authoring depth, the
     feature derivation, and the stress-test rigor. Omit a line only if the probe was
     genuinely not applicable. -->

- **Scope boundary:** {{intent_scope}}
- **Delivery:** {{intent_delivery}}  <!-- whole vs subset now; first shippable value; sequencing constraints -->
- **Depth / rigor:** {{intent_depth}}
- **UX-bearing:** {{intent_ux_bearing}}  <!-- yes → Screens & Flows + prototype obligated; no → the waiver line -->
- **Constraints:** {{intent_constraints}}
- **Out of scope:** {{intent_out_of_scope}}

---

## Overview

{{overview}}

---

## User Stories

<!-- An INDEX only. Story content lives in per-story files: stories/US-*.md — each carries
     the story text, its acceptance scenarios, and its FEAT-ID mapping. The only
     story-native status is `rejected` (the derivation filter's verdict, with the why,
     recorded in the story file); every other status is derived by following the story's
     FEAT-ID to the feature map. One row per story. -->

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | {{story_hook}} | P1 | FEAT-XXX | homed |
| [US-2](stories/US-2.md) | {{story_hook}} | P2 | — | rejected — {{why, one line}} |

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
     is binding (screens, data, actions); the prototype's pixels are advisory. Feature
     column lands at derivation as a re-tag pass (FEAT-XXX per row); screens of unselected
     features greyed coming-soon; a filter-rejected story's screens kept, greyed, marked
     rejected. -->

{{screens_and_flows}}

<!-- UX-bearing form:

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | [name] | [one line] | [fields/collections rendered] | FEAT-001 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | [name] | SCR-001 → [action] → SCR-002 | US-1 / [scenario] | FEAT-001 |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with bun or
open `prototype/index.html` directly. Flows and data are binding; layout and styling advisory.
-->

---

## Feature Selection

<!-- Authored per mochiko:authoring-feature-map after stories are drafted. Derivation
     output (proposed entries, deltas, index-line drafts) stages in this spec workspace;
     the live map is written only at spec acceptance, as one atomic batch. The selection
     itself is the user's ruling — the derived table and filter verdicts are the producer's,
     the Selected line records the user's choice. -->

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-XXX | [name] | new (`proposed`) | US-1, US-3 | SC-1, SC-2 |
| FEAT-YYY | [name] | delta on delivered | US-2 | SC-3 |

### Filter rejections

<!-- One line per rejected story: the why, verbatim from its story file. Omit when none. -->

- US-4 — rejected: {{why, one line}}

### Selection

<!-- The user's ruling. Deferred SCs are the ones covered only by unselected features —
     they travel with the `proposed` entry's obligations line and are shown here at the
     moment of choice, never discovered at close. One-sided seams ride the same entries. -->

- **Selected (build now, dependency order):** FEAT-XXX, FEAT-YYY
- **Deferred (`proposed` on the map):** FEAT-ZZZ — carries SC-4, SC-5 and [seam obligations]
- **Deferred SCs:** SC-4, SC-5 — wait until FEAT-ZZZ builds

---

## Assumptions

{{assumptions}}

---

## Open Questions

{{open_questions}}
