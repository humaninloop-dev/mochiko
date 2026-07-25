---
name: patterns-technical-decisions
description: This skill MUST be invoked when making and documenting a technology or architecture decision — evaluating two or more alternatives against weighted criteria, capturing the trade-offs and consequences of the choice, scoring brownfield alignment with the existing stack, and recording the rationale as a decision record (ADR). SHOULD also invoke when the work involves evaluating alternatives, weighing trade-offs and consequences, a decision record or ADR, decision rationale ("why we chose"), brownfield-alignment scoring, or marking a decision NEEDS CLARIFICATION. Owns the decision-making technique and ADR record depth; the decisions it produces are recorded in the constraints-and-decisions.md artifact owned by mochiko:authoring-technical-requirements.
---

# Making Technical Decisions

## Overview

Provide a complete framework for technology decisions: evaluate alternatives against consistent criteria, make informed choices, and document decisions so future maintainers understand WHY choices were made.

This skill owns the decision-making **technique** — how to evaluate, decide, and document well. The `constraints-and-decisions.md` **artifact** the decisions land in is owned by `mochiko:authoring-technical-requirements`; this skill references that artifact rather than restating it (see *Where decisions are recorded* below).

## When to Use

- Choosing between technology options (libraries, frameworks, services)
- When a decision is blocked on missing information and must be flagged **NEEDS CLARIFICATION**
- Documenting architectural decisions for the team
- When a technology choice needs a documented justification
- Evaluating existing stack vs new dependencies
- Any decision with long-term maintenance implications

## When NOT to Use

- **Trivial changes** - No architectural impact, obvious solution
- **Decisions already documented** - Existing ADR covers the scenario
- **Emergency hotfixes** - Document decision post-facto, don't block fix
- **Pure implementation details** - Internal code structure without external impact
- **Reversible choices** - Easily changed later without consequence

## Decision Workflow

```
1. EVALUATE    →    2. DECIDE    →    3. DOCUMENT
   Options           Best fit          For posterity
```

### Phase 1: Evaluate Options

For each decision point, consider 2-3 alternatives minimum, evaluated against the eight criteria — Fit, Complexity, Team Familiarity, Ecosystem, Scalability, Security, Cost, Brownfield Alignment.

See [EVALUATION-MATRIX.md](references/EVALUATION-MATRIX.md) for the detailed criteria table, scoring, and technology category comparisons.

### Phase 2: Decide

Score options against weighted criteria. Document:
- Which option scores best
- Why criteria were weighted as they were
- What trade-offs are accepted

Use the two comparison shapes in [EVALUATION-MATRIX.md](references/EVALUATION-MATRIX.md) — the weighted decision matrix or the quick side-by-side options comparison; don't invent a third format.

**When a decision cannot be resolved** — missing information, or an unsettled judgment call — mark it **NEEDS CLARIFICATION** rather than guessing. Marking the gap is this skill's job. *Driving* its resolution — looping the decision back for another round, or escalating a genuine judgment call to the human gate — belongs to the command supervisor that runs the loop, not to this skill (see `mochiko:loop-discipline`).

### Phase 3: Document

Record decisions in ADR format for future maintainers — Status, Context, Decision, Rationale (connected to criteria), Alternatives Considered, and Consequences/trade-offs.

See [DECISION-RECORD.md](references/DECISION-RECORD.md) for the full ADR format, rationale best practices, consequence documentation, and dependency tracking.

## Where decisions are recorded

Feature-scope decision records are written into the **`constraints-and-decisions.md`** artifact — but that artifact's shape is **not** this skill's to define. **Project-scope** decisions (a project carrying the knowledge-management module) land in the decisions layer instead — a `DECISIONS.md` index row + a `.mochiko/decisions/<date>-<slug>.md` record, schema per the module's project-pinned copy — same technique, different destination. Its file structure, its Section-2 Technology-Decisions **`D-XXX`** field schema, and the constraint↔decision / infrastructure-planning traceability are owned by `mochiko:authoring-technical-requirements`. Do not restate the artifact template here.

Boundary (handoff `mochiko:authoring-technical-requirements` → this skill):

| Owns | Primitive |
|------|-----------|
| The **artifact** — `constraints-and-decisions.md` template, the `D-XXX` decision field schema, `C-XXX`↔`D-XXX` / `IP-XXX` traceability | `mochiko:authoring-technical-requirements` |
| The **technique** — evaluating alternatives, weighing trade-offs and consequences, brownfield-alignment scoring, and the ADR record depth that fills each `D-XXX` slot | this skill |

Author the decision *content* with the technique in this skill; place it into the `D-XXX` slots that `mochiko:authoring-technical-requirements` defines.

## Brownfield Alignment

Always check the existing stack first — the alignment-scoring table (High = prefer reuse … None = avoid or escalate) lives in [EVALUATION-MATRIX.md](references/EVALUATION-MATRIX.md) under Brownfield Considerations.

## Quality Checklist

Before finalizing:

**Evaluation:**
- [ ] At least 2-3 alternatives considered
- [ ] Criteria weighted by project context
- [ ] Each option has pros/cons
- [ ] Brownfield alignment assessed

**Documentation:**
- [ ] Context explains WHY decision is needed
- [ ] Rationale connects to specific criteria
- [ ] Trade-offs explicitly documented
- [ ] Governance alignment checked
- [ ] Dependencies between decisions mapped
