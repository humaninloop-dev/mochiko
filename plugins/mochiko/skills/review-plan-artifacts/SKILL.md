---
name: review-plan-artifacts
description: This skill MUST be invoked to grade a producer's plan artifacts against the completeness checklist — the analysis set (requirements, constraints-and-decisions, NFRs) and the design set (data-model, API contracts, quickstart) — checking FR→TR coverage, orphan technical requirements, testable/measurable criteria, NFR measurability, entity and endpoint coverage, data-sensitivity annotations present, schema-model consistency, integration-boundary presence, and cross-artifact consistency (does the design honor the decisions). Emits a severity-classified gap report (Critical/Important/Minor) and a 3-state verdict (ready / needs-revision / critical-gaps). SHOULD also invoke whenever a plan loop's completeness-review step needs an independent grade of the planning artifacts, or when re-reviewing after a FAIL-loop revision. The completeness (mirror-checklist) half of the plan producer↔validator pair; does NOT cover cross-artifact feasibility / buildability / contradiction (that is mochiko:review-feasibility); defaults to FAIL; run by an independent validator, never the author.
---

# Reviewing Plan Artifacts

## Overview

Find gaps in planning artifacts and emit issues that must be resolved before the plan proceeds. This
is a **mirror checklist**: a fixed set of named checks, each with a fixed question and a severity,
producing a verdict derived mechanically from the issue counts. Focus on design completeness,
coverage, measurability, and cross-artifact consistency — not implementation details, and not
whether the design can be built (that is a separate review; see *Scope* below).

**Violating the letter of the rules is violating the spirit of the rules.** Running the checklist
"in spirit" while skipping checks, or downgrading a severity to avoid a hard finding, is the exact
failure this skill exists to prevent. (The generic anti-rationalization doctrine lives in
`loop-discipline`; the review-specific red flags are at the foot of this file.)

## Scope — what this skill grades, and what it hands off

| Lens | Question | Owner |
|------|----------|-------|
| **Completeness** | Is everything present, traceable, measurable, and internally consistent with the decisions that were made? | **this skill** |
| **Feasibility** | Can these artifacts be built together, or do they contradict / overreach? | `mochiko:review-feasibility` |

