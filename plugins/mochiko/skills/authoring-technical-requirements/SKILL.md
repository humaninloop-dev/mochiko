---
name: authoring-technical-requirements
description: This skill MUST be invoked when authoring `constraints-and-decisions.md` — hard constraints (C-XXX), technology decisions (D-XXX), infrastructure provisioning (IP-XXX), and the thin INT-XXX / DS-XXX declarations — plus the NFR-XXX grammar the architecture store's concern rows carry, each traced to a business source. Fires in `/mochiko:implement`'s design phase, or at build time through the gated `baseline-delta.md` path. SHOULD also invoke on 'C-', 'D-', 'NFR-', 'IP-', or 'technical constraints'. Owns the artifact structure — NOT the decision technique (mochiko:patterns-technical-decisions).
---

# Authoring Technical Constraints and Decisions

## Overview

Translate business specifications into the design-time constraint layer: the
`constraints-and-decisions.md` artifact and the NFR rows the architecture store carries. Every
element traces to a business source. Every target is measurable. Every constraint accounts for
its design impact.

Both surfaces follow the deliverable envelope in
[`artifact-format.md`](../../templates/artifact-format.md): dense by construction — **the
statement carries the content** (no Description field re-explaining the ID line), entries
one line each, upstream text cited by ID and never re-quoted, the summary tables at each
artifact's head are its **ID index** (the reviewer coverage surface). Density is not a
gap; a gap is missing substance.

> **Every element answers the design ladder** (`mochiko:patterns-plan-minimalism`) before it enters the package; the simplest-execution stops are disclosed by the design phase as it authors, never re-derived here.

**Violating the letter of the rules is violating the spirit of the rules.**

## When NOT to Use

- **Writing business requirements** -- Use `mochiko:authoring-requirements` instead
- **Evaluating alternatives / making a technology decision** -- This skill owns the `constraints-and-decisions.md` artifact structure (the D-XXX field schema) and the C↔D traceability; the decision *technique* — comparing alternatives, weighing trade-offs, ADR depth, the ≥2-alternatives discipline — lives in `mochiko:patterns-technical-decisions`. **Project-scope** decisions (the knowledge-management decisions layer — `DECISIONS.md` + `.mochiko/decisions/`) are not this artifact's; only feature-scope D-XXX records live here
- **Designing solutions** -- This skill defines the problem space, not solutions
- **Choosing technologies** -- Constraints document real boundaries, not preferences
- **Slicing the build** -- cycle-card structuring is `mochiko:patterns-vertical-tdd`, downstream of this layer

## The Constraint Layers

