---
name: review-specifications
description: This skill MUST be invoked when reviewing an already-drafted specification for gaps — finding missing requirements, ambiguities, unstated assumptions, and missing edge cases in an existing spec, and generating product-framed clarifying questions with concrete options and severity (Critical / Important / Minor). Reach for it on post-draft review work such as "review spec", "find gaps", "what's missing", "is the spec complete", or "clarify requirements" against a spec that already exists. SHOULD also invoke when checking spec.md for completeness or checking user stories for missing acceptance criteria before downstream design begins. This produces gap-finding INPUT (a severity-bucketed gap report plus clarifying questions), not a clearing PASS/FAIL verdict. For enriching a sparse or vague feature idea before any spec is drafted, use mochiko:analysis-iterative instead.
---

# Reviewing Specifications

## Overview

Find gaps in specifications and generate clarifying questions that a product owner or stakeholder can answer. Focus on WHAT is missing, not HOW to implement. This produces gap-finding input — the severity-bucketed gaps and clarifying questions feed a reviewer's judgment; the skill does not emit a clearing PASS/FAIL verdict of its own.

## When to Use

- Reviewing a spec.md before implementation begins
- Validating requirements completeness after a spec is drafted
- Generating questions for stakeholder clarification
- Checking user stories for missing acceptance criteria
- As a gap-review checkpoint before downstream planning and design begins
- When reviewing a drafted specification for gaps as an independent reviewer

## When NOT to Use

- **Technical architecture review** - Use design review tools instead
- **Code review** - Different skill domain entirely
- **Implementation planning** - Focus on design, not spec gaps
- **Performance specifications** - Technical concern, not product
- **When spec doesn't exist yet** - Use `mochiko:authoring-requirements` first
- **Enriching a sparse or vague feature idea before a spec is drafted** - that is pre-spec input enrichment; use `mochiko:analysis-iterative`

## Core Principle

**Ask product questions, not implementation questions.**

| Wrong (Technical) | Right (Product) |
|-------------------|-----------------|
| "What happens if the database connection fails?" | "What should users see if the system is temporarily unavailable?" |
| "Should we use optimistic or pessimistic locking?" | "Can two users edit the same item simultaneously?" |
| "What's the retry policy for failed API calls?" | "How long should users wait before seeing an error?" |
| "What HTTP status code for invalid input?" | "What message should users see for invalid input?" |

## Question Format

Every question must be framed as a decision the stakeholder can make:

```markdown
**Question**: [Clear product decision]

**Options**:
1. [Concrete choice] - [What this means for users]
2. [Concrete choice] - [What this means for users]
3. [Concrete choice] - [What this means for users]

**Why this matters**: [User or business impact]
```

## Gap Categories

Focus on these user-facing gaps:

| Category | Example Questions |
|----------|-------------------|
| **User expectations** | "What should users see when...?" |
| **Business rules** | "Is X allowed? Under what conditions?" |
| **Scope boundaries** | "Is Y in scope for this feature?" |
| **Success/failure states** | "What happens if the user...?" |
| **Permissions** | "Who can do X? Who cannot?" |

## What to Avoid

- Implementation details (databases, APIs, protocols)
- Technical edge cases (connection failures, race conditions)
- Architecture decisions (caching, queuing, scaling)
- Performance specifications (latency, throughput)

These are valid concerns but belong in later design and implementation work, not specification.

## Severity Classification

| Severity | Definition | Action |
|----------|------------|--------|
| **Critical** | Cannot build without this answer | Must ask now |
| **Important** | Will cause rework if not clarified | Should ask now |
| **Minor** | Polish issue, can defer | Log and continue |

## Output Format

The report structure — the findings list (type + severity, machine-first YAML),
clarifications with concrete options and impact, the recommended verdict, and the one-line
`strengths:` field — is single-sourced at `templates/advocate-report-template.md`; fill
that structure rather than inventing one. When invoked outside a workflow that names a
report path, return the same structure inline.

## Review Process

1. **Read the full specification** before identifying gaps
2. **Check each user story** for completeness
3. **Verify success criteria** are measurable
4. **Identify missing edge cases** for each flow
5. **Classify gaps** by severity
6. **Generate questions** with concrete options
7. **Group related gaps** to avoid overwhelming stakeholders

## Quality Checklist

Before finalizing the review, verify:

- [ ] All user stories reviewed for completeness
- [ ] Success criteria checked for measurability
- [ ] Edge cases identified for each main flow
- [ ] Gaps classified by severity (Critical/Important/Minor)
- [ ] All questions are product-focused (not technical)
- [ ] Each question has 2-3 concrete options
- [ ] "Why this matters" explains user/business impact
- [ ] Related gaps grouped together
- [ ] No implementation details in questions

## Common Mistakes

### Technical Questions Instead of Product Questions
❌ "What retry policy should we use?"
✅ "How long should users wait before seeing an error?"

### Vague Questions
❌ "What about errors?"
✅ "What message should users see when payment fails?"

### Open-Ended Questions Without Options
❌ "How should we handle this case?"
✅ "Options: (1) Show warning and continue, (2) Block action, (3) Ask for confirmation"

### Too Many Gaps at Once
❌ Presenting 20+ gaps to stakeholders
✅ Limit to 5-7 critical/important gaps per review round

### Missing "Why This Matters"
❌ Just listing the gap without context
✅ Explain user or business impact for each question

### Implementation Bias
❌ "Should we cache this data?" (assumes caching)
✅ "How quickly should users see updated data?"

### Scope Creep Disguised as Gaps
❌ Adding new features as "missing requirements"
✅ Only clarify scope of existing features

### Ignoring Existing Context
❌ Asking questions already answered elsewhere
✅ Reference existing patterns and decisions before asking

## Related Skills

- **`mochiko:authoring-requirements`** — drafts the requirements this skill reviews; it runs *before* this skill (if no spec exists yet, author one first).
- **`mochiko:analysis-iterative`** — pre-spec input enrichment for a sparse or vague idea (producer-side, before a draft exists). This skill is the post-draft, reviewer-side counterpart; their triggers are deliberately disjoint (enrich an idea vs. review a draft for gaps).
