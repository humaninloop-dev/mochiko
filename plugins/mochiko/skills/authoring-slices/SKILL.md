---
name: authoring-slices
description: This skill MUST be invoked when decomposing an accepted feature specification into graduation slices — ordered, named groups of user stories that graduate through the design→build→verify pipeline as independent units — authoring the `slices.md` overlay: story→slice assignment (every story exactly one home), dependency-closed ordering, foundation-slice designation, cross-cutting extend obligations, the Feature-Done section (SC-XXX coverage map + cross-slice seams), and the spec stamp. SHOULD also invoke when the work involves "slice the spec", "decompose the spec", "graduation slices", "story slices", "foundation slice", "slices.md", or grouping user stories into independently graduating increments. Includes the null exit: a spec that would not yield at least two slices with distinct value seams gets a recommend-whole-spec outcome, never a forced decomposition. Boundary: this slices STORIES into pipeline-unit groups at spec level (which stories advance together) — NOT stories into TDD cycles within one slice's implementation (cycle structuring is mochiko:patterns-vertical-tdd, downstream); it authors the overlay, never edits spec.md, and never grades its own output (grading is mochiko:review-slices).
---

# Authoring Graduation Slices

**Violating the letter of the rules is violating the spirit of the rules.**

## Overview

A whole-spec pipeline moves every user story through design, tasking, and implementation together — the highest-value stories cannot reach verified working code until the entire spec has crossed each stage, and every stage's artifacts spread attention across all stories at once. A **graduation-slice decomposition** fixes both: `slices.md` is a small, reviewed **overlay** on an accepted `spec.md` that groups its user stories into ordered, named slices, each of which graduates through the downstream stages as its own unit.

This skill is the slicing judgment plus the artifact authoring. The output conforms to [`slices-template.md`](../../templates/slices-template.md) — the canonical `slices.md` structure, including the **Graduation contract** section that tells downstream consumers how to honor the decomposition. The overlay **indexes** the spec; it never rewrites it. `spec.md` stays whole and stays the single source of stories, requirements, and success criteria. Density per the deliverable envelope ([`artifact-format.md`](../../templates/artifact-format.md)): rationale ≤ 3 lines per slice, seams and obligations one line each — the overlay is an index, not an essay.

## Vocabulary — two "slices", two levels

| Term | Level | What it groups | Owner |
|------|-------|----------------|-------|
| **Graduation slice** | Spec / pipeline | User stories that advance through design→build→verify together | **this skill** |
| **Vertical slice (cycle)** | Implementation, within one slice's scope | Test-first tasks delivering one observable increment | `mochiko:patterns-vertical-tdd` (downstream) |

Same ethos — smaller, independently verifiable units — at two different altitudes. Never conflate them: a graduation slice will later be *cut into* vertical-slice cycles by the downstream task structuring.

## When to Use

- Decomposing an accepted `spec.md` into graduation slices (`slices.md`)
- Designating the foundation slice and ordering the rest
- Placing cross-cutting stories and recording their extend obligations
- Authoring the Feature-Done section (SC coverage map + seams) at decomposition time
- Revising `slices.md` after a reviewer's gap list

## When NOT to Use

- **Structuring cycles or tasks** — story→cycle mapping and TDD ordering are `mochiko:patterns-vertical-tdd`
- **Grading a decomposition** — that is `mochiko:review-slices`, run by an independent reviewer, never the author
- **Before the spec is accepted** — the overlay indexes a settled story set; slicing a moving spec produces a stale stamp on arrival
- **Editing stories, requirements, or success criteria** — the overlay never touches `spec.md`; a story that cannot be placed is a finding to surface, not a story to rewrite
- **Tiny specs** — when the null exit fires (below), recommend whole-spec and write nothing

## The invariants (hard rules)

