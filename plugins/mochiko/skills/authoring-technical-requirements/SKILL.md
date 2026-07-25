---
name: authoring-technical-requirements
description: This skill MUST be invoked when authoring the technical-requirements layer of a feature specification — decomposing business functional requirements into technical requirements (TR-XXX), documenting hard constraints (C-XXX), defining measurable non-functional requirements (NFR-XXX) with numeric targets, and specifying infrastructure-provisioning requirements (IP-XXX), each traced to a business source. SHOULD also invoke when the work involves "TR-", "C-", "NFR-", "IP-", "technical requirements", "hard constraints", "non-functional requirements", "infrastructure provisioning", or authoring the constraints-and-decisions.md artifact and its C↔D / IP traceability. This skill owns the constraints-and-decisions.md artifact structure (the D-XXX field schema) and traceability — NOT the decision-evaluation technique (use mochiko:patterns-technical-decisions to evaluate alternatives and write ADRs). Produces requirements.md, constraints-and-decisions.md, and nfrs.md from a business specification.
---

# Authoring Technical Requirements

## Overview

Translate business specifications into three traceable analysis artifacts: requirements, constraints-and-decisions, and NFRs. Every artifact traces to a business source. Every target is measurable. Every constraint accounts for its design impact.

All three artifacts follow the deliverable envelope in
[`artifact-format.md`](../../templates/artifact-format.md): dense by construction — **the
statement carries the content** (no Description field re-explaining the ID line), entries
one line each, upstream text cited by ID and never re-quoted, the summary tables at each
artifact's head are its **ID index** (the reviewer coverage surface). Density is not a
gap; a gap is missing substance.

**Violating the letter of the rules is violating the spirit of the rules.**

## When to Use

- Translating functional requirements into technical requirements (TR-XXX)
- Documenting hard technical constraints (C-XXX)
- Defining measurable non-functional requirements (NFR-XXX)
- Specifying infrastructure-provisioning requirements (IP-XXX)
- Flagging which external systems the feature depends on, as an analysis concern (INT-XXX declaration — the per-endpoint boundary is authored in `mochiko:patterns-api-contracts`)
- Flagging which data the feature treats as sensitive, as an analysis concern (DS-XXX declaration — the per-attribute classification is authored in `mochiko:patterns-entity-modeling`)

## When NOT to Use

- **Writing business requirements** -- Use `mochiko:authoring-requirements` instead
- **Evaluating alternatives / making a technology decision** -- This skill owns the `constraints-and-decisions.md` artifact structure (the D-XXX field schema) and the C↔D traceability; the decision *technique* — comparing alternatives, weighing trade-offs, ADR depth, the ≥2-alternatives discipline — lives in `mochiko:patterns-technical-decisions`
- **Designing solutions** -- This skill defines the problem space, not solutions
- **Choosing technologies** -- Constraints document real boundaries, not preferences
- **Implementation planning** -- Use planning skills after technical requirements exist

## The Three Analysis Artifacts

Each artifact uses a distinct ID prefix and traces to business sources: requirements.md (TR-XXX), constraints-and-decisions.md (C-XXX / D-XXX / IP-XXX), and nfrs.md (NFR-XXX).

> **Analysis declarations vs. downstream authoring.** This skill *declares* integration points and sensitive data as analysis-level requirements (a thin INT-XXX / DS-XXX declaration — *which* external systems the feature depends on, and *which* data it treats as sensitive). It does **not** author their downstream structure:
> - The per-endpoint integration boundary — the `x-integration` OpenAPI extension (system, protocol, criticality, failure modes, authentication) — is authored on the wrapping operation in `mochiko:patterns-api-contracts`.
> - The per-attribute data-sensitivity taxonomy (the four classification levels, encryption / retention / access / audit / masking, compliance mapping, and the canonical `data-model.md` template) is authored against the data model in `mochiko:patterns-entity-modeling`.
>
> Declare the requirement here; author the boundary and the classification there.

See [ARTIFACT-TEMPLATES.md](references/ARTIFACT-TEMPLATES.md) for complete field definitions and examples.

### 1. Technical Requirements (requirements.md) -- TR-XXX

Map every business FR to one or more TRs, each addressing a distinct technical concern the FR implies but does not state (worked decomposition + field definitions: ARTIFACT-TEMPLATES.md).

**No orphan TRs.** Every TR maps to at least one FR. **No unmapped FRs.** Every FR has at least one TR.

**No exceptions:** Not for "simple" systems. Not for "obvious" mappings. Not even when the FR appears to map 1:1 — decompose anyway.

### 2. Constraints and Decisions (constraints-and-decisions.md) -- C-XXX / D-XXX

Document hard boundaries (constraints) and the technology decisions shaped by them, in a single unified artifact.

**Section 1: Hard Constraints (C-XXX)** and **Section 2: Technology Decisions (D-XXX)** — field schemas in ARTIFACT-TEMPLATES.md.

**Constraints are facts, not preferences.** Each decision record MUST reference the constraints that shaped the choice. Each constraint impact field SHOULD reference decisions it influences.

