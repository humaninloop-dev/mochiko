# Feasibility Lens — Cross-Artifact Contradiction Hunting

These are **hunting lenses, not a checklist.** Each names a seam between two artifacts where an impossibility can hide, the question to ask at that seam, a worked example of the conflict, and the evidence a real finding needs. You are not ticking boxes — you are trying to prove the combination cannot be built. A lens is "clean" only when you actually *looked through it* and could not break the combination; a skipped lens is not a clean lens.

Throughout: feasibility findings are **cross-artifact** — the seam is between two *elements*, never merely between two files (the architecture store holds NFR targets and the topology spine on one surface; the seam between them counts). A flaw inside a single artifact (a vague NFR, an incomplete requirement) is the completeness sibling's territory, not a feasibility finding — with one exception: **class 7** (excess / wrong altitude, below) is the one remove-shaped class and may fire from a single artifact. You report the conflict; the lead routes it and owns the loop.

---

## 1. Constraint ↔ Decision conflict

**Seam:** hard constraints (C-XXX) ↔ technology decisions (D-XXX).
**Question:** does any technology choice violate a stated hard constraint?

**Worked example.** `C-002: the system MUST run fully on-premises with no external network egress.` `D-004: use a hosted SaaS vector database.` The decision cannot satisfy the constraint — egress is required to reach the SaaS. Conflict.

**Evidence a finding needs:** the `C-XXX` and the `D-XXX`, plus the specific incompatibility (what the decision requires that the constraint forbids).
**Resolvable vs fundamental:** swapping to a self-hosted equivalent is a *resolvable* revision; if the constraint and the only viable technology are mutually exclusive, it is *fundamental* (→ `infeasible`).

---

## 2. NFR ↔ Constraint impossibility

**Seam:** non-functional targets ↔ constraints / chosen technologies.
**Question:** can the NFR target be met *given* the constraints or the technologies already chosen?

**Worked example.** `NFR-003: p95 latency < 50 ms for users worldwide.` `C-005: single region, no CDN or edge presence.` Physics and network distance make the target unreachable for distant users under that constraint. Impossible.

**Evidence:** the `NFR-XXX`, the `C-XXX`/`D-XXX`, and *why* the target is unachievable under it.
**Boundary watch:** "the NFR has no measurement method" is the sibling's *measurability* finding. "The NFR target cannot be met under the constraints" is yours. If both are true, you take the impossibility, the sibling takes the measurability.

---

## 3. Requirement ↔ Constraint contradiction

**Seam:** technical requirements (TR-XXX) ↔ constraints.
**Question:** does any requirement assume a capability not available under the stated constraints?

**Worked example.** `TR-009: deliver real-time push notifications to mobile clients.` `C-007: no persistent connections permitted; polling only.` The requirement assumes a capability the constraint removes. Contradiction.

**Evidence:** the `TR-XXX`, the `C-XXX`, and the missing capability the requirement depends on.
**Resolvable vs fundamental:** if a constraint-compatible mechanism exists (e.g. scheduled polling that meets the intent), *resolvable*; if the requirement's core value cannot survive the constraint, *fundamental*.

---

## 4. Decision ↔ Decision conflict

**Seam:** technology decisions ↔ each other (D-XXX ↔ D-XXX).
**Question:** are any two technology choices mutually incompatible?

**Worked example.** `D-001: Postgres as the only datastore; no additional services.` `D-006: use Elasticsearch for full-text search.` The second decision introduces the service the first forbids. The choices contradict each other.

