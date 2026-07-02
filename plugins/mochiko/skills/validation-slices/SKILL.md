---
name: validation-slices
description: This skill MUST be invoked to grade a producer's `slices.md` — the graduation-slice decomposition overlay of an accepted feature specification — against the slicing checklist: story coverage (every spec story exactly one home, no orphans or inventions), dependency-closed ordering (no forward dependencies), foundation-slice legitimacy (establishes the shared design core AND delivers a testable journey), slice sizing, cross-cutting extend-obligation visibility, Feature-Done completeness (every SC-XXX mapped to a verifying slice; cross-slice seams named), spec-stamp accuracy, overlay purity (no spec edits, no stored stage fields), and the depth second-guess in both directions (a spec that should have taken the whole-spec null exit, or a whole-spec recommendation that hides real value seams). Emits a severity-classified gap report (Critical/Important/Minor) and a RECOMMENDED 3-state verdict (ready / needs-revision / critical-gaps) — the clearing verdict is lead-owned. SHOULD also invoke whenever a decomposition loop's review step needs an independent grade of `slices.md`, or when re-reviewing after a FAIL-loop revision. Boundary: grades the story→slice decomposition overlay ONLY — not task artifacts (mochiko:validation-task-artifacts), not plan artifacts (mochiko:validation-plan-artifacts), and not the spec itself (mochiko:analysis-specifications). Defaults to FAIL; run by an independent reviewer, never the author.
---

# Reviewing a Graduation-Slice Decomposition

## Overview

Find gaps in a `slices.md` decomposition and emit issues that must be resolved before any slice
graduates into the downstream pipeline. This is a **mirror checklist with a judgment core**: the
coverage, ordering, stamp, and Feature-Done checks are mechanical; whether a grouping is a
journey-coherent slice and whether the foundation choice is right are judgment calls no grep
settles. The reviewer checks exactly what the producer (`mochiko:authoring-slices`) is taught to
author.

The output is a **gap-finding report plus a RECOMMENDED 3-state verdict** (ready / needs-revision /
critical-gaps) — an *input* to the loop, **not** a clearing PASS/FAIL. The lead owns the clearing
decision and routes the revision; this skill recommends, the lead routes.

**Vocabulary — two "slices", two levels.** A **graduation slice** (what this skill grades) is a
spec-level pipeline unit: a group of user stories that graduate through design→build→verify
together. It is distinct from a **vertical slice (cycle)** — the implementation-level TDD
increment *within one slice's scope*, structured by `mochiko:patterns-vertical-tdd` and graded by
`mochiko:validation-task-artifacts`. Never conflate them: a `slices.md` whose groups are shaped
like cycles (single test-first increments rather than story journeys) has the wrong altitude —
flag it under the journey-coherence and sizing checks.

**Violating the letter of the rules is violating the spirit of the rules.** Skipping the spec
cross-read because the overlay "looks complete", or waving through a foundation slice that ships no
observable value, is the exact failure this review exists to prevent.

## Scope — the decomposition overlay, and the boundaries

| Lens | Artifacts | Owner |
|------|-----------|-------|
| **Decomposition quality** | `slices.md` (against the spec it indexes) | **this skill** |
| **Spec product gaps** | `spec.md` in its own authoring loop | `mochiko:analysis-specifications` |
| **Plan completeness / feasibility** | requirements, NFRs, data-model, contracts | `mochiko:validation-plan-artifacts` / `mochiko:validation-feasibility` |
| **Task-artifact quality** | `task-mapping.md`, `tasks.md` | `mochiko:validation-task-artifacts` |

The nearest boundary is the spec: this review **reads `spec.md`** (it must, to verify coverage) but
grades only the *decomposition* against it — a product gap discovered in the spec itself is
surfaced as an out-of-scope observation for the lead, never fixed or graded here.

## When to Use

- Grading a drafted `slices.md` before the decomposition is accepted
- Grading a producer's recommend-whole-spec (null exit) outcome — the depth call is reviewable even when no file was written
- Re-reviewing after a FAIL-loop revision

## When NOT to Use

- **Grading task or plan artifacts** — use their reviewers (see *Scope*)
- **Reviewing the spec itself** — use `mochiko:analysis-specifications`
- **Authoring or fixing the decomposition** — you surface gaps and hand them back; re-slicing is the producer's job
- **During active drafting** — wait for the producer to finish the round

## The checks

Read **both** files — `slices.md` *and* the `spec.md` it indexes. Coverage cannot be verified from
the overlay alone. Run every check; do not stop at the first finding.