**No exceptions:** Not for "well-known" constraints. Not for "obvious" technology choices. Not even when the team has consensus — document the constraint and its source explicitly.

> **The decision *technique* lives in `mochiko:patterns-technical-decisions`** — reach it to evaluate alternatives, and record its result in the D-XXX slots. This skill owns only the field schema and the C↔D traceability.

**Section 3: Infrastructure Requirements (IP-XXX)** — field schema in ARTIFACT-TEMPLATES.md.

**Every constraint that implies platform work gets an IP-XXX item.** Constraints document boundaries; IP-XXX items document what those boundaries require operationally.

### 3. Non-Functional Requirements (nfrs.md) -- NFR-XXX

Define measurable quality attributes. Every NFR has a numeric target. Field schema in ARTIFACT-TEMPLATES.md.

**"Fast" is not a requirement.** "p95 response time < 200ms under 1000 concurrent users, measured by APM" is.

**No exceptions:** Not for "standard" performance expectations. Not for "obvious" availability targets. Every NFR gets a number, a measurement method, and a source — no deferrals to "later during design."

### 4. System Integrations -- INT-XXX (thin analysis declaration)

At the analysis layer, flag **which** external systems the feature depends on and **how critical** each is — an INT-XXX declaration that becomes a technical requirement ("the feature MUST integrate with `<system>`; its unavailability is `<criticality>`"). This is the requirement that an integration exists and matters; it is **not** the integration's wire-level contract — the per-endpoint `x-integration` boundary is authored in `mochiko:patterns-api-contracts` (the canonical home).

**Optimistic integration maps are incomplete.** Every external dependency fails eventually — so every INT-XXX declaration MUST carry through to documented failure modes and a fallback when its boundary is authored in `mochiko:patterns-api-contracts`. A declaration with no downstream boundary is an outage waiting to happen.

### 5. Data Sensitivity -- DS-XXX (thin analysis declaration)

At the analysis layer, flag **which** data the feature treats as sensitive — a DS-XXX declaration that becomes a technical requirement ("the feature handles `<data>`, which is sensitive and MUST be classified and protected"). This is the requirement that sensitive data is present and must be governed; it is **not** the per-attribute classification itself — that is authored against the data model in `mochiko:patterns-entity-modeling` (the canonical home).

## Traceability Rules

Every artifact connects to others. No artifact stands alone.

See [TRACEABILITY-PATTERNS.md](references/TRACEABILITY-PATTERNS.md) for detailed cross-reference patterns and dependency chains.

**Mandatory links:**
- TR -> FR (every technical requirement traces to business source)
- NFR -> source (every quality attribute has a justification)
- C -> D (constraints reference the decisions they shape; decisions reference constraints that shaped them)
- C -> impact (every constraint identifies what it restricts)
- C/NFR -> IP (constraints and NFRs with infrastructure implications reference IP-XXX items)

## Technology-Agnostic Writing

Describe WHAT the system must achieve, not HOW — "Must use PostgreSQL" becomes "Must support ACID transactions on relational data."

**Exception:** Constraints MAY name specific technologies when they reflect real infrastructure facts (e.g., "existing production database is PostgreSQL 15").

## Quality Checklist

Before finalizing, verify:

- [ ] Every FR has at least one TR (no unmapped business requirements)
- [ ] Every TR maps to at least one FR (no orphan technical requirements)
- [ ] Every TR has testable acceptance criteria
- [ ] Every constraint has a source, type, and severity classification
- [ ] Every decision references the constraints that shaped it (C-XXX ↔ D-XXX)
- [ ] Every NFR has a numeric target AND measurement method
- [ ] Every constraint implying platform provisioning has a corresponding IP-XXX
- [ ] Cross-references between artifacts are consistent
- [ ] Language is technology-agnostic (except real infrastructure constraints)
- [ ] ID sequences are sequential with no gaps (TR-001, TR-002..., C-001..., D-001..., IP-001...)

## Red Flags -- STOP and Restart Properly

If any excuse in the table below arises as a thought mid-authoring, STOP immediately — a shortcut is being rationalized; restart with the full process. No exceptions: not for "simple" systems, "well-understood" domains, "tight timelines", or a spec that seems complete and thorough.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Requirements are straightforward, TRs would just duplicate FRs" | Straightforward FRs hide technical complexity. Decompose anyway -- translation is the job, not transcription. |
| "NFR targets can be refined later during design" | Targets set during design are reverse-engineered from implementation, not derived from business needs. Define now. |
| "Only a few integrations, formal mapping is overkill" | Few integrations with undocumented failure modes cause the worst outages. Catalogue every one. |
| "Data classification is a security team concern" | Every technical requirement that touches data needs classification before design. Security reviews supplement, not replace. |
| "Constraints are well-known to the team" | Implicit constraints cause the costliest mid-implementation discoveries. Make every one explicit. |
| "This is a simple system" | Simple systems with missing technical requirements become complex debugging sessions. Follow the full process. |
