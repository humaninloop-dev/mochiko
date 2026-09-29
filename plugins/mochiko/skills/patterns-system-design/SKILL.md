---
name: patterns-system-design
description: This skill MUST be invoked when designing an architecture delta — the diagram craft and altitude discipline for a structural change drafted against the product architecture store: a C4-container delta diagram, sequence diagrams for qualifying flows, a checkpoint table of the changed store elements, and a conditional deployment view. SHOULD also invoke on 'architecture delta', 'architecture design', 'container diagram', 'C4', or 'system topology'. Reads the store's spine for the current state; never re-derives it. Distinct from `authoring-architecture-store` (store grammar and lifecycle); does not grade its own output.
---

# Designing an Architecture Delta

## Overview

Design the **shape** of a structural change before its detail: what the components are, where the
boundaries cut, how the pieces talk, and how the proposed system differs from the one the product
architecture store already describes. The store is the standing topology; this skill governs the
**altitude and the diagram craft** of the **delta** drawn against it — the drawing the user signs
(`architecture.md` in the run's home). The change itself is written in place in the store as
`proposed (<key>)` elements, through `mochiko:authoring-architecture-store`, and flipped at
sign-off. The delta is drawn **before**
`data-model.md` and `contracts/`, which conform to the approved shape.

## When NOT to Use

- **The store itself** — its grammar, element lifecycle, statuses, graduation, health view, and the derived root index are `mochiko:authoring-architecture-store`. This skill draws the delta; that one owns what the delta is written into.
- **What stance a concern row takes** — shelf dimensions, suggested defaults, and upgrade triggers are `mochiko:patterns-architecture-shelves`, dealt at the `/mochiko:architecture` desk. This skill draws structure; it never forms a stance.
- **Entity / data-model design** — `mochiko:patterns-entity-modeling` details the approved shape's data downstream; it does not decide topology.
- **API-contract design** — `mochiko:patterns-api-contracts` details the approved shape's endpoints downstream.
- **Whether the structure is paid for at all** — the design-time weight ladder (is this component or layer earned by a requirement?) lives in `mochiko:patterns-plan-minimalism`; this skill governs the *altitude* of what you draw (container level, not lower), that one governs the *amount*.
- **Evaluating a structural fork's alternatives** — the decision *technique* (≥2 alternatives, ADR depth) is `mochiko:patterns-technical-decisions`; this skill records the resulting shape and links the ruling.

## The baseline is the store

The delta's current-state half is **read from the store's topology spine**
(`.mochiko/product/architecture/spine.md`), never re-derived:

1. **The store exists** → the spine *is* the current state. Read it. Do not reconstruct what is
   already written down, and do not redraw neighborhoods the change does not touch.
2. **No store yet** → reconstruction is the `/mochiko:architecture` desk's work, not this skill's:
   the desk bootstraps the store from what exists and the user confirms it. **Never design a delta
   on an unconfirmed baseline — a wrong baseline makes the whole delta a fiction.**
3. **Greenfield** → the spine is empty; the target *is* the whole picture (the delta degenerates
   cleanly).

## The delta's pieces

A delta carries up to four pieces. Diagram conventions, delta styling, and copy-ready mermaid
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

### 3. Checkpoint table of changed elements

A **checkpoint table** of the change — one row per store element the pinned-base diff adds, changes,
or cuts: `id — kind — name — change (adds · changes · cuts) — status`. It is the sign-off surface
where no render exists, and the signed state where no commit is made.
**Altitude check — every element the change writes as `kind: container` is a genuine container,
not a C4-level-3 construct:** each must be a separately deployable or independently runnable piece
(a service, worker, store, queue, external system). A code-level layer *inside* one process — an
application or domain layer, a module, a port or a trait — is **not** a container and does not earn
a `container` row; it belongs in the detailed design, not here. Boundary and flow elements and
`AX-XXX` rows carry their own kinds and are not held to this check. Each row links **the ruling
that made it**, which lives on the store element the change writes — the spine element (`SPN-XXX`)
it adds, moves, or retires, or the concern row (`AX-XXX`) it answers — and, where an
analysis-origin `D-XXX` row governs the fork, that row. **Link, never restate the decision.** Every
new, modified or removed box in the diagram has its `container` row in the table, and every
`container` row is a styled box; unchanged collaborators are drawn plain and take no row.

### 4. Deployment view — conditional

A deployment view (runtime/infra placement) **only when the change alters deployment reality**.
**Trigger:** the feature carries `IP-XXX` infrastructure-provisioning rows. No `IP-XXX` → omit it,
and record the omission in one line rather than shipping an empty section.

## Scope the diagram to the delta neighborhood

The container diagram scopes to the **delta neighborhood** — the changed components plus their
**direct collaborators**, not the whole system. Past a size threshold — **default: ~12 rendered
nodes** (boxes) — the full system view is **linked, never inlined**; the delta shows the
neighborhood and points at the spine for the wider map. This keeps a 2-node delta from rendering a
50-box wall. The count is overridable per project, but an override must **assert the altitude** —
that every extra node is a genuine container (per the altitude check above), not merely cite a
larger count. A high node count that is really sub-container detail is drift, not a legitimate
override.

## The no-delta judgment

A run whose work changes nothing structurally still says so: a **one-line claim** — "this feature
changes nothing structurally" — recorded in the design-phase package and shown at the gate. **The no-delta
judgment is always shown, never made silently.** No diagram is drawn and the store is untouched: the
standing spine already carries the unchanged picture, and redrawing it would assert a delta where
none exists.

## Density

The delta follows the deliverable envelope in
[`artifact-format.md`](../../templates/artifact-format.md) — dense, one read,
statement-carries-the-content. Density is not a gap; a gap is a missing component, an unlabelled
arrow, or a qualifying flow with no sequence diagram.

## Quality Checklist

Before handing the delta off, verify:

- [ ] The current state is read from the store's spine — not reconstructed here, not assumed
- [ ] Every new, modified or removed box in the container diagram has its `container` row in the checkpoint table, and every `container` row is a styled box; unchanged collaborators are drawn plain and take no row
- [ ] Every element the table lists as `kind: container` is a deployable/runnable **container** — no application/domain layer, module, port, or trait (C4-level-3 detail) inside a single process
- [ ] Every arrow carries protocol + purpose; every node names its technology; boundaries are subgraphs
- [ ] The delta is visually marked (new/modified styled, removed struck)
- [ ] Every qualifying flow (≥2 components, non-trivial ordering/failure) has a sequence diagram
- [ ] Each changed element carries the ruling that made it, and the table links it (link, not restatement)
- [ ] The change is in the store as `proposed` elements; the drawing carries no register text and no current-state narrative
- [ ] The deployment view is present iff IP-XXX rows exist (else its absence is recorded)
- [ ] The diagram scopes to the delta neighborhood; the wider system is linked past the threshold
- [ ] A run that changes nothing structurally records the one-line no-delta claim rather than drawing a diagram
- [ ] The diagram renders (valid mermaid) — it is meant to be seen, not read as source

## Common Mistakes

| Mistake | ❌ Bad | ✅ Good |
|---------|--------|---------|
| Bare arrows | `A --> B` | `A -->|HTTPS / fetch profile| B` |
| No technology | `Profile API` | `Profile API<br/>(Node/Express)` |
| Delta invisible | new + existing boxes styled identically | new/modified via `classDef`, removed struck |
| Redrawing the unchanged | the whole spine reseeded into the delta | the delta shows what changes; the spine already holds the rest |
| Topology in the wrong place | a new component first appearing in `data-model.md` | the component declared in the delta, the data drawn downstream against it |
| Sequence cap by priority | only P1 journeys get sequences | every qualifying flow does — P1 is the floor, not the cap |
| Whole-system wall | 50 boxes for a 2-node change | the delta neighborhood inlined, the rest linked |
| Restated decisions | the rationale re-typed under the diagram | the element carries the ruling; the table links it |
| Sub-container table rows | rows like `Preflight domain (Rust · no I/O)`, `Engine port trait` inside one process | one `container` row per deployable/runnable piece; code-level layers live in the detailed design |