1. **Exactly one home.** Every user story is assigned to exactly one slice. An unassigned story means the decomposition is incomplete; a story in two slices means the boundary is wrong.
2. **Dependency closure.** A slice must be designable and buildable given **only** the slices ordered before it. No forward dependencies — if S2 needs something S4 delivers, the ordering is invalid.
3. **Foundation legitimacy.** The first slice — the **foundation slice** — must both (a) establish the shared design core (the entities, contracts, and constraints most other slices depend on) and (b) deliver a testable user journey of its own. A pure-plumbing slice is forbidden: infrastructure without observable value belongs in foundation *cycles* inside a slice, downstream — not in a valueless first slice that delays every user-visible outcome.
4. **Ordering: dependency beats priority.** Order by dependency closure first; among independent slices, higher-priority stories (P1 before P2 before P3) graduate earlier.
5. **Complete coverage.** The union of all slices is exactly the spec's story set — no orphans, no inventions.
6. **The overlay never edits the spec.** `slices.md` references story IDs (`US-#`) and success-criterion IDs (`SC-#`); it never restates, splits, or amends their content.

## Sizing

Soft target **2–4 stories per slice**. A single-story slice is fine when the story is genuinely independent. A slice above 4 stories must carry an explicit justification in its rationale — an oversized slice recreates the diluted, everything-at-once stage work the decomposition exists to fix.

## Cross-cutting stories

A story that touches several slices' territory (the classic: "audit logging on every action") still gets exactly one home:

- **Earliest meaningful home.** Assign it to the earliest slice where its independent test can pass meaningfully — where the story is demonstrably delivered for that slice's scope.
- **Explicit extend obligations.** Every later slice the story touches gets a recorded **extend obligation** in `slices.md`: when that slice graduates, its design must extend the story's behavior to its scope. Obligations are named and visible — silent spread is scope leak.
- **The un-homeable escalation.** If the story's independent test cannot pass meaningfully in *any* single slice, the story is mis-specified — usually a non-functional requirement wearing a story costume. Surface it as a spec-amendment finding; do not force an assignment.

## The null exit

Decompose only when the spec yields **at least two slices with distinct value seams** — separable user journeys whose independent graduation is worth per-slice pipeline overhead. When it does not: **write no `slices.md`**; disclose the recommend-whole-spec outcome and its reasoning in your report, and stop. A forced decomposition of a small spec is overhead without the focus benefit. The absence of `slices.md` *is* the whole-spec state — never write a stub.

## The Feature-Done section — declared now, verified at feature-close

Independent graduation destroys the free whole-feature ending a single pipeline had, so the feature-level done-condition must be **declared at decomposition time** — before any slice graduates — not invented after the last slice ships:

- **SC coverage map.** Every success criterion (`SC-#`) in the spec maps to the slice(s) whose verification covers it, with a one-line *how*. An SC that no slice can verify is a decomposition gap — fix the slicing now, while it is cheap.
- **Cross-slice seams.** Name each integration point where one slice's stories interact with another's, and what must hold across it. Per-slice verification covers each side; the seam itself is exactly what it misses.

The section is **executed** later, at feature-close, when every slice has shipped — it is authored here so the done-condition exists before the loop it governs.

## The spec stamp (staleness guard)

Record in the header: the spec's full story-ID set, its status, and the stamp date. Downstream consumers compare the live `spec.md` story set against the stamp before every slice-scoped run — a mismatch means the spec moved after decomposition, and the decomposition must be redone before any slice proceeds. The stamp is what makes the overlay safely *derivative*: it can go stale, but never silently.

## Process

1. **Read `spec.md` itself** — the user stories (IDs, priorities, independent tests), functional requirements, success criteria, and edge cases. The overlay indexes these by ID; you must hold the real set.
2. **Sketch the dependency structure** — which stories presuppose which (data another story creates, capability another story establishes).
3. **Check the null exit** — fewer than two distinct value seams → recommend whole-spec, report, stop.
4. **Designate the foundation slice** — the minimal story set satisfying invariant 3.
5. **Group the remainder** — by user-journey cohesion within the dependency structure; apply sizing.
6. **Place cross-cutting stories** — earliest meaningful home + extend obligations; flag the un-homeable.
7. **Order the slices** — dependency closure first, then priority; verify invariant 2 holds for every slice.
8. **Author the Feature-Done section** — the SC coverage map (close any uncovered SC by re-slicing, not by dropping it) and the seams.
9. **Fill `slices-template.md`** — including per-slice rationale (why these stories graduate together; for the foundation slice, why it satisfies invariant 3), the Graduation contract section, and the spec stamp.

