# Evaluation Matrix Reference

The criteria, the two comparison shapes, and brownfield alignment scoring for a technology decision.

## Evaluation criteria

| Criterion | Question |
|-----------|----------|
| **Fit** | Does it cover every requirement, with no gaps? |
| **Complexity** | What do implementing, learning and operating it cost? |
| **Team Familiarity** | Does the team already know it? |
| **Ecosystem** | Are the docs, libraries and community healthy? |
| **Scalability** | How do performance and cost hold up at scale? |
| **Security** | What is its posture, its record, and its known vulnerabilities? |
| **Cost** | What are licensing, infrastructure and maintenance? |
| **Brownfield Alignment** | Does it fit the existing stack without conflict? |

## Decision matrix format

For a decision that needs weighted scoring:

```markdown
## Decision: [What needs to be decided]

### Context

[Why this decision matters. What depends on it.]

### Criteria Weights

| Criterion | Weight | Reason |
|-----------|--------|--------|
| Fit | High | Must fully solve auth requirements |
| Team Familiarity | Medium | Small team, learning time matters |
| Brownfield Alignment | High | Must integrate with existing Express app |

### Options Evaluated

| Option | Fit | Complexity | Team Familiarity | Alignment | Score |
|--------|-----|------------|------------------|-----------|-------|
| JWT + refresh tokens | High | Low | High | High | **Best** |
| Session cookies | Medium | Low | High | High | Good |
| OAuth2 only | Low | Medium | Low | Medium | Poor |

### Decision: JWT with refresh tokens

### Rationale

[Why this was chosen - connect to criteria]

### Trade-offs Accepted

[What are we giving up by choosing this option]
```

## Comparison table format

For a quick side-by-side evaluation:

```markdown
### Options Comparison

| Aspect | Option A | Option B | Option C |
|--------|----------|----------|----------|
| **Description** | [Brief] | [Brief] | [Brief] |
| **Pros** | + Pro 1<br>+ Pro 2 | + Pro 1<br>+ Pro 2 | + Pro 1<br>+ Pro 2 |
| **Cons** | - Con 1<br>- Con 2 | - Con 1<br>- Con 2 | - Con 1<br>- Con 2 |
| **Best For** | [When to use] | [When to use] | [When to use] |
| **Risk** | Low/Medium/High | Low/Medium/High | Low/Medium/High |
```

## Brownfield considerations

Check the existing stack before adding to it, and prefer extension over addition. A new dependency
that competes with an existing one (a second ORM) or breaks an existing pattern (callbacks in an
async codebase) is a conflict.

### Alignment scoring

| Scenario | Alignment | Action |
|----------|-----------|--------|
| Existing dep solves problem | High | Prefer reuse |
| New dep, same ecosystem | Medium | Document justification |
| New dep, different ecosystem | Low | Strong justification needed |
| Conflicting with existing | None | Avoid or escalate |
