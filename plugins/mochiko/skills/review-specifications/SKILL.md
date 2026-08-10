---
name: review-specifications
description: This skill MUST be invoked when reviewing an already-drafted specification for gaps — finding missing requirements, ambiguities, unstated assumptions, and missing edge cases in an existing spec, grading its feature layer against the map (derivation honesty, filter rejections justified, dedup against the actual map at the run-open git baseline, granularity, entry well-formedness, delta legality, selection-card deferred-SC honesty, specs-index agreement), walking a UX-bearing spec's Screens & Flows manifest against its served prototype (every FLOW-XXX clickable, every SCR-XXX reachable, every P1 scenario pathed; manifest↔prototype drift blocking), and generating product-framed clarifying questions with concrete options and severity (Critical / Important / Minor). Reach for it on post-draft review work such as "review spec", "find gaps", "what's missing", "is the spec complete", or "clarify requirements" against a spec that already exists. SHOULD also invoke when checking spec.md for completeness or checking user stories for missing acceptance criteria before downstream design begins. This produces gap-finding INPUT (a severity-bucketed gap report plus clarifying questions), not a clearing PASS/FAIL verdict. For enriching a sparse or vague feature idea before any spec is drafted, use mochiko:analysis-iterative instead.
---

# Reviewing Specifications

## Overview

Find gaps in specifications and generate clarifying questions that a product owner or stakeholder can answer. Focus on WHAT is missing, not HOW to implement. This produces gap-finding input — the severity-bucketed gaps and clarifying questions feed a reviewer's judgment; the skill does not emit a clearing PASS/FAIL verdict of its own.

## When to Use

- Reviewing a drafted spec.md for gaps as an independent reviewer — the gap-review checkpoint before downstream planning and design begins
- Validating requirements completeness after a spec is drafted
- Generating questions for stakeholder clarification
- Checking user stories for missing acceptance criteria

## When NOT to Use

- **Technical architecture review** - Use design review tools instead
- **Code review** - Different skill domain entirely (the one narrow exception, the code-minimalism lens on produced code, is `mochiko:review-code-minimalism`, implement-side)
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

Implementation details (databases, APIs, protocols), technical edge cases, architecture decisions,
and performance targets are valid concerns — they belong in later design work, not spec review.

## Question Format

Every question must be framed as a decision the stakeholder can make: 2-3 concrete options with
what each means for users, plus why it matters. The exact shape is the Clarifications block of
`templates/advocate-report-template.md` — fill that structure, don't invent a variant.

## Gap Categories

Focus on these user-facing gaps:

| Category | Example Questions |
|----------|-------------------|
| **User expectations** | "What should users see when...?" |
| **Business rules** | "Is X allowed? Under what conditions?" |
| **Scope boundaries** | "Is Y in scope for this feature?" |
| **Success/failure states** | "What happens if the user...?" |
| **Permissions** | "Who can do X? Who cannot?" |

The five requirement-defect classes those questions hunt (the canonical hunt taxonomy — the
`devils-advocate` persona names these classes and leans on this section for the detail):

| Class | What it looks like |
|-------|--------------------|
| **Missing requirements** | features mentioned but not specified; implicit expectations; dependencies on undefined behavior |
| **Ambiguities** | vague terms without quantification; requirements open to interpretation; unclear boundaries and limits |
| **Edge cases** | empty states; cancelled mid-flow; missing permissions; unstated limits (max items, max size) |
| **Assumption gaps** | assumptions that should be requirements (and the reverse); hidden dependencies |
| **Contradictions** | requirements that conflict; inconsistent terminology; mutually exclusive acceptance criteria |

**External and regulatory claims.** A spec asserting a regulatory obligation, compliance limit,
or other product-legal fact carries a floor-class external claim: verify it per
[../review-brainstorm/references/EXTERNAL-CLAIMS.md](../review-brainstorm/references/EXTERNAL-CLAIMS.md)
— the single source of the trigger and mechanics, not restated here. An undisclosed
external claim is a gap like any other.

## The feature layer

The spec's feature derivation and staged map delta are graded with the spec — same reviewer,
same report: the reviewer who reads the stories is the only one who can see derivation
dishonesty. The map machinery (derivation method, entry shape, delta grammar, write rules) is
single-sourced in `mochiko:authoring-feature-map` and its templates — this table is the
reviewer's mirror, not a second home. **Baseline rule:** the map-delta baseline is the **git
state of the map at run open** — grade staged writes against the actual map files at that
baseline, never against a workspace copy of the map.

