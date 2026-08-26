# Strip notes — `templates/feasibility-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: machine-first YAML, strengths → one-line field).

## [v0.91.0] Plan-package wording re-scoped to the design phase; `requirements.md` leaves the reviewed set

- **Disposition:** superseded → the design-phase framing. `/mochiko:plan` and its accepted
  package no longer exist, so every clause keying the report to "the package" named a dead
  surface; `requirements.md` is dead as a mandatory artifact and could no longer head the
  `artifacts_reviewed` example. Four sites re-worded, nothing else touched.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 (plan retires; the design phase is the
  producing home), D3 (`requirements.md` dies as a mandatory artifact — its gap-finding-fence slot
  re-keys to the sufficiency report plus the design-phase deltas), D4 (dead gates, and the
  sufficiency report as the durable assessment record), and D5 (`review-feasibility` re-scopes at
  build time to grade the design-phase output); `DECISIONS.md` 2026-08-26 row.)
- **Content (superseded, verbatim, four sites):**
  (1) The opening line: "The feasibility reviewer's cross-artifact critique of a feature's
  analysis and design artifacts". Now: "of a feature's design-phase output".
  (2) The `artifacts_reviewed` frontmatter example: `[requirements.md,
  constraints-and-decisions.md]`. Now: `[sufficiency-report.md, constraints-and-decisions.md]`.
  (3) The `hunt_coverage` inline comment's condition: "a1–a3 present iff the package carries a
  store delta". Now: "iff the design phase produced a store delta".
  (4) Usage Note 6's matching condition: "`a1`–`a3` exactly when the package carries a store
  delta". Now: "exactly when the design phase produced a store delta".
- **Kept deliberately:** everything else — the machine-first envelope, frontmatter-only clean
  reviews, the three-state verdict and its escalation branch, the four mandatory gate-fuel fields,
  the three distinct taxonomies, the `hunt_coverage` proof-of-hunt floor and its bounce rule, and
  Note 4's path-agnostic stance. The re-scope changed *which run's artifacts* are reviewed, never
  the report's shape or the reviewer's bar. The store-delta conditional itself survives — only the
  surface it names moved from the package to the design phase.
- **Consumers assessed:** `mochiko:review-feasibility` fills this template and is re-scoped in the
  same wave (record D5) — the sites re-worded here are the ones its brief and its disclosure floor
  reference. The `commands/plan.md` Report-envelope bullet that bound this template by path is
  deleted in this same wave; the binding re-appears in the `implement.md` rewrite's Reports tool
  (record D4, Build surface). `templates/report-format.md` is untouched: these are frontmatter and
  usage-note edits, no prose section added or removed.


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
