# Specification-Input Enrichment

A focused variant of the iterative-analysis engine for conditioning a sparse feature description into a specification-ready brief. Use it when a feature description is thin on the **Who + Problem + Value** triad and needs filling out before a specification is authored.

## When this variant fits

The work in front of you is: a raw feature description is missing some of the Who / Problem / Value triad, and it needs conditioning before it reaches whoever authors the specification (the requirements producer).

Whether a description is sparse enough to enrich, and which triad elements are missing, is decided by the caller before this skill runs. If the caller named the missing elements, fill exactly those; if it did not, infer the gaps from the original input through the opening questions. (How a caller signals "enrich this" and passes the known-missing elements is a caller-side dispatch concern — see the agent-dispatch briefing — not something this skill parses.)

## Question Agenda

Follow the standard iterative questioning pattern: ONE question at a time with options and recommendations.

### Part 1: Fill triad gaps (conditional)

Only ask about triad elements that are actually missing — those the caller named, or those the original input leaves unclear.

#### 1. WHO Question (if Who is missing)

```
**Question**: Who is the primary user of this feature?

**Options:**
- **A) End user / Customer**: External user of the product
- **B) Internal user / Admin**: Team member or administrator
- **C) Developer / API consumer**: Technical user integrating with the system

**My Recommendation**: [Based on context clues in original input]
```

#### 2. PROBLEM Question (if Problem is missing)

```
Based on [actor choice], let's clarify the pain point.

**Question**: What problem does this solve for {actor}?

**Options:**
- **A) Efficiency**: Task takes too long or requires too many steps
- **B) Capability**: Something they can't do today at all
- **C) Reliability**: Current solution is error-prone or fails

**My Recommendation**: [Based on context clues]
```

#### 3. VALUE Question (if Value is missing)

```
Now that we know {actor} faces {problem}, let's clarify the value.

**Question**: Why does solving this matter?

**Options:**
- **A) Revenue impact**: Enables new revenue or reduces churn
- **B) Efficiency gains**: Saves time or reduces costs
- **C) User satisfaction**: Improves experience or removes friction

**My Recommendation**: [Based on what the problem implies]
```

### Part 2: Key Decisions (always ask)

These questions are ALWAYS asked, regardless of what was detected in the input.

#### 4. SCOPE Question (always)

```
Let's define boundaries to keep this focused.

**Question**: What's explicitly OUT of scope for v1 of this feature?

**Options:**
- **A) Advanced features**: Keep to core happy path, defer power-user features
- **B) Edge cases**: Handle main flow only, document edge cases for later
- **C) Integrations**: Skip third-party integrations initially

**My Recommendation**: [Based on feature complexity and what would be MVP]
```

#### 5. SUCCESS Question (always)

```
Finally, let's define how we'll know this works.

**Question**: How will you know this feature is working correctly?

**Options:**
- **A) Metric improvement**: Measurable KPI change (conversion, time, errors)
- **B) User feedback**: Qualitative satisfaction signals
- **C) Capability validation**: Users can complete the new workflow end-to-end

**My Recommendation**: [Based on the problem being solved]
```

## Output

After the questions are answered, generate the enriched description using the [ENRICHMENT.md](ENRICHMENT.md) template.

## Completion

After generating the enriched description:

1. **Output the enriched description** using the ENRICHMENT.md template.
2. **Conclude** — do NOT ask any follow-up questions; the enrichment is complete.

The enriched description is the deliverable. Whoever requested the enrichment reads the Summary directly and carries it forward as the starting input for the specification. This skill does not decide what runs next — it finishes by producing the artifact.

## Key Differences from General Analysis

| Aspect | General Analysis | Specification-Input Enrichment |
|--------|---------------|--------------------------|
| Questions | Adaptive per turn: structured options, open probes, or confirmations based on user state | Two-part: triad gaps (conditional) + key decisions (always) |
| Depth | Adapts to complexity — converges when decisions are settled | 2-5 questions depending on gaps |
| Output | SYNTHESIS.md format | ENRICHMENT.md format |
| Purpose | Explore any topic | Condition a sparse feature description before specification |
