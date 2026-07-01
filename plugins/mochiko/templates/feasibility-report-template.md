# Feasibility Review Report Template

This template defines the structure for the Feasibility Review Report — a feasibility reviewer's cross-artifact critique of a feature's analysis and design artifacts, recording the contradictions found across artifacts, a three-state feasibility verdict, the per-issue resolution fuel the human gate reads, and the design's strengths.

---

```markdown
# Feasibility Review Report

> Feature: {{feature_id}}
> Round: {{round}}
> Generated: {{timestamp}}

---

## Artifacts Reviewed

{{artifacts_reviewed}}

---

## Cross-Artifact Contradictions

> Impossible combinations that no single artifact reveals in isolation.

| # | Description | Evidence | Impact | Severity | Suggested Resolution |
|---|-------------|----------|--------|----------|----------------------|
| 1 | {{description}} | {{artifact_a}} ↔ {{artifact_b}} | {{impact}} | {{severity}} | {{suggested_resolution}} |

_If none: "No cross-artifact contradictions found."_

---

## Constraint-Decision Conflicts

> Technology choices that violate stated hard constraints.

| # | Description | Evidence | Impact | Severity | Suggested Resolution |
|---|-------------|----------|--------|----------|----------------------|
| 1 | {{description}} | {{c_id}}: {{c_name}} ↔ {{d_id}}: {{d_name}} | {{impact}} | {{severity}} | {{suggested_resolution}} |

_If none: "No constraint-decision conflicts found."_

---

## NFR-Constraint Impossibilities

> Quality targets that cannot be met given stated constraints or chosen technologies.

| # | Description | Evidence | Impact | Severity | Suggested Resolution |
|---|-------------|----------|--------|----------|----------------------|
| 1 | {{description}} | {{nfr_id}}: {{target}} ↔ {{constraint_or_decision}} | {{impact}} | {{severity}} | {{suggested_resolution}} |

_If none: "No NFR-constraint impossibilities found."_

---

## Verdict

**Status**: {{verdict}}

**Verdict options**: feasible | needs-revision | infeasible

**Rationale**: {{verdict_rationale}}

---

## Strengths Noted

- {{strength_1}}
- {{strength_2}}
```

---

## Usage Notes

1. **Placeholders**: `{{mustache}}` style — the reviewer replaces each with actual content; this is a reference shape, not literal output.

2. **Verdict values** — the reviewer's judgment of whether the reviewed artifacts can be built *together*:
   - `feasible` — no blocking cross-artifact conflicts; the artifacts can be built together as specified.
   - `needs-revision` — addressable cross-artifact conflicts remain; the producer revises the artifacts and the review re-runs. A routine revision cycle.
   - `infeasible` — a fundamental conflict that routine revision cannot resolve; it requires a **business-level decision / escalation** (e.g. relax a hard constraint, change a technology decision, or rescope). This is a **different consequence** than `needs-revision` — do **not** collapse `infeasible` into `needs-revision`, into `feasible`, or into a generic FAIL. The escalation branch is the entire reason the third state exists.

3. **Severity levels**: `Critical` (blocking — the system cannot be built until this is resolved) | `Important` (a real conflict that should be resolved but is not strictly blocking). Severity triages which concerns surface first; it is distinct from **Impact** (the design consequence) — both are recorded per issue.

4. **The three issue taxonomies** — each surfaces a different class of cross-artifact conflict, none of them visible inside any single artifact:
   - **Cross-Artifact Contradictions** — impossible combinations spanning two artifacts.
   - **Constraint-Decision Conflicts** — a technology decision that violates a stated hard constraint.
   - **NFR-Constraint Impossibilities** — a quality target unachievable given the stated constraints or chosen technologies.
   Keep the three distinct — this report is cross-artifact *feasibility* (can these pieces be built together?), separate from intra-artifact *completeness*, which is the completeness reviewer's territory.

5. **Per-issue gate fuel — the four fields the human gate reads.** Every issue row, in all three tables, supplies the same four fields the command lead reads straight into its human gate, one question per issue. The columns are named to align one-to-one with those gate fields:
   - **Description** → the concern itself
   - **Evidence** → the artifacts / IDs cited (the conflicting pair shown as `A ↔ B`)
   - **Impact** → the consequence on the design (distinct from Severity)
   - **Suggested Resolution** → the proposed fix or escalation
   Keep all four populated for every issue — the gate renders each field by name, and dropping any one starves it. The row `#` (with its taxonomy) serves as the issue id for the gate's header.

6. **Routing and write location belong to the command lead, not this artifact.** The verdict drives the command's feasibility review loop (proceed on `feasible` / revise on `needs-revision` / escalate for a business decision on `infeasible`) and the ordering of feasibility-before-completeness — that routing lives in the command lead, never baked into this report. The filled report is seeded, written, and collected by the lead at `.mochiko/specs/<feature>/feasibility-report.md`; this template stays path-agnostic.

7. **This is a reference template** — the feasibility reviewer fills in actual content following this structure.
