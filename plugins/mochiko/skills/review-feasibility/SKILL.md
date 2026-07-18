---
name: review-feasibility
description: This skill MUST be invoked to grade plan analysis and design artifacts for cross-artifact FEASIBILITY — adversarially hunting contradictions, impossibilities, and buildability conflicts that no single artifact reveals in isolation: constraint-decision conflicts, NFR-constraint impossibilities, requirement-constraint contradictions, decision-decision conflicts, NFR-design feasibility, and constraint-design buildability — emitting a 3-state `feasible / needs-revision / infeasible` verdict with per-issue evidence, impact, and suggested resolution. SHOULD also invoke whenever a producer's analysis or design artifacts (requirements, constraints-and-decisions, NFRs, data-model, contracts) need an independent buildability review, or when re-reviewing after a structural revision (new or changed constraints, expanded requirement scope, modified NFR targets). The feasibility reviewer's driver — the adversarial-critique half of the cross-artifact review pair: its sibling grades coverage / measurability / consistency / presence, this skill grades contradiction / impossibility / buildability. Never defaults to `feasible`; grades a different agent's artifacts, never the author's own; operates over plan artifacts, NOT the constitution.
---

# Reviewing Feasibility

## Overview

Feasibility review answers one judgment question the analysis and design artifacts cannot answer about themselves: **can these pieces actually be built together as specified?** It hunts the *impossible combination* — a contradiction, impossibility, or buildability conflict that lives in the **intersection** of two artifacts and that neither artifact reveals in isolation.

This is **adversarial critique, not a checklist.** A checklist asks "is each thing present, measurable, consistent?" — you ask "do these two things, each fine on its own, make each other impossible?" There is no box to tick; there is a combination to try to break. The skill's done-work is a reasoned 3-state verdict, not a count of passed items.

**Looking buildable is not being buildable.** A clean review is one where you *actively hunted* every contradiction class and found nothing — never one where nothing jumped out, or the producer is careful, or there was no time. Absence of a contradiction *you went looking for* is evidence; absence of looking is not.

This is the **feasibility** half of a two-form cross-artifact review. The other half — coverage, measurability, cross-artifact consistency, presence/traceability — is a separate completeness reviewer running a mirror-checklist skill (`mochiko:review-plan-artifacts`). The two forms are deliberately disjoint; see *The boundary* below.

## When to Use

- When the producer's analysis artifacts (requirements, constraints-and-decisions, NFRs) need an independent buildability review before the work proceeds — typically ahead of the completeness pass, though that sequencing is the lead's, not this skill's.
- When design artifacts (data-model, contracts) exist and you must confirm the design *as specified* can meet the NFR targets and is deployable under the stated constraints.
- When re-reviewing after a **structural** revision — new or changed constraints, expanded requirement scope, or modified NFR targets — that could introduce a new cross-artifact conflict.

## When NOT to Use

- **Completeness, coverage, measurability, presence, or traceability review** — that is the mirror-checklist sibling (`mochiko:review-plan-artifacts`), a different form on a different reviewer. See *The boundary*.
- **Grading a constitution** — that is `mochiko:validation-constitution`, a different artifact domain. This skill operates over plan analysis/design artifacts only. (Guardrail G1.)
- **Authoring or revising the artifacts** — you review someone else's work; you never write or fix the artifacts you grade. (Independence.)
- **Single-artifact internal review** — an NFR that is vague, a requirement that is incomplete *on its own* is not feasibility. Feasibility is strictly **cross-artifact**: it lives between two artifacts.

## What you hunt — the six classes

Cross-artifact contradictions / impossibilities / buildability only. Each class is a *lens* onto a seam between artifacts where an impossibility hides — not a checkbox. Hunting heuristics and worked examples for each are in [references/FEASIBILITY-LENS.md](references/FEASIBILITY-LENS.md).

