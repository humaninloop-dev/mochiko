---
name: patterns-system-design
description: This skill MUST be invoked when designing a feature's architecture at design time — authoring the per-feature `architecture.md`: a C4-container-level delta diagram, sequence diagrams for qualifying flows, a D-XXX-linked component table, and a conditional deployment view. SHOULD also invoke on 'architecture design', 'container diagram', 'C4', 'system topology', or 'architecture delta'. Seeds the current-state baseline before drafting the delta. Distinct from `authoring-architecture` (repo ARCHITECTURE.md); does not grade its own output.
---

# Designing Feature Architecture

## Overview

Design the **shape** of a feature before its detail: what the components are, where the boundaries
cut, how the pieces talk, and how the proposed system differs from the one that exists today. The
artifact is `architecture.md` in the feature's spec dir — a **delta view** (current state + proposed
target, the structural change made visible) that the detailed design artifacts then conform to. It
is the design-time architecture surface, authored **before** `data-model.md` and `contracts/`.

## When NOT to Use

- **Repo-level `ARCHITECTURE.md`** — that living, current-state operating doc is `mochiko:authoring-architecture`, folded post-hoc at a landing. This skill is the design-time, feature-scope, delta artifact — distinct file, distinct moment.
- **Entity / data-model design** — `mochiko:patterns-entity-modeling` details the approved shape's data downstream; it does not decide topology.
- **API-contract design** — `mochiko:patterns-api-contracts` details the approved shape's endpoints downstream.
- **Evaluating a structural fork's alternatives** — the decision *technique* (≥2 alternatives, ADR depth) is `mochiko:patterns-technical-decisions`; this skill records the resulting shape and links its D-XXX rows.

## Seed the baseline before you design on it

The delta's current-state half must be real, not assumed:

1. **`ARCHITECTURE.md` exists** → seed the current state from it.
2. **Absent (the bootstrap)** → reconstruct the baseline topology from the code (and `codebase-analysis.md` when present), mark it **reconstructed** with a confidence note, and treat it as the seed only once it is **confirmed**. Never design a delta on an unconfirmed baseline — a wrong baseline makes the whole delta a fiction. The confirmed baseline is what lands as the initial `ARCHITECTURE.md` downstream.
3. **Greenfield** → the current state is empty; the target *is* the whole picture (the delta degenerates cleanly, no bootstrap needed).

## The four pieces

`architecture.md` carries four pieces. Diagram conventions, delta styling, and copy-ready mermaid
templates for each live in [DIAGRAM-CONVENTIONS.md](references/DIAGRAM-CONVENTIONS.md).

### 1. Container delta diagram (the sign-off surface)

One mermaid **flowchart** of the target state at C4 **container** level — services, workers, stores,
queues, external systems. **C4-as-method, flowchart-as-carrier**: mermaid's dedicated C4 syntax is
experimental and renders unreliably, so use standard `flowchart` syntax and apply C4 discipline by
hand — `subgraph` blocks for boundaries, the technology named in each node label, arrows labelled
with **protocol + purpose** (`HTTPS / fetch profile`, not a bare line). Mark the **delta visually**:
new and modified components styled distinctly (via `classDef`), removed ones struck. This diagram is
what the reader approves — it must render, never ship as a raw code block masquerading as a picture.

### 2. Sequence diagrams for qualifying flows

One mermaid `sequenceDiagram` per **qualifying flow**: any flow that **crosses ≥2 components and has
non-trivial ordering or failure semantics** — a user journey *or* a system flow (async settlement,
retry, webhook re-entry, saga). P1 user journeys are the **floor, never the cap**: an ordering- or
failure-critical system flow qualifies even when no P1 journey names it. A topology diagram cannot
show ordering or what happens when a step fails; the sequence view is where that lives.

### 3. Component table + delta summary

A **container-level register** — one line per deployable/runnable piece, mirroring `ARCHITECTURE.md`'s
form: `name — responsibility — boundary — status (new / modified / existing)`. ("Component" here is
the container-level register sense, not C4-level-3.) Below it, a **delta summary** (prose) linking
each structural change to the **D-XXX** row that ruled it — link, never restate the decision. Every
box in the diagram appears in the table and vice versa.

### 4. Deployment view — conditional

A deployment view (runtime/infra placement) **only when the feature changes deployment reality**.
**Trigger:** the feature carries `IP-XXX` infrastructure-provisioning rows. No `IP-XXX` → omit it,
and record the omission in one line rather than shipping an empty section.

