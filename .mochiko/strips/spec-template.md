# Strip notes — `templates/spec-template.md`

Entry formats: `strips/README.md`. First entry v0.58.0 — earlier spec-template edits rode
their wave's command entries (`strips/specify.md`), predating the one-file-per-primitive
convention's application here.

## [v0.74.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/spec.yaml + mochiko-cli template spec
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
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
````
- **Kept deliberately:** Every line of guidance preserved — lifted into `plugins/mochiko/schemas/spec.yaml` (skeleton / contract / overview / register / density) and rendered by `mochiko-cli template spec`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). Nothing dropped.
- **Consumers assessed:** `commands/specify.md` spec read-pointer (re-pointed by P4) · `skills/mochiko/SKILL.md` router row (re-described CLI/schema-delivered by P5) · `skills/authoring-prototype/SKILL.md` (re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.58.0] Delivery Slices section out — Feature Selection in; stories become an index; slice tags re-key to FEAT
- **Disposition:** superseded → the feature-map-layer restructure: a **Feature Selection** section (derived-features table, filter rejections, the user's selection with the deferred-SC list per D21); the User Stories section becomes an index over `stories/US-*.md` files (D10 — story text, acceptance scenarios, FEAT-ID mapping live there; only story-native status is `rejected` with the why); Screens & Flows tag grammar re-keys slice → FEAT (R10 re-tag pass, rejected-story screens kept greyed).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md` — D4 slices retire, D10 workspace restructure, D20 acceptance-time writes, D21 deferred-SC visibility)
- **Content (superseded, verbatim):**

  The Intent section's Delivery comment: `<!-- increments vs whole; first shippable value; sequencing constraints -->` (now `whole vs subset now; …`) and the header comment's `Governs the authoring depth, the Delivery slices shape, and the stress-test rigor.` (now `the feature derivation`).

  The header placeholders `# {{feature_title}}` / `> Feature: {{feature_id}}` — re-keyed to `# {{spec_title}}` / `> Spec: {{spec_id}}` (audit fix: under D3/D9 a spec is not a feature, and `feature_id` collides with the FEAT-XXX grammar; matches specify.md's `<spec>` workspace slug). No other `feature_`-prefixed placeholder existed (grep-verified).

  The Screens & Flows `Slice` columns (`| SCR-001 | [name] | [one line] | [fields/collections rendered] | S1 |`, `| FLOW-001 | [name] | SCR-001 → [action] → SCR-002 | US-1 / [scenario] | S1 |`) and the comment clause `Slice column only when Delivery Slices decomposes.`

  The whole Delivery Slices section:

  ````markdown
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
  ````
- **Kept deliberately:** the artifact-format envelope header, Intent/Overview/Edge Cases/FR/Key Entities/SC/Assumptions/Open Questions sections whole; the Screens & Flows binding-manifest discipline (binding flows, advisory pixels, waiver line) — only the tag column re-keys. **The Graduation contract's every clause survives re-keyed to features**: per-feature runs in dependency order + shared-artifact accumulation + extend-mode + `[MODIFY]` breaking amendment + regression safety live in `authoring-feature-map` and land in `plan.md`/`implement.md` per D18 (wave 3); the Feature-Done SC-coverage map becomes derivation-time SC re-homing (D21); cross-slice seams become later-landing-feature ownership (D13) + one-sided-seam obligations on `proposed` entries (D21); extend obligations become entry obligations lines (D21/R2). Feature-close, which the seams clause referenced, was itself dissolved into implement's acceptance landing (D13).
- **Consumers assessed:** `specify.md` (co-edited this wave, its own strip entry) · `authoring-slices` (retired this wave, its own supersession entry) · `review-specifications` + `authoring-prototype` (slice-tag/Delivery-Slices grade references — wave-owned edits per the build plan; prototype's re-tag grammar noted in its wave brief) · `plan.md` / `implement.md` / `patterns-vertical-tdd` / `tasks-template.md` (Graduation-contract and slice-layout consumers — wave-3 producer's territory, hand-off noted, not edited here) · `ARCHITECTURE.md` + `artifact-format.md` (ripple wave).
