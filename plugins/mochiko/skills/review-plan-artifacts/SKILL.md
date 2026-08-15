---
name: review-plan-artifacts
description: This skill MUST be invoked to grade a plan package against the approved artifact proposal — conformance (every proposed artifact present, nothing materially past approved depth; material divergence auto-FAILs — BLOCKING) and honesty of disclosed rung claims against `mochiko:patterns-plan-minimalism` (advisory), plus completeness (coverage, measurability, cycle-card quality, consistency) within scope. Emits a 3-state verdict (ready / needs-revision / critical-gaps). Does NOT cover feasibility (`review-feasibility`); defaults to FAIL; run by an independent validator, never the author.
---

# Reviewing Plan Artifacts

## Overview

Grade a plan package before it proceeds. The run's floor is **the approved artifact proposal** — the
lead's rung-justified proposal the user approved at the plan-the-plan gate, not a fixed, mandated
artifact set. Three lenses:

- **Conformance to the approved proposal** — BLOCKING: every proposed artifact is present and within
  its approved depth; **material divergence** (an unproposed artifact, or an element class materially
  past its approved depth) auto-FAILs the package.
- **Adopt-first disclosure** — BLOCKING, a named sibling check at conformance strength (the approved
  proposal does not scope it): a commodity-category decision naming neither a real shelf candidate nor
  "no shelf candidate exists" is a finding. Trigger and floor: `mochiko:patterns-adopt-first`.
  Whether the rationale actually beats the named candidate is advisory — it rides the lane below.
- **Rung-claim honesty** — advisory: the ladder stops each producing seat disclosed are graded for
  honesty against `mochiko:patterns-plan-minimalism` (the standard, never restated here). This is a
  disclosure grade; the independent excess/altitude *hunt* over the package is
  `mochiko:review-feasibility`'s hunt class 7 — a different seat and grade.
- **Completeness within scope** — the mirror checklist below, applied to the proposed artifacts: a
  fixed set of named checks, each with a fixed question and a severity, over coverage, measurability,
  and cross-artifact consistency.

Not implementation details, and not whether the design can be built (that is `review-feasibility`;
see *Scope* below).

**Violating the letter of the rules is violating the spirit of the rules.** Running the checklist
"in spirit" while skipping checks, or downgrading a severity to avoid a hard finding, is the exact
failure this skill exists to prevent. (The review-specific red flags are at the foot of this
file.)

The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts
themselves — review evidence that lives only in conversation is a floor violation.

## Scope — what this skill grades, and what it hands off

| Lens | Question | Owner |
|------|----------|-------|
| **Completeness** | Is everything the approved proposal specified present, within approved depth, traceable, measurable, and internally consistent with the decisions that were made? | **this skill** |
| **Feasibility** | Can these artifacts be built together, or do they contradict / overreach? | `mochiko:review-feasibility` |