| # | Check | The question | Typical severity |
|---|-------|--------------|------------------|
| 1 | **Story coverage** | Does the union of slices equal the spec's story set exactly — no orphaned `US-#`, none invented? | Critical |
| 2 | **Exactly one home** | Is any story assigned to two slices, or to none? | Critical |
| 3 | **Dependency closure** | Can each slice be designed and built given only the slices ordered before it? Any forward dependency? | Critical |
| 4 | **Foundation legitimacy** | Does the first slice establish the shared design core AND deliver a testable user journey? (Pure plumbing = Critical; core-establishing but valueless = Critical; journey without the shared core = Important) | Critical / Important |
| 5 | **Ordering rationale** | Among independent slices, do higher-priority stories graduate earlier? Is each ordering choice justified? | Important |
| 6 | **Sizing** | Any slice above the soft 2–4 story target without explicit justification? A grouping so large it recreates whole-spec dilution? | Important |
| 7 | **Journey coherence** | Is each slice a coherent user journey — or a horizontal layer ("all API stories") wearing a slice name? | Critical |
| 8 | **Cross-cutting visibility** | Is every cross-cutting story homed at its earliest meaningful slice, with an explicit extend obligation on every later slice it touches? Any silent spread? | Important |
| 9 | **Feature-Done: SC coverage** | Does every `SC-#` in the spec map to at least one verifying slice with a stated *how*? (Mechanical: list the spec's SCs, check the map.) | Critical |
| 10 | **Feature-Done: seams** | Are cross-slice integration points named, with what must hold across each? Do the slices' story assignments imply seams the section omits? | Important |
| 11 | **Spec stamp** | Does the stamp's story-ID set match the live `spec.md` exactly? Is status and date recorded? | Critical |
| 12 | **Overlay purity** | Does the overlay restate, split, or amend any story or requirement? Any stored stage/status tracking fields? | Important |
| 13 | **Depth second-guess** | Both directions: should this spec have taken the whole-spec null exit (fewer than two distinct value seams — forced decomposition)? Or does a recommend-whole-spec outcome hide genuinely separable journeys? | Important |

Check 13 grades the **depth call itself** — including when the producer wrote no file. A wrong-depth
finding is a scope-type gap for the lead to route, not a reason to stretch the other checks.

## Issue classification

| Severity | Definition | Action |
|----------|------------|--------|
| **Critical** | The decomposition would mis-scope or block downstream runs; must resolve | Return to the producer |
| **Important** | Significant gap; should resolve this round | Flag for this round |
| **Minor** | Polish item; can defer | Note for later |

## Verdict criteria

Derived mechanically from the issue counts — the mapping itself carries no judgment. This is the
**recommended** verdict the lead reads; the lead owns the clearing decision.

| Verdict | Criteria |
|---------|----------|
| **ready** | Zero Critical, zero Important issues |
| **needs-revision** | 1–3 Important issues, fixable in one round |
| **critical-gaps** | 1+ Critical, or 4+ Important issues |

**Never default to `ready`.** It is earned by a completed run of all thirteen checks with nothing
surfacing — never by the overlay looking tidy.

## Review process

1. **Gather** — Read `slices.md` and `spec.md` (the real files, never a summary); extract the
   spec's story-ID set, priority marks, and SC-ID set.
2. **Execute** — run every check above; for each finding record the evidence (quote the overlay
   and/or the spec), the severity, and an actionable suggested fix.
3. **Cross-reference** — coverage (check 1) and SC coverage (check 9) are set comparisons against
   the spec; do them explicitly, not by impression.
4. **Emit** — the report in the shared `mochiko:advocate-report-template` shape: the
   severity-classified gap table, clarifying questions where a gap needs a product decision, the
   recommended verdict with rationale, and genuine strengths.

## Quality checklist

Before finalizing the review:

- [ ] `spec.md` itself was Read — coverage verified by set comparison, not impression
- [ ] All thirteen checks executed
- [ ] Every issue carries quoted evidence and an actionable fix
- [ ] Severities match the classification table, not convenience
- [ ] Verdict derived mechanically from the counts
- [ ] Strengths acknowledged

## Common Mistakes

### Grading the overlay without the spec
Bad: checking `slices.md` for internal tidiness. Good: coverage, SC mapping, and the stamp are all comparisons **against `spec.md`** — read it.

### Accepting a plumbing foundation
Bad: "the foundation is infrastructure, value comes later." Good: a first slice with no testable journey is a Critical structural issue — that is the whole-spec batch's worst property, reintroduced.

### Treating layer groupings as slices
Bad: "the API slice, the UI slice." Good: each slice must read as a user journey; horizontal grouping is Critical, however balanced the sizes.

### Missing the silent seam
Bad: checking only the seams the section lists. Good: derive the seams the story assignments *imply* and flag the ones the section omits.

### Stretching past a wrong depth
Bad: carefully grading a five-story spec's three-slice decomposition. Good: check 13 first-classes the question — recommend the null exit and mark the rest of the review moot.

## Red Flags — STOP and re-check

- "The producer clearly read the spec; I don't need to" *(coverage by impression — the classic miss)*
- "The foundation choice is probably fine" — probably is not verified; check both halves of legitimacy
- "The SC map looks complete" — list the spec's SCs and tick them; looks-complete is not a set comparison
- "Flagging depth feels like blocking" — wrong depth shipped anyway is the expensive version of the same news
- "Zero findings — good decomposition" — go back and hunt harder; some strength, gap, or implied seam is always worth surfacing

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "The stamp is one story off; close enough" | A stale overlay silently mis-scopes every downstream run. Exact match or Critical. |
| "Extend obligations are implied by the story text" | Implied scope is silent scope — it becomes unplanned work in a later slice's design. Recorded or flagged. |
| "The ordering works even if one dependency is forward" | A forward dependency blocks a slice mid-pipeline, after its design work is spent. Closure is a hard invariant. |
| "It's a small spec, a light review is fine" | Small specs are where the depth second-guess matters most — the null exit exists for them. |
| "The producer is the slicing expert" | Producer craft is irrelevant to the verdict. Evidence-based review only. |

## Related

- `mochiko:authoring-slices` — the producer-side skill this review mirrors; the reviewer checks exactly what it teaches the producer to author
- [`slices-template.md`](../../templates/slices-template.md) — the canonical structure the graded artifact must conform to
- `mochiko:advocate-report-template` — the assembled report shape the lead reads
- `mochiko:validation-task-artifacts` / `mochiko:validation-plan-artifacts` / `mochiko:analysis-specifications` — the neighboring reviewers this skill is disjoint from (see *Scope*)
- `loop-discipline` — the source of the independent-validation and anti-rationalization doctrine this skill operationalizes
