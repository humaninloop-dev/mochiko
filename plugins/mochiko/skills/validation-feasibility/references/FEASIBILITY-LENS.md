# Feasibility Lens — Cross-Artifact Contradiction Hunting

These are **hunting lenses, not a checklist.** Each names a seam between two artifacts where an impossibility can hide, the question to ask at that seam, a worked example of the conflict, and the evidence a real finding needs. You are not ticking boxes — you are trying to prove the combination cannot be built. A lens is "clean" only when you actually *looked through it* and could not break the combination; a skipped lens is not a clean lens.

Throughout: feasibility findings are **cross-artifact**. A flaw inside a single artifact (a vague NFR, an incomplete requirement) is the completeness sibling's territory, not a feasibility finding. You report the conflict *between* artifacts; the lead routes it and owns the loop.

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

## Severity within a feasibility finding

Severity here is internal to the feasibility lens (distinct from the sibling's coverage severities):

| Severity | Meaning | Drives |
|----------|---------|--------|
| **Critical** | the combination cannot be built, or a fundamental conflict needs a business decision | `infeasible` if fundamental; otherwise a Critical `needs-revision` |
| **Important** | a real conflict that a bounded revision can close | `needs-revision` |

A finding's severity and its resolvable/fundamental classification together set the verdict.

---

## The four-field gate fuel (every finding)

This is the **output contract** of the review — what each finding must carry so the lead can route it and the human can decide. (The report's markdown *shape* — headers, tables, frontmatter — is owned by the feasibility-report template; this is the content that shape must hold.)

- **description** — the conflict in one sentence.
- **evidence** — the artifact IDs in tension and the specific incompatibility. Never "they seem to conflict"; always "C-XXX requires X, D-XXX requires not-X."
- **impact** — what breaks downstream if it ships unresolved.
- **suggested_resolution** — one concrete, actionable move: *relax the NFR* / *change the decision* / *add the IP-XXX* / *escalate for a business decision*.

---

## Verdict criteria (recap)

- **`feasible`** — every lens hunted; zero cross-artifact contradictions. Earned by a completed hunt, never by default.
- **`needs-revision`** — one or more contradictions, all resolvable; each reported with its suggested resolution.
- **`infeasible`** — one or more fundamental conflicts requiring a business-level decision; escalates to the human gate. Never flattened into `needs-revision`.

---

## What this lens is NOT

- Not a coverage checklist — "is every FR mapped?" is the completeness sibling's.
- Not measurability-in-isolation — "does this NFR have a measurement method?" is the sibling's.
- Not consistency / traceability / presence — "do the entity names match the requirement references?" is the sibling's.
- Not constitution grading — that is a different artifact domain entirely (G1).

Cross-artifact contradiction, impossibility, and buildability — nothing else.
