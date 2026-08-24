# Strip notes — `templates/feasibility-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: machine-first YAML, strengths → one-line field).

## [v0.82.0] `hunt_coverage` field added; Usage Note 5 clean-review set widened (audit fix 1)

- **Disposition:** superseded (Note 5) + addition riding the decision row (the field, Note 6).
  `review-feasibility`'s new one-line-per-class disclosure floor had no envelope-legal home —
  the template's frontmatter could not carry it and Note 5's frontmatter-only clause plus
  report-format rule 9 would have had the lead bounce any report holding it (v0.82.0 audit
  finding 1). The bounded field is now the proof-of-hunt's only home.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`; `DECISIONS.md`
  2026-08-22 row).
- **Content (superseded, verbatim):** Note 5's closing clause `— \`verdict_basis\` and
  \`strengths\` still filled.` (now also names `hunt_coverage`).
- **Kept deliberately:** the whole machine-first envelope — frontmatter-only clean reviews,
  the four gate-fuel fields, the three-state verdict notes — unchanged.
- **Consumers assessed:** `review-feasibility` (fills it — its floor mandates the disclosure
  and binds this template by path; the field's grammar lives here); `commands/plan.md`'s
  Report envelope bullet (binds by path); `report-format.md` rule 2 (the field is
  frontmatter, not a prose section — no sanction needed).

## [v0.22.0] Three issue tables → taxonomy-keyed findings YAML; Strengths Noted → `strengths:` field
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2): consumed by the lead's verdict + the human gate's per-issue rendering; no downstream reads
- **Content:** the three separate markdown tables (Cross-Artifact Contradictions · Constraint-Decision Conflicts · NFR-Constraint Impossibilities, each Description/Evidence/Impact/Severity/Suggested-Resolution + an "If none" line) → one `findings:` list with a `taxonomy:` key preserving the three classes; the `## Verdict` prose block → `verdict:` + `verdict_basis:`; `## Strengths Noted` bullets → `strengths:` one-liner; `## Artifacts Reviewed` prose → the `artifacts_reviewed:` list. Preserved intact: the three-state verdict with the never-flatten-`infeasible` warning, all four per-issue gate-fuel fields (`gap`/`at`/`impact`/`fix`), severity-vs-impact distinction, lead-owned routing.