## Quality checklist

Before handing off:

- [ ] Every spec story assigned to exactly one slice; no orphans, no inventions
- [ ] Every slice designable/buildable from earlier slices only (no forward dependencies)
- [ ] Foundation slice establishes the shared design core AND delivers a testable journey
- [ ] Ordering respects dependencies; priority breaks ties
- [ ] Slice sizes within the soft target, or explicitly justified
- [ ] Cross-cutting stories homed earliest-meaningful, obligations recorded on every touched slice
- [ ] Every SC-# mapped to a verifying slice; cross-slice seams named
- [ ] Spec stamp records the full story-ID set, status, and date
- [ ] Rationale recorded per slice; no status/stage fields (state is derived from the workspace)
- [ ] `spec.md` untouched

## Common Mistakes

### Horizontal grouping
Bad: "all the API stories" in one slice, "all the UI stories" in another — layers, not journeys. Good: each slice is a coherent set of user journeys; the layers live inside every slice.

### Priority-only ordering
Bad: all P1 stories first regardless of dependency. Good: dependency closure orders; priority breaks ties among independent slices.

### The everything-foundation
Bad: stuffing every P1 story into the foundation slice "to be safe" — that is the whole-spec batch wearing a slice costume. Good: the foundation is the *minimal* set that establishes the shared core and still delivers a journey.

### Silent cross-cutting spread
Bad: assigning "audit logging" to slice 1 and assuming later slices will remember. Good: an explicit extend obligation recorded on every slice it touches.

### Tracking fields
Bad: a `status: planned` column per slice. Good: no stored stage — a slice's pipeline position is derived from which artifacts exist in its workspace directory.

### Stub on null exit
Bad: writing a one-slice `slices.md` for a tiny spec. Good: no file; the recommendation and reasoning go in the report.

## Red Flags — STOP and re-slice

- "We'll figure out the shared design when we get there" — foundation designation exists precisely so you don't
- "This story kind of belongs in both slices" — exactly-one-home plus an extend obligation, or the boundary is wrong
- "The spec is small, but slicing can't hurt" — it costs a full per-slice pipeline pass; take the null exit
- "I'll add a stage column so progress is visible" — state is workspace-derived; a stored field goes stale and lies
- "That SC is really an end-state thing, no slice covers it" — then the decomposition is wrong; re-slice until every SC has a verifying home
- "I'll just tweak the story wording so it fits" — the overlay never edits the spec; surface the finding instead

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Dependency order and priority order are basically the same" | Until they are not — and a forward dependency blocks a slice mid-pipeline. Check closure explicitly. |
| "The foundation slice doesn't need user value, it's foundation" | A valueless first slice delays every observable outcome — the exact cost this decomposition exists to remove. Plumbing goes in foundation cycles, downstream. |
| "Extend obligations are obvious from the story text" | Obvious scope is silent scope. Unrecorded obligations become unplanned work in a later slice's design. |
| "We can map the SCs after the slices ship" | A done-condition invented at the end is post-hoc. Declared-before-the-loop is the point of authoring it here. |
| "The spec just changed a little; the stamp is close enough" | A stale overlay silently mis-scopes every downstream run. Mismatch means re-slice — that is the guard working. |

## Related

- [`slices-template.md`](../../templates/slices-template.md) — the canonical `slices.md` structure this skill fills, including the Graduation contract downstream consumers honor
- `mochiko:review-slices` — the reviewer-side mirror of this skill; grades what this skill is taught to author, run by an independent reviewer, never the author
- `mochiko:patterns-vertical-tdd` — downstream: cuts one slice's scope into vertical-slice TDD cycles (the other "slice", one level below)
- `mochiko:authoring-user-stories` — upstream: the story quality (IDs, priorities, independent tests) this decomposition indexes
- `loop-discipline` — the loop, bounds, and gates around this authoring are the lead's; referenced, never restated
