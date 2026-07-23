# Feasibility Review Report Template

The feasibility reviewer's cross-artifact critique of a feature's analysis and design
artifacts — the conflicts no single artifact reveals in isolation, a three-state verdict,
and the per-issue resolution fuel the human gate reads. Envelope + shared rules:
`templates/report-format.md` — this file carries only the feasibility payload.

---

```markdown
---
report: feasibility
feature: {{feature_id}}
round: {{round}}
artifacts_reviewed: [requirements.md, constraints-and-decisions.md, nfrs.md]   # + data-model.md, contracts/api.yaml on a design review
verdict: feasible | needs-revision | infeasible
verdict_basis: "{{one line — what drives the verdict}}"
strengths: "{{one line, comma-brief}}"
findings:
  - {id: F1,
     taxonomy: cross-artifact | constraint-decision | nfr-impossibility,
     sev: Critical | Important,
     at: "{{the conflicting pair, shown as A ↔ B with IDs (e.g. C-003 ↔ D-007)}}",
     gap: "{{the conflict, one line}}",
     impact: "{{the design consequence, one line}}",
     fix: "{{proposed resolution or escalation, one line}}"}
---
```

---

## Usage Notes

1. **Verdict values** — whether the reviewed artifacts can be built *together*:
   - `feasible` — no blocking cross-artifact conflicts.
   - `needs-revision` — addressable conflicts; the producer revises and the review re-runs.
     A routine revision cycle.
   - `infeasible` — a fundamental conflict routine revision cannot resolve; it requires a
     **business-level decision / escalation** (relax a hard constraint, change a technology
     decision, or rescope). A **different consequence** than `needs-revision` — do **not**
     collapse `infeasible` into `needs-revision`, into `feasible`, or into a generic FAIL.
     The escalation branch is the entire reason the third state exists.
2. **The three taxonomies stay distinct** (the `taxonomy:` key) — each surfaces a different
   class of conflict invisible inside any single artifact: **cross-artifact** (impossible
   combinations spanning two artifacts) · **constraint-decision** (a technology decision
   violating a stated hard constraint) · **nfr-impossibility** (a quality target
   unachievable given the constraints or chosen technologies). This report is cross-artifact
   *feasibility*; intra-artifact *completeness* is the completeness reviewer's territory.
3. **Per-issue gate fuel — four fields the human gate renders by name, all mandatory:**
   `gap` (the concern) · `at` (the evidence — the conflicting pair as `A ↔ B` with IDs) ·
   `impact` (the design consequence, distinct from `sev`) · `fix` (the proposed resolution
   or escalation). Dropping any one starves the gate. `sev` triages which concerns surface
   first: `Critical` (cannot be built until resolved) | `Important` (real, not strictly
   blocking).
4. **Routing and write location belong to the command lead, not this artifact** — what
   happens on each verdict (proceed / revise / escalate) lives in the lead's loop; the
   filled report is seeded, written, and collected by the lead. This template stays
   path-agnostic.
5. **A clean review** (`verdict: feasible`, `findings: []`) is frontmatter-only —
   `verdict_basis` and `strengths` still filled.
