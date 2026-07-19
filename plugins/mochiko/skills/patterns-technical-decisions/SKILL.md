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

For each decision point, consider 2-3 alternatives minimum.

**Quick Criteria Reference:**

| Criterion | Key Question |
|-----------|--------------|
| **Fit** | Does it solve the problem fully? |
| **Complexity** | How hard to implement and maintain? |
| **Team Familiarity** | Does the team know this tech? |
| **Ecosystem** | Good docs, active community? |
| **Scalability** | Will it grow with the project? |
| **Security** | Good security posture? |
| **Cost** | Total cost of ownership? |
| **Brownfield Alignment** | Fits existing stack? |

See [EVALUATION-MATRIX.md](references/EVALUATION-MATRIX.md) for detailed criteria, scoring, and technology category comparisons.

### Phase 2: Decide

Score options against weighted criteria. Document:
- Which option scores best
- Why criteria were weighted as they were
- What trade-offs are accepted

**Quick Comparison Format:**

| Option | Pros | Cons | Alignment | Verdict |
|--------|------|------|-----------|---------|
| Option A | + Fast, + Simple | - New dep | High | **Best** |
| Option B | + Familiar | - Slow | Medium | Good |
| Option C | + Feature-rich | - Complex | Low | Poor |

**When a decision cannot be resolved** — missing information, or an unsettled judgment call — mark it **NEEDS CLARIFICATION** rather than guessing. Marking the gap is this skill's job. *Driving* its resolution — looping the decision back for another round, or escalating a genuine judgment call to the human gate — belongs to the command supervisor that runs the loop, not to this skill (see `mochiko:loop-discipline`).

### Phase 3: Document

Record decisions in ADR format for future maintainers.

**Quick Decision Record:**

```markdown
## Decision: [Title]

**Status**: Proposed | Accepted | Deprecated

**Context**: [Why this decision is needed]

**Decision**: [What we chose]

**Rationale**: [Why - connect to criteria]

**Trade-offs Accepted**: [What we gave up]
```

See [DECISION-RECORD.md](references/DECISION-RECORD.md) for full ADR format, consequences, and dependency tracking.

## Where decisions are recorded

The decision records this skill produces are written into the **`constraints-and-decisions.md`** artifact — but that artifact's shape is **not** this skill's to define. Its file structure, its Section-2 Technology-Decisions **`D-XXX`** field schema, and the constraint↔decision / infrastructure-planning traceability are owned by `mochiko:authoring-technical-requirements`. Do not restate the artifact template here.

Boundary (handoff `mochiko:authoring-technical-requirements` → this skill):

| Owns | Primitive |
|------|-----------|
| The **artifact** — `constraints-and-decisions.md` template, the `D-XXX` decision field schema, `C-XXX`↔`D-XXX` / `IP-XXX` traceability | `mochiko:authoring-technical-requirements` |
| The **technique** — evaluating alternatives, weighing trade-offs and consequences, brownfield-alignment scoring, and the ADR record depth that fills each `D-XXX` slot | this skill |

Author the decision *content* with the technique in this skill; place it into the `D-XXX` slots that `mochiko:authoring-technical-requirements` defines.

## Brownfield Alignment

Always check existing stack first:

| Scenario | Alignment | Action |
|----------|-----------|--------|
| Existing dep solves problem | High | Prefer reuse |
| New dep, same ecosystem | Medium | Document justification |
| New dep, different ecosystem | Low | Strong justification needed |
| Conflicting with existing | None | Avoid or escalate |

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

## Common Mistakes

### Single Option "Evaluation"
❌ "We evaluated Option A and chose it"
✅ "We compared Option A, Option B, and Option C against weighted criteria"

### Shiny Object Syndrome
❌ Choosing newest technology because it's trending
✅ Require strong justification for unfamiliar dependencies over existing stack

### Vague Rationale
❌ "We chose JWT because it's better"
✅ "We chose JWT because: stateless (fits our scale), team familiarity (3/4 devs), ecosystem support"

### Ignoring Team Skills
❌ Choosing Rust for a Python team without accounting for learning curve
✅ Weight team familiarity criterion appropriately in evaluation matrix

### Missing Trade-offs
❌ Only listing positives of chosen option
✅ Explicitly document what was given up: "Trade-off: JWT requires token refresh handling"

### Orphan Decisions
❌ Decisions documented in isolation
✅ Map decision dependencies: "D2 (session storage) depends on D1 (auth mechanism)"

### Governance Blindness
❌ Making decisions that violate project principles
✅ Check alignment with the project governance before finalizing