These are the two halves of plan review, run by two independent reviewers. This skill keeps
coverage / measurability / presence / consistency; it deliberately does **not** grade cross-artifact
contradictions (TR↔constraint, NFR↔constraint), NFR-design feasibility, or constraint-design
buildability — those hand off to `review-feasibility`. The full check-by-check seam is the
boundary table in [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md#scope-boundary--handoff-to-review-feasibility).

## When NOT to Use

- **Implementation code review** — use code-review tooling instead (one narrow carve-out exists: the code-minimalism lens on produced code is `mochiko:review-code-minimalism`, implement-side — not this skill)
- **Cross-artifact feasibility / buildability / contradiction review** — use `mochiko:review-feasibility`
- **Specification review** — use `mochiko:review-specifications`
- **Constitution review** — use `mochiko:validation-constitution`
- **During active drafting** — wait for artifact completion before review

## Review Focus by Artifact Type

The caller supplies which artifacts are in scope; this skill grades whichever sets are handed to it.
Detailed checklists are in [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md).

| Artifact type | Focus | Key checks |
|---------------|-------|------------|
| **Analysis artifacts** | requirements, constraints/decisions, NFRs | FR→TR coverage, orphan TRs, testable criteria, sourced constraints, ≥2 alternatives + rationale, NFR measurability, IP coverage |
| **Architecture** | architecture.md (design-time topology + delta) | component-table↔diagram coverage, qualifying-flow sequence coverage, delta-summary D-XXX links, component status annotations |
| **Design artifacts** | data-model, API contracts, quickstart | entity coverage, relationships, sensitivity annotations present, endpoint coverage, schemas, error handling, integration-boundary presence |
| **Cycle cards** | tasks.md | vertical integrity (each card an observable end-to-end behavior, never a horizontal layer), `**TEST:**` gate present and in the grammar (real infrastructure, never a test-suite re-run), story traceability (every P1/P2 story on ≥1 card; Simple/Split/Merge case + rationale recorded), sizing (1–3 sessions or justified), dependency minimality/explicitness (foundation sequenced, `[P]` only where truly independent), brownfield exposure stated (`none` counts), **no task lists or file paths** (pre-written decomposition is a finding — the builder decomposes at build time) |
| **Cross-artifact** | the sets together | alignment, consistency (design honors the decisions *and the approved architecture*; cards' acceptance criteria cite real spec/plan IDs), traceability |

> Brownfield codebase-discovery review is **out of scope** for plan-core completeness — it belongs to
> the discovery track (see ARTIFACT-CHECKLISTS.md). Sequencing of which set is reviewed when is the
> lead's call, not this skill's.

## Issue Classification

Three severities — Critical / Important / Minor. The severity table, classification rules,
issue-documentation formats, and the working report shape are single-sourced in
[ISSUE-TEMPLATES.md](references/ISSUE-TEMPLATES.md).

## Review Process

### Step 2: Run the deterministic pre-assert

Before the model review, run the Tier-1 checker for the cheap, greppable slice (unresolved markers,
required sections, traceability presence, PII annotations, entity consistency):

```bash
python scripts/check-artifacts.py .mochiko/specs/<feature>/<artifact>.md [<more-artifacts>.md ...]
```

A `failed` count here is ground truth — fold it straight into the issue list before judging anything
by hand. (See ARTIFACT-CHECKLISTS.md → Automated Validation.)

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
  architecture_conformance: pass   # data-model/contracts conform to the approved architecture
  sensitivity_annotations: pass
  integration_boundaries: pass
```

## Verdict Criteria

Derived mechanically from the issue counts (single-sourced in [ISSUE-TEMPLATES.md → Verdict
Criteria](references/ISSUE-TEMPLATES.md#verdict-criteria)), **with one override that takes
precedence**: material divergence from the approved proposal — an unproposed artifact, or an element
class materially past its approved depth — is a package auto-FAIL (critical-gaps), independent of the
count mapping. Rung-honesty findings are advisory and never drive the verdict.

## Quality Checklist

Before finalizing the review, verify:

- [ ] The deterministic pre-assert was run and its failures folded in
- [ ] Graded against the approved proposal, not a fixed artifact set
- [ ] Conformance checked — no unproposed artifact, nothing materially past approved depth (material divergence = auto-FAIL)
- [ ] Adopt-first disclosure present on every commodity-category decision (blocking)
- [ ] Disclosed rung claims graded for honesty against `mochiko:patterns-plan-minimalism` (advisory)
- [ ] All applicable artifact-type checks executed
- [ ] Issues properly classified by severity
- [ ] Evidence cited for each issue
- [ ] Suggested fixes are actionable
- [ ] Verdict matches the issue counts
- [ ] Cross-artifact concerns noted
- [ ] Feasibility/buildability findings handed to `review-feasibility`, not graded here
- [ ] The one-line `strengths:` field filled

## Common Mistakes

| Mistake | Bad | Good |
|---------|-----|------|
| Over-classified severity | Marking style issues "Critical" | Reserve Critical for issues that genuinely block progress |
| Missing evidence | "The data model is incomplete" | "The data model is missing the User entity referenced in FR-003" |
| Vague suggestions | "Fix the contracts" | "Add the error-response schema for the 404 case in GET /users/{id}" |
| Reviewing implementation details | Commenting on code patterns, variable names, or framework choices | Design completeness, traceability, and consistency |
| Skipping cross-artifact checks | Reviewing only the new artifact in isolation | Always verify consistency with the prior artifact set |
| Grading feasibility here | Blocking on "the design can't meet the latency NFR" or "these two constraints contradict" | Note it as a cross-artifact concern and hand it to `review-feasibility` — that is its gate, not this one |

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
- `mochiko:patterns-plan-minimalism` — the plan-time ladder; this skill grades the rung stops it disclosed (honesty advisory) and package conformance to the approved proposal (blocking)
- `mochiko:advocate-report-template` — the deliverable report shape the lead reads