## Scope the diagram to the delta neighborhood

The container diagram scopes to the **delta neighborhood** — the changed components plus their
**direct collaborators**, not the whole system. Past a size threshold — **default: ~12 rendered
nodes** (boxes), overridable per project — the full system view is **linked, never inlined**; the
artifact shows the neighborhood and points to the wider map. This keeps a 2-node delta from rendering
a 50-box wall.

## The no-delta form

Every feature produces the artifact, including one that changes nothing structurally. A **no-delta**
feature presents the **unchanged** container diagram (reseeded from the baseline) plus a **one-line
claim** — "this feature changes nothing structurally" — for approval. The no-delta judgment is always
shown, never made silently. On a large system, the scale bound still governs: show the neighborhood
the feature touches, link the rest.

## architecture.md Structure

Follows the deliverable envelope in [`artifact-format.md`](../../templates/artifact-format.md) —
dense, one read, statement-carries-the-content. Density is not a gap; a gap is a missing component,
an unlabelled arrow, or a qualifying flow with no sequence diagram.

````markdown
# Architecture: {feature_id}

> Container-level topology and the current→target delta. Sign-off surface for the shape; detail
> (entities, endpoints) is drawn downstream against the approved target.

## Baseline  *(current state)*

Seeded from `ARCHITECTURE.md` · or **reconstructed** from code (confidence: {high/medium/low}) · or
greenfield (empty). {one line stating which}

## Container Diagram  *(target; delta marked)*

```mermaid
flowchart TB
  %% subgraph boundaries, technology in node labels, protocol+purpose on arrows,
  %% classDef for new/modified, strike for removed — see DIAGRAM-CONVENTIONS.md
```

## Components  *(container-level register)*

| Component | Responsibility | Boundary | Status |
|-----------|----------------|----------|--------|
| Profile API | serves + edits user profiles | owns Profile store | existing |
| Avatar Worker | resizes uploaded avatars | reads queue, writes blob store | new |

### Delta summary

- **Avatar Worker (new)** — decouples image processing from the request path. Ruled in **D-004**.
- {each structural change → its D-XXX row; link, never restate}

## Key Flows

```mermaid
sequenceDiagram
  %% one per qualifying flow (≥2 components, non-trivial ordering/failure) — see DIAGRAM-CONVENTIONS.md
```

## Deployment  *(conditional — only when IP-XXX rows exist)*

{runtime/infra placement, or one line: "no deployment change — no IP-XXX rows"}
````

## Quality Checklist

Before handing the architecture off, verify:

- [ ] The current-state baseline is seeded from `ARCHITECTURE.md`, or reconstructed **and confirmed** (confidence noted), or greenfield-empty
- [ ] Every box in the container diagram appears in the component table, and vice versa
- [ ] Every arrow carries protocol + purpose; every node names its technology; boundaries are subgraphs
- [ ] The delta is visually marked (new/modified styled, removed struck)
- [ ] Every qualifying flow (≥2 components, non-trivial ordering/failure) has a sequence diagram
- [ ] Every component is marked new / modified / existing
- [ ] The delta summary links each structural change to a D-XXX row (link, not restatement)
- [ ] The deployment view is present iff IP-XXX rows exist (else its absence is recorded)
- [ ] The diagram scopes to the delta neighborhood; the wider system is linked past the threshold
- [ ] A no-delta feature still shows the reseeded diagram + the one-line no-structural-change claim
- [ ] The diagram renders (valid mermaid) — it is meant to be seen, not read as source

## Common Mistakes

| Mistake | ❌ Bad | ✅ Good |
|---------|--------|---------|
| Bare arrows | `A --> B` | `A -->|HTTPS / fetch profile| B` |
| No technology | `Profile API` | `Profile API<br/>(Node/Express)` |
| Delta invisible | new + existing boxes styled identically | new/modified via `classDef`, removed struck |
| Topology in the wrong artifact | a new component first appearing in `data-model.md` | the component declared here, the data drawn downstream against it |
| Sequence cap by priority | only P1 journeys get sequences | every qualifying flow does — P1 is the floor, not the cap |
| Whole-system wall | 50 boxes for a 2-node change | the delta neighborhood inlined, the rest linked |
| Restated decisions | the rationale re-typed under the diagram | the delta summary links the D-XXX row |
