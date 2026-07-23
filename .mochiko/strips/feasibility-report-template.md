# Strip notes — `templates/feasibility-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: machine-first YAML, strengths → one-line field).

## [v0.22.0] Three issue tables → taxonomy-keyed findings YAML; Strengths Noted → `strengths:` field
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2): consumed by the lead's verdict + the human gate's per-issue rendering; no downstream reads
- **Content:** the three separate markdown tables (Cross-Artifact Contradictions · Constraint-Decision Conflicts · NFR-Constraint Impossibilities, each Description/Evidence/Impact/Severity/Suggested-Resolution + an "If none" line) → one `findings:` list with a `taxonomy:` key preserving the three classes; the `## Verdict` prose block → `verdict:` + `verdict_basis:`; `## Strengths Noted` bullets → `strengths:` one-liner; `## Artifacts Reviewed` prose → the `artifacts_reviewed:` list. Preserved intact: the three-state verdict with the never-flatten-`infeasible` warning, all four per-issue gate-fuel fields (`gap`/`at`/`impact`/`fix`), severity-vs-impact distinction, lead-owned routing.
