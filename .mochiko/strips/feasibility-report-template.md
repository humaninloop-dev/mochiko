# Strip notes — `templates/feasibility-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: machine-first YAML, strengths → one-line field).

## [v0.77.0] Template superseded by schema — `schemas/feasibility-report.yaml` + `mochiko-cli template feasibility-report`
- **Disposition:** superseded → `schemas/feasibility-report.yaml` + `mochiko-cli template feasibility-report` (D8 raw-Read fallback when the binary is absent)
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class A seat report templates per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters (NOT prose chapters); the Usage Notes fold into the per-section `contract:` + the `overview:`; `skeleton:` = the frontmatter block (feasibility is frontmatter-only on a clean review — no sanctioned prose block; the skeleton is the frontmatter block alone); `form: report-format.md` stays the single home for the shared report doctrine (register / findings-schema / conditional-prose / no-self-verdict) — pointer only, NOT restated as per-section checks. The `--check` lines are authored NET-NEW (reports have no validator/checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
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
~~~
- **Kept deliberately:** nothing dropped — every frontmatter field and every Usage Note has a home in the schema (fields in `skeleton:` + section `contract:`; notes folded into `contract:`/`overview:`). The `form: report-format.md` envelope pointer is preserved. The three-state verdict with the never-flatten-`infeasible` warning, the three distinct taxonomies, and all four gate-fuel fields (`gap`/`at`/`impact`/`fix`) survive in the Verdict + Findings contracts + checks.
- **Consumers assessed:** router row `skills/mochiko/SKILL.md:87` (feasibility). No other static consumer named (plan §4). Re-points are owned by seat P5 (router/consumer rows, two-arm `mochiko-cli template feasibility-report; if absent Read plugins/mochiko/schemas/feasibility-report.yaml`) — NOT this seat; each re-pointed consumer appends its own supersession strip.

## [v0.22.0] Three issue tables → taxonomy-keyed findings YAML; Strengths Noted → `strengths:` field
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2): consumed by the lead's verdict + the human gate's per-issue rendering; no downstream reads
- **Content:** the three separate markdown tables (Cross-Artifact Contradictions · Constraint-Decision Conflicts · NFR-Constraint Impossibilities, each Description/Evidence/Impact/Severity/Suggested-Resolution + an "If none" line) → one `findings:` list with a `taxonomy:` key preserving the three classes; the `## Verdict` prose block → `verdict:` + `verdict_basis:`; `## Strengths Noted` bullets → `strengths:` one-liner; `## Artifacts Reviewed` prose → the `artifacts_reviewed:` list. Preserved intact: the three-state verdict with the never-flatten-`infeasible` warning, all four per-issue gate-fuel fields (`gap`/`at`/`impact`/`fix`), severity-vs-impact distinction, lead-owned routing.
