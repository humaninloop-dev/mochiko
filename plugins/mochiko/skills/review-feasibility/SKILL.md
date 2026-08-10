---
name: review-feasibility
description: This skill MUST be invoked to grade plan analysis/design artifacts for cross-artifact FEASIBILITY — hunting contradictions, impossibilities, buildability conflicts; plus the architecture pass when `architecture.md` is in scope. Emits a 3-state `feasible / needs-revision / infeasible` verdict. The adversarial half of the plan pair; its sibling `review-plan-artifacts` grades coverage/measurability/presence, this grades contradiction/buildability. Never defaults to `feasible`; not the constitution.
---

# Reviewing Feasibility

## Overview

Feasibility review answers one judgment question the analysis and design artifacts cannot answer about themselves: **can these pieces actually be built together as specified?** It hunts the *impossible combination* — a contradiction, impossibility, or buildability conflict that lives in the **intersection** of two artifacts and that neither artifact reveals in isolation.

This is **adversarial critique, not a checklist.** A checklist asks "is each thing present, measurable, consistent?" — you ask "do these two things, each fine on its own, make each other impossible?" There is no box to tick; there is a combination to try to break. The skill's done-work is a reasoned 3-state verdict, not a count of passed items.

**Looking buildable is not being buildable.** A clean review is one where you *actively hunted* every contradiction class and found nothing — never one where nothing jumped out, or the producer is careful, or there was no time. Absence of a contradiction *you went looking for* is evidence; absence of looking is not.

This is the **feasibility** half of a two-form cross-artifact review. The other half — coverage, measurability, cross-artifact consistency, presence/traceability — is a separate completeness reviewer running a mirror-checklist skill (`mochiko:review-plan-artifacts`). The two forms are deliberately disjoint; see *The boundary* below.

## When NOT to Use

- **Completeness, coverage, measurability, presence, or traceability review** — that is the mirror-checklist sibling (`mochiko:review-plan-artifacts`), a different form on a different reviewer. See *The boundary*.
- **Grading a constitution** — that is `mochiko:validation-constitution`, a different artifact domain. This skill operates over plan analysis/design artifacts only. (Guardrail G1.) The **architecture pass** below is not an exception: it reads the governance surface **as an input** and grades whether the proposed *topology* conforms to it — a conformance check on a plan artifact, never a grade of whether the constitution itself is well-formed.
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

**External premises behind a verdict.** A feasibility call resting on an outside-repo claim is
itself a load-bearing external claim: verify it per
[../review-brainstorm/references/EXTERNAL-CLAIMS.md](../review-brainstorm/references/EXTERNAL-CLAIMS.md)
— the single source of the trigger, floor classes, and inline-check mechanics; none of it
restated here. Carry the claim's disclosure line in the finding's evidence; an undisclosed
external claim is a finding of its own.

## The architecture pass *(when `architecture.md` is in scope)*

