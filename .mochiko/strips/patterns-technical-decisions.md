# Strip notes — `skills/patterns-technical-decisions/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 3 (patterns-\* cluster,
batch-3 ratified 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
First strip assessment of this skill (never-stripped band 30–70): body 161 → 89 lines = **45%**,
in-band.

## [v0.46.0] loop-discipline pointer out
- **Disposition:** superseded → the sentence stands without the citation
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "...not to this skill (see `mochiko:loop-discipline`)." → "...not to this skill."
- **Consumers assessed:** plan command briefs unchanged.

## [v0.27.0] Quick-layer restatements and fully-homed Common Mistakes stripped (body 161 → 89, −45%)
- **Disposition:** relocated → verified reference homes, each Read before landing: the
  Quick-Criteria table → `references/EVALUATION-MATRIX.md` Evaluation Criteria (same 8
  criteria, richer — Description + Questions-to-Ask columns; the criterion names stay in-body
  as a one-line enumeration) · the Quick-Comparison table → the reference's two canonical
  shapes (weighted decision matrix + side-by-side options comparison) — the in-body table was
  a divergent *third* shape for the same function, and the pointer now says "don't invent a
  third format" (drift repaired) · the Quick-Decision-Record block →
  `references/DECISION-RECORD.md` Decision Record Format (fuller — adds Alternatives
  Considered + structured Consequences; the field enumeration stays in-body as one line) ·
  the Brownfield-Alignment table → EVALUATION-MATRIX Alignment Scoring (verbatim identical,
  all 4 rows; the check-existing-stack-first sentence stays with the pointer) · deleted
  (Tier 1): all seven Common-Mistakes rows, each homed — single-option-evaluation (Phase-1
  "2-3 alternatives minimum" + checklist), shiny-object (Alignment Scoring strong-justification
  row + Brownfield Questions "prefer extension over addition"), vague-rationale
  (DECISION-RECORD Good/Bad Rationale — near-verbatim JWT material), ignoring-team-skills
  (Team Familiarity criterion + the Criteria-Weights worked example), missing-trade-offs
  (checklist + Phase-2 + Consequence Documentation), orphan-decisions (checklist + Dependency
  Documentation, D2-depends-on-D1 example verbatim), governance-blindness (checklist +
  Governance Alignment section)
- **Tier failed:** 1 throughout — every stripped block restated a verified reference home or
  in-file statement; no Tier-2 deletions
- **Content:** four quick-layer blocks (criteria table, comparison table, mini-ADR, alignment
  table) and the seven mistake subsections; nothing written to `templates/` — D4's destination
  ban not engaged
- **Consumers assessed:** wave-open enumeration — 8 citing files, none references the stripped
  blocks or a section anchor. Kept: the Decision Workflow spine, the NEEDS CLARIFICATION
  paragraph (unique boundary with `mochiko:loop-discipline` — marking is this skill's job,
  driving resolution is the supervisor's), the "Where decisions are recorded" ownership section
  + boundary table (the SKILL-side statement is fuller than DECISION-RECORD's point-of-use
  paragraph — each sits at its own point of use, not duplication-only), Quality Checklist,
  both When-to sections. Reference audit: EVALUATION-MATRIX's category tables carry an
  explicit illustrative-only disclaimer; DECISION-RECORD's artifact-boundary paragraph is
  correctly scoped — both clean, untouched. Session ruling: wave-3 batch-3 ratified 2026-07-25.