| # | Check | Question | Typical severity |
|---|-------|----------|------------------|
| 1 | Derivation honesty | Every proposed feature traceable to the stories that inform it — no feature no story supports? | Critical |
| 2 | Disposition completeness | Every drafted story homed to exactly one feature, or rejected by the filter with the why recorded in the story file? | Critical |
| 3 | Dedup against the map | Every proposed entry deduplicated by capability against the actual map files at the run-open baseline? | Critical |
| 4 | Granularity | Each entry within the granularity guide — one-breath capability, extent ≤ ~3 lines — or split? | Important |
| 5 | Entry well-formedness | Every proposed entry in the entry-template shape, all fields carried? | Important |
| 6 | Delta legality | Every delta in the full grammar — what grows, in-flight mark, named spec; no `delivered` status regressed? | Critical |
| 7 | SC re-homing | Every SC-XXX mapped to a verifying feature; deferred SCs and one-sided seams on the owning entry's obligations line? | Critical |
| 8 | In-flight handling | Touches to in-flight territory read into the owning spec and resolved by reference, sequenced delta, or escalation — never silent contradiction? | Critical |
| 9 | Selection-card honesty | The deferred-SC list visible on the selection card, agreeing with the SC split? | Important |
| 10 | Specs-index agreement | The staged specs-index row agrees with the staged map writes (slug, FEAT-IDs, outcomes)? | Important |

## The Screens & Flows section (UX-bearing specs)

Graded with the spec — same reviewer, same report. Either a SCR/FLOW manifest with its
`prototype/` app, or the waiver line "No UX surface — prototype waived at intent." Both shapes
get graded (the waiver via check 8, against the Intent section's UX-bearing ruling). The walk is
adversarial, not ceremonial: **serve the prototype and click it** (bun, or open
`prototype/index.html` directly) — a skeptic walking the app finds the gap a text read cannot.
Authority split guard: flows, screens, and data shown are binding; layout and styling are
advisory — a cosmetic finding against a low-fi prototype is a wrong-altitude finding.

| # | Check | Question | Typical severity |
|---|-------|----------|------------------|
| 1 | Screen reachability | Every SCR-XXX row renders a reachable page in the served app? | Critical |
| 2 | Flow walkability | Every FLOW-XXX clickable end-to-end, no dead ends? | Critical |
| 3 | Scenario coverage | Every P1 story acceptance scenario carries a FLOW click path? | Critical |
| 4 | Flow traceability | Every FLOW-XXX keyed to a real story scenario — no scope invention? | Critical |
| 5 | Drift (both directions) | No page without a SCR row; no manifest row without its page? | Critical |
| 6 | Data-shape honesty | Screens show honest data shape (realistic fields/cardinality), enough to expose layout and flow problems? | Important |
| 7 | FEAT tags | Post-derivation → every SCR/FLOW row FEAT-tagged; out-of-selection and filter-rejected screens greyed but reachable, rejected ones marked with the rejection pointer? | Important |
| 8 | Waiver second-guess | The waiver line, on a spec whose stories imply user-facing screens — or a prototype the intent ruled out? Grades the UX-bearing call against the Intent section. | Important |

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
5. **Grade the feature layer** against the 10-check table above, reading the actual map
   files at the run-open git baseline first (never the workspace copy)
6. **Grade the Screens & Flows section** against the 8-check table above — serve the
   prototype and walk it; read the Intent section's UX-bearing ruling first
7. **Classify gaps** by severity
8. **Generate questions** with concrete options
9. **Group related gaps** to avoid overwhelming stakeholders

The spec follows the deliverable envelope (`templates/artifact-format.md`): one-line
scenarios (2-3 per story), one-line FR/SC/edge-case entries, compact entities. **Density
is never itself a gap** — grade substance (a missing flow, an unmeasurable criterion, an
unstated assumption), never prose volume.

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
- [ ] Feature layer graded (all 10 checks; delta baseline = git state of the map at run open)
- [ ] Screens & Flows section graded (all 8 checks; the prototype actually served and walked; the waiver line via the waiver second-guess)

## Common Mistakes

| Mistake | ❌ | ✅ |
|---------|-----|-----|
| Technical instead of product questions | "What retry policy should we use?" | "How long should users wait before seeing an error?" |
| Vague questions | "What about errors?" | "What message should users see when payment fails?" |
| Open-ended without options | "How should we handle this case?" | "Options: (1) Show warning and continue, (2) Block action, (3) Ask for confirmation" |
| Too many gaps at once | Presenting 20+ gaps to stakeholders | Limit to 5-7 critical/important gaps per review round |
| Missing "why this matters" | Just listing the gap without context | Explain user or business impact for each question |
| Implementation bias | "Should we cache this data?" (assumes caching) | "How quickly should users see updated data?" |
| Scope creep disguised as gaps | Adding new features as "missing requirements" | Only clarify scope of existing features |
| Ignoring existing context | Asking questions already answered elsewhere | Reference existing patterns and decisions before asking |

## Related Skills

- **`mochiko:authoring-requirements`** — drafts the requirements this skill reviews; it runs *before* this skill (if no spec exists yet, author one first).
- **`mochiko:authoring-feature-map`** — single source of the map machinery (derivation method, entry shape, delta grammar, write rules) the feature-layer checks mirror; produces the derivation and staged map delta this skill grades (independent reviewer, never the author).
- **`mochiko:analysis-iterative`** — pre-spec input enrichment for a sparse or vague idea (producer-side, before a draft exists). This skill is the post-draft, reviewer-side counterpart; their triggers are deliberately disjoint (enrich an idea vs. review a draft for gaps).