| # | Class | The question | Artifacts in tension |
|---|-------|--------------|----------------------|
| 1 | **Constraint ↔ decision conflict** | Does a technology choice violate a stated hard constraint? | constraints (C-XXX) ↔ decisions (D-XXX) |
| 2 | **NFR ↔ constraint impossibility** | Can an NFR target be met given the constraints or chosen technologies? | NFRs ↔ constraints / decisions |
| 3 | **Requirement ↔ constraint contradiction** | Does a requirement assume a capability not available under the constraints? | technical requirements ↔ constraints |
| 4 | **Decision ↔ decision conflict** | Are two technology choices mutually incompatible? | decisions ↔ decisions |
| 5 | **NFR ↔ design feasibility** | Can the design *as specified* meet the NFR targets? | NFRs ↔ data-model / contracts |
| 6 | **Constraint ↔ design buildability** | Are the design artifacts buildable/deployable given the constraints and captured infrastructure? | constraints / infrastructure ↔ data-model / contracts |

## The boundary (mirror it exactly)

This skill and the completeness sibling split the cross-artifact surface on a clean line. Hold it:

| | Feasibility (this skill) | Completeness sibling (`review-plan-artifacts`) |
|---|--------------------------|-----------------------------------------------------|
| **Form** | adversarial critique (judgment) | mirror checklist (objective criteria) |
| **Owns** | **contradiction / impossibility / buildability** | **coverage / measurability / consistency / presence** |
| **Asks** | "can these be built *together*?" | "is each thing present, mapped, measurable, consistent?" |
| **Verdict** | `feasible / needs-revision / infeasible` | `ready / needs-revision / critical-gaps` |

Where the two brush — e.g. an NFR that is *both* unmeasurable *and* impossible to meet — you take the **impossibility**; the sibling takes the **measurability**. You do **not** review whether every FR is mapped to a requirement, whether alternatives were considered, whether an NFR is individually measurable, or whether the formatting is right. Those are the sibling's. Reaching into them is boundary creep — a Common Mistake below.

## Core Process

### Step 1: Gather the cross-artifact context

Read the actual artifacts under review — never a summary, never the producer's say-so. For an analysis review: requirements, constraints-and-decisions, NFRs. For a design review: add data-model and contracts. A verdict given from anything but the artifacts themselves is not a verdict.

### Step 2: Hunt each contradiction class

Load [references/FEASIBILITY-LENS.md](references/FEASIBILITY-LENS.md) and look through each of the six lenses in turn, across the artifact pairs it names. Do not stop at the first clean lens — try to break every combination. The goal is to *prove the system can't be built*; only when you genuinely cannot do you call it feasible.

### Step 3: Capture each finding as gate fuel

For every contradiction found, record the four fields the human gate will consume:

| Field | What it states |
|-------|----------------|
| **description** | the conflict, in one sentence |
| **evidence** | the specific artifact IDs in tension (e.g. `C-003` vs `D-002`) and the incompatibility |
| **impact** | what breaks downstream if it ships unresolved |
| **suggested_resolution** | a concrete, actionable fix (relax the NFR / change the decision / add infrastructure / escalate) |

Vague evidence ("these seem to conflict") is not a finding. Cite the IDs.

### Step 4: Classify each finding — resolvable vs fundamental

A finding is either a **resolvable** contradiction (a revision closes it) or a **fundamental** conflict that requires a business-level decision (relax a hard requirement, drop a constraint, change a foundational choice). This classification drives the verdict — and is exactly where the distinct `infeasible` state is earned or lost.

### Step 5: Determine the 3-state verdict

| Verdict | When | What it means downstream |
|---------|------|--------------------------|
| **`feasible`** | every lens hunted, zero cross-artifact contradictions | the artifacts can be built together as specified; the work proceeds |
| **`needs-revision`** | ≥1 contradiction, all **resolvable** | a routine revision round closes them; specify each conflict + its suggested resolution |
| **`infeasible`** | ≥1 **fundamental** conflict requiring a business-level decision | this is **not** a routine revision — it **escalates** to the human gate with a clear explanation |

**Preserve `infeasible` as a distinct state.** It is a business-level escalation, not a louder `needs-revision`. Collapsing a fundamental conflict into `needs-revision` to keep the loop moving silently drops the most important signal this review produces — the one that says "no amount of revision fixes this; a human must decide." Never flatten it.