**Evidence:** both `D-XXX`, and the incompatibility (one decision's premise denies the other).
**Resolvable vs fundamental:** usually *resolvable* (drop or replace one decision — e.g. Postgres full-text search), unless both are load-bearing commitments to external parties, then *fundamental*.

---

## 5. NFR ↔ Design feasibility *(design-phase)*

**Seam:** non-functional targets ↔ the design (data-model, contracts).
**Question:** can the design **as specified** actually meet the NFR targets?

**Worked example.** `NFR-002: list endpoint returns in < 100 ms over 1M rows.` The data-model defines the entity with no index or denormalization path, and the contract's list endpoint specifies an unfiltered full-collection read. The design cannot hit the target. Infeasible-as-designed.

**Evidence:** the `NFR-XXX`, the specific design element (entity/attribute/endpoint), and why that design cannot achieve the target.
**Boundary watch:** "the schema doesn't match the data model" is the sibling's *consistency* finding. "The design, even if internally consistent, cannot meet the NFR" is yours.

---

## 6. Constraint ↔ Design buildability *(design-phase)*

**Seam:** constraints / captured infrastructure (IP-XXX) ↔ the design (data-model, contracts).
**Question:** are the design artifacts buildable and deployable *given* the constraints and the infrastructure actually captured?

**Worked example.** The contract's event flow requires a managed message queue, but `C-004` forbids new infrastructure and no `IP-XXX` provisions a queue. The design names capability the constraints/infrastructure do not allow to exist. Not buildable as drawn.

**Evidence:** the design element that needs the capability, the `C-XXX`/`IP-XXX` that withholds or fails to provision it, and the buildability gap.
**Resolvable vs fundamental:** adding the missing `IP-XXX` (if the constraint permits) is *resolvable*; if the constraint categorically forbids the only infrastructure the design needs, *fundamental*.

---

## Class 7 — unjustified structure / wrong altitude (remove-shaped)

**Seam:** requirements / asserted floor ↔ any single design artifact — the one class that can fire from a single artifact.
**Question:** does an element exist that no ratified requirement or asserted floor obligation pays for, or is an artifact drawn below its declared altitude (C4 level-3 detail inside a container-level artifact)?

Its *necessity* test is the simplest-execution ladder's rung 1 — does any ratified requirement or floor obligation pay for this? — owned by `mochiko:patterns-plan-minimalism` (the ladder itself, never restated here); its *altitude* test is the artifact's own declared level (a container register carrying application-layer, domain, or port-trait rows is drawn below its bar).

**Blocking-capable, bounded:** a survivor blocks `feasible` and lands the verdict at `needs-revision`, but never alone at `infeasible` — excess is resolvable by cutting or re-leveling, not a business-level impossibility.
**Calibration:** a class-7 finding is admissible only when it names the cheaper alternative *or* the specific bar breached; a floor-, compliance-module-, or NFR-derived obligation is never excess, however heavy it looks.
**Interrogatory round:** before ruling you may put questions to the producer ("which requirement pays for this element?"), and their answers land on record beside the finding — the exchange informs you, the verdict stays yours alone.

**The adopt-first limb — needed but should have been adopted.** Hand-built work in a commodity category where a proven adoptable component exists — with no written rationale beating a named candidate — is remove-shaped excess even though a requirement pays for the capability; the cheaper alternative the finding must name is that component. Commodity trigger, disclosure floor, and the constraint-collision route-back are `mochiko:patterns-adopt-first`'s, never restated here.

---

## Architecture pass {#architecture-pass}

Fires when the plan package carries a **store delta** — the drafted topology + `AX-XXX` concern-row
changes authored by `mochiko:patterns-system-design` against the standing store at
`.mochiko/product/architecture/`, graded before the user's sign-off writes it. Two lens groups —
**topology feasibility** (A1–A2) and **governance conformance** (A3) —
both cross-artifact, both adversarial. Same discipline as the six classes: you are trying to prove the
topology cannot be built or cannot conform, not ticking a box.

### A1. NFR ↔ Topology feasibility

**Seam:** non-functional targets (`NFR-XXX`, on the store's `AX-XXX` concern rows) ↔ the proposed
topology (the `SPN-XXX` spine elements as the delta amends them). Both sides live in one store —
the seam is between two *elements*, and reading it is no less cross-artifact for that.
**Question:** can the *component shape and the way the pieces talk* meet the NFR targets — before any data-model or contract detail exists?

**Worked example.** `NFR-004: p95 end-to-end < 120 ms.` The container diagram routes a single user request synchronously through four services in series, each with its own network hop and datastore call. The serial hop budget alone exceeds the target. The topology cannot meet the NFR as drawn.

**Evidence:** the `NFR-XXX`, the specific topology element (the sync chain, the single region, the missing cache/queue), and why the shape cannot hit the target.
**Resolvable vs fundamental:** collapsing the chain, adding async/caching, or co-locating is *resolvable*; if the NFR and the only shape the constraints allow are mutually exclusive, *fundamental*.
**Boundary watch:** "the NFR has no measurement method" is the completeness sibling's; "the topology can't meet it" is yours. This is class 5 (NFR↔design) lifted one level up — at the container shape, upstream of data-model/contracts.

### A2. Constraint ↔ Topology buildability

**Seam:** constraints / captured infrastructure (`C-XXX` / `IP-XXX`, still in
`constraints-and-decisions.md`) ↔ the topology the delta proposes.
**Question:** is the topology buildable and deployable *given* the constraints and the infrastructure actually provisioned?

**Worked example.** The delta introduces a managed message queue as a new `in-flight` component, but `C-006` forbids new managed infrastructure and no `IP-XXX` row provisions a queue. The topology names a component the constraints do not allow to exist. Not buildable as drawn.

**Evidence:** the `SPN-XXX` element needing the capability, the `C-XXX`/`IP-XXX` that withholds it, and the buildability gap.
**Resolvable vs fundamental:** adding the missing `IP-XXX` (if the constraint permits) or re-shaping to an allowed mechanism is *resolvable*; a categorical forbiddance of the only infrastructure the shape needs is *fundamental*. This is class 6 (constraint↔design) lifted one level up.

### A3. Topology ↔ Governance conformance

**Seam:** the proposed topology ↔ the constitution's architectural surface (governance region + layer-rules + domain-dependency registry), read **as input**.
**Question:** does the topology conform to the governance the project already ratified — layer-import rules, the dependency allowlist, and the principles the architecture cites as binding it?

**Worked example.** The governance region carries a BE-HEX layer rule: `domain MUST NOT import infrastructure`. The container diagram draws the domain service calling the datastore adapter directly, crossing the forbidden boundary. Or: the architecture asserts `respects BE-HEX layering per GI-007`, but a drawn dependency violates exactly that principle. Non-conforming.

**Worked example — the floor-asserted limb.** `AX-001 Identity & auth` reads `decided`, but the delta's topology routes an internal admin surface around the boundary the FLOOR-SEC card asserts ("auth enforced at all boundaries"). The stance is legal vocabulary; the shape does not honor it. Non-conforming — and note the split: *whether the stance word is legal* on a floor-asserted category is the completeness sibling's mechanical check, *whether the shape honors it* is yours.

**Evidence:** the governance surface (the layer rule / allowlist entry / `GI-XXX` / the floor card), the topology element that breaks it, and the specific violation. "Cites the principle" is not "satisfies the principle" — verify, don't take the assertion.
**The two exits (never a silent pass):** a non-conforming topology surfaces with exactly two exits — **redesign to conform**, or a **user-ruled amendment/waiver** through `governance-ledger.md`. The feature-level review never overrules the constitution. A conflict with a conforming redesign available is *resolvable* (`needs-revision`); one where the governance and the required shape are mutually exclusive is *fundamental* (`infeasible`, escalates for the amendment/waiver decision).
**Boundary watch:** you grade the *topology's conformance* to governance (a plan artifact against an input), never whether the governance itself is well-formed — that is `validation-constitution`, a different domain.

---

## Severity within a feasibility finding

Severity here is internal to the feasibility lens (distinct from the sibling's coverage severities):

| Severity | Meaning | Drives |
|----------|---------|--------|
| **Critical** | the combination cannot be built, or a fundamental conflict needs a business decision | `infeasible` if fundamental; otherwise a Critical `needs-revision` |
| **Important** | a real conflict that a bounded revision can close | `needs-revision` |

A finding's severity and its resolvable/fundamental classification together set the verdict.

---

## The four-field gate fuel (every finding)

This is the **output contract** of the review — what each finding must carry so the lead can route it and the human can decide. (The report's markdown *shape* — headers, tables, frontmatter — is owned by `templates/feasibility-report-template.md`, which the review fills under the `templates/report-format.md` envelope; these are the template's own field names.)

- **gap** — the conflict in one sentence.
- **at** — the artifact IDs in tension and the specific incompatibility. Never "they seem to conflict"; always "C-XXX requires X, D-XXX requires not-X."
- **impact** — what breaks downstream if it ships unresolved.
- **fix** — one concrete, actionable move: *relax the NFR* / *change the decision* / *add the IP-XXX* / *escalate for a business decision*.

---

## Verdict criteria (recap)

- **`feasible`** — every lens hunted; zero cross-artifact contradictions; no surviving class-7 finding. Earned by a completed hunt, never by default.
- **`needs-revision`** — one or more contradictions or class-7 findings, all resolvable; each reported with its fix.
- **`infeasible`** — one or more fundamental conflicts requiring a business-level decision; escalates to the human gate. Never flattened into `needs-revision`; class 7 never alone earns it.

Classify each finding resolvable vs fundamental first — that classification, with severity, sets the verdict.

---

## What this lens is NOT

- Not a coverage checklist — "is every FR mapped?" is the completeness sibling's.
- Not measurability-in-isolation — "does this NFR have a measurement method?" is the sibling's.
- Not consistency / traceability / presence — "do the entity names match the requirement references?" is the sibling's. On the store delta, "does every delta element appear in the diagram?", "is this stance one of the four legal words?", "is every element keyed to this feature?", and "do data-model/contracts conform to the signed store delta?" are the sibling's too.
- Not constitution grading — you never judge whether the constitution *itself* is well-formed (G1, `validation-constitution`'s domain). The architecture pass's governance-conformance lens reads the governance surface only **as an input**, to grade the *topology's* conformance to it.

Cross-artifact contradiction, impossibility, and buildability — plus class 7 and, when the package carries a store delta, topology feasibility and governance conformance (the architecture pass). Nothing else.

---

## Reviewer guardrails — the tells that void a hunt

Catch yourself at any of these and restart the hunt at the class where you cut the corner:

| Tell | Reality |
|------|---------|
| "It looks buildable — the analyst is careful" / "nothing jumped out, so it's feasible" | `feasible` is earned by a completed hunt, never by nothing surfacing on a skim. |
| "This conflict is probably fine in practice" / "it's minor, I'll skip it" | A minor-looking conflict can be the one that can't be built. Record it with evidence. |
| "I'll call it `needs-revision` so we don't have to escalate" / "infeasible is too strong" | If a revision can't close it, it *is* `infeasible`. Softening it strips the human's decision signal. |
| "While I'm here I'll check the coverage too" / "this is basically a completeness gap" | Boundary creep — route by the boundary to the sibling, don't absorb its territory. |
| "I can tell from the summary" / "I'll trust the analyst's report" | Independence means grading the artifacts themselves, never the author's account of them. |
| Reviewing one artifact in isolation | Feasibility lives *between* elements; a single-artifact gap is the sibling's — class 7 the sole exception. |
| Vague evidence ("these seem to conflict") | Not a finding. Cite the IDs in tension (`C-XXX` ↔ `D-XXX`). |
| Reviewing the constitution | G1: plan artifacts only. The constitution has its own validator. |
| Restating the loop / round cap / human-gate mechanics | Those are the lead's — its command states them. Reference, never restate. |
