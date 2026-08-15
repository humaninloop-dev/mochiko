---
name: authoring-requirements
description: This skill MUST be invoked when authoring the functional-requirements layer of a feature specification — technology-agnostic functional requirements in FR-XXX format, edge cases, and measurable success criteria in SC-XXX format. SHOULD also invoke when the work involves 'functional requirements', 'FR-', 'success criteria', 'SC-', 'RFC 2119', 'MUST SHOULD MAY', or 'edge cases'.
---

# Authoring Requirements

## Overview

Write technology-agnostic functional requirements, identify edge cases, and define measurable success criteria. Focus on WHAT the system does and WHY, never HOW it's implemented.

**A constraint states a capability, never a posture.** "The corpus is files on disk" is a storage posture wearing constraint clothes when the real requirement is "retrieval is locally computable" — it names no product and passes the leakage check, yet freezes a plan-time choice downstream seats then read as ratified.

The artifact this layer lands in (`spec.md`) follows the deliverable envelope in
[`artifact-format.md`](../../templates/artifact-format.md) — dense by construction,
human-legible: one-line FR/SC/edge-case entries, ≤ 3-line overview prose, omit empty
sections, no restated doctrine. Density is not a gap; a gap is missing substance.

**Boundary — authored inside the product-manager's frame.** The PM owns *which* capabilities (features, the story filter, selection advice — map machinery: `mochiko:authoring-feature-map`); this craft owns *how well* the requirements are written. Neither edits the other's verdicts; a disagreement escalates to the user.

## When NOT to Use

- **Implementation planning** - Use design docs or architecture decisions instead
- **Technical architecture decisions** - Use ADRs or technical design documents
- **When requirements already exist and are validated** - Don't duplicate work
- **API endpoint specifications** - These belong to the design/plan track, not business requirements; keep concrete endpoint contracts out of FRs
- **Data model design** - This belongs to the design/plan track; describe entities only conceptually here (see Key Entities), not as schemas
- **User story authoring** - Use `mochiko:authoring-user-stories` instead (this skill focuses on the underlying requirements)

## Functional Requirements Format

Write requirements using the FR-XXX format with RFC 2119 keywords:

```markdown
## Functional Requirements

- **FR-001**: System MUST [specific capability]
- **FR-002**: Users MUST be able to [specific action]
- **FR-003**: System SHOULD [recommended behavior]
- **FR-004**: System MAY [optional capability]
```

### RFC 2119 Keywords

MUST (absolute; no exceptions) · SHOULD (recommended; valid exceptions may exist) · MAY
(optional). Detailed usage: [RFC-2119-KEYWORDS.md](references/RFC-2119-KEYWORDS.md).

### FR Numbering Rules

Sequential, three-digit padded, no gaps (FR-001, FR-002…); group related requirements together.

## Edge Cases

Identify 3-5 boundary conditions that need explicit handling, drawn from the five categories:
system limits · invalid input · external failures · concurrent access · permission boundaries.
Patterns, examples, and documentation formats: [EDGE-CASES.md](references/EDGE-CASES.md).

## Success Criteria Format

Define 3-5 measurable outcomes using SC-XXX format:

```markdown
## Success Criteria

- **SC-001**: Users complete the task creation flow in under 2 minutes
- **SC-002**: 95% of users successfully create their first recurring task
- **SC-003**: Support tickets related to task scheduling decrease by 50%
```

### Success Criteria Rules

1. **Technology-agnostic**: No API metrics, database stats, or code coverage
2. **User/business focused**: Observable by stakeholders
3. **Measurable**: Quantifiable where possible
4. **Outcome-oriented**: What changes, not what's built

## Key Entities (Optional)

When the feature involves data, describe entities conceptually and compactly — one
purpose line plus concept-level attribute and relationship lists:

### Entity Description Rules

- Describe purpose, not schema — one line
- List attributes as concepts, not columns
- Focus on relationships, not foreign keys
- No data types, constraints, or indexes (the full model is `data-model.md`, downstream)

## Validation Script

Validate format (FR/SC numbering, RFC 2119 keywords, technology-agnostic language):

```bash
python scripts/validate-requirements.py .mochiko/specs/<feature>/spec.md
```

## Quality Checklist

Before finalizing, verify:

- [ ] All FRs use RFC 2119 keywords (MUST/SHOULD/MAY)
- [ ] FR numbers are sequential with no gaps
- [ ] No technology or implementation details mentioned
- [ ] 3-5 edge cases identified
- [ ] All SCs are measurable outcomes
- [ ] SCs focus on user/business value
- [ ] Entities described conceptually (if applicable)