**Never default to `feasible`.** The not-cleared state is the absence of a clean, completed hunt. You award `feasible` only after working every class and finding nothing — never because the artifacts look buildable or the author is careful.

### Step 6: Emit the review

Write the verdict, the per-issue gate fuel, and noted strengths into the feasibility report at the location your instructions specify (the feature's `feasibility-report` in the workspace). The report's markdown shape is owned by the feasibility-report template; this skill owns the *judgments the report must contain* — the 3-state verdict and the four fields per issue. Do not invent routing or "next steps" — what happens on each verdict is the lead's loop, not this skill's.

## Independence (stated by role)

- You grade artifacts authored by a **different agent** — the producer of the analysis/design. You never review your own authoring. Independence here is the separate-agent structure, not a sentence in this skill.
- Your verdict is **input**, not the gate. The lead reads the artifacts and your report and owns the clearing verdict; the lead drives any revision round and presents `infeasible`/`needs-revision` issues to the human. Loop ownership, the round bound, and the human gate are the lead's — see `loop-discipline`; this skill does not restate or own them.
- The per-issue gate fuel (description / evidence / impact / suggested_resolution) is what the human gate consumes when a finding is a genuine judgment call. Route findings by their kind per `loop-discipline`'s gap routing — a fundamental conflict is a preference/scope gap for the human, not a knowledge gap.
- **G1:** you operate over plan analysis/design artifacts, never the constitution. You are not the constitution validator and you neither reference nor recreate constitution grading.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Reviewing coverage / measurability / presence | That is the completeness sibling. You own contradiction / impossibility / buildability only. |
| Flattening `infeasible` into `needs-revision` | The business-level escalation is the point. Keep all three states distinct. |
| Reviewing one artifact in isolation | Feasibility lives *between* artifacts. A single-artifact gap is not a feasibility finding. |
| Grading from a summary or the producer's claim | Read the artifacts themselves. A PASS off a summary is not a PASS. |
| Defaulting to `feasible` | Award it only after actively hunting every class. Absence of looking ≠ feasible. |
| Vague evidence | Cite the artifact IDs in tension (`C-XXX` vs `D-XXX`). "Seems to conflict" is not a finding. |
| Reviewing the constitution | G1: plan artifacts only. The constitution has its own validator. |
| Restating the loop / round cap / human-gate mechanics here | Those are the lead's, governed by `loop-discipline`. Reference, never restate. |

## Red Flags — STOP and re-hunt

If you catch yourself thinking any of these, you are rationalizing away the hunt or the escalation. Stop and restart from Step 2:

- "It looks buildable — the analyst is careful."
- "Nothing jumped out, so it's feasible."
- "This conflict is probably fine in practice."
- "I'll call it `needs-revision` so we don't have to escalate." *(the `infeasible`-flattening tell)*
- "While I'm here I'll check the coverage too." *(boundary creep into the sibling's job)*
- "I can tell from the summary."

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "No obvious contradiction, so it's feasible" | `feasible` is earned by a completed hunt, not by nothing surfacing on a skim. |
| "Calling it infeasible is too strong" | If a revision can't close it, it *is* infeasible. Softening it strips the human's decision signal. |
| "This is basically a completeness gap" | Then it is the sibling's, not yours — route by the boundary, don't absorb it. |
| "I'll trust the analyst's report summary" | Independence means grading the artifacts, not the author's account of them. |
| "The conflict is minor, I'll skip it" | A minor-looking constraint-decision conflict can be the one that can't be built. Record it with evidence. |

## Related

- `loop-discipline` — the loop, the bound, and the human gate are the lead's; your verdict is input and your per-issue fuel feeds the human gate. Referenced, not restated.
- `mochiko:review-plan-artifacts` — the completeness sibling (mirror-checklist form). It owns coverage / measurability / consistency / presence; this skill owns contradiction / impossibility / buildability. Deliberately disjoint triggers.
- the **feasibility-report** — the destination artifact this review fills; its markdown shape is owned by that template, while this skill owns the verdict and per-issue judgments it must contain.