When the design-time architecture artifact (`architecture.md`, owned by
`mochiko:patterns-system-design`) is under review, the hunt gains an **architecture pass** on top of
the six classes — two lens groups, both cross-artifact, both adversarial. Hunting heuristics and
worked examples are in [references/FEASIBILITY-LENS.md](references/FEASIBILITY-LENS.md#architecture-pass).

**A. Topology feasibility** — classes 5–6 lifted to the container level, upstream of the detailed design:

| Lens | The question | Artifacts in tension |
|------|--------------|----------------------|
| **NFR ↔ topology** | Can the *proposed component shape and interaction styles* hit the NFR targets? (a sync call-chain across four services vs a p95 target; single-region topology vs a global-latency NFR) | NFRs ↔ architecture |
| **Constraint ↔ topology** | Is the topology buildable/deployable under the constraints and captured `IP-XXX`? (a shape needing a managed queue the constraints forbid and no `IP-XXX` provisions) | constraints / IP ↔ architecture |

**B. Governance conformance** — does the proposed topology conform to the constitution's
*architectural surface*? Read the governance region + relevant rules files (layer-rules, the
domain-dependency registry when attached) **as input**, and grade:

- **Layer rules honored** — the topology's dependencies respect the layer-import rules; no boundary the layer governance forbids.
- **Dependency allowlist** — cross-component / cross-domain dependencies stay within the declared allowlist.
- **GI-linked principles satisfiable** — the principles the architecture cites as binding it (`respects BE-HEX layering per GI-XXX`) are actually satisfied by the topology, not merely asserted.

Conformance is **verified, not asserted** — a topology that *cites* a principle but *violates* it is a governance-conformance finding, not a pass.

**Routing — never silent approval at a feature gate.** A topology that must break a governance
surface is **never awarded `feasible` silently.** It surfaces with exactly two exits: **redesign to
conform**, or a **user-ruled amendment/waiver** through the existing `governance-ledger.md` machinery.
The feature-level review never overrules the constitution. Which exit is taken is the lead's/human's
routing — you report the conflict and that these are the only two exits; you do
not clear it yourself. (A governance-conformance conflict with no conforming redesign is a
**fundamental** finding — the `infeasible` escalation, not a louder `needs-revision`.)

## The boundary (mirror it exactly)

This skill and the completeness sibling split the cross-artifact surface on a clean line. Hold it:

| | Feasibility (this skill) | Completeness sibling (`review-plan-artifacts`) |
|---|--------------------------|-----------------------------------------------------|
| **Form** | adversarial critique (judgment) | mirror checklist (objective criteria) |
| **Owns** | **contradiction / impossibility / buildability** | **coverage / measurability / consistency / presence** |
| **Asks** | "can these be built *together*?" | "is each thing present, mapped, measurable, consistent?" |
| **Verdict** | `feasible / needs-revision / infeasible` | `ready / needs-revision / critical-gaps` |

Where the two brush — e.g. an NFR that is *both* unmeasurable *and* impossible to meet — you take the **impossibility**; the sibling takes the **measurability**. You do **not** review whether every FR is mapped to a requirement, whether alternatives were considered, whether an NFR is individually measurable, or whether the formatting is right. Those are the sibling's. Reaching into them is boundary creep — a Common Mistake below.

**On the architecture artifact,** the same line is drawn one level up: you own **topology feasibility + governance conformance** (the architecture pass); the sibling owns **component-table↔diagram coverage, qualifying-flow sequence coverage, and whether `data-model.md` / contracts conform to the approved shape**. "Can this topology be built and does it honor governance?" is yours; "are the architecture's own pieces present and covered?" is the sibling's.

## Core Process

### Step 2: Hunt each contradiction class

Load [references/FEASIBILITY-LENS.md](references/FEASIBILITY-LENS.md) and look through each of the six lenses in turn, across the artifact pairs it names. Do not stop at the first clean lens — try to break every combination. The goal is to *prove the system can't be built*; only when you genuinely cannot do you call it feasible.

### Step 3: Capture each finding as gate fuel

For every contradiction found, record the four fields the human gate will consume — `gap` (the
conflict, one sentence), `at` (the artifact IDs in tension, e.g. `C-003` ↔ `D-002`), `impact`
(what breaks downstream if it ships unresolved), and `fix` (a concrete resolution — relax the
NFR / change the decision / add infrastructure / escalate) — as the feasibility-report template
defines them. Vague evidence ("these seem to conflict") is not a finding. Cite the IDs.

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

## Independence (stated by role)

- You grade artifacts authored by a **different agent** — the producer of the analysis/design. You never review your own authoring. Independence here is the separate-agent structure, not a sentence in this skill.
- The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation.
- Your verdict is **input**, not the gate. The lead reads the artifacts and your report and owns the clearing verdict; the lead drives any revision round and presents `infeasible`/`needs-revision` issues to the human. Loop ownership, the round bound, and the human gate are the lead's — its command states them; this skill does not restate or own them.
- The per-issue gate fuel (the `gap` / `at` / `impact` / `fix` fields) is what the human gate consumes when a finding is a genuine judgment call. Routing each finding is the lead's judgment — a fundamental conflict is the human's to rule, never something investigation can settle.
- **G1:** you operate over plan analysis/design artifacts, never the constitution. You are not the constitution validator and you neither reference nor recreate constitution grading — its well-formedness is not yours to judge. The architecture pass's governance-conformance lens is consistent with this: it reads the governance surface **as an input** to grade the *topology's* conformance to it, never to grade the constitution itself.

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
| Restating the loop / round cap / human-gate mechanics here | Those are the lead's — its command states them. Reference, never restate. |

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

- `mochiko:review-plan-artifacts` — the completeness sibling (mirror-checklist form). It owns coverage / measurability / consistency / presence; this skill owns contradiction / impossibility / buildability. Deliberately disjoint triggers.
- the **feasibility-report** — the destination artifact this review fills; its markdown shape is owned by that template, while this skill owns the verdict and per-issue judgments it must contain.