Each layer uses a distinct ID prefix and traces to business sources: `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX, plus the thin INT-XXX / DS-XXX declarations), and the architecture store's concern rows (NFR-XXX — the ids are this skill's grammar, the row is the store's home).

> **Declarations vs. downstream authoring.** This skill *declares* integration points and sensitive data as thin rows on `constraints-and-decisions.md` (INT-XXX / DS-XXX — *which* external systems the feature depends on, and *which* data it treats as sensitive). It does **not** author their downstream structure:
> - The per-endpoint integration boundary — the `x-integration` OpenAPI extension (system, protocol, criticality, failure modes, authentication) — is authored on the wrapping operation in `mochiko:patterns-api-contracts`.
> - The per-attribute data-sensitivity taxonomy (the four classification levels, encryption / retention / access / audit / masking, compliance mapping, and the canonical `data-model.md` template) is authored against the data model in `mochiko:patterns-entity-modeling`.
>
> Declare the requirement here; author the boundary and the classification there.

See [ARTIFACT-TEMPLATES.md](references/ARTIFACT-TEMPLATES.md) for complete field definitions and examples.

### 1. Constraints and Decisions (constraints-and-decisions.md) -- C-XXX / D-XXX

Document hard boundaries (constraints) and the technology decisions shaped by them, in a single unified artifact.

**Section 1: Hard Constraints (C-XXX)** and **Section 2: Technology Decisions (D-XXX)** — field schemas in ARTIFACT-TEMPLATES.md.

**Constraints are facts, not preferences.** Each decision record MUST reference the constraints that shaped the choice. Each constraint impact field SHOULD reference decisions it influences.

**No exceptions:** Not for "well-known" constraints. Not for "obvious" technology choices. Not even when the team has consensus — document the constraint and its source explicitly.

> **The decision *technique* lives in `mochiko:patterns-technical-decisions`** — reach it to evaluate alternatives, and record its result in the D-XXX slots. This skill owns only the field schema and the C↔D traceability.

**Structural decisions are not this artifact's.** A *topology* choice — component boundaries,
interaction style, responsibility placement — is recorded in the architecture store's delta, whose
ruling **is** its decision record; it never becomes a D-XXX row here. The D-XXX rows in this
artifact are technology decisions with one origin: this skill's analysis-time author. Where a
technology decision and a topology choice are genuinely entangled, record the technology decision
here and let the store's delta carry the shape, cross-citing by ID.

**Section 3: Infrastructure Requirements (IP-XXX)** — field schema in ARTIFACT-TEMPLATES.md.

**Every constraint that implies platform work gets an IP-XXX item.** Constraints document boundaries; IP-XXX items document what those boundaries require operationally.

### 2. Non-Functional Requirements -- NFR-XXX (homed in the architecture store)

Define measurable quality attributes. Every NFR has a numeric target. **There is no `nfrs.md`:** an
NFR-XXX row lives as fields on its architecture-store concern row, so one concern has one home —
stance, pattern, targets, as-built, drift together. The **ids and the grammar are this skill's**
(numeric target · measurement method · source · category), the **row shape is the store's**
(`plugins/mochiko/schemas/architecture-store.yaml`). **An NFR's source is its business source** —
the FR-XXX or SC-XXX whose promise the target serves — so the trace chain resolves
FR-XXX / SC-XXX → NFR-XXX. At design time a new or changed target reaches the store the
way every other store write does: drafted in the package, written at the user's sign-off. Grammar
and field definitions in ARTIFACT-TEMPLATES.md.

**"Fast" is not a requirement.** "p95 response time < 200ms under 1000 concurrent users, measured by APM" is.

**No exceptions:** Not for "standard" performance expectations. Not for "obvious" availability targets. Every NFR gets a number, a measurement method, and a source — no deferrals to "later during design."

### 3. System Integrations -- INT-XXX (thin declaration)

Flag **which** external systems the feature depends on and **how critical** each is — an INT-XXX row on `constraints-and-decisions.md` ("the feature MUST integrate with `<system>`; its unavailability is `<criticality>`"). This is the record that an integration exists and matters; it is **not** the integration's wire-level contract — the per-endpoint `x-integration` boundary is authored in `mochiko:patterns-api-contracts` (the canonical home).

**Optimistic integration maps are incomplete.** Every external dependency fails eventually — so every INT-XXX declaration MUST carry through to documented failure modes and a fallback when its boundary is authored in `mochiko:patterns-api-contracts`. A declaration with no downstream boundary is an outage waiting to happen.

### 4. Data Sensitivity -- DS-XXX (thin declaration)

Flag **which** data the feature treats as sensitive — a DS-XXX row on `constraints-and-decisions.md` ("the feature handles `<data>`, which is sensitive and MUST be classified and protected"). This is the record that sensitive data is present and must be governed; it is **not** the per-attribute classification itself — that is authored against the data model in `mochiko:patterns-entity-modeling` (the canonical home).

## Traceability Rules

Every artifact connects to others. No artifact stands alone.

See [TRACEABILITY-PATTERNS.md](references/TRACEABILITY-PATTERNS.md) for detailed cross-reference patterns and dependency chains.

**Mandatory links:**
- NFR -> FR/SC (every quality attribute traces to the business promise it serves)
- C -> D (constraints reference the decisions they shape; decisions reference constraints that shaped them)
- C -> impact (every constraint identifies what it restricts)
- C/NFR -> IP (constraints and NFRs with infrastructure implications reference IP-XXX items)

## Technology-Agnostic Writing

Describe WHAT the system must achieve, not HOW — "Must use PostgreSQL" becomes "Must support ACID transactions on relational data."

**Exception:** Constraints MAY name specific technologies when they reflect real infrastructure facts (e.g., "existing production database is PostgreSQL 15").

## Quality Checklist

Before finalizing, verify:

- [ ] Every constraint has a source, type, and severity classification
- [ ] Every decision references the constraints that shaped it (C-XXX ↔ D-XXX)
- [ ] No topology choice recorded as a D-XXX row here — structural rulings live in the architecture store's delta
- [ ] Every NFR has a numeric target AND measurement method AND its FR-XXX / SC-XXX source, on its store concern row (never a standalone `nfrs.md`)
- [ ] Every constraint implying platform provisioning has a corresponding IP-XXX
- [ ] Every INT-XXX and DS-XXX declaration is present as a row, with its downstream home named
- [ ] Cross-references between artifacts are consistent
- [ ] Language is technology-agnostic (except real infrastructure constraints)
- [ ] ID sequences are sequential with no gaps (C-001..., D-001..., IP-001..., INT-001..., DS-001...)

## Red Flags -- STOP and Restart Properly

If any excuse in the table below arises as a thought mid-authoring, STOP immediately — a shortcut is being rationalized; restart with the full process. No exceptions: not for "simple" systems, "well-understood" domains, "tight timelines", or a spec that seems complete and thorough.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "NFR targets can be refined later during design" | Targets set during design are reverse-engineered from implementation, not derived from business needs. Define now. |
| "Only a few integrations, formal mapping is overkill" | Few integrations with undocumented failure modes cause the worst outages. Catalogue every one. |
| "Data classification is a security team concern" | Every constraint that touches data needs classification before design. Security reviews supplement, not replace. |
| "Constraints are well-known to the team" | Implicit constraints cause the costliest mid-implementation discoveries. Make every one explicit. |
| "This is a simple system" | Simple systems with missing constraints become complex debugging sessions. Follow the full process. |