These are the two halves of plan review, run by two independent reviewers. This skill keeps
coverage / measurability / presence / consistency; it deliberately does **not** grade cross-artifact
contradictions (TR↔constraint, NFR↔constraint), NFR-design feasibility, or constraint-design
buildability — those hand off to `review-feasibility`. The full check-by-check seam is the
boundary table in [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md#scope-boundary--handoff-to-review-feasibility).

## When to Use

- Grading an **analysis artifact set** (e.g. requirements.md + constraints-and-decisions.md + nfrs.md) for completeness
- Grading a **design artifact set** (e.g. data-model.md + contracts/api.yaml + quickstart.md) for completeness
- Verifying **cross-artifact consistency** before the plan advances
- Re-reviewing planning artifacts after a FAIL-loop revision

## When NOT to Use

- **Implementation code review** — use code-review tooling instead
- **Cross-artifact feasibility / buildability / contradiction review** — use `mochiko:review-feasibility`
- **Task artifact review** — use `mochiko:review-task-artifacts`
- **Specification review** — use `mochiko:review-specifications`
- **Constitution review** — use `mochiko:validation-constitution`
- **During active drafting** — wait for artifact completion before review

## Review Focus by Artifact Type

The caller supplies which artifacts are in scope; this skill grades whichever sets are handed to it.
Detailed checklists are in [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md).

| Artifact type | Focus | Key checks |
|---------------|-------|------------|
| **Analysis artifacts** | requirements, constraints/decisions, NFRs | FR→TR coverage, orphan TRs, testable criteria, sourced constraints, ≥2 alternatives + rationale, NFR measurability, IP coverage |
| **Design artifacts** | data-model, API contracts, quickstart | entity coverage, relationships, sensitivity annotations present, endpoint coverage, schemas, error handling, integration-boundary presence |
| **Cross-artifact** | the sets together | alignment, consistency (design honors the decisions), traceability |

> Brownfield codebase-discovery review is **out of scope** for plan-core completeness — it belongs to
> the discovery track (see ARTIFACT-CHECKLISTS.md). Sequencing of which set is reviewed when is the
> lead's call, not this skill's.

## Issue Classification

| Severity | Definition | Action |
|----------|------------|--------|
| **Critical** | Blocks progress; must resolve | Return to the responsible producer |
| **Important** | Significant gap; should resolve | Flag for this round |
| **Minor** | Polish item; can defer | Note for later |

See [ISSUE-TEMPLATES.md](references/ISSUE-TEMPLATES.md) for classification rules, issue-documentation
formats, and the working report shape.

## Review Process

### Step 1: Gather context

Read and understand:
- The artifact being reviewed
- The spec / upstream requirements it should satisfy
- Prior artifacts (for consistency checks)
- Constitution principles (for compliance)

### Step 2: Run the deterministic pre-assert

Before the model review, run the Tier-1 checker for the cheap, greppable slice (unresolved markers,
required sections, traceability presence, PII annotations, entity consistency):

```bash
python scripts/check-artifacts.py .mochiko/specs/<feature>/<artifact>.md [<more-artifacts>.md ...]
```

A `failed` count here is ground truth — fold it straight into the issue list before judging anything
by hand. (See ARTIFACT-CHECKLISTS.md → Automated Validation.)

### Step 3: Execute the checklist

For each check in the applicable artifact-type checklist:
1. Ask the question
2. Look for evidence in the artifact
3. If an issue is found, classify its severity
4. Document the issue with evidence

### Step 4: Cross-reference

- Check traceability (requirement → artifact)
- Check consistency (artifacts agree with each other and honor the decisions)
- Check completeness (nothing obviously missing)

### Step 5: Emit the report

In the `advocate-report-template.md` shape (machine-first — findings YAML):

- Classify the verdict from the issue counts (mechanical — see *Verdict Criteria*)
- One finding entry per issue: evidence anchor (`at:`) and an actionable one-line fix
- Fill the one-line `strengths:` field — what was genuinely done well

## Incremental Review Mode

When a review covers a fresh artifact set alongside a previously-reviewed one, review incrementally
to save time without losing rigor. **The caller supplies which artifacts are the new set and which
are the prior set** — this skill does not decide that (it has no view of the workflow's sequence).

### Full review — the {new} set

- Execute **all** applicable checks from [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md)
- Document issues with full evidence
- This is the primary focus — no shortcuts

### Consistency check — the {prior} set

- Use the [Cross-Artifact Consistency](references/ARTIFACT-CHECKLISTS.md#cross-artifact-consistency) checklist
- Do NOT re-read the prior artifacts in full
- Spot-check: entity names, requirement IDs, decision references, constraint alignment
- Flag only inconsistencies between artifacts
- **Time budget**: 1–2 minutes per prior artifact

### When to escalate to a full re-review

- If 2+ consistency issues are found in one prior artifact → re-read that specific artifact in full
- If a contradiction is detected → report it as an issue; the lead routes it (a design-vs-decided
  contradiction is a Critical consistency issue here; a contradiction *between requirements/constraints*
  is `review-feasibility`'s)
- If unsure → note the uncertainty and recommend a targeted review

### Report shape (incremental mode)

The same `advocate-report-template.md` shape, with the incremental fields set and this
skill's consistency-check results as an extra frontmatter block (permitted per the
envelope):

```yaml
incremental: true
scope:
  full_review: [data-model.md, contracts/api.yaml]
  consistency_only: [requirements.md, constraints-and-decisions.md, nfrs.md]
consistency_checks:   # pass/fail per check; a fail also lands as a finding
  entity_names: pass
  schemas: pass
  decisions_honored: pass
  sensitivity_annotations: pass
  integration_boundaries: pass
```

## Verdict Criteria

Derived mechanically from the issue counts — the mapping itself carries no judgment:

| Verdict | Criteria |
|---------|----------|
| **ready** | Zero Critical, zero Important issues |
| **needs-revision** | 1–3 Important issues, fixable in one round |
| **critical-gaps** | 1+ Critical, or 4+ Important issues |

## Quality Checklist

Before finalizing the review, verify:

- [ ] The deterministic pre-assert was run and its failures folded in
- [ ] All applicable artifact-type checks executed
- [ ] Issues properly classified by severity
- [ ] Evidence cited for each issue
- [ ] Suggested fixes are actionable
- [ ] Verdict matches the issue counts
- [ ] Cross-artifact concerns noted
- [ ] Feasibility/buildability findings handed to `review-feasibility`, not graded here
- [ ] The one-line `strengths:` field filled

## Common Mistakes

### Over-classification of severity
Bad: marking style issues "Critical".
Good: reserve Critical for issues that genuinely block progress.

### Missing evidence
Bad: "The data model is incomplete."
Good: "The data model is missing the User entity referenced in FR-003."

### Vague suggestions
Bad: "Fix the contracts."
Good: "Add the error-response schema for the 404 case in GET /users/{id}."

### Reviewing implementation details
Bad: commenting on code patterns, variable names, or framework choices.
Good: focus on design completeness, traceability, and consistency.

### Skipping cross-artifact checks
Bad: reviewing only the new artifact in isolation.
Good: always verify consistency with the prior artifact set.

### Grading feasibility here
Bad: blocking on "the design can't meet the latency NFR" or "these two constraints contradict".
Good: note it as a cross-artifact concern and hand it to `review-feasibility` — that is its gate, not this one.

## Red Flags — STOP and Restart Properly

If you notice yourself thinking any of these, STOP immediately:

- "This case is different because…" — It is not. Run the checklist.
- "I'm following the spirit, not the letter" — The letter IS the spirit.
- "The artifacts look good enough" — Good enough is not ready. Evidence or rejection.
- "I'll skip the consistency check, the previous review was thorough" — Previous reviews do not guarantee current consistency.
- "This severity is only Minor, not Important" — If you are rationalizing severity DOWN, it is probably the higher level.
- "I'll note it but not block on it" — If it meets Critical or Important criteria, it blocks. Period.

## Common Rationalizations

| Rationalization | Counter |
|-----------------|---------|
| "The spec was vague, so the artifact can be vague" | Vagueness in the spec is a gap to flag, not permission to propagate. |
| "This is a minor feature, full review is overkill" | Scale of feature does not change the review process. Every artifact gets every applicable check. |
| "Time pressure means we should skip cross-artifact checks" | Cross-artifact inconsistencies caught now save days of rework later. |
| "The producer is senior, they know what they're doing" | Producer seniority is irrelevant. Evidence-based review only. |
| "I already found enough issues" | Finding issues is not a quota. Run every check, document every finding. |
| "This check doesn't apply to this type of feature" | If the check is in the applicable checklist, it applies. Flag N/A with justification if genuinely inapplicable. |
| "The constraint is obvious, it doesn't need documentation" | Obvious constraints are the ones most often violated. Document them. |

## Related

- `mochiko:review-feasibility` — the feasibility / buildability / contradiction half of plan review; the boundary with this skill is the table in ARTIFACT-CHECKLISTS.md
- `mochiko:advocate-report-template` — the deliverable report shape the lead reads
- `loop-discipline` — the source of the anti-rationalization and independent-validation doctrine this skill operationalizes
